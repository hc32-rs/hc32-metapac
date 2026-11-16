
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40040000,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v38",
                block: "ADC",
                ir: &adc_v38::REGISTERS,
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
                version: "v14",
                block: "ADC",
                ir: &adc_v14::REGISTERS,
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
                version: "v15",
                block: "ADC",
                ir: &adc_v15::REGISTERS,
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
                version: "v4",
                block: "AOS",
                ir: &aos_v4::REGISTERS,
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
                    299,
                ),
            },
            PeripheralDmaChannel {
                signal: "STRG",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    299,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CAN1",
        address: 0x40009000,
        registers: Some(
            PeripheralRegisters {
                kind: "can",
                version: "v4",
                block: "CAN",
                ir: &can_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_CAN1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CAN2",
        address: 0x40009000,
        registers: Some(
            PeripheralRegisters {
                kind: "can",
                version: "v4",
                block: "CAN",
                ir: &can_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_CAN2",
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
                version: "v9",
                block: "CMP",
                ir: &cmp_v9::REGISTERS,
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
                    444,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    444,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP2",
        address: 0x4004a010,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v10",
                block: "CMP",
                ir: &cmp_v10::REGISTERS,
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
                    445,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    445,
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
                version: "v9",
                block: "CMP",
                ir: &cmp_v9::REGISTERS,
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
                    446,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    446,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP4",
        address: 0x4004a010,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v10",
                block: "CMP",
                ir: &cmp_v10::REGISTERS,
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
                    447,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    447,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CMU",
        address: 0x4004c400,
        registers: Some(
            PeripheralRegisters {
                kind: "cmu",
                version: "v4",
                block: "CMU",
                ir: &cmu_v4::REGISTERS,
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
                version: "v8",
                block: "CRC",
                ir: &crc_v8::REGISTERS,
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
                version: "v3",
                block: "CTC",
                ir: &ctc_v3::REGISTERS,
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
        name: "DAC1",
        address: 0x40041000,
        registers: Some(
            PeripheralRegisters {
                kind: "dac",
                version: "v5",
                block: "DAC",
                ir: &dac_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_DAC1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC2",
        address: 0x40041000,
        registers: Some(
            PeripheralRegisters {
                kind: "dac",
                version: "v5",
                block: "DAC",
                ir: &dac_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_DAC2",
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
                version: "v4",
                block: "DBGC",
                ir: &dbgc_v4::REGISTERS,
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
                version: "v4",
                block: "DCU",
                ir: &dcu_v4::REGISTERS,
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
                version: "v4",
                block: "DCU",
                ir: &dcu_v4::REGISTERS,
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
                version: "v4",
                block: "DCU",
                ir: &dcu_v4::REGISTERS,
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
                version: "v4",
                block: "DCU",
                ir: &dcu_v4::REGISTERS,
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
        name: "DCU5",
        address: 0x40057000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v5",
                block: "DCU",
                ir: &dcu_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_DCU5",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCU5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    59,
                ),
            },
            PeripheralDmaChannel {
                signal: "DCU5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    59,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "DCU6",
        address: 0x40057000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v5",
                block: "DCU",
                ir: &dcu_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_DCU6",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCU6",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    60,
                ),
            },
            PeripheralDmaChannel {
                signal: "DCU6",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    60,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "DCU7",
        address: 0x40057000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v5",
                block: "DCU",
                ir: &dcu_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_DCU7",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCU7",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    61,
                ),
            },
            PeripheralDmaChannel {
                signal: "DCU7",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    61,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "DCU8",
        address: 0x40057000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v5",
                block: "DCU",
                ir: &dcu_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_DCU8",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCU8",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    62,
                ),
            },
            PeripheralDmaChannel {
                signal: "DCU8",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    62,
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
                version: "v4",
                block: "DMA",
                ir: &dma_v4::REGISTERS,
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
                signal: "TC4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    36,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    37,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC6",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    38,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC7",
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
                signal: "BTC4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    44,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    45,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC6",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    46,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC7",
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
                signal: "TC4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    36,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    37,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC6",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    38,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC7",
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
            PeripheralDmaChannel {
                signal: "BTC4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    44,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    45,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC6",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    46,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC7",
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
        name: "DMA2",
        address: 0x40053000,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "v4",
                block: "DMA",
                ir: &dma_v4::REGISTERS,
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
                    64,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    65,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    66,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    67,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    68,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    69,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC6",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    70,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC7",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    71,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    72,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    73,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    74,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    75,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    76,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    77,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC6",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    78,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC7",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    79,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    64,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    65,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    66,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    67,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    68,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    69,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC6",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    70,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC7",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    71,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    72,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    73,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    74,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    75,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    76,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    77,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC6",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    78,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC7",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    79,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "DMC",
        address: 0x88000400,
        registers: Some(
            PeripheralRegisters {
                kind: "dmc",
                version: "common",
                block: "DMC",
                ir: &dmc_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_DMC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DVP",
        address: 0x40055800,
        registers: Some(
            PeripheralRegisters {
                kind: "dvp",
                version: "common",
                block: "DVP",
                ir: &dvp_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_DVP",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "FRAMSTA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    84,
                ),
            },
            PeripheralDmaChannel {
                signal: "LINESTA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    85,
                ),
            },
            PeripheralDmaChannel {
                signal: "LINEEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    86,
                ),
            },
            PeripheralDmaChannel {
                signal: "FRAMEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    87,
                ),
            },
            PeripheralDmaChannel {
                signal: "SQUERR",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    88,
                ),
            },
            PeripheralDmaChannel {
                signal: "FIFOERR",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    89,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAREQ",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    90,
                ),
            },
            PeripheralDmaChannel {
                signal: "FRAMSTA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    84,
                ),
            },
            PeripheralDmaChannel {
                signal: "LINESTA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    85,
                ),
            },
            PeripheralDmaChannel {
                signal: "LINEEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    86,
                ),
            },
            PeripheralDmaChannel {
                signal: "FRAMEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    87,
                ),
            },
            PeripheralDmaChannel {
                signal: "SQUERR",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    88,
                ),
            },
            PeripheralDmaChannel {
                signal: "FIFOERR",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    89,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAREQ",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    90,
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
                version: "v4",
                block: "EFM",
                ir: &efm_v4::REGISTERS,
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
                version: "v6",
                block: "EMB",
                ir: &emb_v6::REGISTERS,
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
                version: "v6",
                block: "EMB",
                ir: &emb_v6::REGISTERS,
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
                version: "v6",
                block: "EMB",
                ir: &emb_v6::REGISTERS,
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
                version: "v6",
                block: "EMB",
                ir: &emb_v6::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB4",
        address: 0x40017c80,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v7",
                block: "EMB",
                ir: &emb_v7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB5",
        address: 0x40017c80,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v7",
                block: "EMB",
                ir: &emb_v7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB6",
        address: 0x40017c80,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v7",
                block: "EMB",
                ir: &emb_v7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ETH",
        address: 0x40050000,
        registers: Some(
            PeripheralRegisters {
                kind: "eth",
                version: "v1",
                block: "ETH",
                ir: &eth_v1::REGISTERS,
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
        name: "FMAC1",
        address: 0x40058000,
        registers: Some(
            PeripheralRegisters {
                kind: "fmac",
                version: "common",
                block: "FMAC",
                ir: &fmac_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_FMAC1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "FMAC_1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    91,
                ),
            },
            PeripheralDmaChannel {
                signal: "FMAC_1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    91,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "FMAC2",
        address: 0x40058000,
        registers: Some(
            PeripheralRegisters {
                kind: "fmac",
                version: "common",
                block: "FMAC",
                ir: &fmac_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_FMAC2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "FMAC_2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    92,
                ),
            },
            PeripheralDmaChannel {
                signal: "FMAC_2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    92,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "FMAC3",
        address: 0x40058000,
        registers: Some(
            PeripheralRegisters {
                kind: "fmac",
                version: "common",
                block: "FMAC",
                ir: &fmac_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_FMAC3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "FMAC_3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    93,
                ),
            },
            PeripheralDmaChannel {
                signal: "FMAC_3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    93,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "FMAC4",
        address: 0x40058000,
        registers: Some(
            PeripheralRegisters {
                kind: "fmac",
                version: "common",
                block: "FMAC",
                ir: &fmac_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_FMAC4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "FMAC_4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    94,
                ),
            },
            PeripheralDmaChannel {
                signal: "FMAC_4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    94,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40053800,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v16",
                block: "GPIO",
                ir: &gpio_v16::REGISTERS,
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
                version: "v2",
                block: "HASH",
                ir: &hash_v2::REGISTERS,
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "HASH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    401,
                ),
            },
            PeripheralDmaChannel {
                signal: "HASH",
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
        name: "HRPWM",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "hrpwm",
                version: "v1",
                block: "HRPWM",
                ir: &hrpwm_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_HRPWM",
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
                    432,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    433,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    434,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    435,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    432,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    433,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    434,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
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
                    436,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    437,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    438,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    439,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    436,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    437,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    438,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
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
                    440,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    441,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    442,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    443,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    440,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    441,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    442,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    443,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C4",
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
                        field: "PWC_FCG1_I2C4",
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
                    448,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    449,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    450,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    451,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    448,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    449,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    450,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
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
        name: "I2C5",
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
                        field: "PWC_FCG1_I2C5",
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
                    452,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    453,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    454,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    455,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    452,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    453,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    454,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
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
        name: "I2C6",
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
                        field: "PWC_FCG1_I2C6",
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
                    456,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    457,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    458,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    459,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    456,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    457,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    458,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    459,
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
                version: "v21",
                block: "I",
                ir: &i_v21::REGISTERS,
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
                    384,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    385,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    384,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    385,
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
                version: "v21",
                block: "I",
                ir: &i_v21::REGISTERS,
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
                    387,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    388,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    387,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    388,
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
                version: "v21",
                block: "I",
                ir: &i_v21::REGISTERS,
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
                    416,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    417,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    416,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
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
        name: "I2S4",
        address: 0x4001e000,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v21",
                block: "I",
                ir: &i_v21::REGISTERS,
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
                    419,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    420,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    419,
                ),
            },
            PeripheralDmaChannel {
                signal: "RXIRQOUT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    420,
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
                version: "v4",
                block: "INTC",
                ir: &intc_v4::REGISTERS,
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
        name: "MAU",
        address: 0x40055000,
        registers: Some(
            PeripheralRegisters {
                kind: "mau",
                version: "v1",
                block: "MAU",
                ir: &mau_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_MAU",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SQRT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    83,
                ),
            },
            PeripheralDmaChannel {
                signal: "SQRT",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    83,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "MPU",
        address: 0x40050000,
        registers: Some(
            PeripheralRegisters {
                kind: "mpu",
                version: "v4",
                block: "MPU",
                ir: &mpu_v4::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "NFC",
        address: 0x88100000,
        registers: Some(
            PeripheralRegisters {
                kind: "nfc",
                version: "v1",
                block: "NFC",
                ir: &nfc_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_NFC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OTS",
        address: 0x40010600,
        registers: Some(
            PeripheralRegisters {
                kind: "ots",
                version: "v2",
                block: "OTS",
                ir: &ots_v2::REGISTERS,
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
                    463,
                ),
            },
            PeripheralDmaChannel {
                signal: "OTS",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    463,
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
                version: "v4",
                block: "PERIC",
                ir: &peric_v4::REGISTERS,
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
                version: "v4",
                block: "PWC",
                ir: &pwc_v4::REGISTERS,
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
        address: 0x4004cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "rmu",
                version: "v4",
                block: "RMU",
                ir: &rmu_v4::REGISTERS,
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
                version: "v8",
                block: "RTC",
                ir: &rtc_v8::REGISTERS,
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
        name: "SDIOC1",
        address: 0x40070000,
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
                    402,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAW",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    403,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAR",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    402,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAW",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    403,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SDIOC2",
        address: 0x40070000,
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
                    405,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAW",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    406,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAR",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    405,
                ),
            },
            PeripheralDmaChannel {
                signal: "DMAW",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    406,
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
                version: "v3",
                block: "SMC",
                ir: &smc_v3::REGISTERS,
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
                    310,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    311,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    312,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    313,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    314,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    310,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    311,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    312,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    313,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
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
                    315,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    316,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    317,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    318,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    319,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    315,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    316,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    317,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    318,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
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
                    342,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    343,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    344,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    345,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    346,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    342,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    343,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    344,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    345,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    346,
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
                    347,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    348,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    349,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    350,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    351,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    347,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    348,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    349,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    350,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    351,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI5",
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
                        field: "PWC_FCG1_SPI5",
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
                    374,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    375,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    376,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    377,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    378,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    374,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    375,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    376,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    377,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    378,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI6",
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
                        field: "PWC_FCG1_SPI6",
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
                    379,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    380,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    381,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    382,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    383,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    379,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    380,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    381,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    382,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    383,
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
                version: "v4",
                block: "SRAMC",
                ir: &sramc_v4::REGISTERS,
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
                    98,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
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
                    98,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
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
        name: "TMR21",
        address: 0x40024800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v17",
                block: "TMR",
                ir: &tmr_v17::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR2_1",
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
        name: "TMR22",
        address: 0x40024800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v17",
                block: "TMR",
                ir: &tmr_v17::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR2_2",
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
                    104,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    105,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    106,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    107,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    104,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    105,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    106,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    107,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR23",
        address: 0x40024800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v17",
                block: "TMR",
                ir: &tmr_v17::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR2_3",
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
                    108,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    109,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    110,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    111,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    108,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    109,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    110,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    111,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR24",
        address: 0x40024800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v17",
                block: "TMR",
                ir: &tmr_v17::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR2_4",
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
                    112,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    113,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    114,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    115,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    112,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    113,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    114,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    115,
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
                version: "v18",
                block: "TMR",
                ir: &tmr_v18::REGISTERS,
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
                signal: "SCMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    136,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    137,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    138,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    139,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    140,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    141,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    136,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    137,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    138,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    139,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    140,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
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
        name: "TMR42",
        address: 0x40038000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v18",
                block: "TMR",
                ir: &tmr_v18::REGISTERS,
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
                signal: "SCMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    152,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    153,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    154,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    155,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    156,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    157,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    152,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    153,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    154,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    155,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    156,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
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
        name: "TMR43",
        address: 0x40038000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v18",
                block: "TMR",
                ir: &tmr_v18::REGISTERS,
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
                signal: "SCMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    168,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    169,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    170,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    171,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    172,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    173,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    168,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    169,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    170,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    171,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    172,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
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
        name: "TMR61",
        address: 0x40018000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v19",
                block: "TMR",
                ir: &tmr_v19::REGISTERS,
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
                    179,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    180,
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
                    179,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    180,
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
                version: "v19",
                block: "TMR",
                ir: &tmr_v19::REGISTERS,
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
                    187,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    188,
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
                    187,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    188,
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
                version: "v19",
                block: "TMR",
                ir: &tmr_v19::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR6_3",
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
                    160,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    161,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    162,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    163,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    164,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    165,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    166,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    167,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    195,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    196,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    160,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    161,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    162,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    163,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    164,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    165,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    166,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    167,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    195,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    196,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR64",
        address: 0x40018000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v19",
                block: "TMR",
                ir: &tmr_v19::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR6_4",
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
                    208,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    209,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    210,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    211,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    212,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    213,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    214,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    215,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    219,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    220,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    208,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    209,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    210,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    211,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    212,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    213,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    214,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    215,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    219,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    220,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR65",
        address: 0x40018000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v19",
                block: "TMR",
                ir: &tmr_v19::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR6_5",
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
                    224,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    225,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    226,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    227,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    228,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    229,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    230,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    231,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    235,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    236,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    224,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    225,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    226,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    227,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    228,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    229,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    230,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    231,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    235,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    236,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR66",
        address: 0x40018000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v19",
                block: "TMR",
                ir: &tmr_v19::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR6_6",
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
                    240,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    241,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    242,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    243,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    244,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    245,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    246,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    247,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    251,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    252,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    240,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    241,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    242,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    243,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    244,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    245,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    246,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    247,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    251,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    252,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR67",
        address: 0x40018000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v19",
                block: "TMR",
                ir: &tmr_v19::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR6_7",
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
                    256,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    257,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    258,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    259,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    260,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
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
                signal: "SCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    267,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    268,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    256,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    257,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    258,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    259,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    260,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA2",
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
                signal: "SCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    267,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    268,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR68",
        address: 0x40018000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v19",
                block: "TMR",
                ir: &tmr_v19::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR6_8",
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
                    272,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    273,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    274,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    275,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    276,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    277,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    278,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    279,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    283,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    284,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    272,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    273,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    274,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    275,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    276,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    277,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    278,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    279,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    283,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    284,
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
                version: "v20",
                block: "TMR",
                ir: &tmr_v20::REGISTERS,
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
                version: "v9",
                block: "TMRA",
                ir: &tmra_v9::REGISTERS,
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
                    237,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    238,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    239,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    237,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    238,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    239,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA10",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v9",
                block: "TMRA",
                ir: &tmra_v9::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMRA_10",
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
                    355,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    356,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    357,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    355,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    356,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    357,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA11",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v9",
                block: "TMRA",
                ir: &tmra_v9::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMRA_11",
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
                    358,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    359,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    360,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    358,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    359,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    360,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA12",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v9",
                block: "TMRA",
                ir: &tmra_v9::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMRA_12",
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
                    361,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    362,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    363,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    361,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    362,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    363,
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
                version: "v9",
                block: "TMRA",
                ir: &tmra_v9::REGISTERS,
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
                    253,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    254,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    255,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    253,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    254,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    255,
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
                version: "v9",
                block: "TMRA",
                ir: &tmra_v9::REGISTERS,
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
                    269,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    270,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    271,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    269,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    270,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    271,
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
                version: "v9",
                block: "TMRA",
                ir: &tmra_v9::REGISTERS,
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
                    285,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    286,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    287,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    285,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    286,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
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
        name: "TMRA5",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v9",
                block: "TMRA",
                ir: &tmra_v9::REGISTERS,
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
                    320,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    321,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    322,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    320,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    321,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    322,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA6",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v9",
                block: "TMRA",
                ir: &tmra_v9::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMRA_6",
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
                    323,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    324,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    325,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    323,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    324,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    325,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA7",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v9",
                block: "TMRA",
                ir: &tmra_v9::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMRA_7",
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
                    326,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    327,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    328,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    326,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    327,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    328,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA8",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v9",
                block: "TMRA",
                ir: &tmra_v9::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMRA_8",
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
                    329,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    330,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    331,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    329,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    330,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    331,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA9",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v9",
                block: "TMRA",
                ir: &tmra_v9::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMRA_9",
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
                    352,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    353,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    354,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    352,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    353,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    354,
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
                version: "v10",
                block: "USART",
                ir: &usart_v10::REGISTERS,
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
                    300,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    301,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    302,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    303,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    304,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    300,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    301,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    302,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    303,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    304,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART10",
        address: 0x40021c00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v11",
                block: "USART",
                ir: &usart_v11::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_USART10",
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
                    426,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    427,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    428,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    429,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    430,
                ),
            },
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    426,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    427,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    428,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    429,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    430,
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
                version: "v10",
                block: "USART",
                ir: &usart_v10::REGISTERS,
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
                    305,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    306,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    307,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    308,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    309,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    305,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    306,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    307,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    308,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    309,
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
                version: "v12",
                block: "USART",
                ir: &usart_v12::REGISTERS,
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
                signal: "EI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    332,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    333,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    334,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    335,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    332,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    333,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    334,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    335,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART4",
        address: 0x4001d400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v12",
                block: "USART",
                ir: &usart_v12::REGISTERS,
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
                    336,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    337,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    338,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    339,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    336,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    337,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    338,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    339,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART5",
        address: 0x4001dc00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v13",
                block: "USART",
                ir: &usart_v13::REGISTERS,
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
                signal: "BRKWKPI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    364,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    365,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    366,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    367,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    368,
                ),
            },
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    364,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    365,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    366,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    367,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    368,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART6",
        address: 0x40020c00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v14",
                block: "USART",
                ir: &usart_v14::REGISTERS,
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
                signal: "EI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    369,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    370,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    371,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    372,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    373,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    369,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    370,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    371,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    372,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
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
        name: "USART7",
        address: 0x4001cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v10",
                block: "USART",
                ir: &usart_v10::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_USART7",
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
                    390,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    391,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    392,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    393,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    394,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    390,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    391,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    392,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    393,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    394,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART8",
        address: 0x4001d400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v12",
                block: "USART",
                ir: &usart_v12::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_USART8",
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
                    395,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    396,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    397,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    398,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    395,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    396,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    397,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    398,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART9",
        address: 0x4001d400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v12",
                block: "USART",
                ir: &usart_v12::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_USART9",
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
                    422,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    423,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    424,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    425,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    422,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    423,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    424,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    425,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USBFS",
        address: 0x40080000,
        registers: Some(
            PeripheralRegisters {
                kind: "usbfs",
                version: "v3",
                block: "USBFS",
                ir: &usbfs_v3::REGISTERS,
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
                    52,
                ),
            },
            PeripheralDmaChannel {
                signal: "SOF",
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
        name: "USBHS",
        address: 0x400c0000,
        registers: Some(
            PeripheralRegisters {
                kind: "usbhs",
                version: "common",
                block: "USBHS",
                ir: &usbhs_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_USBHS",
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
                    467,
                ),
            },
            PeripheralDmaChannel {
                signal: "REFUDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    467,
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
            #[path="../registers/adc_v14.rs"] pub mod adc_v14;
#[path="../registers/adc_v15.rs"] pub mod adc_v15;
#[path="../registers/adc_v38.rs"] pub mod adc_v38;
#[path="../registers/aes_v4.rs"] pub mod aes_v4;
#[path="../registers/aos_v4.rs"] pub mod aos_v4;
#[path="../registers/can_v4.rs"] pub mod can_v4;
#[path="../registers/cmp_v10.rs"] pub mod cmp_v10;
#[path="../registers/cmp_v9.rs"] pub mod cmp_v9;
#[path="../registers/cmu_v4.rs"] pub mod cmu_v4;
#[path="../registers/crc_v8.rs"] pub mod crc_v8;
#[path="../registers/ctc_v3.rs"] pub mod ctc_v3;
#[path="../registers/dac_v5.rs"] pub mod dac_v5;
#[path="../registers/dbgc_v4.rs"] pub mod dbgc_v4;
#[path="../registers/dcu_v4.rs"] pub mod dcu_v4;
#[path="../registers/dcu_v5.rs"] pub mod dcu_v5;
#[path="../registers/dma_v4.rs"] pub mod dma_v4;
#[path="../registers/dmc_common.rs"] pub mod dmc_common;
#[path="../registers/dvp_common.rs"] pub mod dvp_common;
#[path="../registers/efm_v4.rs"] pub mod efm_v4;
#[path="../registers/emb_v6.rs"] pub mod emb_v6;
#[path="../registers/emb_v7.rs"] pub mod emb_v7;
#[path="../registers/eth_v1.rs"] pub mod eth_v1;
#[path="../registers/fcm_v1.rs"] pub mod fcm_v1;
#[path="../registers/fmac_common.rs"] pub mod fmac_common;
#[path="../registers/gpio_v16.rs"] pub mod gpio_v16;
#[path="../registers/hash_v2.rs"] pub mod hash_v2;
#[path="../registers/hrpwm_v1.rs"] pub mod hrpwm_v1;
#[path="../registers/i_v18.rs"] pub mod i_v18;
#[path="../registers/i_v21.rs"] pub mod i_v21;
#[path="../registers/icg_v1.rs"] pub mod icg_v1;
#[path="../registers/intc_v4.rs"] pub mod intc_v4;
#[path="../registers/keyscan_common.rs"] pub mod keyscan_common;
#[path="../registers/mau_v1.rs"] pub mod mau_v1;
#[path="../registers/mpu_v4.rs"] pub mod mpu_v4;
#[path="../registers/nfc_v1.rs"] pub mod nfc_v1;
#[path="../registers/ots_v2.rs"] pub mod ots_v2;
#[path="../registers/peric_v4.rs"] pub mod peric_v4;
#[path="../registers/pwc_v4.rs"] pub mod pwc_v4;
#[path="../registers/qspi_v2.rs"] pub mod qspi_v2;
#[path="../registers/rmu_v4.rs"] pub mod rmu_v4;
#[path="../registers/rtc_v8.rs"] pub mod rtc_v8;
#[path="../registers/sdioc_common.rs"] pub mod sdioc_common;
#[path="../registers/smc_v3.rs"] pub mod smc_v3;
#[path="../registers/spi_v8.rs"] pub mod spi_v8;
#[path="../registers/sramc_v4.rs"] pub mod sramc_v4;
#[path="../registers/swdt_v1.rs"] pub mod swdt_v1;
#[path="../registers/tmr_v17.rs"] pub mod tmr_v17;
#[path="../registers/tmr_v18.rs"] pub mod tmr_v18;
#[path="../registers/tmr_v19.rs"] pub mod tmr_v19;
#[path="../registers/tmr_v20.rs"] pub mod tmr_v20;
#[path="../registers/tmr_v7.rs"] pub mod tmr_v7;
#[path="../registers/tmra_v9.rs"] pub mod tmra_v9;
#[path="../registers/trng_v3.rs"] pub mod trng_v3;
#[path="../registers/usart_v10.rs"] pub mod usart_v10;
#[path="../registers/usart_v11.rs"] pub mod usart_v11;
#[path="../registers/usart_v12.rs"] pub mod usart_v12;
#[path="../registers/usart_v13.rs"] pub mod usart_v13;
#[path="../registers/usart_v14.rs"] pub mod usart_v14;
#[path="../registers/usbfs_v3.rs"] pub mod usbfs_v3;
#[path="../registers/usbhs_common.rs"] pub mod usbhs_common;
#[path="../registers/wdt_common.rs"] pub mod wdt_common;
