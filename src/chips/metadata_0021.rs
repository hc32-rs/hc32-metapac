
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x4000b800,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v28",
                block: "ADC",
                ir: &adc_v28::REGISTERS,
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
                    113,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    114,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    115,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    116,
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
        address: 0x40000c00,
        registers: Some(
            PeripheralRegisters {
                kind: "aos",
                version: "v11",
                block: "AOS",
                ir: &aos_v11::REGISTERS,
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
                    124,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP",
        address: 0x4000c800,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v16",
                block: "CMP",
                ir: &cmp_v16::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_CMP",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IRQ",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    112,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CMP",
                interrupt: "CMP",
            },
        ],
    },
    Peripheral {
        name: "CMU",
        address: 0x40014400,
        registers: Some(
            PeripheralRegisters {
                kind: "cmu",
                version: "v11",
                block: "CMU",
                ir: &cmu_v11::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FCM_CMU",
                interrupt: "FCM_CMU",
            },
        ],
    },
    Peripheral {
        name: "CRC",
        address: 0x40015400,
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
        name: "CTC",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "ctc",
                version: "v4",
                block: "CTC",
                ir: &ctc_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_CTC",
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
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgc",
                version: "v12",
                block: "DBGC",
                ir: &dbgc_v12::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGC_T",
        address: 0xe0042000,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgc",
                version: "v13",
                block: "DBGC",
                ir: &dbgc_v13::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA",
        address: 0x40013000,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "v9",
                block: "DMA",
                ir: &dma_v9::REGISTERS,
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
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    9,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    10,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    11,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    12,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    13,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC3",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    14,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC3",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    15,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "DMA",
                interrupt: "DMA",
            },
        ],
    },
    Peripheral {
        name: "EFM",
        address: 0x40000400,
        registers: Some(
            PeripheralRegisters {
                kind: "efm",
                version: "v11",
                block: "EFM",
                ir: &efm_v11::REGISTERS,
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
                    20,
                ),
            },
            PeripheralDmaChannel {
                signal: "DOPTEND",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    24,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EFM",
                interrupt: "EFM",
            },
        ],
    },
    Peripheral {
        name: "EMB",
        address: 0x40006c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v15",
                block: "EMB",
                ir: &emb_v15::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_EMB",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EMB",
                interrupt: "EMB",
            },
        ],
    },
    Peripheral {
        name: "FCM",
        address: 0x40005400,
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
        interrupts: &[],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40013800,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v13",
                block: "GPIO",
                ir: &gpio_v13::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EXTINT0_1",
                interrupt: "EXTINT0_1",
            },
            PeripheralInterrupt {
                signal: "EXTINT2_3",
                interrupt: "EXTINT2_3",
            },
            PeripheralInterrupt {
                signal: "EXTINT4_5",
                interrupt: "EXTINT4_5",
            },
            PeripheralInterrupt {
                signal: "EXTINT6_7",
                interrupt: "EXTINT6_7",
            },
        ],
    },
    Peripheral {
        name: "I2C",
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v16",
                block: "I",
                ir: &i_v16::REGISTERS,
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
                    96,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    97,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    98,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
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
                signal: "I2C_RXFULL_TXEMPTY",
                interrupt: "I2C_RXFULL_TXEMPTY",
            },
            PeripheralInterrupt {
                signal: "I2C_IDLE_ERROR",
                interrupt: "I2C_IDLE_ERROR",
            },
        ],
    },
    Peripheral {
        name: "ICGA",
        address: 0x1000a00,
        registers: Some(
            PeripheralRegisters {
                kind: "icga",
                version: "common",
                block: "ICGA",
                ir: &icga_common::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ICGB",
        address: 0x40017800,
        registers: Some(
            PeripheralRegisters {
                kind: "icgb",
                version: "common",
                block: "ICGB",
                ir: &icgb_common::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "INTC",
        address: 0x40011000,
        registers: Some(
            PeripheralRegisters {
                kind: "intc",
                version: "v11",
                block: "INTC",
                ir: &intc_v11::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "MCAN",
        address: 0x40011400,
        registers: Some(
            PeripheralRegisters {
                kind: "mcan",
                version: "v2",
                block: "MCAN",
                ir: &mcan_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_MCAN",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "MCAN",
                interrupt: "MCAN",
            },
        ],
    },
    Peripheral {
        name: "MCANRAM",
        address: 0x40011800,
        registers: Some(
            PeripheralRegisters {
                kind: "mcanram",
                version: "v3",
                block: "MCANRAM",
                ir: &mcanram_v3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PERIC",
        address: 0x40013400,
        registers: Some(
            PeripheralRegisters {
                kind: "peric",
                version: "v9",
                block: "PERIC",
                ir: &peric_v9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWC",
        address: 0x40014000,
        registers: Some(
            PeripheralRegisters {
                kind: "pwc",
                version: "v12",
                block: "PWC",
                ir: &pwc_v12::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "LVD",
                interrupt: "LVD",
            },
        ],
    },
    Peripheral {
        name: "RMU",
        address: 0x40014100,
        registers: Some(
            PeripheralRegisters {
                kind: "rmu",
                version: "v9",
                block: "RMU",
                ir: &rmu_v9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0x4000d400,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "v12",
                block: "RTC",
                ir: &rtc_v12::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_RTC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ALM",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    121,
                ),
            },
            PeripheralDmaChannel {
                signal: "PRD",
                dma: Some(
                    "DMA",
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
        name: "SPI1",
        address: 0x40003800,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v13",
                block: "SPI",
                ir: &spi_v13::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_SPI1",
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
                    86,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    87,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    88,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    89,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    90,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI1_RXFULL_TXEMPTY",
                interrupt: "SPI1_RXFULL_TXEMPTY",
            },
            PeripheralInterrupt {
                signal: "SPI1_IDLE_ERROR",
                interrupt: "SPI1_IDLE_ERROR",
            },
        ],
    },
    Peripheral {
        name: "SPI2",
        address: 0x40003800,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v13",
                block: "SPI",
                ir: &spi_v13::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_SPI2",
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
                    91,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    92,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    93,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    94,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    95,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI2_RXFULL_TXEMPTY",
                interrupt: "SPI2_RXFULL_TXEMPTY",
            },
            PeripheralInterrupt {
                signal: "SPI2_IDLE_ERROR",
                interrupt: "SPI2_IDLE_ERROR",
            },
        ],
    },
    Peripheral {
        name: "SRAMC",
        address: 0x40010c00,
        registers: Some(
            PeripheralRegisters {
                kind: "sramc",
                version: "v7",
                block: "SRAMC",
                ir: &sramc_v7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SWDT",
        address: 0x4000cc00,
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
        name: "TMR0",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v35",
                block: "TMR",
                ir: &tmr_v35::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMR0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    27,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    28,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR0",
                interrupt: "TMR0",
            },
        ],
    },
    Peripheral {
        name: "TMR21",
        address: 0x40005c00,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v36",
                block: "TMR",
                ir: &tmr_v36::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMR2_1",
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
                    29,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    30,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    31,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    32,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR2",
                interrupt: "TMR2",
            },
        ],
    },
    Peripheral {
        name: "TMR22",
        address: 0x40005c00,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v36",
                block: "TMR",
                ir: &tmr_v36::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMR2_2",
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
                    33,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    34,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    35,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    36,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR4",
        address: 0x40006800,
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
                        register: "FCG0",
                        field: "CMU_FCG_TMR4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMPUH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    37,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPUL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    38,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPVH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    39,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPVL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    40,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPWH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    41,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPWL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    42,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    43,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    44,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADU",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    45,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADV",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    46,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADW",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    47,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    48,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    49,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    50,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    51,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    52,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    53,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR4_CMP_OVF_UDF_RELOAD",
                interrupt: "TMR4_CMP_OVF_UDF_RELOAD",
            },
            PeripheralInterrupt {
                signal: "TMR4_SCMP",
                interrupt: "TMR4_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMRA",
        address: 0x40007000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v15",
                block: "TMRA",
                ir: &tmra_v15::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRA",
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
                    54,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    55,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    56,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMRA",
                interrupt: "TMRA",
            },
        ],
    },
    Peripheral {
        name: "USART1",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v31",
                block: "USART",
                ir: &usart_v31::REGISTERS,
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
                signal: "RI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    60,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    61,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    62,
                ),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    63,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    64,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    65,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART1_WKUP_RXFULL_TXEMPT",
                interrupt: "USART1_WKUP_RXFULL_TXEMPT",
            },
            PeripheralInterrupt {
                signal: "USART1_ERR_TXEND_TXCMPL_RTO",
                interrupt: "USART1_ERR_TXEND_TXCMPL_RTO",
            },
        ],
    },
    Peripheral {
        name: "USART2",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v31",
                block: "USART",
                ir: &usart_v31::REGISTERS,
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
                signal: "RI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    66,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    67,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    68,
                ),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    69,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    70,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    71,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART2_WKUP_RXFULL_TXEMPT",
                interrupt: "USART2_WKUP_RXFULL_TXEMPT",
            },
            PeripheralInterrupt {
                signal: "USART2_ERR_TXEND_TXCMPL_RTO",
                interrupt: "USART2_ERR_TXEND_TXCMPL_RTO",
            },
        ],
    },
    Peripheral {
        name: "USART3",
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v32",
                block: "USART",
                ir: &usart_v32::REGISTERS,
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
                signal: "RI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    72,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    73,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    74,
                ),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    75,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    76,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    77,
                ),
            },
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    78,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART3_WKUP_RXFULL_TXEMPT",
                interrupt: "USART3_WKUP_RXFULL_TXEMPT",
            },
            PeripheralInterrupt {
                signal: "USART3_ERR_TXEND_TXCMPL_RTO",
                interrupt: "USART3_ERR_TXEND_TXCMPL_RTO",
            },
        ],
    },
    Peripheral {
        name: "USART4",
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v32",
                block: "USART",
                ir: &usart_v32::REGISTERS,
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
                signal: "RI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    79,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    80,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    81,
                ),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    82,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    83,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    84,
                ),
            },
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    85,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART4_WKUP_RXFULL_TXEMPT",
                interrupt: "USART4_WKUP_RXFULL_TXEMPT",
            },
            PeripheralInterrupt {
                signal: "USART4_ERR_TXEND_TXCMPL_RTO",
                interrupt: "USART4_ERR_TXEND_TXCMPL_RTO",
            },
        ],
    },
];
                pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "EXTINT00_EXTINT01",
        number: 0,
    },
    Interrupt {
        name: "EXTINT02_EXTINT03",
        number: 1,
    },
    Interrupt {
        name: "EXTINT04_EXTINT05",
        number: 2,
    },
    Interrupt {
        name: "EXTINT06_EXTINT07",
        number: 3,
    },
    Interrupt {
        name: "DMA",
        number: 4,
    },
    Interrupt {
        name: "EFM",
        number: 5,
    },
    Interrupt {
        name: "TMR0",
        number: 6,
    },
    Interrupt {
        name: "TMR2",
        number: 7,
    },
    Interrupt {
        name: "TMR4_CMP_OVF_UDF_RELOAD",
        number: 8,
    },
    Interrupt {
        name: "TMR4_SCMP",
        number: 9,
    },
    Interrupt {
        name: "TMRA",
        number: 10,
    },
    Interrupt {
        name: "USART1_WKUP_RI_TI",
        number: 11,
    },
    Interrupt {
        name: "USART1_EI_TEDNI_TCI_RTO",
        number: 12,
    },
    Interrupt {
        name: "USART2_RI_TI",
        number: 13,
    },
    Interrupt {
        name: "USART2_EI_TEDNI_TCI_RTO",
        number: 14,
    },
    Interrupt {
        name: "USART3_RI_TI",
        number: 15,
    },
    Interrupt {
        name: "USART3_EI_TEDNI_TCI_RTO_BRKWKP",
        number: 16,
    },
    Interrupt {
        name: "USART4_RI_TI",
        number: 17,
    },
    Interrupt {
        name: "USART4_EI_TEDNI_TCI_RTO_BRKWKP",
        number: 18,
    },
    Interrupt {
        name: "SPI1_SPRI_SPTI_SPTEND",
        number: 19,
    },
    Interrupt {
        name: "SPI1_SPII_SPEI",
        number: 20,
    },
    Interrupt {
        name: "SPI2_SPRI_SPTI_SPTEND",
        number: 21,
    },
    Interrupt {
        name: "SPI2_SPII_SPEI",
        number: 22,
    },
    Interrupt {
        name: "I2C_RXI_TXI",
        number: 23,
    },
    Interrupt {
        name: "I2C_TEI_EEI",
        number: 24,
    },
    Interrupt {
        name: "MCAN",
        number: 25,
    },
    Interrupt {
        name: "FCM_XTAL",
        number: 26,
    },
    Interrupt {
        name: "EMB",
        number: 27,
    },
    Interrupt {
        name: "CMP",
        number: 29,
    },
    Interrupt {
        name: "ADC",
        number: 30,
    },
    Interrupt {
        name: "EVENTPORT_RTC_CTC_SWDT",
        number: 31,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v28.rs"] pub mod adc_v28;
#[path="../registers/aos_v11.rs"] pub mod aos_v11;
#[path="../registers/cmp_v16.rs"] pub mod cmp_v16;
#[path="../registers/cmu_v11.rs"] pub mod cmu_v11;
#[path="../registers/crc_v6.rs"] pub mod crc_v6;
#[path="../registers/ctc_v4.rs"] pub mod ctc_v4;
#[path="../registers/dbgc_v12.rs"] pub mod dbgc_v12;
#[path="../registers/dbgc_v13.rs"] pub mod dbgc_v13;
#[path="../registers/dma_v9.rs"] pub mod dma_v9;
#[path="../registers/efm_v11.rs"] pub mod efm_v11;
#[path="../registers/emb_v15.rs"] pub mod emb_v15;
#[path="../registers/fcm_v4.rs"] pub mod fcm_v4;
#[path="../registers/gpio_v13.rs"] pub mod gpio_v13;
#[path="../registers/i_v16.rs"] pub mod i_v16;
#[path="../registers/icga_common.rs"] pub mod icga_common;
#[path="../registers/icgb_common.rs"] pub mod icgb_common;
#[path="../registers/intc_v11.rs"] pub mod intc_v11;
#[path="../registers/mcan_v2.rs"] pub mod mcan_v2;
#[path="../registers/mcanram_v3.rs"] pub mod mcanram_v3;
#[path="../registers/peric_v9.rs"] pub mod peric_v9;
#[path="../registers/pwc_v12.rs"] pub mod pwc_v12;
#[path="../registers/rmu_v9.rs"] pub mod rmu_v9;
#[path="../registers/rtc_v12.rs"] pub mod rtc_v12;
#[path="../registers/spi_v13.rs"] pub mod spi_v13;
#[path="../registers/sramc_v7.rs"] pub mod sramc_v7;
#[path="../registers/swdt_v1.rs"] pub mod swdt_v1;
#[path="../registers/tmr_v14.rs"] pub mod tmr_v14;
#[path="../registers/tmr_v35.rs"] pub mod tmr_v35;
#[path="../registers/tmr_v36.rs"] pub mod tmr_v36;
#[path="../registers/tmra_v15.rs"] pub mod tmra_v15;
#[path="../registers/usart_v31.rs"] pub mod usart_v31;
#[path="../registers/usart_v32.rs"] pub mod usart_v32;
