
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40030000,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v23",
                block: "ADC",
                ir: &adc_v23::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_ADC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EOCA",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    480,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    481,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCC",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    482,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    483,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    484,
                ),
            },
            PeripheralDmaChannel {
                signal: "DRHF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    485,
                ),
            },
            PeripheralDmaChannel {
                signal: "DRF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    486,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ADC",
                interrupt: "ADC",
            },
        ],
    },
    Peripheral {
        name: "AOS",
        address: 0x40010800,
        registers: Some(
            PeripheralRegisters {
                kind: "aos",
                version: "v10",
                block: "AOS",
                ir: &aos_v10::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_AOS",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "STRG",
                dma: Some(
                    "DMA",
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
        address: 0x4004a000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v14",
                block: "CMP",
                ir: &cmp_v14::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    416,
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
        address: 0x4004a000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v14",
                block: "CMP",
                ir: &cmp_v14::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMP2",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    417,
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
        address: 0x4004a000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v14",
                block: "CMP",
                ir: &cmp_v14::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMP3",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    418,
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
        name: "CMP_COMMON",
        address: 0x4004a000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v15",
                block: "CMP",
                ir: &cmp_v15::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CMU",
        address: 0x40054400,
        registers: Some(
            PeripheralRegisters {
                kind: "cmu",
                version: "v10",
                block: "CMU",
                ir: &cmu_v10::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "XTAL_STOP",
                interrupt: "XTAL_STOP",
            },
        ],
    },
    Peripheral {
        name: "CRC",
        address: 0x40050400,
        registers: Some(
            PeripheralRegisters {
                kind: "crc",
                version: "v9",
                block: "CRC",
                ir: &crc_v9::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_CRC",
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
                version: "v11",
                block: "DBGC",
                ir: &dbgc_v11::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DCU",
        address: 0x40052000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcu",
                version: "v10",
                block: "DCU",
                ir: &dcu_v10::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_DCU",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCU",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    55,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "DCU",
                interrupt: "DCU",
            },
        ],
    },
    Peripheral {
        name: "DMA",
        address: 0x40053000,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "v8",
                block: "DMA",
                ir: &dma_v8::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_DMA",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TC0",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    33,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    34,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    35,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    36,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    37,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    38,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC3",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    39,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC3",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    40,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "DMA_Error",
                interrupt: "DMA_ERROR",
            },
            PeripheralInterrupt {
                signal: "DMA_TC0_BTC0",
                interrupt: "DMA_TC0_BTC0",
            },
            PeripheralInterrupt {
                signal: "DMA_TC1_BTC1",
                interrupt: "DMA_TC1_BTC1",
            },
            PeripheralInterrupt {
                signal: "DMA_TC2_BTC2",
                interrupt: "DMA_TC2_BTC2",
            },
            PeripheralInterrupt {
                signal: "DMA_TC3_BTC3",
                interrupt: "DMA_TC3_BTC3",
            },
        ],
    },
    Peripheral {
        name: "EFM",
        address: 0x40010400,
        registers: Some(
            PeripheralRegisters {
                kind: "efm",
                version: "v9",
                block: "EFM",
                ir: &efm_v9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OPTEND",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    51,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PEERR_RDCOL",
                interrupt: "PEERR_RDCOL",
            },
            PeripheralInterrupt {
                signal: "OPTEND",
                interrupt: "OPTEND",
            },
        ],
    },
    Peripheral {
        name: "EMB0",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v12",
                block: "EMB",
                ir: &emb_v12::REGISTERS,
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
                version: "v12",
                block: "EMB",
                ir: &emb_v12::REGISTERS,
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
        name: "FCM",
        address: 0x40048400,
        registers: Some(
            PeripheralRegisters {
                kind: "fcm",
                version: "v4",
                block: "FCM",
                ir: &fcm_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_FCM",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FCM",
                interrupt: "FCM",
            },
        ],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40053800,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v11",
                block: "GPIO",
                ir: &gpio_v11::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "HALL",
        address: 0x4001e800,
        registers: Some(
            PeripheralRegisters {
                kind: "hall",
                version: "common",
                block: "HALL",
                ir: &hall_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_HALL",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HALL",
                interrupt: "HALL",
            },
        ],
    },
    Peripheral {
        name: "I2C",
        address: 0x4004e000,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v15",
                block: "I",
                ir: &i_v15::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_I2C",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    448,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    449,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    450,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    451,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "I2C",
                interrupt: "I2C",
            },
        ],
    },
    Peripheral {
        name: "ICG",
        address: 0x20000,
        registers: Some(
            PeripheralRegisters {
                kind: "icg",
                version: "v9",
                block: "ICG",
                ir: &icg_v9::REGISTERS,
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
                version: "v10",
                block: "INTC",
                ir: &intc_v10::REGISTERS,
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
                signal: "EXTINT0_SWINT16",
                interrupt: "EXTINT0_SWINT16",
            },
            PeripheralInterrupt {
                signal: "EXTINT1_SWINT17",
                interrupt: "EXTINT1_SWINT17",
            },
            PeripheralInterrupt {
                signal: "EXTINT2_SWINT18",
                interrupt: "EXTINT2_SWINT18",
            },
            PeripheralInterrupt {
                signal: "EXTINT3_SWINT19",
                interrupt: "EXTINT3_SWINT19",
            },
            PeripheralInterrupt {
                signal: "EXTINT4_SWINT20",
                interrupt: "EXTINT4_SWINT20",
            },
            PeripheralInterrupt {
                signal: "EXTINT5_SWINT21",
                interrupt: "EXTINT5_SWINT21",
            },
            PeripheralInterrupt {
                signal: "EXTINT6_SWINT22",
                interrupt: "EXTINT6_SWINT22",
            },
            PeripheralInterrupt {
                signal: "EXTINT7_SWINT23",
                interrupt: "EXTINT7_SWINT23",
            },
            PeripheralInterrupt {
                signal: "FPU_ERR",
                interrupt: "FPU_ERR",
            },
        ],
    },
    Peripheral {
        name: "MAU",
        address: 0x40054800,
        registers: Some(
            PeripheralRegisters {
                kind: "mau",
                version: "v3",
                block: "MAU",
                ir: &mau_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_MAU",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SQRT",
                dma: Some(
                    "DMA",
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
        name: "PERIC",
        address: 0x40054c00,
        registers: Some(
            PeripheralRegisters {
                kind: "peric",
                version: "v7",
                block: "PERIC",
                ir: &peric_v7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWC",
        address: 0x40054000,
        registers: Some(
            PeripheralRegisters {
                kind: "pwc",
                version: "v9",
                block: "PWC",
                ir: &pwc_v9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "LVD1",
                interrupt: "LVD1",
            },
            PeripheralInterrupt {
                signal: "LVD2",
                interrupt: "LVD2",
            },
        ],
    },
    Peripheral {
        name: "RMU",
        address: 0x40054100,
        registers: Some(
            PeripheralRegisters {
                kind: "rmu",
                version: "v8",
                block: "RMU",
                ir: &rmu_v8::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI",
        address: 0x4001c000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v12",
                block: "SPI",
                ir: &spi_v12::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_SPI",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    330,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    331,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    332,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    333,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    334,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI",
                interrupt: "SPI",
            },
        ],
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
                version: "v32",
                block: "TMR",
                ir: &tmr_v32::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMR0_1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    96,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    97,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    98,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    99,
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
                version: "v32",
                block: "TMR",
                ir: &tmr_v32::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMR0_2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    100,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    101,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    102,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    103,
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
        name: "TMR4",
        address: 0x40017000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v33",
                block: "TMR",
                ir: &tmr_v33::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMR4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPUH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    288,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    289,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    290,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    291,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    292,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    293,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    294,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPXL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    295,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    296,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    297,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADU",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    298,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADV",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    299,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADW",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    300,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADX",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    301,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    302,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    303,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    304,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    305,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    306,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    307,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP6",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    308,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP7",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    309,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR4_GCMP",
                interrupt: "TMR4_GCMP",
            },
            PeripheralInterrupt {
                signal: "TMR4_OVF_UDF",
                interrupt: "TMR4_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR4_RELOAD",
                interrupt: "TMR4_RELOAD",
            },
            PeripheralInterrupt {
                signal: "TMR4_SCM",
                interrupt: "TMR4_SCM",
            },
        ],
    },
    Peripheral {
        name: "TMRA1",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v13",
                block: "TMRA",
                ir: &tmra_v13::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRA_1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    139,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    140,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA",
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
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v13",
                block: "TMRA",
                ir: &tmra_v13::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRA_2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    155,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    156,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA",
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
                signal: "TMRA_2_CMP",
                interrupt: "TMRA_2_CMP",
            },
        ],
    },
    Peripheral {
        name: "TMRA3",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v13",
                block: "TMRA",
                ir: &tmra_v13::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRA_3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    171,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    172,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA",
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
                signal: "TMRA_3_CMP",
                interrupt: "TMRA_3_CMP",
            },
        ],
    },
    Peripheral {
        name: "TMRA4",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v13",
                block: "TMRA",
                ir: &tmra_v13::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRA_4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    187,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    188,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA",
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
                signal: "TMRA_4_CMP",
                interrupt: "TMRA_4_CMP",
            },
        ],
    },
    Peripheral {
        name: "TMRA5",
        address: 0x40016000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v14",
                block: "TMRA",
                ir: &tmra_v14::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRA_5",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    340,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    341,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA",
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
        name: "USART1",
        address: 0x4001d000,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v29",
                block: "USART",
                ir: &usart_v29::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_USART1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    321,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    322,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    323,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    324,
                ),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    325,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    326,
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
                signal: "USART1_WKUP",
                interrupt: "USART1_WKUP",
            },
        ],
    },
    Peripheral {
        name: "USART2",
        address: 0x4001d400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v30",
                block: "USART",
                ir: &usart_v30::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_USART2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    352,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    353,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    354,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    355,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    356,
                ),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    357,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    358,
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
                version: "v30",
                block: "USART",
                ir: &usart_v30::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_USART3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    384,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    385,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    386,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    387,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    388,
                ),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    389,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    390,
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
        address: 0x4001d400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v30",
                block: "USART",
                ir: &usart_v30::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_USART4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    391,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    392,
                ),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    393,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    394,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    395,
                ),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    396,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    397,
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
                    "DMA",
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
        name: "DMA_ERR",
        number: 24,
    },
    Interrupt {
        name: "DMA_TC0_BTC0",
        number: 25,
    },
    Interrupt {
        name: "DMA_TC1_BTC1",
        number: 26,
    },
    Interrupt {
        name: "DMA_TC2_BTC2",
        number: 27,
    },
    Interrupt {
        name: "DMA_TC3_BTC3",
        number: 28,
    },
    Interrupt {
        name: "EFM_PEERR_RDCOL",
        number: 29,
    },
    Interrupt {
        name: "EFM_OPTEND",
        number: 30,
    },
    Interrupt {
        name: "DCU",
        number: 31,
    },
    Interrupt {
        name: "FPU",
        number: 32,
    },
    Interrupt {
        name: "MAU",
        number: 33,
    },
    Interrupt {
        name: "TMR0_1",
        number: 34,
    },
    Interrupt {
        name: "TMR0_2",
        number: 35,
    },
    Interrupt {
        name: "XTAL",
        number: 36,
    },
    Interrupt {
        name: "SWDT",
        number: 37,
    },
    Interrupt {
        name: "TMRA_1_OVF_UDF",
        number: 38,
    },
    Interrupt {
        name: "TMRA_1_CMP",
        number: 39,
    },
    Interrupt {
        name: "TMRA_2_OVF_UDF",
        number: 40,
    },
    Interrupt {
        name: "TMRA_2_CMP",
        number: 41,
    },
    Interrupt {
        name: "TMRA_3_OVF_UDF",
        number: 42,
    },
    Interrupt {
        name: "TMRA_3_CMP",
        number: 43,
    },
    Interrupt {
        name: "TMRA_4_OVF_UDF",
        number: 44,
    },
    Interrupt {
        name: "TMRA_4_CMP",
        number: 45,
    },
    Interrupt {
        name: "EMB_GR0",
        number: 46,
    },
    Interrupt {
        name: "EMB_GR1",
        number: 47,
    },
    Interrupt {
        name: "TMR4_GCMP",
        number: 48,
    },
    Interrupt {
        name: "TMR4_OVF_UDF",
        number: 49,
    },
    Interrupt {
        name: "TMR4_RELOAD",
        number: 50,
    },
    Interrupt {
        name: "TMR4_SCMP",
        number: 51,
    },
    Interrupt {
        name: "USART1",
        number: 52,
    },
    Interrupt {
        name: "USART1_TCI",
        number: 53,
    },
    Interrupt {
        name: "SPI",
        number: 54,
    },
    Interrupt {
        name: "TMRA_5_OVF_UDF",
        number: 55,
    },
    Interrupt {
        name: "TMRA_5_CMP",
        number: 56,
    },
    Interrupt {
        name: "USART2",
        number: 57,
    },
    Interrupt {
        name: "USART2_TCI",
        number: 58,
    },
    Interrupt {
        name: "USART3",
        number: 59,
    },
    Interrupt {
        name: "USART3_TCI",
        number: 60,
    },
    Interrupt {
        name: "USART4",
        number: 61,
    },
    Interrupt {
        name: "USART4_TCI",
        number: 62,
    },
    Interrupt {
        name: "HALL",
        number: 63,
    },
    Interrupt {
        name: "CMP1",
        number: 64,
    },
    Interrupt {
        name: "CMP2",
        number: 65,
    },
    Interrupt {
        name: "CMP3",
        number: 66,
    },
    Interrupt {
        name: "I2C",
        number: 67,
    },
    Interrupt {
        name: "USART1_WUPI",
        number: 68,
    },
    Interrupt {
        name: "LVD1",
        number: 69,
    },
    Interrupt {
        name: "LVD2",
        number: 70,
    },
    Interrupt {
        name: "FCM",
        number: 71,
    },
    Interrupt {
        name: "WDT",
        number: 72,
    },
    Interrupt {
        name: "ADC",
        number: 73,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v23.rs"] pub mod adc_v23;
#[path="../registers/aos_v10.rs"] pub mod aos_v10;
#[path="../registers/cmp_v14.rs"] pub mod cmp_v14;
#[path="../registers/cmp_v15.rs"] pub mod cmp_v15;
#[path="../registers/cmu_v10.rs"] pub mod cmu_v10;
#[path="../registers/crc_v9.rs"] pub mod crc_v9;
#[path="../registers/dbgc_v11.rs"] pub mod dbgc_v11;
#[path="../registers/dcu_v10.rs"] pub mod dcu_v10;
#[path="../registers/dma_v8.rs"] pub mod dma_v8;
#[path="../registers/efm_v9.rs"] pub mod efm_v9;
#[path="../registers/emb_v12.rs"] pub mod emb_v12;
#[path="../registers/fcm_v4.rs"] pub mod fcm_v4;
#[path="../registers/gpio_v11.rs"] pub mod gpio_v11;
#[path="../registers/hall_common.rs"] pub mod hall_common;
#[path="../registers/i_v15.rs"] pub mod i_v15;
#[path="../registers/icg_v9.rs"] pub mod icg_v9;
#[path="../registers/intc_v10.rs"] pub mod intc_v10;
#[path="../registers/mau_v3.rs"] pub mod mau_v3;
#[path="../registers/peric_v7.rs"] pub mod peric_v7;
#[path="../registers/pwc_v9.rs"] pub mod pwc_v9;
#[path="../registers/rmu_v8.rs"] pub mod rmu_v8;
#[path="../registers/spi_v12.rs"] pub mod spi_v12;
#[path="../registers/swdt_v1.rs"] pub mod swdt_v1;
#[path="../registers/tmr_v32.rs"] pub mod tmr_v32;
#[path="../registers/tmr_v33.rs"] pub mod tmr_v33;
#[path="../registers/tmra_v13.rs"] pub mod tmra_v13;
#[path="../registers/tmra_v14.rs"] pub mod tmra_v14;
#[path="../registers/usart_v29.rs"] pub mod usart_v29;
#[path="../registers/usart_v30.rs"] pub mod usart_v30;
#[path="../registers/wdt_common.rs"] pub mod wdt_common;
