
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40040000,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v13",
                block: "ADC",
                ir: &adc_v13::REGISTERS,
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
                    480,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    481,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    482,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    483,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    480,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    481,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    482,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    483,
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
                version: "v24",
                block: "ADC",
                ir: &adc_v24::REGISTERS,
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
                    484,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    485,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    486,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    487,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    484,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    485,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    486,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    487,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "ADC3",
        address: 0x40040800,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v32",
                block: "ADC",
                ir: &adc_v32::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_ADC3",
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
                    488,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    489,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    490,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    491,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    488,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    489,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    490,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    491,
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
                version: "v4",
                block: "AES",
                ir: &aes_v4::REGISTERS,
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
                version: "v1",
                block: "AOS",
                ir: &aos_v1::REGISTERS,
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
                    415,
                ),
            },
            PeripheralDmaChannel {
                signal: "STRG",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    415,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP1",
        address: 0x40038800,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v1",
                block: "CMP",
                ir: &cmp_v1::REGISTERS,
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
                    312,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    312,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP2",
        address: 0x40038900,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v2",
                block: "CMP",
                ir: &cmp_v2::REGISTERS,
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
                    313,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP2",
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
        name: "CMP3",
        address: 0x40038c00,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v3",
                block: "CMP",
                ir: &cmp_v3::REGISTERS,
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
                    314,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    314,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP4",
        address: 0x40038d00,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v4",
                block: "CMP",
                ir: &cmp_v4::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMP4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    315,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    315,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CMU",
        address: 0x40048000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmu",
                version: "v1",
                block: "CMU",
                ir: &cmu_v1::REGISTERS,
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
                version: "v6",
                block: "CRC",
                ir: &crc_v6::REGISTERS,
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
        name: "CTC",
        address: 0x40049c00,
        registers: Some(
            PeripheralRegisters {
                kind: "ctc",
                version: "v1",
                block: "CTC",
                ir: &ctc_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_CTC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC",
        address: 0x40041000,
        registers: Some(
            PeripheralRegisters {
                kind: "dac",
                version: "v3",
                block: "DAC",
                ir: &dac_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_DAC",
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
                version: "v1",
                block: "DBGC",
                ir: &dbgc_v1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DCU1",
        address: 0x40056000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v1",
                block: "DCU",
                ir: &dcu_v1::REGISTERS,
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
        address: 0x40056000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v1",
                block: "DCU",
                ir: &dcu_v1::REGISTERS,
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
        address: 0x40056000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v1",
                block: "DCU",
                ir: &dcu_v1::REGISTERS,
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
        address: 0x40056000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v1",
                block: "DCU",
                ir: &dcu_v1::REGISTERS,
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
                version: "v1",
                block: "DMA",
                ir: &dma_v1::REGISTERS,
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
                    33,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    34,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    35,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    36,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    37,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
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
                signal: "BTC3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    40,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    41,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    42,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    43,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    44,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    33,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    34,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    35,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    36,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    37,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
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
                signal: "BTC3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    40,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    41,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    42,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    43,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    44,
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
                version: "v1",
                block: "DMA",
                ir: &dma_v1::REGISTERS,
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
                    65,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    66,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    67,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    68,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    69,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    70,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    71,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    72,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    73,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    74,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    75,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    76,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    65,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    66,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    67,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    68,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    69,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    70,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    71,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    72,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    73,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    74,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    75,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    76,
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
                version: "v1",
                block: "EFM",
                ir: &efm_v1::REGISTERS,
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
                    51,
                ),
            },
            PeripheralDmaChannel {
                signal: "OPTEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    51,
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
                version: "v1",
                block: "EMB",
                ir: &emb_v1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB1",
        address: 0x40017c20,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v2",
                block: "EMB",
                ir: &emb_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB2",
        address: 0x40017c20,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v2",
                block: "EMB",
                ir: &emb_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB3",
        address: 0x40017c20,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v2",
                block: "EMB",
                ir: &emb_v2::REGISTERS,
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
                version: "v7",
                block: "GPIO",
                ir: &gpio_v7::REGISTERS,
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
        address: 0x4003b400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v10",
                block: "I",
                ir: &i_v10::REGISTERS,
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
                    288,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    289,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    290,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    291,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    288,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    289,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    290,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    291,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C2",
        address: 0x4003b400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v10",
                block: "I",
                ir: &i_v10::REGISTERS,
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
                    292,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    293,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    294,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    295,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    292,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    293,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    294,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    295,
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
                version: "v1",
                block: "ICG",
                ir: &icg_v1::REGISTERS,
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
                version: "v1",
                block: "INTC",
                ir: &intc_v1::REGISTERS,
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
        name: "MCAN1",
        address: 0x40029000,
        registers: Some(
            PeripheralRegisters {
                kind: "mcan",
                version: "v1",
                block: "MCAN",
                ir: &mcan_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_MCAN1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "MCAN2",
        address: 0x40029000,
        registers: Some(
            PeripheralRegisters {
                kind: "mcan",
                version: "v1",
                block: "MCAN",
                ir: &mcan_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_MCAN2",
                    },
                ),
            },
        ),
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
                version: "v1",
                block: "MPU",
                ir: &mpu_v1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PERIC",
        address: 0x40055400,
        registers: Some(
            PeripheralRegisters {
                kind: "peric",
                version: "v1",
                block: "PERIC",
                ir: &peric_v1::REGISTERS,
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
                version: "v1",
                block: "PWC",
                ir: &pwc_v1::REGISTERS,
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
                version: "v1",
                block: "QSPI",
                ir: &qspi_v1::REGISTERS,
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
        address: 0x4004cce0,
        registers: Some(
            PeripheralRegisters {
                kind: "rmu",
                version: "v1",
                block: "RMU",
                ir: &rmu_v1::REGISTERS,
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
                version: "v6",
                block: "RTC",
                ir: &rtc_v6::REGISTERS,
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
                    121,
                ),
            },
            PeripheralDmaChannel {
                signal: "PRD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    122,
                ),
            },
            PeripheralDmaChannel {
                signal: "ALM",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    121,
                ),
            },
            PeripheralDmaChannel {
                signal: "PRD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    122,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SMC",
        address: 0x88000000,
        registers: Some(
            PeripheralRegisters {
                kind: "smc",
                version: "v1",
                block: "SMC",
                ir: &smc_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_SMC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI1",
        address: 0x4001c000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v7",
                block: "SPI",
                ir: &spi_v7::REGISTERS,
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
                    334,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    335,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    336,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    337,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    338,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    334,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    335,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    336,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    337,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    338,
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
                version: "v7",
                block: "SPI",
                ir: &spi_v7::REGISTERS,
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
                    366,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    367,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    368,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    369,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    370,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    366,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    367,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    368,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    369,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    370,
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
                version: "v7",
                block: "SPI",
                ir: &spi_v7::REGISTERS,
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
                    371,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    372,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    373,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    374,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    375,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    371,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    372,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    373,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    374,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    375,
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
                version: "v1",
                block: "SRAMC",
                ir: &sramc_v1::REGISTERS,
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
                version: "v1",
                block: "SWDT",
                ir: &swdt_v1::REGISTERS,
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
                version: "v1",
                block: "TMR",
                ir: &tmr_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR0_1",
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
                    96,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    97,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    98,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    99,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    96,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    97,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    98,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    99,
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
                version: "v1",
                block: "TMR",
                ir: &tmr_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR0_2",
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
                    100,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    101,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    102,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    103,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    100,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    101,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    102,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    103,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR41",
        address: 0x40038000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v2",
                block: "TMR",
                ir: &tmr_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR4_1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPUH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    192,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    193,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    194,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    195,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    196,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    197,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    198,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    199,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    200,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    201,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADU",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    202,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADV",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    203,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADW",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    204,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    205,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    206,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    207,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    208,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    209,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    210,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    211,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP6",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    212,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP7",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    213,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    192,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    193,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    194,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    195,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    196,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    197,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    198,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    199,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    200,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    201,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADU",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    202,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADV",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    203,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADW",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    204,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADX",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    205,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    206,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    207,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    208,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    209,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    210,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    211,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP6",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    212,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP7",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    213,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR42",
        address: 0x40038400,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v3",
                block: "TMR",
                ir: &tmr_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR4_2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPUH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    224,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    225,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    226,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    227,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    228,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    229,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    230,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    231,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    232,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    233,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADU",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    234,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADV",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    235,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADW",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    236,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    237,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    238,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    239,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    240,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    241,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    242,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    243,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP6",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    244,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP7",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    245,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    224,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    225,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    226,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    227,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    228,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    229,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    230,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    231,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    232,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    233,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADU",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    234,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADV",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    235,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADW",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    236,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADX",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    237,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    238,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    239,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    240,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    241,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    242,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    243,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP6",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    244,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP7",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    245,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR43",
        address: 0x40038e00,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v4",
                block: "TMR",
                ir: &tmr_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR4_3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPUH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    256,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    257,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    258,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    259,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    260,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    261,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    262,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    263,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    264,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    265,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADU",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    266,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADV",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    267,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADW",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    268,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    269,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    270,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    271,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    272,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    273,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    274,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    275,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP6",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    276,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP7",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    277,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    256,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    257,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    258,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    259,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    260,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    261,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    262,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    263,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    264,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    265,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADU",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    266,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADV",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    267,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADW",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    268,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADX",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    269,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    270,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    271,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    272,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    273,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    274,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    275,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP6",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    276,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP7",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    277,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR61",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v5",
                block: "TMR",
                ir: &tmr_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR6_1",
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
                    137,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    138,
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
                    137,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    138,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR62",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v5",
                block: "TMR",
                ir: &tmr_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR6_2",
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
                    144,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    145,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    146,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    147,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    148,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    149,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    150,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    151,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    153,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    154,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    144,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    145,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    146,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    147,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    148,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    149,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    150,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    151,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    153,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    154,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR6_COMMON",
        address: 0x4003c300,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v6",
                block: "TMR",
                ir: &tmr_v6::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA1",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v1",
                block: "TMRA",
                ir: &tmra_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMRA_1",
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
                    139,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    140,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    141,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    139,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    140,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    141,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA2",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v1",
                block: "TMRA",
                ir: &tmra_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMRA_2",
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
                    155,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    156,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    157,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    155,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    156,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    157,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA3",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v1",
                block: "TMRA",
                ir: &tmra_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMRA_3",
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
                    171,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    172,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    173,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    171,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    172,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    173,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA4",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v1",
                block: "TMRA",
                ir: &tmra_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMRA_4",
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
                    187,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    188,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    189,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    187,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    188,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    189,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA5",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v1",
                block: "TMRA",
                ir: &tmra_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMRA_5",
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
                    340,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    341,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    342,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    340,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    341,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    342,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TRNG",
        address: 0x40042000,
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
                    492,
                ),
            },
            PeripheralDmaChannel {
                signal: "END",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    492,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART1",
        address: 0x4001cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v1",
                block: "USART",
                ir: &usart_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_USART1",
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
                    321,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    322,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    323,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    324,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    326,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    321,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    322,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    323,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    324,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    326,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART2",
        address: 0x4001cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v1",
                block: "USART",
                ir: &usart_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_USART2",
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
                    328,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    329,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    330,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    331,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    333,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    328,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    329,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    330,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    331,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    333,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART3",
        address: 0x4001d400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "USART",
                ir: &usart_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_USART3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    352,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    353,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    354,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    355,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    358,
                ),
            },
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    352,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    353,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    354,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    355,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    358,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART4",
        address: 0x4001cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v1",
                block: "USART",
                ir: &usart_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_USART4",
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
                    360,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    361,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    362,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    363,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    365,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    360,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    361,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    362,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    363,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    365,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART5",
        address: 0x4001cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v1",
                block: "USART",
                ir: &usart_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_USART5",
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
                    385,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    386,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    387,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    388,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    390,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    385,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    386,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    387,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    388,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    390,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART6",
        address: 0x4001d400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "USART",
                ir: &usart_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_USART6",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    391,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    392,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    393,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    394,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    396,
                ),
            },
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    391,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    392,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    393,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    394,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    396,
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
                    471,
                ),
            },
            PeripheralDmaChannel {
                signal: "REFUDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    471,
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
        name: "EXTINT_PORT_EIRQ0",
        number: 16,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ1",
        number: 17,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ2",
        number: 18,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ3",
        number: 19,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ4",
        number: 20,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ5",
        number: 21,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ6",
        number: 22,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ7",
        number: 23,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ8",
        number: 24,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ9",
        number: 25,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ10",
        number: 26,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ11",
        number: 27,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ12",
        number: 28,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ13",
        number: 29,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ14",
        number: 30,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ15",
        number: 31,
    },
    Interrupt {
        name: "DMA1_ERR",
        number: 32,
    },
    Interrupt {
        name: "DMA1_TC0_BTC0",
        number: 33,
    },
    Interrupt {
        name: "DMA1_TC1_BTC1",
        number: 34,
    },
    Interrupt {
        name: "DMA1_TC2_BTC2",
        number: 35,
    },
    Interrupt {
        name: "DMA1_TC3_BTC3",
        number: 36,
    },
    Interrupt {
        name: "DMA1_TC4_BTC4",
        number: 37,
    },
    Interrupt {
        name: "DMA1_TC5_BTC5",
        number: 38,
    },
    Interrupt {
        name: "EFM_PEERR_RDCOL",
        number: 39,
    },
    Interrupt {
        name: "EFM_OPTEND",
        number: 40,
    },
    Interrupt {
        name: "QSPI",
        number: 41,
    },
    Interrupt {
        name: "DCU1",
        number: 42,
    },
    Interrupt {
        name: "DCU2",
        number: 43,
    },
    Interrupt {
        name: "DCU3",
        number: 44,
    },
    Interrupt {
        name: "DCU4",
        number: 45,
    },
    Interrupt {
        name: "DMA2_ERR",
        number: 46,
    },
    Interrupt {
        name: "DMA2_TC0_BTC0",
        number: 47,
    },
    Interrupt {
        name: "DMA2_TC1_BTC1",
        number: 48,
    },
    Interrupt {
        name: "DMA2_TC2_BTC2",
        number: 49,
    },
    Interrupt {
        name: "DMA2_TC3_BTC3",
        number: 50,
    },
    Interrupt {
        name: "DMA2_TC4_BTC4",
        number: 51,
    },
    Interrupt {
        name: "DMA2_TC5_BTC5",
        number: 52,
    },
    Interrupt {
        name: "TMR0_1",
        number: 53,
    },
    Interrupt {
        name: "TMR0_2",
        number: 54,
    },
    Interrupt {
        name: "RTC",
        number: 55,
    },
    Interrupt {
        name: "XTAL",
        number: 56,
    },
    Interrupt {
        name: "WKTM",
        number: 57,
    },
    Interrupt {
        name: "SWDT",
        number: 58,
    },
    Interrupt {
        name: "TMR6_1_GCMP",
        number: 59,
    },
    Interrupt {
        name: "TMR6_1_OVF_UDF",
        number: 60,
    },
    Interrupt {
        name: "TMR6_1_DTE",
        number: 61,
    },
    Interrupt {
        name: "TMR6_1_SCMP",
        number: 62,
    },
    Interrupt {
        name: "TMRA_1_OVF_UDF",
        number: 63,
    },
    Interrupt {
        name: "TMRA_1_CMP",
        number: 64,
    },
    Interrupt {
        name: "TMR6_2_GCMP",
        number: 65,
    },
    Interrupt {
        name: "TMR6_2_OVF_UDF",
        number: 66,
    },
    Interrupt {
        name: "TMR6_2_DTE",
        number: 67,
    },
    Interrupt {
        name: "TMR6_2_SCMP",
        number: 68,
    },
    Interrupt {
        name: "TMRA_2_OVF_UDF",
        number: 69,
    },
    Interrupt {
        name: "TMRA_2_CMP",
        number: 70,
    },
    Interrupt {
        name: "TMRA_3_OVF_UDF",
        number: 71,
    },
    Interrupt {
        name: "TMRA_3_CMP",
        number: 72,
    },
    Interrupt {
        name: "TMRA_4_OVF_UDF",
        number: 73,
    },
    Interrupt {
        name: "TMRA_4_CMP",
        number: 74,
    },
    Interrupt {
        name: "TMR4_1_GCMP",
        number: 75,
    },
    Interrupt {
        name: "TMR4_1_OVF_UDF",
        number: 76,
    },
    Interrupt {
        name: "TMR4_1_RELOAD",
        number: 77,
    },
    Interrupt {
        name: "TMR4_1_SCMP",
        number: 78,
    },
    Interrupt {
        name: "TMR4_2_GCMP",
        number: 79,
    },
    Interrupt {
        name: "TMR4_2_OVF_UDF",
        number: 80,
    },
    Interrupt {
        name: "TMR4_2_RELOAD",
        number: 81,
    },
    Interrupt {
        name: "TMR4_2_SCMP",
        number: 82,
    },
    Interrupt {
        name: "TMR4_3_GCMP",
        number: 83,
    },
    Interrupt {
        name: "TMR4_3_OVF_UDF",
        number: 84,
    },
    Interrupt {
        name: "TMR4_3_RELOAD",
        number: 85,
    },
    Interrupt {
        name: "TMR4_3_SCMP",
        number: 86,
    },
    Interrupt {
        name: "I2C1",
        number: 87,
    },
    Interrupt {
        name: "I2C2",
        number: 88,
    },
    Interrupt {
        name: "CMP1",
        number: 89,
    },
    Interrupt {
        name: "CMP2",
        number: 90,
    },
    Interrupt {
        name: "CMP3",
        number: 91,
    },
    Interrupt {
        name: "CMP4",
        number: 92,
    },
    Interrupt {
        name: "USART1",
        number: 93,
    },
    Interrupt {
        name: "USART1_TCI",
        number: 94,
    },
    Interrupt {
        name: "USART2",
        number: 95,
    },
    Interrupt {
        name: "USART2_TCI",
        number: 96,
    },
    Interrupt {
        name: "SPI1",
        number: 97,
    },
    Interrupt {
        name: "TMRA_5_OVF_UDF",
        number: 98,
    },
    Interrupt {
        name: "TMRA_5_CMP",
        number: 99,
    },
    Interrupt {
        name: "EVENT_PORT1",
        number: 100,
    },
    Interrupt {
        name: "EVENT_PORT2",
        number: 101,
    },
    Interrupt {
        name: "EVENT_PORT3",
        number: 102,
    },
    Interrupt {
        name: "EVENT_PORT4",
        number: 103,
    },
    Interrupt {
        name: "USART3",
        number: 104,
    },
    Interrupt {
        name: "USART3_TCI",
        number: 105,
    },
    Interrupt {
        name: "USART4",
        number: 106,
    },
    Interrupt {
        name: "USART4_TCI",
        number: 107,
    },
    Interrupt {
        name: "SPI2",
        number: 108,
    },
    Interrupt {
        name: "SPI3",
        number: 109,
    },
    Interrupt {
        name: "EMB_GR0",
        number: 110,
    },
    Interrupt {
        name: "EMB_GR1",
        number: 111,
    },
    Interrupt {
        name: "EMB_GR2",
        number: 112,
    },
    Interrupt {
        name: "EMB_GR3",
        number: 113,
    },
    Interrupt {
        name: "USART5",
        number: 114,
    },
    Interrupt {
        name: "USART5_TCI",
        number: 115,
    },
    Interrupt {
        name: "USART6",
        number: 116,
    },
    Interrupt {
        name: "USART6_TCI",
        number: 117,
    },
    Interrupt {
        name: "MCAN1_INT0",
        number: 118,
    },
    Interrupt {
        name: "MCAN1_INT1",
        number: 119,
    },
    Interrupt {
        name: "MCAN2_INT0",
        number: 120,
    },
    Interrupt {
        name: "MCAN2_INT1",
        number: 121,
    },
    Interrupt {
        name: "USART1_WUPI",
        number: 122,
    },
    Interrupt {
        name: "FCM",
        number: 125,
    },
    Interrupt {
        name: "WDT",
        number: 126,
    },
    Interrupt {
        name: "CTC",
        number: 127,
    },
    Interrupt {
        name: "ADC1",
        number: 128,
    },
    Interrupt {
        name: "ADC2",
        number: 129,
    },
    Interrupt {
        name: "ADC3",
        number: 130,
    },
    Interrupt {
        name: "TRNG",
        number: 131,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v13.rs"] pub mod adc_v13;
#[path="../registers/adc_v24.rs"] pub mod adc_v24;
#[path="../registers/adc_v32.rs"] pub mod adc_v32;
#[path="../registers/aes_v4.rs"] pub mod aes_v4;
#[path="../registers/aos_v1.rs"] pub mod aos_v1;
#[path="../registers/cmp_v1.rs"] pub mod cmp_v1;
#[path="../registers/cmp_v2.rs"] pub mod cmp_v2;
#[path="../registers/cmp_v3.rs"] pub mod cmp_v3;
#[path="../registers/cmp_v4.rs"] pub mod cmp_v4;
#[path="../registers/cmu_v1.rs"] pub mod cmu_v1;
#[path="../registers/crc_v6.rs"] pub mod crc_v6;
#[path="../registers/ctc_v1.rs"] pub mod ctc_v1;
#[path="../registers/dac_v3.rs"] pub mod dac_v3;
#[path="../registers/dbgc_v1.rs"] pub mod dbgc_v1;
#[path="../registers/dcu_v1.rs"] pub mod dcu_v1;
#[path="../registers/dma_v1.rs"] pub mod dma_v1;
#[path="../registers/efm_v1.rs"] pub mod efm_v1;
#[path="../registers/emb_v1.rs"] pub mod emb_v1;
#[path="../registers/emb_v2.rs"] pub mod emb_v2;
#[path="../registers/fcm_v1.rs"] pub mod fcm_v1;
#[path="../registers/gpio_v7.rs"] pub mod gpio_v7;
#[path="../registers/hash_v1.rs"] pub mod hash_v1;
#[path="../registers/i_v10.rs"] pub mod i_v10;
#[path="../registers/icg_v1.rs"] pub mod icg_v1;
#[path="../registers/intc_v1.rs"] pub mod intc_v1;
#[path="../registers/keyscan_common.rs"] pub mod keyscan_common;
#[path="../registers/mcan_v1.rs"] pub mod mcan_v1;
#[path="../registers/mpu_v1.rs"] pub mod mpu_v1;
#[path="../registers/peric_v1.rs"] pub mod peric_v1;
#[path="../registers/pwc_v1.rs"] pub mod pwc_v1;
#[path="../registers/qspi_v1.rs"] pub mod qspi_v1;
#[path="../registers/rmu_v1.rs"] pub mod rmu_v1;
#[path="../registers/rtc_v6.rs"] pub mod rtc_v6;
#[path="../registers/smc_v1.rs"] pub mod smc_v1;
#[path="../registers/spi_v7.rs"] pub mod spi_v7;
#[path="../registers/sramc_v1.rs"] pub mod sramc_v1;
#[path="../registers/swdt_v1.rs"] pub mod swdt_v1;
#[path="../registers/tmr_v1.rs"] pub mod tmr_v1;
#[path="../registers/tmr_v2.rs"] pub mod tmr_v2;
#[path="../registers/tmr_v3.rs"] pub mod tmr_v3;
#[path="../registers/tmr_v4.rs"] pub mod tmr_v4;
#[path="../registers/tmr_v5.rs"] pub mod tmr_v5;
#[path="../registers/tmr_v6.rs"] pub mod tmr_v6;
#[path="../registers/tmra_v1.rs"] pub mod tmra_v1;
#[path="../registers/trng_v3.rs"] pub mod trng_v3;
#[path="../registers/usart_v1.rs"] pub mod usart_v1;
#[path="../registers/usart_v2.rs"] pub mod usart_v2;
#[path="../registers/wdt_common.rs"] pub mod wdt_common;
