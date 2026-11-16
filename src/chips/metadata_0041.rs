
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40040000,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v35",
                block: "ADC",
                ir: &adc_v35::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "ADC_1",
                interrupt: "ADC_1",
            },
        ],
    },
    Peripheral {
        name: "ADC2",
        address: 0x40040400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v36",
                block: "ADC",
                ir: &adc_v36::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "ADC_2",
                interrupt: "ADC_2",
            },
        ],
    },
    Peripheral {
        name: "ADC3",
        address: 0x40040800,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v37",
                block: "ADC",
                ir: &adc_v37::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "ADC_3",
                interrupt: "ADC_3",
            },
        ],
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
                version: "v3",
                block: "AOS",
                ir: &aos_v3::REGISTERS,
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
        name: "CAN1",
        address: 0x40078000,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "CAN_1",
                interrupt: "CAN_1",
            },
        ],
    },
    Peripheral {
        name: "CAN2",
        address: 0x40078000,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "CAN_2",
                interrupt: "CAN_2",
            },
        ],
    },
    Peripheral {
        name: "CAN3",
        address: 0x40078000,
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
                        field: "PWC_FCG1_CAN3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CAN_3",
                interrupt: "CAN_3",
            },
        ],
    },
    Peripheral {
        name: "CMP1",
        address: 0x40038400,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v7",
                block: "CMP",
                ir: &cmp_v7::REGISTERS,
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
                    305,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    305,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CMP1",
                interrupt: "CMP1",
            },
        ],
    },
    Peripheral {
        name: "CMP2",
        address: 0x40038500,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v8",
                block: "CMP",
                ir: &cmp_v8::REGISTERS,
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
                    306,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    306,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CMP2",
                interrupt: "CMP2",
            },
        ],
    },
    Peripheral {
        name: "CMP3",
        address: 0x40038400,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v7",
                block: "CMP",
                ir: &cmp_v7::REGISTERS,
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
                    307,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    307,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CMP3",
                interrupt: "CMP3",
            },
        ],
    },
    Peripheral {
        name: "CMP4",
        address: 0x40038500,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v8",
                block: "CMP",
                ir: &cmp_v8::REGISTERS,
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
                    308,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    308,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CMP4",
                interrupt: "CMP4",
            },
        ],
    },
    Peripheral {
        name: "CMU",
        address: 0x40048000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmu",
                version: "v3",
                block: "CMU",
                ir: &cmu_v3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CMU_XTAL_STOP",
                interrupt: "CMU_XTAL_STOP",
            },
            PeripheralInterrupt {
                signal: "CMU_FCM",
                interrupt: "CMU_FCM",
            },
        ],
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
                version: "v2",
                block: "CTC",
                ir: &ctc_v2::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "CTC",
                interrupt: "CTC",
            },
        ],
    },
    Peripheral {
        name: "DAC1",
        address: 0x40041000,
        registers: Some(
            PeripheralRegisters {
                kind: "dac",
                version: "v4",
                block: "DAC",
                ir: &dac_v4::REGISTERS,
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
                version: "v4",
                block: "DAC",
                ir: &dac_v4::REGISTERS,
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
        name: "DAC3",
        address: 0x40041000,
        registers: Some(
            PeripheralRegisters {
                kind: "dac",
                version: "v4",
                block: "DAC",
                ir: &dac_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_DAC3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC4",
        address: 0x40041000,
        registers: Some(
            PeripheralRegisters {
                kind: "dac",
                version: "v4",
                block: "DAC",
                ir: &dac_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_DAC4",
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
                version: "v3",
                block: "DBGC",
                ir: &dbgc_v3::REGISTERS,
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
                version: "v3",
                block: "DCU",
                ir: &dcu_v3::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "DCU_1",
                interrupt: "DCU_1",
            },
        ],
    },
    Peripheral {
        name: "DCU2",
        address: 0x40056000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v3",
                block: "DCU",
                ir: &dcu_v3::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "DUC_2",
                interrupt: "DUC_2",
            },
        ],
    },
    Peripheral {
        name: "DCU3",
        address: 0x40056000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v3",
                block: "DCU",
                ir: &dcu_v3::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "DCU_3",
                interrupt: "DCU_3",
            },
        ],
    },
    Peripheral {
        name: "DMA1",
        address: 0x40053000,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "v3",
                block: "DMA",
                ir: &dma_v3::REGISTERS,
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
                signal: "TC6",
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
                signal: "TC7",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    47,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC7",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    48,
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
            PeripheralDmaChannel {
                signal: "TC6",
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
                signal: "TC7",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    47,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC7",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    48,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "DMA1_Error",
                interrupt: "DMA1_ERROR",
            },
            PeripheralInterrupt {
                signal: "DMA1_TC0_BTC0",
                interrupt: "DMA1_TC0_BTC0",
            },
            PeripheralInterrupt {
                signal: "DMA1_TC1_BTC1",
                interrupt: "DMA1_TC1_BTC1",
            },
            PeripheralInterrupt {
                signal: "DMA1_TC2_BTC2",
                interrupt: "DMA1_TC2_BTC2",
            },
            PeripheralInterrupt {
                signal: "DMA1_TC3_BTC3",
                interrupt: "DMA1_TC3_BTC3",
            },
            PeripheralInterrupt {
                signal: "DMA1_TC4_BTC4",
                interrupt: "DMA1_TC4_BTC4",
            },
            PeripheralInterrupt {
                signal: "DMA1_TC5_BTC5",
                interrupt: "DMA1_TC5_BTC5",
            },
            PeripheralInterrupt {
                signal: "DMA1_TC6_BTC6",
                interrupt: "DMA1_TC6_BTC6",
            },
            PeripheralInterrupt {
                signal: "DMA1_TC7_BTC7",
                interrupt: "DMA1_TC7_BTC7",
            },
        ],
    },
    Peripheral {
        name: "DMA2",
        address: 0x40053000,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "v3",
                block: "DMA",
                ir: &dma_v3::REGISTERS,
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
                signal: "TC6",
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
                signal: "TC7",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    79,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC7",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    80,
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
            PeripheralDmaChannel {
                signal: "TC6",
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
                signal: "TC7",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    79,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC7",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    80,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "DMA2_Error",
                interrupt: "DMA2_ERROR",
            },
            PeripheralInterrupt {
                signal: "DMA2_TC0_BTC0",
                interrupt: "DMA2_TC0_BTC0",
            },
            PeripheralInterrupt {
                signal: "DMA2_TC1_BTC1",
                interrupt: "DMA2_TC1_BTC1",
            },
            PeripheralInterrupt {
                signal: "DMA2_TC2_BTC2",
                interrupt: "DMA2_TC2_BTC2",
            },
            PeripheralInterrupt {
                signal: "DMA2_TC3_BTC3",
                interrupt: "DMA2_TC3_BTC3",
            },
            PeripheralInterrupt {
                signal: "DMA2_TC4_BTC4",
                interrupt: "DMA2_TC4_BTC4",
            },
            PeripheralInterrupt {
                signal: "DMA2_TC5_BTC5",
                interrupt: "DMA2_TC5_BTC5",
            },
            PeripheralInterrupt {
                signal: "DMA2_TC6_BTC6",
                interrupt: "DMA2_TC6_BTC6",
            },
            PeripheralInterrupt {
                signal: "DMA2_TC7_BTC7",
                interrupt: "DMA2_TC7_BTC7",
            },
        ],
    },
    Peripheral {
        name: "EFM",
        address: 0x40010400,
        registers: Some(
            PeripheralRegisters {
                kind: "efm",
                version: "v3",
                block: "EFM",
                ir: &efm_v3::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "EFM_PEERR_RDCOL",
                interrupt: "EFM_PEERR_RDCOL",
            },
            PeripheralInterrupt {
                signal: "EFM_OPEND",
                interrupt: "EFM_OPEND",
            },
        ],
    },
    Peripheral {
        name: "EMB0",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v4",
                block: "EMB",
                ir: &emb_v4::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EMB_GR0",
                interrupt: "EMB_GR0",
            },
        ],
    },
    Peripheral {
        name: "EMB1",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v4",
                block: "EMB",
                ir: &emb_v4::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EMB_GR1",
                interrupt: "EMB_GR1",
            },
        ],
    },
    Peripheral {
        name: "EMB2",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v4",
                block: "EMB",
                ir: &emb_v4::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EMB_GR2",
                interrupt: "EMB_GR2",
            },
        ],
    },
    Peripheral {
        name: "EMB3",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v4",
                block: "EMB",
                ir: &emb_v4::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EMB_GR3",
                interrupt: "EMB_GR3",
            },
        ],
    },
    Peripheral {
        name: "EMB4",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v4",
                block: "EMB",
                ir: &emb_v4::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EMB_GR4",
                interrupt: "EMB_GR4",
            },
        ],
    },
    Peripheral {
        name: "EMB5",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v4",
                block: "EMB",
                ir: &emb_v4::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EMB_GR5",
                interrupt: "EMB_GR5",
            },
        ],
    },
    Peripheral {
        name: "EMB6",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v4",
                block: "EMB",
                ir: &emb_v4::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EMB_GR6",
                interrupt: "EMB_GR6",
            },
        ],
    },
    Peripheral {
        name: "EMB_CTL1",
        address: 0x0,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v5",
                block: "EMB",
                ir: &emb_v5::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "FMAC_1",
                interrupt: "FMAC_1",
            },
        ],
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "FMAC_2",
                interrupt: "FMAC_2",
            },
        ],
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "FMAC_3",
                interrupt: "FMAC_3",
            },
        ],
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "FMAC_4",
                interrupt: "FMAC_4",
            },
        ],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40053800,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v9",
                block: "GPIO",
                ir: &gpio_v9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EVENT_PORT1",
                interrupt: "EVENT_PORT1",
            },
            PeripheralInterrupt {
                signal: "EVENT_PORT2",
                interrupt: "EVENT_PORT2",
            },
            PeripheralInterrupt {
                signal: "EVENT_PORT3",
                interrupt: "EVENT_PORT3",
            },
            PeripheralInterrupt {
                signal: "EVENT_PORT4",
                interrupt: "EVENT_PORT4",
            },
        ],
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
                    379,
                ),
            },
            PeripheralDmaChannel {
                signal: "HASH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    379,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CPM",
                interrupt: "CPM",
            },
        ],
    },
    Peripheral {
        name: "I2C1",
        address: 0x4003b400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v20",
                block: "I",
                ir: &i_v20::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "I2C_1",
                interrupt: "I2C_1",
            },
        ],
    },
    Peripheral {
        name: "I2C2",
        address: 0x4003b400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v20",
                block: "I",
                ir: &i_v20::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "I2C_2",
                interrupt: "I2C_2",
            },
        ],
    },
    Peripheral {
        name: "I2C3",
        address: 0x4003b400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v20",
                block: "I",
                ir: &i_v20::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "I2C_3",
                interrupt: "I2C_3",
            },
        ],
    },
    Peripheral {
        name: "ICG",
        address: 0x400,
        registers: Some(
            PeripheralRegisters {
                kind: "icg",
                version: "v3",
                block: "ICG",
                ir: &icg_v3::REGISTERS,
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
                version: "v3",
                block: "INTC",
                ir: &intc_v3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL0_SWINT0",
                interrupt: "GLOBAL0_SWINT0",
            },
            PeripheralInterrupt {
                signal: "GLOBAL1_SWINT1",
                interrupt: "GLOBAL1_SWINT1",
            },
            PeripheralInterrupt {
                signal: "GLOBAL2_SWINT2",
                interrupt: "GLOBAL2_SWINT2",
            },
            PeripheralInterrupt {
                signal: "GLOBAL3_SWINT3",
                interrupt: "GLOBAL3_SWINT3",
            },
            PeripheralInterrupt {
                signal: "GLOBAL4_SWINT4",
                interrupt: "GLOBAL4_SWINT4",
            },
            PeripheralInterrupt {
                signal: "GLOBAL5_SWINT5",
                interrupt: "GLOBAL5_SWINT5",
            },
            PeripheralInterrupt {
                signal: "GLOBAL6_SWINT6",
                interrupt: "GLOBAL6_SWINT6",
            },
            PeripheralInterrupt {
                signal: "GLOBAL7_SWINT7",
                interrupt: "GLOBAL7_SWINT7",
            },
            PeripheralInterrupt {
                signal: "GLOBAL8_SWINT8",
                interrupt: "GLOBAL8_SWINT8",
            },
            PeripheralInterrupt {
                signal: "GLOBAL9_SWINT9",
                interrupt: "GLOBAL9_SWINT9",
            },
            PeripheralInterrupt {
                signal: "GLOBAL10_SWINT10",
                interrupt: "GLOBAL10_SWINT10",
            },
            PeripheralInterrupt {
                signal: "GLOBAL11_SWINT11",
                interrupt: "GLOBAL11_SWINT11",
            },
            PeripheralInterrupt {
                signal: "GLOBAL12_SWINT12",
                interrupt: "GLOBAL12_SWINT12",
            },
            PeripheralInterrupt {
                signal: "GLOBAL13_SWINT13",
                interrupt: "GLOBAL13_SWINT13",
            },
            PeripheralInterrupt {
                signal: "GLOBAL14_SWINT14",
                interrupt: "GLOBAL14_SWINT14",
            },
            PeripheralInterrupt {
                signal: "GLOBAL15_SWINT15",
                interrupt: "GLOBAL15_SWINT15",
            },
            PeripheralInterrupt {
                signal: "GLOBAL16_EXINT0",
                interrupt: "GLOBAL16_EXINT0",
            },
            PeripheralInterrupt {
                signal: "GLOBAL17_EXINT1",
                interrupt: "GLOBAL17_EXINT1",
            },
            PeripheralInterrupt {
                signal: "GLOBAL18_EXINT2",
                interrupt: "GLOBAL18_EXINT2",
            },
            PeripheralInterrupt {
                signal: "GLOBAL19_EXINT3",
                interrupt: "GLOBAL19_EXINT3",
            },
            PeripheralInterrupt {
                signal: "GLOBAL20_EXINT4",
                interrupt: "GLOBAL20_EXINT4",
            },
            PeripheralInterrupt {
                signal: "GLOBAL21_EXINT5",
                interrupt: "GLOBAL21_EXINT5",
            },
            PeripheralInterrupt {
                signal: "GLOBAL22_EXINT6",
                interrupt: "GLOBAL22_EXINT6",
            },
            PeripheralInterrupt {
                signal: "GLOBAL23_EXINT7",
                interrupt: "GLOBAL23_EXINT7",
            },
            PeripheralInterrupt {
                signal: "GLOBAL24_EXINT8",
                interrupt: "GLOBAL24_EXINT8",
            },
            PeripheralInterrupt {
                signal: "GLOBAL25_EXINT9",
                interrupt: "GLOBAL25_EXINT9",
            },
            PeripheralInterrupt {
                signal: "GLOBAL26_EXINT10",
                interrupt: "GLOBAL26_EXINT10",
            },
            PeripheralInterrupt {
                signal: "GLOBAL27_EXINT11",
                interrupt: "GLOBAL27_EXINT11",
            },
            PeripheralInterrupt {
                signal: "GLOBAL28_EXINT12",
                interrupt: "GLOBAL28_EXINT12",
            },
            PeripheralInterrupt {
                signal: "GLOBAL29_EXINT13",
                interrupt: "GLOBAL29_EXINT13",
            },
            PeripheralInterrupt {
                signal: "GLOBAL30_EXINT14",
                interrupt: "GLOBAL30_EXINT14",
            },
            PeripheralInterrupt {
                signal: "GLOBAL31_EXINT15",
                interrupt: "GLOBAL31_EXINT15",
            },
        ],
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "MAU",
                interrupt: "MAU",
            },
        ],
    },
    Peripheral {
        name: "MDIO",
        address: 0x40028400,
        registers: Some(
            PeripheralRegisters {
                kind: "mdio",
                version: "common",
                block: "MDIO",
                ir: &mdio_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "PWC_FCG1_MDIO",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "MDIO",
                interrupt: "MDIO",
            },
        ],
    },
    Peripheral {
        name: "MPU",
        address: 0x40050000,
        registers: Some(
            PeripheralRegisters {
                kind: "mpu",
                version: "v3",
                block: "MPU",
                ir: &mpu_v3::REGISTERS,
            },
        ),
        rcc: None,
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
                    467,
                ),
            },
            PeripheralDmaChannel {
                signal: "OTS",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    467,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "OTS",
                interrupt: "OTS",
            },
        ],
    },
    Peripheral {
        name: "PERIC",
        address: 0x40055400,
        registers: Some(
            PeripheralRegisters {
                kind: "peric",
                version: "v3",
                block: "PERIC",
                ir: &peric_v3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PLA",
        address: 0x4004fc00,
        registers: Some(
            PeripheralRegisters {
                kind: "pla",
                version: "v1",
                block: "PLA",
                ir: &pla_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "PWC_FCG0_PLA",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PLA",
                interrupt: "PLA",
            },
        ],
    },
    Peripheral {
        name: "PWC",
        address: 0x40048000,
        registers: Some(
            PeripheralRegisters {
                kind: "pwc",
                version: "v3",
                block: "PWC",
                ir: &pwc_v3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PWC_WKTM",
                interrupt: "PWC_WKTM",
            },
            PeripheralInterrupt {
                signal: "PWC_PVD1",
                interrupt: "PWC_PVD1",
            },
            PeripheralInterrupt {
                signal: "PWC_PVD2",
                interrupt: "PWC_PVD2",
            },
        ],
    },
    Peripheral {
        name: "QSPI",
        address: 0x9c000000,
        registers: Some(
            PeripheralRegisters {
                kind: "qspi",
                version: "v3",
                block: "QSPI",
                ir: &qspi_v3::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "QSPI",
                interrupt: "QSPI",
            },
        ],
    },
    Peripheral {
        name: "RMU",
        address: 0x4004cce0,
        registers: Some(
            PeripheralRegisters {
                kind: "rmu",
                version: "v3",
                block: "RMU",
                ir: &rmu_v3::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "RTC",
                interrupt: "RTC",
            },
        ],
    },
    Peripheral {
        name: "SMC",
        address: 0x88000000,
        registers: Some(
            PeripheralRegisters {
                kind: "smc",
                version: "v2",
                block: "SMC",
                ir: &smc_v2::REGISTERS,
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
                    330,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    331,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    332,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    333,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    334,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    330,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    331,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    332,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    333,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    334,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI_1",
                interrupt: "SPI_1",
            },
        ],
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
                    335,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    336,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    337,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    338,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    339,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    335,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    336,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    337,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    338,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    339,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI_2",
                interrupt: "SPI_2",
            },
        ],
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
                    362,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    363,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    364,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    365,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    366,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    362,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    363,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    364,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    365,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    366,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI_3",
                interrupt: "SPI_3",
            },
        ],
    },
    Peripheral {
        name: "SPI4",
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
                    367,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    368,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    369,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    370,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    371,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    367,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    368,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    369,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    370,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    371,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI_4",
                interrupt: "SPI_4",
            },
        ],
    },
    Peripheral {
        name: "SRAMC",
        address: 0x40050800,
        registers: Some(
            PeripheralRegisters {
                kind: "sramc",
                version: "v3",
                block: "SRAMC",
                ir: &sramc_v3::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "SWDT",
                interrupt: "SWDT",
            },
        ],
    },
    Peripheral {
        name: "TMR01",
        address: 0x40024000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v12",
                block: "TMR",
                ir: &tmr_v12::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR0_1",
                interrupt: "TMR0_1",
            },
        ],
    },
    Peripheral {
        name: "TMR02",
        address: 0x40024000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v12",
                block: "TMR",
                ir: &tmr_v12::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR0_2",
                interrupt: "TMR0_2",
            },
        ],
    },
    Peripheral {
        name: "TMR21",
        address: 0x40024800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v13",
                block: "TMR",
                ir: &tmr_v13::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR2_1",
                interrupt: "TMR2_1",
            },
            PeripheralInterrupt {
                signal: "TMR2_1",
                interrupt: "TMR2_1",
            },
        ],
    },
    Peripheral {
        name: "TMR22",
        address: 0x40024800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v13",
                block: "TMR",
                ir: &tmr_v13::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR2_2",
                interrupt: "TMR2_2",
            },
            PeripheralInterrupt {
                signal: "TMR2_2",
                interrupt: "TMR2_2",
            },
        ],
    },
    Peripheral {
        name: "TMR23",
        address: 0x40024800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v13",
                block: "TMR",
                ir: &tmr_v13::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR2_3",
                interrupt: "TMR2_3",
            },
            PeripheralInterrupt {
                signal: "TMR2_3",
                interrupt: "TMR2_3",
            },
        ],
    },
    Peripheral {
        name: "TMR24",
        address: 0x40024800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v13",
                block: "TMR",
                ir: &tmr_v13::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR2_4",
                interrupt: "TMR2_4",
            },
            PeripheralInterrupt {
                signal: "TMR2_4",
                interrupt: "TMR2_4",
            },
        ],
    },
    Peripheral {
        name: "TMR4",
        address: 0x40038000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v14",
                block: "TMR",
                ir: &tmr_v14::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "1GCMPUH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    288,
                ),
            },
            PeripheralDmaChannel {
                signal: "1GCMPUL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    289,
                ),
            },
            PeripheralDmaChannel {
                signal: "1GCMPVH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    290,
                ),
            },
            PeripheralDmaChannel {
                signal: "1GCMPVL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    291,
                ),
            },
            PeripheralDmaChannel {
                signal: "1GCMPWH",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    292,
                ),
            },
            PeripheralDmaChannel {
                signal: "1GCMPWL",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    293,
                ),
            },
            PeripheralDmaChannel {
                signal: "1OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    294,
                ),
            },
            PeripheralDmaChannel {
                signal: "1UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    295,
                ),
            },
            PeripheralDmaChannel {
                signal: "1SCMP0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    299,
                ),
            },
            PeripheralDmaChannel {
                signal: "1SCMP1",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    300,
                ),
            },
            PeripheralDmaChannel {
                signal: "1SCMP2",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    301,
                ),
            },
            PeripheralDmaChannel {
                signal: "1SCMP3",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    302,
                ),
            },
            PeripheralDmaChannel {
                signal: "1SCMP4",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    303,
                ),
            },
            PeripheralDmaChannel {
                signal: "1SCMP5",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    304,
                ),
            },
            PeripheralDmaChannel {
                signal: "1GCMPUH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    288,
                ),
            },
            PeripheralDmaChannel {
                signal: "1GCMPUL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    289,
                ),
            },
            PeripheralDmaChannel {
                signal: "1GCMPVH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    290,
                ),
            },
            PeripheralDmaChannel {
                signal: "1GCMPVL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    291,
                ),
            },
            PeripheralDmaChannel {
                signal: "1GCMPWH",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    292,
                ),
            },
            PeripheralDmaChannel {
                signal: "1GCMPWL",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    293,
                ),
            },
            PeripheralDmaChannel {
                signal: "1OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    294,
                ),
            },
            PeripheralDmaChannel {
                signal: "1UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    295,
                ),
            },
            PeripheralDmaChannel {
                signal: "1SCMP0",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    299,
                ),
            },
            PeripheralDmaChannel {
                signal: "1SCMP1",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    300,
                ),
            },
            PeripheralDmaChannel {
                signal: "1SCMP2",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    301,
                ),
            },
            PeripheralDmaChannel {
                signal: "1SCMP3",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    302,
                ),
            },
            PeripheralDmaChannel {
                signal: "1SCMP4",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    303,
                ),
            },
            PeripheralDmaChannel {
                signal: "1SCMP5",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    304,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR4_1_GCMP",
                interrupt: "TMR4_1_GCMP",
            },
            PeripheralInterrupt {
                signal: "TMR4_1_OVF_UDF",
                interrupt: "TMR4_1_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR4_1_RELOAD",
                interrupt: "TMR4_1_RELOAD",
            },
            PeripheralInterrupt {
                signal: "TMR4_1_SCM",
                interrupt: "TMR4_1_SCM",
            },
        ],
    },
    Peripheral {
        name: "TMR61",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v15",
                block: "TMR",
                ir: &tmr_v15::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR6_1_GCMP",
                interrupt: "TMR6_1_GCMP",
            },
            PeripheralInterrupt {
                signal: "TMR6_1_OVF_UDF",
                interrupt: "TMR6_1_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR6_1_GDTE",
                interrupt: "TMR6_1_GDTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_1_SCMP",
                interrupt: "TMR6_1_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR610",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v15",
                block: "TMR",
                ir: &tmr_v15::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR6_10",
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
                    281,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    282,
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
                    281,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    282,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR6_10_GCMP",
                interrupt: "TMR6_10_GCMP",
            },
            PeripheralInterrupt {
                signal: "TMR6_10_OVF_UDF",
                interrupt: "TMR6_10_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR6_10_GDTE",
                interrupt: "TMR6_10_GDTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_1_SCMP",
                interrupt: "TMR6_1_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR62",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v15",
                block: "TMR",
                ir: &tmr_v15::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR6_2_GCMP",
                interrupt: "TMR6_2_GCMP",
            },
            PeripheralInterrupt {
                signal: "TMR6_2_OVF_UDF",
                interrupt: "TMR6_2_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR6_2_GDTE",
                interrupt: "TMR6_2_GDTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_2_SCMP",
                interrupt: "TMR6_2_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR63",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v15",
                block: "TMR",
                ir: &tmr_v15::REGISTERS,
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
                    169,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    170,
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
                    169,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    170,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR6_3_GCMP",
                interrupt: "TMR6_3_GCMP",
            },
            PeripheralInterrupt {
                signal: "TMR6_3_OVF_UDF",
                interrupt: "TMR6_3_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR6_3_GDTE",
                interrupt: "TMR6_3_GDTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_3_SCMP",
                interrupt: "TMR6_3_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR64",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v15",
                block: "TMR",
                ir: &tmr_v15::REGISTERS,
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
                    176,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    177,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    178,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    179,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    180,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    181,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    182,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    183,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    185,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    186,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    176,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    177,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    178,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    179,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    180,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    181,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    182,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    183,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    185,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    186,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR6_4_GCMP",
                interrupt: "TMR6_4_GCMP",
            },
            PeripheralInterrupt {
                signal: "TMR6_4_OVF_UDF",
                interrupt: "TMR6_4_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR6_4_GDTE",
                interrupt: "TMR6_4_GDTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_4_SCMP",
                interrupt: "TMR6_4_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR65",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v15",
                block: "TMR",
                ir: &tmr_v15::REGISTERS,
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
                    192,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    193,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    194,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    195,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    196,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    197,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    198,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    199,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    201,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    202,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    192,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    193,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    194,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    195,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    196,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    197,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    198,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    199,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    201,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    202,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR6_5_GCMP",
                interrupt: "TMR6_5_GCMP",
            },
            PeripheralInterrupt {
                signal: "TMR6_5_OVF_UDF",
                interrupt: "TMR6_5_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR6_5_GDTE",
                interrupt: "TMR6_5_GDTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_5_SCMP",
                interrupt: "TMR6_5_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR66",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v15",
                block: "TMR",
                ir: &tmr_v15::REGISTERS,
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
                    217,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    218,
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
                    217,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    218,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR6_6_GCMP",
                interrupt: "TMR6_6_GCMP",
            },
            PeripheralInterrupt {
                signal: "TMR6_6_OVF_UDF",
                interrupt: "TMR6_6_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR6_6_GDTE",
                interrupt: "TMR6_6_GDTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_6_SCMP",
                interrupt: "TMR6_6_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR67",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v15",
                block: "TMR",
                ir: &tmr_v15::REGISTERS,
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
                    233,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    234,
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
                    233,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    234,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR6_7_GCMP",
                interrupt: "TMR6_7_GCMP",
            },
            PeripheralInterrupt {
                signal: "TMR6_7_OVF_UDF",
                interrupt: "TMR6_7_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR6_7_GDTE",
                interrupt: "TMR6_7_GDTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_7_SCMP",
                interrupt: "TMR6_7_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR68",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v15",
                block: "TMR",
                ir: &tmr_v15::REGISTERS,
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
                    249,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    250,
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
                    249,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    250,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR6_8_GCMP",
                interrupt: "TMR6_8_GCMP",
            },
            PeripheralInterrupt {
                signal: "TMR6_8_OVF_UDF",
                interrupt: "TMR6_8_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR6_8_GDTE",
                interrupt: "TMR6_8_GDTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_8_SCMP",
                interrupt: "TMR6_8_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR69",
        address: 0x4003c000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v15",
                block: "TMR",
                ir: &tmr_v15::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG2",
                        field: "PWC_FCG2_TMR6_9",
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
                    265,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    266,
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
                    265,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    266,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR6_9_GCMP",
                interrupt: "TMR6_9_GCMP",
            },
            PeripheralInterrupt {
                signal: "TMR6_9_OVF_UDF",
                interrupt: "TMR6_9_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR6_9_GDTE",
                interrupt: "TMR6_9_GDTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_9_SCMP",
                interrupt: "TMR6_9_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR6_COMMON",
        address: 0x4003c300,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v16",
                block: "TMR",
                ir: &tmr_v16::REGISTERS,
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
                version: "v3",
                block: "TMRA",
                ir: &tmra_v3::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMRA_1_OVF_UDF",
                interrupt: "TMRA_1_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMRA_1_CMP",
                interrupt: "TMRA_1_CMP",
            },
        ],
    },
    Peripheral {
        name: "TMRA2",
        address: 0x4003a400,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v4",
                block: "TMRA",
                ir: &tmra_v4::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMRA_2_OVF_UDF",
                interrupt: "TMRA_2_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMRA_1_CMP",
                interrupt: "TMRA_1_CMP",
            },
        ],
    },
    Peripheral {
        name: "TMRA3",
        address: 0x4003a800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v5",
                block: "TMRA",
                ir: &tmra_v5::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMRA_3_OVF_UDF",
                interrupt: "TMRA_3_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMRA_2_CMP",
                interrupt: "TMRA_2_CMP",
            },
        ],
    },
    Peripheral {
        name: "TMRA4",
        address: 0x4003ac00,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v6",
                block: "TMRA",
                ir: &tmra_v6::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMRA_4_OVF_UDF",
                interrupt: "TMRA_4_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMRA_3_CMP",
                interrupt: "TMRA_3_CMP",
            },
        ],
    },
    Peripheral {
        name: "TMRA5",
        address: 0x40026000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v7",
                block: "TMRA",
                ir: &tmra_v7::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMRA_5_OVF_UDF",
                interrupt: "TMRA_5_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMRA_5_CMP",
                interrupt: "TMRA_5_CMP",
            },
        ],
    },
    Peripheral {
        name: "TMRA6",
        address: 0x40026400,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v8",
                block: "TMRA",
                ir: &tmra_v8::REGISTERS,
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
                    343,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    344,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    345,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    343,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    344,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    345,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMRA_6_OVF_UDF",
                interrupt: "TMRA_6_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMRA_6_CMP",
                interrupt: "TMRA_6_CMP",
            },
        ],
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "CPM",
                interrupt: "CPM",
            },
        ],
    },
    Peripheral {
        name: "USART1",
        address: 0x4001cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v4",
                block: "USART",
                ir: &usart_v4::REGISTERS,
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
                    320,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    321,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    322,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
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
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    320,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    321,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    322,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
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
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART1",
                interrupt: "USART1",
            },
            PeripheralInterrupt {
                signal: "USART1_TC",
                interrupt: "USART1_TC",
            },
            PeripheralInterrupt {
                signal: "USART1_WUPI",
                interrupt: "USART1_WUPI",
            },
        ],
    },
    Peripheral {
        name: "USART2",
        address: 0x4001d000,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v5",
                block: "USART",
                ir: &usart_v5::REGISTERS,
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
                    325,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    326,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    327,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    328,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    329,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    325,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    326,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    327,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    328,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    329,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART2",
                interrupt: "USART2",
            },
            PeripheralInterrupt {
                signal: "USART2_TC",
                interrupt: "USART2_TC",
            },
        ],
    },
    Peripheral {
        name: "USART3",
        address: 0x4001d400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v6",
                block: "USART",
                ir: &usart_v6::REGISTERS,
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
                    356,
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
                    356,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART3",
                interrupt: "USART3",
            },
            PeripheralInterrupt {
                signal: "USART3_TC",
                interrupt: "USART3_TC",
            },
        ],
    },
    Peripheral {
        name: "USART4",
        address: 0x40020c00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v7",
                block: "USART",
                ir: &usart_v7::REGISTERS,
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
                    357,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    358,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    359,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    360,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    361,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    357,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    358,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    359,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    360,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    361,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART4",
                interrupt: "USART4",
            },
            PeripheralInterrupt {
                signal: "USART4_TC",
                interrupt: "USART4_TC",
            },
        ],
    },
    Peripheral {
        name: "USART5",
        address: 0x40021000,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v8",
                block: "USART",
                ir: &usart_v8::REGISTERS,
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
                    384,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    385,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    386,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
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
                signal: "EI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    384,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    385,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    386,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
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
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART5",
                interrupt: "USART5",
            },
            PeripheralInterrupt {
                signal: "USART5_TC",
                interrupt: "USART5_TC",
            },
        ],
    },
    Peripheral {
        name: "USART6",
        address: 0x40021400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v9",
                block: "USART",
                ir: &usart_v9::REGISTERS,
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
                    389,
                ),
            },
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
                signal: "BRKWKPI",
                dma: Some(
                    "DMA2",
                ),
                channel: None,
                request: Some(
                    389,
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
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART6",
                interrupt: "USART6",
            },
            PeripheralInterrupt {
                signal: "USART6_TC",
                interrupt: "USART6_TC",
            },
        ],
    },
    Peripheral {
        name: "USBFS",
        address: 0x40080000,
        registers: Some(
            PeripheralRegisters {
                kind: "usbfs",
                version: "v2",
                block: "USBFS",
                ir: &usbfs_v2::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "USBFS",
                interrupt: "USBFS",
            },
        ],
    },
    Peripheral {
        name: "VREF",
        address: 0x4004f800,
        registers: Some(
            PeripheralRegisters {
                kind: "vref",
                version: "common",
                block: "VREF",
                ir: &vref_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG3",
                        field: "PWC_FCG3_VREF",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "WDT",
                interrupt: "WDT",
            },
        ],
    },
];
                pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "INT000",
        number: 0,
    },
    Interrupt {
        name: "SWINT0",
        number: 0,
    },
    Interrupt {
        name: "SWINT1",
        number: 1,
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
        name: "SWINT2",
        number: 2,
    },
    Interrupt {
        name: "SWINT3",
        number: 3,
    },
    Interrupt {
        name: "INT003",
        number: 3,
    },
    Interrupt {
        name: "SWINT4",
        number: 4,
    },
    Interrupt {
        name: "INT004",
        number: 4,
    },
    Interrupt {
        name: "SWINT5",
        number: 5,
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
        name: "SWINT6",
        number: 6,
    },
    Interrupt {
        name: "SWINT7",
        number: 7,
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
        name: "SWINT8",
        number: 8,
    },
    Interrupt {
        name: "INT009",
        number: 9,
    },
    Interrupt {
        name: "SWINT9",
        number: 9,
    },
    Interrupt {
        name: "INT010",
        number: 10,
    },
    Interrupt {
        name: "SWINT10",
        number: 10,
    },
    Interrupt {
        name: "SWINT11",
        number: 11,
    },
    Interrupt {
        name: "INT011",
        number: 11,
    },
    Interrupt {
        name: "SWINT12",
        number: 12,
    },
    Interrupt {
        name: "INT012",
        number: 12,
    },
    Interrupt {
        name: "SWINT13",
        number: 13,
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
        name: "SWINT14",
        number: 14,
    },
    Interrupt {
        name: "INT015",
        number: 15,
    },
    Interrupt {
        name: "SWINT15",
        number: 15,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ0",
        number: 16,
    },
    Interrupt {
        name: "SWINT16",
        number: 16,
    },
    Interrupt {
        name: "SWINT17",
        number: 17,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ1",
        number: 17,
    },
    Interrupt {
        name: "SWINT18",
        number: 18,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ2",
        number: 18,
    },
    Interrupt {
        name: "SWINT19",
        number: 19,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ3",
        number: 19,
    },
    Interrupt {
        name: "SWINT20",
        number: 20,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ4",
        number: 20,
    },
    Interrupt {
        name: "SWINT21",
        number: 21,
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
        name: "SWINT22",
        number: 22,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ7",
        number: 23,
    },
    Interrupt {
        name: "SWINT23",
        number: 23,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ8",
        number: 24,
    },
    Interrupt {
        name: "SWINT24",
        number: 24,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ9",
        number: 25,
    },
    Interrupt {
        name: "SWINT25",
        number: 25,
    },
    Interrupt {
        name: "SWINT26",
        number: 26,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ10",
        number: 26,
    },
    Interrupt {
        name: "SWINT27",
        number: 27,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ11",
        number: 27,
    },
    Interrupt {
        name: "SWINT28",
        number: 28,
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
        name: "SWINT29",
        number: 29,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ14",
        number: 30,
    },
    Interrupt {
        name: "SWINT30",
        number: 30,
    },
    Interrupt {
        name: "SWINT31",
        number: 31,
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
        name: "DMA1_TC6_BTC6",
        number: 39,
    },
    Interrupt {
        name: "DMA1_TC7_BTC7",
        number: 40,
    },
    Interrupt {
        name: "EFM_PEERR_RDCOL",
        number: 41,
    },
    Interrupt {
        name: "EFM_OPTEND",
        number: 42,
    },
    Interrupt {
        name: "QSPI",
        number: 43,
    },
    Interrupt {
        name: "DCU1",
        number: 44,
    },
    Interrupt {
        name: "DCU2",
        number: 45,
    },
    Interrupt {
        name: "DCU3",
        number: 46,
    },
    Interrupt {
        name: "DMA2_ERR",
        number: 47,
    },
    Interrupt {
        name: "DMA2_TC0_BTC0",
        number: 48,
    },
    Interrupt {
        name: "DMA2_TC1_BTC1",
        number: 49,
    },
    Interrupt {
        name: "DMA2_TC2_BTC2",
        number: 50,
    },
    Interrupt {
        name: "DMA2_TC3_BTC3",
        number: 51,
    },
    Interrupt {
        name: "DMA2_TC4_BTC4",
        number: 52,
    },
    Interrupt {
        name: "DMA2_TC5_BTC5",
        number: 53,
    },
    Interrupt {
        name: "DMA2_TC6_BTC6",
        number: 54,
    },
    Interrupt {
        name: "DMA2_TC7_BTC7",
        number: 55,
    },
    Interrupt {
        name: "MAU",
        number: 56,
    },
    Interrupt {
        name: "FMAC1_FIR",
        number: 57,
    },
    Interrupt {
        name: "FMAC2_FIR",
        number: 58,
    },
    Interrupt {
        name: "FMAC3_FIR",
        number: 59,
    },
    Interrupt {
        name: "FMAC4_FIR",
        number: 60,
    },
    Interrupt {
        name: "TMR0_1_CMP",
        number: 61,
    },
    Interrupt {
        name: "TMR0_2_CMP",
        number: 62,
    },
    Interrupt {
        name: "TMR2_1_CMP",
        number: 63,
    },
    Interrupt {
        name: "TMR2_1_OVF",
        number: 64,
    },
    Interrupt {
        name: "TMR2_2_CMP",
        number: 65,
    },
    Interrupt {
        name: "TMR2_2_OVF",
        number: 66,
    },
    Interrupt {
        name: "TMR2_3_CMP",
        number: 67,
    },
    Interrupt {
        name: "TMR2_3_OVF",
        number: 68,
    },
    Interrupt {
        name: "TMR2_4_CMP",
        number: 69,
    },
    Interrupt {
        name: "TMR2_4_OVF",
        number: 70,
    },
    Interrupt {
        name: "RTC",
        number: 71,
    },
    Interrupt {
        name: "XTAL",
        number: 72,
    },
    Interrupt {
        name: "WKTM",
        number: 73,
    },
    Interrupt {
        name: "SWDT",
        number: 74,
    },
    Interrupt {
        name: "TMR6_1_GCMP",
        number: 75,
    },
    Interrupt {
        name: "TMR6_1_OVF_UDF",
        number: 76,
    },
    Interrupt {
        name: "TMR6_1_DTE",
        number: 77,
    },
    Interrupt {
        name: "TMR6_1_SCMP",
        number: 78,
    },
    Interrupt {
        name: "TMRA_1_OVF_UDF",
        number: 79,
    },
    Interrupt {
        name: "TMRA_1_CMP",
        number: 80,
    },
    Interrupt {
        name: "TMR6_2_GCMP",
        number: 81,
    },
    Interrupt {
        name: "TMR6_2_OVF_UDF",
        number: 82,
    },
    Interrupt {
        name: "TMR6_2_DTE",
        number: 83,
    },
    Interrupt {
        name: "TMR6_2_SCMP",
        number: 84,
    },
    Interrupt {
        name: "TMRA_2_OVF_UDF",
        number: 85,
    },
    Interrupt {
        name: "TMRA_2_CMP",
        number: 86,
    },
    Interrupt {
        name: "TMR6_3_GCMP",
        number: 87,
    },
    Interrupt {
        name: "TMR6_3_OVF_UDF",
        number: 88,
    },
    Interrupt {
        name: "TMR6_3_DTE",
        number: 89,
    },
    Interrupt {
        name: "TMR6_3_SCMP",
        number: 90,
    },
    Interrupt {
        name: "TMRA_3_OVF_UDF",
        number: 91,
    },
    Interrupt {
        name: "TMRA_3_CMP",
        number: 92,
    },
    Interrupt {
        name: "TMR6_4_GCMP",
        number: 93,
    },
    Interrupt {
        name: "TMR6_4_OVF_UDF",
        number: 94,
    },
    Interrupt {
        name: "TMR6_4_DTE",
        number: 95,
    },
    Interrupt {
        name: "TMR6_4_SCMP",
        number: 96,
    },
    Interrupt {
        name: "TMRA_4_OVF_UDF",
        number: 97,
    },
    Interrupt {
        name: "TMRA_4_CMP",
        number: 98,
    },
    Interrupt {
        name: "TMR6_5_GCMP",
        number: 99,
    },
    Interrupt {
        name: "TMR6_5_OVF_UDF",
        number: 100,
    },
    Interrupt {
        name: "TMR6_5_DTE",
        number: 101,
    },
    Interrupt {
        name: "TMR6_5_SCMP",
        number: 102,
    },
    Interrupt {
        name: "TMR6_6_GCMP",
        number: 104,
    },
    Interrupt {
        name: "TMR6_6_OVF_UDF",
        number: 105,
    },
    Interrupt {
        name: "TMR6_6_DTE",
        number: 106,
    },
    Interrupt {
        name: "TMR6_6_SCMP",
        number: 107,
    },
    Interrupt {
        name: "TMR6_7_GCMP",
        number: 108,
    },
    Interrupt {
        name: "TMR6_7_OVF_UDF",
        number: 109,
    },
    Interrupt {
        name: "TMR6_7_DTE",
        number: 110,
    },
    Interrupt {
        name: "TMR6_7_SCMP",
        number: 111,
    },
    Interrupt {
        name: "TMR6_8_GCMP",
        number: 112,
    },
    Interrupt {
        name: "TMR6_8_OVF_UDF",
        number: 113,
    },
    Interrupt {
        name: "TMR6_8_DTE",
        number: 114,
    },
    Interrupt {
        name: "TMR6_8_SCMP",
        number: 115,
    },
    Interrupt {
        name: "TMR6_9_GCMP",
        number: 116,
    },
    Interrupt {
        name: "TMR6_9_OVF_UDF",
        number: 117,
    },
    Interrupt {
        name: "TMR6_9_DTE",
        number: 118,
    },
    Interrupt {
        name: "TMR6_9_SCMP",
        number: 119,
    },
    Interrupt {
        name: "TMR6_10_GCMP",
        number: 120,
    },
    Interrupt {
        name: "TMR6_10_OVF_UDF",
        number: 121,
    },
    Interrupt {
        name: "TMR6_10_DTE",
        number: 122,
    },
    Interrupt {
        name: "TMR6_10_SCMP",
        number: 123,
    },
    Interrupt {
        name: "TMR4_1_GCMP",
        number: 124,
    },
    Interrupt {
        name: "TMR4_1_OVF_UDF",
        number: 125,
    },
    Interrupt {
        name: "TMR4_1_RELOAD",
        number: 126,
    },
    Interrupt {
        name: "TMR4_1_SCMP",
        number: 127,
    },
    Interrupt {
        name: "CMP1",
        number: 128,
    },
    Interrupt {
        name: "CMP2",
        number: 129,
    },
    Interrupt {
        name: "CMP3",
        number: 130,
    },
    Interrupt {
        name: "CMP4",
        number: 131,
    },
    Interrupt {
        name: "CAN1_INT",
        number: 132,
    },
    Interrupt {
        name: "CAN2_INT",
        number: 133,
    },
    Interrupt {
        name: "CAN3_INT",
        number: 134,
    },
    Interrupt {
        name: "USART1",
        number: 135,
    },
    Interrupt {
        name: "USART2",
        number: 136,
    },
    Interrupt {
        name: "SPI1",
        number: 137,
    },
    Interrupt {
        name: "SPI2",
        number: 138,
    },
    Interrupt {
        name: "TMRA_5_OVF_UDF",
        number: 139,
    },
    Interrupt {
        name: "TMRA_5_CMP",
        number: 140,
    },
    Interrupt {
        name: "TMRA_6_OVF_UDF",
        number: 141,
    },
    Interrupt {
        name: "TMRA_6_CMP",
        number: 142,
    },
    Interrupt {
        name: "USBFS_GLB",
        number: 143,
    },
    Interrupt {
        name: "EVENT_PORT1",
        number: 144,
    },
    Interrupt {
        name: "EVENT_PORT2",
        number: 145,
    },
    Interrupt {
        name: "EVENT_PORT3",
        number: 146,
    },
    Interrupt {
        name: "EVENT_PORT4",
        number: 147,
    },
    Interrupt {
        name: "USART3",
        number: 148,
    },
    Interrupt {
        name: "USART4",
        number: 149,
    },
    Interrupt {
        name: "SPI3",
        number: 150,
    },
    Interrupt {
        name: "SPI4",
        number: 151,
    },
    Interrupt {
        name: "EMB_GR0",
        number: 152,
    },
    Interrupt {
        name: "EMB_GR1",
        number: 153,
    },
    Interrupt {
        name: "EMB_GR2",
        number: 154,
    },
    Interrupt {
        name: "EMB_GR3",
        number: 155,
    },
    Interrupt {
        name: "EMB_GR4",
        number: 156,
    },
    Interrupt {
        name: "EMB_GR5",
        number: 157,
    },
    Interrupt {
        name: "EMB_GR6",
        number: 158,
    },
    Interrupt {
        name: "HASH",
        number: 159,
    },
    Interrupt {
        name: "USART5",
        number: 160,
    },
    Interrupt {
        name: "USART6",
        number: 161,
    },
    Interrupt {
        name: "MDIO",
        number: 162,
    },
    Interrupt {
        name: "I2C1",
        number: 164,
    },
    Interrupt {
        name: "I2C2",
        number: 165,
    },
    Interrupt {
        name: "I2C3",
        number: 166,
    },
    Interrupt {
        name: "USART1_WUPI",
        number: 167,
    },
    Interrupt {
        name: "LVD1",
        number: 168,
    },
    Interrupt {
        name: "LVD2",
        number: 169,
    },
    Interrupt {
        name: "OTS",
        number: 170,
    },
    Interrupt {
        name: "FCM",
        number: 171,
    },
    Interrupt {
        name: "WDT",
        number: 172,
    },
    Interrupt {
        name: "CTC",
        number: 173,
    },
    Interrupt {
        name: "ADC1",
        number: 174,
    },
    Interrupt {
        name: "ADC2",
        number: 175,
    },
    Interrupt {
        name: "ADC3",
        number: 176,
    },
    Interrupt {
        name: "TRNG",
        number: 177,
    },
    Interrupt {
        name: "USART1_TCI",
        number: 178,
    },
    Interrupt {
        name: "USART2_TCI",
        number: 179,
    },
    Interrupt {
        name: "USART3_TCI",
        number: 180,
    },
    Interrupt {
        name: "USART4_TCI",
        number: 181,
    },
    Interrupt {
        name: "USART5_TCI",
        number: 182,
    },
    Interrupt {
        name: "USART6_TCI",
        number: 183,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v35.rs"] pub mod adc_v35;
#[path="../registers/adc_v36.rs"] pub mod adc_v36;
#[path="../registers/adc_v37.rs"] pub mod adc_v37;
#[path="../registers/aes_v4.rs"] pub mod aes_v4;
#[path="../registers/aos_v3.rs"] pub mod aos_v3;
#[path="../registers/can_v4.rs"] pub mod can_v4;
#[path="../registers/cmp_v7.rs"] pub mod cmp_v7;
#[path="../registers/cmp_v8.rs"] pub mod cmp_v8;
#[path="../registers/cmu_v3.rs"] pub mod cmu_v3;
#[path="../registers/crc_v8.rs"] pub mod crc_v8;
#[path="../registers/ctc_v2.rs"] pub mod ctc_v2;
#[path="../registers/dac_v4.rs"] pub mod dac_v4;
#[path="../registers/dbgc_v3.rs"] pub mod dbgc_v3;
#[path="../registers/dcu_v3.rs"] pub mod dcu_v3;
#[path="../registers/dma_v3.rs"] pub mod dma_v3;
#[path="../registers/efm_v3.rs"] pub mod efm_v3;
#[path="../registers/emb_v4.rs"] pub mod emb_v4;
#[path="../registers/emb_v5.rs"] pub mod emb_v5;
#[path="../registers/fcm_v1.rs"] pub mod fcm_v1;
#[path="../registers/fmac_common.rs"] pub mod fmac_common;
#[path="../registers/gpio_v9.rs"] pub mod gpio_v9;
#[path="../registers/hash_v2.rs"] pub mod hash_v2;
#[path="../registers/i_v20.rs"] pub mod i_v20;
#[path="../registers/icg_v3.rs"] pub mod icg_v3;
#[path="../registers/intc_v3.rs"] pub mod intc_v3;
#[path="../registers/keyscan_common.rs"] pub mod keyscan_common;
#[path="../registers/mau_v1.rs"] pub mod mau_v1;
#[path="../registers/mdio_common.rs"] pub mod mdio_common;
#[path="../registers/mpu_v3.rs"] pub mod mpu_v3;
#[path="../registers/ots_v2.rs"] pub mod ots_v2;
#[path="../registers/peric_v3.rs"] pub mod peric_v3;
#[path="../registers/pla_v1.rs"] pub mod pla_v1;
#[path="../registers/pwc_v3.rs"] pub mod pwc_v3;
#[path="../registers/qspi_v3.rs"] pub mod qspi_v3;
#[path="../registers/rmu_v3.rs"] pub mod rmu_v3;
#[path="../registers/rtc_v6.rs"] pub mod rtc_v6;
#[path="../registers/smc_v2.rs"] pub mod smc_v2;
#[path="../registers/spi_v7.rs"] pub mod spi_v7;
#[path="../registers/sramc_v3.rs"] pub mod sramc_v3;
#[path="../registers/swdt_v1.rs"] pub mod swdt_v1;
#[path="../registers/tmr_v12.rs"] pub mod tmr_v12;
#[path="../registers/tmr_v13.rs"] pub mod tmr_v13;
#[path="../registers/tmr_v14.rs"] pub mod tmr_v14;
#[path="../registers/tmr_v15.rs"] pub mod tmr_v15;
#[path="../registers/tmr_v16.rs"] pub mod tmr_v16;
#[path="../registers/tmra_v3.rs"] pub mod tmra_v3;
#[path="../registers/tmra_v4.rs"] pub mod tmra_v4;
#[path="../registers/tmra_v5.rs"] pub mod tmra_v5;
#[path="../registers/tmra_v6.rs"] pub mod tmra_v6;
#[path="../registers/tmra_v7.rs"] pub mod tmra_v7;
#[path="../registers/tmra_v8.rs"] pub mod tmra_v8;
#[path="../registers/trng_v3.rs"] pub mod trng_v3;
#[path="../registers/usart_v4.rs"] pub mod usart_v4;
#[path="../registers/usart_v5.rs"] pub mod usart_v5;
#[path="../registers/usart_v6.rs"] pub mod usart_v6;
#[path="../registers/usart_v7.rs"] pub mod usart_v7;
#[path="../registers/usart_v8.rs"] pub mod usart_v8;
#[path="../registers/usart_v9.rs"] pub mod usart_v9;
#[path="../registers/usbfs_v2.rs"] pub mod usbfs_v2;
#[path="../registers/vref_common.rs"] pub mod vref_common;
#[path="../registers/wdt_common.rs"] pub mod wdt_common;
