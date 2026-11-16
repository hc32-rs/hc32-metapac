
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40040000,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v33",
                block: "ADC",
                ir: &adc_v33::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_ADC1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EOCA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    448,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    449,
                ),
            },
            PeripheralDmaChannel {
                signal: "CHCMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    450,
                ),
            },
            PeripheralDmaChannel {
                signal: "SEQCMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    451,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    448,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    449,
                ),
            },
            PeripheralDmaChannel {
                signal: "CHCMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    450,
                ),
            },
            PeripheralDmaChannel {
                signal: "SEQCMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    451,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "ADC2",
        address: 0x40040400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v34",
                block: "ADC",
                ir: &adc_v34::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_ADC2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EOCA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    452,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    453,
                ),
            },
            PeripheralDmaChannel {
                signal: "CHCMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    454,
                ),
            },
            PeripheralDmaChannel {
                signal: "SEQCMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    455,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    452,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    453,
                ),
            },
            PeripheralDmaChannel {
                signal: "CHCMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    454,
                ),
            },
            PeripheralDmaChannel {
                signal: "SEQCMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    455,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "AES",
        address: 0x40008000,
        registers: Some(
            PeripheralRegisters {
                kind: "aes",
                version: "v5",
                block: "AES",
                ir: &aes_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_AES",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "AOS",
        address: 0x40010800,
        registers: Some(
            PeripheralRegisters {
                kind: "aos",
                version: "v2",
                block: "AOS",
                ir: &aos_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_AOS",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "STRG",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    319,
                ),
            },
            PeripheralDmaChannel {
                signal: "STRG",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    319,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CAN",
        address: 0x40070400,
        registers: Some(
            PeripheralRegisters {
                kind: "can",
                version: "v3",
                block: "CAN",
                ir: &can_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_CAN",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP1",
        address: 0x4004a000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v5",
                block: "CMP",
                ir: &cmp_v5::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    416,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    416,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP2",
        address: 0x4004a000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v5",
                block: "CMP",
                ir: &cmp_v5::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMP2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    417,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    417,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP3",
        address: 0x4004a000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v5",
                block: "CMP",
                ir: &cmp_v5::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMP3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    418,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    418,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP_COMMON",
        address: 0x4004a000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v6",
                block: "CMP",
                ir: &cmp_v6::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CMU",
        address: 0x40054000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmu",
                version: "v2",
                block: "CMU",
                ir: &cmu_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 0x40008c00,
        registers: Some(
            PeripheralRegisters {
                kind: "crc",
                version: "v7",
                block: "CRC",
                ir: &crc_v7::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_CRC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGC",
        address: 0xe0042000,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgc",
                version: "v2",
                block: "DBGC",
                ir: &dbgc_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DCU1",
        address: 0x40052000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v2",
                block: "DCU",
                ir: &dcu_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_DCU1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCU1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    55,
                ),
            },
            PeripheralDmaChannel {
                signal: "DCU1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    55,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "DCU2",
        address: 0x40052000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v2",
                block: "DCU",
                ir: &dcu_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_DCU2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCU2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    56,
                ),
            },
            PeripheralDmaChannel {
                signal: "DCU2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    56,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "DCU3",
        address: 0x40052000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v2",
                block: "DCU",
                ir: &dcu_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_DCU3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCU3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    57,
                ),
            },
            PeripheralDmaChannel {
                signal: "DCU3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    57,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "DCU4",
        address: 0x40052000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v2",
                block: "DCU",
                ir: &dcu_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_DCU4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCU4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    58,
                ),
            },
            PeripheralDmaChannel {
                signal: "DCU4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    58,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA1",
        address: 0x40053000,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "v2",
                block: "DMA",
                ir: &dma_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_DMA1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TC0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    32,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    33,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    34,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    35,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    40,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    41,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    42,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    43,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    32,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    33,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    34,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    35,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    40,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    41,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    42,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    43,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA2",
        address: 0x40053000,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "v2",
                block: "DMA",
                ir: &dma_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_DMA2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TC0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    36,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    37,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    38,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    39,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    44,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    45,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    46,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    47,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    36,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    37,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    38,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    39,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    44,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    45,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    46,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    47,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "EFM",
        address: 0x40010400,
        registers: Some(
            PeripheralRegisters {
                kind: "efm",
                version: "v2",
                block: "EFM",
                ir: &efm_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OPTEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    52,
                ),
            },
            PeripheralDmaChannel {
                signal: "OPTEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    52,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB0",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v3",
                block: "EMB",
                ir: &emb_v3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB1",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v3",
                block: "EMB",
                ir: &emb_v3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB2",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v3",
                block: "EMB",
                ir: &emb_v3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB3",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v3",
                block: "EMB",
                ir: &emb_v3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "FCM",
        address: 0x40048400,
        registers: Some(
            PeripheralRegisters {
                kind: "fcm",
                version: "v1",
                block: "FCM",
                ir: &fcm_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_FCM",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40053800,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v8",
                block: "GPIO",
                ir: &gpio_v8::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "HASH",
        address: 0x40008400,
        registers: Some(
            PeripheralRegisters {
                kind: "hash",
                version: "v1",
                block: "HASH",
                ir: &hash_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_HASH",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C1",
        address: 0x4004e000,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v18",
                block: "I",
                ir: &i_v18::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_I2C1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    420,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    421,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    422,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    423,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    420,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    421,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    422,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    423,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C2",
        address: 0x4004e000,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v18",
                block: "I",
                ir: &i_v18::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_I2C2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    424,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    425,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    426,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    427,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    424,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    425,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    426,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    427,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C3",
        address: 0x4004e000,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v18",
                block: "I",
                ir: &i_v18::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_I2C3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    428,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    429,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    430,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    431,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    428,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    429,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    430,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    431,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "I2S1",
        address: 0x4001e000,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v19",
                block: "I",
                ir: &i_v19::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_I2S1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TXIRQOUT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    400,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    401,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    400,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    401,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "I2S2",
        address: 0x4001e000,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v19",
                block: "I",
                ir: &i_v19::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_I2S2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TXIRQOUT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    403,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    404,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    403,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    404,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "I2S3",
        address: 0x4001e000,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v19",
                block: "I",
                ir: &i_v19::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_I2S3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TXIRQOUT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    406,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    407,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    406,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    407,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "I2S4",
        address: 0x4001e000,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v19",
                block: "I",
                ir: &i_v19::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_I2S4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TXIRQOUT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    409,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    410,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    409,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    410,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "ICG",
        address: 0x400,
        registers: Some(
            PeripheralRegisters {
                kind: "icg",
                version: "v2",
                block: "ICG",
                ir: &icg_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "INTC",
        address: 0x40051000,
        registers: Some(
            PeripheralRegisters {
                kind: "intc",
                version: "v2",
                block: "INTC",
                ir: &intc_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "KEYSCAN",
        address: 0x40050c00,
        registers: Some(
            PeripheralRegisters {
                kind: "keyscan",
                version: "common",
                block: "KEYSCAN",
                ir: &keyscan_common::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "MPU",
        address: 0x40050000,
        registers: Some(
            PeripheralRegisters {
                kind: "mpu",
                version: "v2",
                block: "MPU",
                ir: &mpu_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OTS",
        address: 0x4004a400,
        registers: Some(
            PeripheralRegisters {
                kind: "ots",
                version: "v1",
                block: "OTS",
                ir: &ots_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_OTS",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OTS",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    435,
                ),
            },
            PeripheralDmaChannel {
                signal: "OTS",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    435,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "PERIC",
        address: 0x40055400,
        registers: Some(
            PeripheralRegisters {
                kind: "peric",
                version: "v2",
                block: "PERIC",
                ir: &peric_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWC",
        address: 0x40048000,
        registers: Some(
            PeripheralRegisters {
                kind: "pwc",
                version: "v2",
                block: "PWC",
                ir: &pwc_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "QSPI",
        address: 0x9c000000,
        registers: Some(
            PeripheralRegisters {
                kind: "qspi",
                version: "v2",
                block: "QSPI",
                ir: &qspi_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_QSPI",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RMU",
        address: 0x400540c0,
        registers: Some(
            PeripheralRegisters {
                kind: "rmu",
                version: "v2",
                block: "RMU",
                ir: &rmu_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0x4004c000,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "v7",
                block: "RTC",
                ir: &rtc_v7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ALM",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    81,
                ),
            },
            PeripheralDmaChannel {
                signal: "PRD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    82,
                ),
            },
            PeripheralDmaChannel {
                signal: "ALM",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    81,
                ),
            },
            PeripheralDmaChannel {
                signal: "PRD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    82,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SDIOC1",
        address: 0x4006fc00,
        registers: Some(
            PeripheralRegisters {
                kind: "sdioc",
                version: "common",
                block: "SDIOC",
                ir: &sdioc_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_SDIOC1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DMAR",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    480,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAW",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    481,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAR",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    480,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAW",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    481,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SDIOC2",
        address: 0x4006fc00,
        registers: Some(
            PeripheralRegisters {
                kind: "sdioc",
                version: "common",
                block: "SDIOC",
                ir: &sdioc_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_SDIOC2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DMAR",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    483,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAW",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    484,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAR",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    483,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAW",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    484,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI1",
        address: 0x4001c000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v8",
                block: "SPI",
                ir: &spi_v8::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_SPI1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    299,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    300,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    301,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    302,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    303,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    299,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    300,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    301,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    302,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    303,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI2",
        address: 0x4001c000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v8",
                block: "SPI",
                ir: &spi_v8::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_SPI2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    304,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    305,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    306,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    307,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    308,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    304,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    305,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    306,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    307,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    308,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI3",
        address: 0x4001c000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v8",
                block: "SPI",
                ir: &spi_v8::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_SPI3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    309,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    310,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    311,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    312,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    313,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    309,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    310,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    311,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    312,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    313,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI4",
        address: 0x4001c000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v8",
                block: "SPI",
                ir: &spi_v8::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_SPI4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    314,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    315,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    316,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    317,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    318,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    314,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    315,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    316,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    317,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    318,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SRAMC",
        address: 0x40050800,
        registers: Some(
            PeripheralRegisters {
                kind: "sramc",
                version: "v2",
                block: "SRAMC",
                ir: &sramc_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SWDT",
        address: 0x40049400,
        registers: Some(
            PeripheralRegisters {
                kind: "swdt",
                version: "v2",
                block: "SWDT",
                ir: &swdt_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR01",
        address: 0x40024000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v7",
                block: "TMR",
                ir: &tmr_v7::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMER0_1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    64,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    65,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    64,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    65,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR02",
        address: 0x40024000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v7",
                block: "TMR",
                ir: &tmr_v7::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMER0_2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    66,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    67,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    66,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    67,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR41",
        address: 0x40017000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v8",
                block: "TMR",
                ir: &tmr_v8::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMER4_1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    368,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    369,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    370,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    371,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    372,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    373,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    368,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    369,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    370,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    371,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    372,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    373,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR42",
        address: 0x40017000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v8",
                block: "TMR",
                ir: &tmr_v8::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMER4_2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    374,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    375,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    376,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    377,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    378,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    379,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    374,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    375,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    376,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    377,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    378,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    379,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR43",
        address: 0x40017000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v8",
                block: "TMR",
                ir: &tmr_v8::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMER4_3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    384,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    385,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    386,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    387,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    388,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    389,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    384,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    385,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    386,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    387,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    388,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    389,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR4_ECER",
        address: 0x40055408,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v9",
                block: "TMR",
                ir: &tmr_v9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR61",
        address: 0x40018000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v10",
                block: "TMR",
                ir: &tmr_v10::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMER6_1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    96,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    97,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    98,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    99,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    100,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    101,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    102,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    103,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    107,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    108,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    96,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    97,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    98,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    99,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    100,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    101,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    102,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    103,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    107,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    108,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR62",
        address: 0x40018000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v10",
                block: "TMR",
                ir: &tmr_v10::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMER6_2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    112,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    113,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    114,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    115,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    116,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    117,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    118,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    119,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    123,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    124,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    112,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    113,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    114,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    115,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    116,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    117,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    118,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    119,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    123,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    124,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR63",
        address: 0x40018000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v10",
                block: "TMR",
                ir: &tmr_v10::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMER6_3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    128,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    129,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    130,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    131,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    132,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    133,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    134,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    135,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    139,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    140,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    128,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    129,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    130,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    131,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    132,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    133,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    134,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    135,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    139,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    140,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR6_COMMON",
        address: 0x40018300,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v11",
                block: "TMR",
                ir: &tmr_v11::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA1",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v2",
                block: "TMRA",
                ir: &tmra_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMERA_1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    256,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    257,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    258,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    256,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    257,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    258,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA2",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v2",
                block: "TMRA",
                ir: &tmra_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMERA_2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    259,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    260,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    261,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    259,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    260,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    261,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA3",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v2",
                block: "TMRA",
                ir: &tmra_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMERA_3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    262,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    263,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    264,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    262,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    263,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    264,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA4",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v2",
                block: "TMRA",
                ir: &tmra_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMERA_4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    265,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    266,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    267,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    265,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    266,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    267,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA5",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v2",
                block: "TMRA",
                ir: &tmra_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMERA_5",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    268,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    269,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    270,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    268,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    269,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    270,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA6",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v2",
                block: "TMRA",
                ir: &tmra_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TIMERA_6",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    272,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    273,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    274,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    272,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    273,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    274,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TRNG",
        address: 0x40041000,
        registers: Some(
            PeripheralRegisters {
                kind: "trng",
                version: "v3",
                block: "TRNG",
                ir: &trng_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_TRNG",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "END",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    456,
                ),
            },
            PeripheralDmaChannel {
                signal: "END",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    456,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART1",
        address: 0x4001d000,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v3",
                block: "USART",
                ir: &usart_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_USART1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    278,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    279,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    280,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    281,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    282,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    278,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    279,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    280,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    281,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    282,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART2",
        address: 0x4001d000,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v3",
                block: "USART",
                ir: &usart_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_USART2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    283,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    284,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    285,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    286,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    287,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    283,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    284,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    285,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    286,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    287,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART3",
        address: 0x4001d000,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v3",
                block: "USART",
                ir: &usart_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_USART3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    288,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    289,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    290,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    291,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    292,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    288,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    289,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    290,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    291,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    292,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART4",
        address: 0x4001d000,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v3",
                block: "USART",
                ir: &usart_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_USART4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    293,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    294,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    295,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    296,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    297,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    293,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    294,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    295,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    296,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    297,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USBFS",
        address: 0x400c0000,
        registers: Some(
            PeripheralRegisters {
                kind: "usbfs",
                version: "v1",
                block: "USBFS",
                ir: &usbfs_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_USBFS",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SOF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    53,
                ),
            },
            PeripheralDmaChannel {
                signal: "SOF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    53,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "WDT",
        address: 0x40049000,
        registers: Some(
            PeripheralRegisters {
                kind: "wdt",
                version: "common",
                block: "WDT",
                ir: &wdt_common::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "REFUDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    439,
                ),
            },
            PeripheralDmaChannel {
                signal: "REFUDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    439,
                ),
            },
        ],
        interrupts: &[],
    },
];
                pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "INT000",
        number: 0,
    },
    Interrupt {
        name: "INT001",
        number: 1,
    },
    Interrupt {
        name: "INT002",
        number: 2,
    },
    Interrupt {
        name: "INT003",
        number: 3,
    },
    Interrupt {
        name: "INT004",
        number: 4,
    },
    Interrupt {
        name: "INT005",
        number: 5,
    },
    Interrupt {
        name: "INT006",
        number: 6,
    },
    Interrupt {
        name: "INT007",
        number: 7,
    },
    Interrupt {
        name: "INT008",
        number: 8,
    },
    Interrupt {
        name: "INT009",
        number: 9,
    },
    Interrupt {
        name: "INT010",
        number: 10,
    },
    Interrupt {
        name: "INT011",
        number: 11,
    },
    Interrupt {
        name: "INT012",
        number: 12,
    },
    Interrupt {
        name: "INT013",
        number: 13,
    },
    Interrupt {
        name: "INT014",
        number: 14,
    },
    Interrupt {
        name: "INT015",
        number: 15,
    },
    Interrupt {
        name: "INT016",
        number: 16,
    },
    Interrupt {
        name: "INT017",
        number: 17,
    },
    Interrupt {
        name: "INT018",
        number: 18,
    },
    Interrupt {
        name: "INT019",
        number: 19,
    },
    Interrupt {
        name: "INT020",
        number: 20,
    },
    Interrupt {
        name: "INT021",
        number: 21,
    },
    Interrupt {
        name: "INT022",
        number: 22,
    },
    Interrupt {
        name: "INT023",
        number: 23,
    },
    Interrupt {
        name: "INT024",
        number: 24,
    },
    Interrupt {
        name: "INT025",
        number: 25,
    },
    Interrupt {
        name: "INT026",
        number: 26,
    },
    Interrupt {
        name: "INT027",
        number: 27,
    },
    Interrupt {
        name: "INT028",
        number: 28,
    },
    Interrupt {
        name: "INT029",
        number: 29,
    },
    Interrupt {
        name: "INT030",
        number: 30,
    },
    Interrupt {
        name: "INT031",
        number: 31,
    },
    Interrupt {
        name: "INT032",
        number: 32,
    },
    Interrupt {
        name: "INT033",
        number: 33,
    },
    Interrupt {
        name: "INT034",
        number: 34,
    },
    Interrupt {
        name: "INT035",
        number: 35,
    },
    Interrupt {
        name: "INT036",
        number: 36,
    },
    Interrupt {
        name: "INT037",
        number: 37,
    },
    Interrupt {
        name: "INT038",
        number: 38,
    },
    Interrupt {
        name: "INT039",
        number: 39,
    },
    Interrupt {
        name: "INT040",
        number: 40,
    },
    Interrupt {
        name: "INT041",
        number: 41,
    },
    Interrupt {
        name: "INT042",
        number: 42,
    },
    Interrupt {
        name: "INT043",
        number: 43,
    },
    Interrupt {
        name: "INT044",
        number: 44,
    },
    Interrupt {
        name: "INT045",
        number: 45,
    },
    Interrupt {
        name: "INT046",
        number: 46,
    },
    Interrupt {
        name: "INT047",
        number: 47,
    },
    Interrupt {
        name: "INT048",
        number: 48,
    },
    Interrupt {
        name: "INT049",
        number: 49,
    },
    Interrupt {
        name: "INT050",
        number: 50,
    },
    Interrupt {
        name: "INT051",
        number: 51,
    },
    Interrupt {
        name: "INT052",
        number: 52,
    },
    Interrupt {
        name: "INT053",
        number: 53,
    },
    Interrupt {
        name: "INT054",
        number: 54,
    },
    Interrupt {
        name: "INT055",
        number: 55,
    },
    Interrupt {
        name: "INT056",
        number: 56,
    },
    Interrupt {
        name: "INT057",
        number: 57,
    },
    Interrupt {
        name: "INT058",
        number: 58,
    },
    Interrupt {
        name: "INT059",
        number: 59,
    },
    Interrupt {
        name: "INT060",
        number: 60,
    },
    Interrupt {
        name: "INT061",
        number: 61,
    },
    Interrupt {
        name: "INT062",
        number: 62,
    },
    Interrupt {
        name: "INT063",
        number: 63,
    },
    Interrupt {
        name: "INT064",
        number: 64,
    },
    Interrupt {
        name: "INT065",
        number: 65,
    },
    Interrupt {
        name: "INT066",
        number: 66,
    },
    Interrupt {
        name: "INT067",
        number: 67,
    },
    Interrupt {
        name: "INT068",
        number: 68,
    },
    Interrupt {
        name: "INT069",
        number: 69,
    },
    Interrupt {
        name: "INT070",
        number: 70,
    },
    Interrupt {
        name: "INT071",
        number: 71,
    },
    Interrupt {
        name: "INT072",
        number: 72,
    },
    Interrupt {
        name: "INT073",
        number: 73,
    },
    Interrupt {
        name: "INT074",
        number: 74,
    },
    Interrupt {
        name: "INT075",
        number: 75,
    },
    Interrupt {
        name: "INT076",
        number: 76,
    },
    Interrupt {
        name: "INT077",
        number: 77,
    },
    Interrupt {
        name: "INT078",
        number: 78,
    },
    Interrupt {
        name: "INT079",
        number: 79,
    },
    Interrupt {
        name: "INT080",
        number: 80,
    },
    Interrupt {
        name: "INT081",
        number: 81,
    },
    Interrupt {
        name: "INT082",
        number: 82,
    },
    Interrupt {
        name: "INT083",
        number: 83,
    },
    Interrupt {
        name: "INT084",
        number: 84,
    },
    Interrupt {
        name: "INT085",
        number: 85,
    },
    Interrupt {
        name: "INT086",
        number: 86,
    },
    Interrupt {
        name: "INT087",
        number: 87,
    },
    Interrupt {
        name: "INT088",
        number: 88,
    },
    Interrupt {
        name: "INT089",
        number: 89,
    },
    Interrupt {
        name: "INT090",
        number: 90,
    },
    Interrupt {
        name: "INT091",
        number: 91,
    },
    Interrupt {
        name: "INT092",
        number: 92,
    },
    Interrupt {
        name: "INT093",
        number: 93,
    },
    Interrupt {
        name: "INT094",
        number: 94,
    },
    Interrupt {
        name: "INT095",
        number: 95,
    },
    Interrupt {
        name: "INT096",
        number: 96,
    },
    Interrupt {
        name: "INT097",
        number: 97,
    },
    Interrupt {
        name: "INT098",
        number: 98,
    },
    Interrupt {
        name: "INT099",
        number: 99,
    },
    Interrupt {
        name: "INT100",
        number: 100,
    },
    Interrupt {
        name: "INT101",
        number: 101,
    },
    Interrupt {
        name: "INT102",
        number: 102,
    },
    Interrupt {
        name: "INT103",
        number: 103,
    },
    Interrupt {
        name: "INT104",
        number: 104,
    },
    Interrupt {
        name: "INT105",
        number: 105,
    },
    Interrupt {
        name: "INT106",
        number: 106,
    },
    Interrupt {
        name: "INT107",
        number: 107,
    },
    Interrupt {
        name: "INT108",
        number: 108,
    },
    Interrupt {
        name: "INT109",
        number: 109,
    },
    Interrupt {
        name: "INT110",
        number: 110,
    },
    Interrupt {
        name: "INT111",
        number: 111,
    },
    Interrupt {
        name: "INT112",
        number: 112,
    },
    Interrupt {
        name: "INT113",
        number: 113,
    },
    Interrupt {
        name: "INT114",
        number: 114,
    },
    Interrupt {
        name: "INT115",
        number: 115,
    },
    Interrupt {
        name: "INT116",
        number: 116,
    },
    Interrupt {
        name: "INT117",
        number: 117,
    },
    Interrupt {
        name: "INT118",
        number: 118,
    },
    Interrupt {
        name: "INT119",
        number: 119,
    },
    Interrupt {
        name: "INT120",
        number: 120,
    },
    Interrupt {
        name: "INT121",
        number: 121,
    },
    Interrupt {
        name: "INT122",
        number: 122,
    },
    Interrupt {
        name: "INT123",
        number: 123,
    },
    Interrupt {
        name: "INT124",
        number: 124,
    },
    Interrupt {
        name: "INT125",
        number: 125,
    },
    Interrupt {
        name: "INT126",
        number: 126,
    },
    Interrupt {
        name: "INT127",
        number: 127,
    },
    Interrupt {
        name: "INT128",
        number: 128,
    },
    Interrupt {
        name: "INT129",
        number: 129,
    },
    Interrupt {
        name: "INT130",
        number: 130,
    },
    Interrupt {
        name: "INT131",
        number: 131,
    },
    Interrupt {
        name: "INT132",
        number: 132,
    },
    Interrupt {
        name: "INT133",
        number: 133,
    },
    Interrupt {
        name: "INT134",
        number: 134,
    },
    Interrupt {
        name: "INT135",
        number: 135,
    },
    Interrupt {
        name: "INT136",
        number: 136,
    },
    Interrupt {
        name: "INT137",
        number: 137,
    },
    Interrupt {
        name: "INT138",
        number: 138,
    },
    Interrupt {
        name: "INT139",
        number: 139,
    },
    Interrupt {
        name: "INT140",
        number: 140,
    },
    Interrupt {
        name: "INT141",
        number: 141,
    },
    Interrupt {
        name: "INT142",
        number: 142,
    },
    Interrupt {
        name: "INT143",
        number: 143,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v33.rs"] pub mod adc_v33;
#[path="../registers/adc_v34.rs"] pub mod adc_v34;
#[path="../registers/aes_v5.rs"] pub mod aes_v5;
#[path="../registers/aos_v2.rs"] pub mod aos_v2;
#[path="../registers/can_v3.rs"] pub mod can_v3;
#[path="../registers/cmp_v5.rs"] pub mod cmp_v5;
#[path="../registers/cmp_v6.rs"] pub mod cmp_v6;
#[path="../registers/cmu_v2.rs"] pub mod cmu_v2;
#[path="../registers/crc_v7.rs"] pub mod crc_v7;
#[path="../registers/dbgc_v2.rs"] pub mod dbgc_v2;
#[path="../registers/dcu_v2.rs"] pub mod dcu_v2;
#[path="../registers/dma_v2.rs"] pub mod dma_v2;
#[path="../registers/efm_v2.rs"] pub mod efm_v2;
#[path="../registers/emb_v3.rs"] pub mod emb_v3;
#[path="../registers/fcm_v1.rs"] pub mod fcm_v1;
#[path="../registers/gpio_v8.rs"] pub mod gpio_v8;
#[path="../registers/hash_v1.rs"] pub mod hash_v1;
#[path="../registers/i_v18.rs"] pub mod i_v18;
#[path="../registers/i_v19.rs"] pub mod i_v19;
#[path="../registers/icg_v2.rs"] pub mod icg_v2;
#[path="../registers/intc_v2.rs"] pub mod intc_v2;
#[path="../registers/keyscan_common.rs"] pub mod keyscan_common;
#[path="../registers/mpu_v2.rs"] pub mod mpu_v2;
#[path="../registers/ots_v1.rs"] pub mod ots_v1;
#[path="../registers/peric_v2.rs"] pub mod peric_v2;
#[path="../registers/pwc_v2.rs"] pub mod pwc_v2;
#[path="../registers/qspi_v2.rs"] pub mod qspi_v2;
#[path="../registers/rmu_v2.rs"] pub mod rmu_v2;
#[path="../registers/rtc_v7.rs"] pub mod rtc_v7;
#[path="../registers/sdioc_common.rs"] pub mod sdioc_common;
#[path="../registers/spi_v8.rs"] pub mod spi_v8;
#[path="../registers/sramc_v2.rs"] pub mod sramc_v2;
#[path="../registers/swdt_v2.rs"] pub mod swdt_v2;
#[path="../registers/tmr_v10.rs"] pub mod tmr_v10;
#[path="../registers/tmr_v11.rs"] pub mod tmr_v11;
#[path="../registers/tmr_v7.rs"] pub mod tmr_v7;
#[path="../registers/tmr_v8.rs"] pub mod tmr_v8;
#[path="../registers/tmr_v9.rs"] pub mod tmr_v9;
#[path="../registers/tmra_v2.rs"] pub mod tmra_v2;
#[path="../registers/trng_v3.rs"] pub mod trng_v3;
#[path="../registers/usart_v3.rs"] pub mod usart_v3;
#[path="../registers/usbfs_v1.rs"] pub mod usbfs_v1;
#[path="../registers/wdt_common.rs"] pub mod wdt_common;
