
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40040000,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v30",
                block: "ADC",
                ir: &adc_v30::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_ADC1",
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
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ADC1_EOCA",
                interrupt: "ADC1_EOCA",
            },
            PeripheralInterrupt {
                signal: "ADC1_EOCB",
                interrupt: "ADC1_EOCB",
            },
            PeripheralInterrupt {
                signal: "ADC1_EOCC",
                interrupt: "ADC1_EOCC",
            },
            PeripheralInterrupt {
                signal: "ADC1_AWD01",
                interrupt: "ADC1_AWD01",
            },
        ],
    },
    Peripheral {
        name: "ADC2",
        address: 0x40040400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v31",
                block: "ADC",
                ir: &adc_v31::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_ADC2",
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
                    485,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    486,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOCC",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    487,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    488,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    489,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ADC2_EOCA",
                interrupt: "ADC2_EOCA",
            },
            PeripheralInterrupt {
                signal: "ADC2_EOCB",
                interrupt: "ADC2_EOCB",
            },
            PeripheralInterrupt {
                signal: "ADC2_EOCC",
                interrupt: "ADC2_EOCC",
            },
            PeripheralInterrupt {
                signal: "ADC2_AWD01",
                interrupt: "ADC2_AWD01",
            },
        ],
    },
    Peripheral {
        name: "AOS",
        address: 0x40010800,
        registers: Some(
            PeripheralRegisters {
                kind: "aos",
                version: "v13",
                block: "AOS",
                ir: &aos_v13::REGISTERS,
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
                    264,
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
                version: "v19",
                block: "CMP",
                ir: &cmp_v19::REGISTERS,
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
                    156,
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
        address: 0x40038800,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "v19",
                block: "CMP",
                ir: &cmp_v19::REGISTERS,
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
                    157,
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
        name: "CMU",
        address: 0x40054000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmu",
                version: "v14",
                block: "CMU",
                ir: &cmu_v14::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "XTAL_STOP_SWINT10",
                interrupt: "XTAL_STOP_SWINT10",
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
                version: "v15",
                block: "DBGC",
                ir: &dbgc_v15::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DFM",
        address: 0x40050000,
        registers: Some(
            PeripheralRegisters {
                kind: "dfm",
                version: "v2",
                block: "DFM",
                ir: &dfm_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "PEDONE",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    86,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "DFM_SWINT18",
                interrupt: "DFM_SWINT18",
            },
        ],
    },
    Peripheral {
        name: "DMA",
        address: 0x40053000,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "v12",
                block: "DMA",
                ir: &dma_v12::REGISTERS,
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
                    48,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    49,
                ),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    50,
                ),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
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
                signal: "DMA_TC0_BC0_SWINT14",
                interrupt: "DMA_TC0_BC0_SWINT14",
            },
            PeripheralInterrupt {
                signal: "DMA_TC1_BTC1_SWINT15",
                interrupt: "DMA_TC1_BTC1_SWINT15",
            },
            PeripheralInterrupt {
                signal: "DMA_ERR_SWINT16",
                interrupt: "DMA_ERR_SWINT16",
            },
        ],
    },
    Peripheral {
        name: "EFM",
        address: 0x40010400,
        registers: Some(
            PeripheralRegisters {
                kind: "efm",
                version: "v14",
                block: "EFM",
                ir: &efm_v14::REGISTERS,
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
                    84,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EFM_SWINT17",
                interrupt: "EFM_SWINT17",
            },
        ],
    },
    Peripheral {
        name: "EFM2",
        address: 0x1001c50,
        registers: Some(
            PeripheralRegisters {
                kind: "efm",
                version: "v15",
                block: "EFM",
                ir: &efm_v15::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB0",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v17",
                block: "EMB",
                ir: &emb_v17::REGISTERS,
            },
        ),
        rcc: None,
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
        name: "EMB1",
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v17",
                block: "EMB",
                ir: &emb_v17::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB2",
        address: 0x40017c40,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v18",
                block: "EMB",
                ir: &emb_v18::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMB3",
        address: 0x40017c40,
        registers: Some(
            PeripheralRegisters {
                kind: "emb",
                version: "v18",
                block: "EMB",
                ir: &emb_v18::REGISTERS,
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
                version: "v15",
                block: "GPIO",
                ir: &gpio_v15::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C",
        address: 0x4003b400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v17",
                block: "I",
                ir: &i_v17::REGISTERS,
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
                    168,
                ),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    169,
                ),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    170,
                ),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    171,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "I2C_RF_SE",
                interrupt: "I2C_RF_SE",
            },
            PeripheralInterrupt {
                signal: "I2C_TC_Err",
                interrupt: "I2C_TC_ERR",
            },
        ],
    },
    Peripheral {
        name: "ICG",
        address: 0x80000,
        registers: Some(
            PeripheralRegisters {
                kind: "icg",
                version: "v11",
                block: "ICG",
                ir: &icg_v11::REGISTERS,
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
                version: "v13",
                block: "INTC",
                ir: &intc_v13::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EXTINT0_SWINT0",
                interrupt: "EXTINT0_SWINT0",
            },
            PeripheralInterrupt {
                signal: "EXTINT1_SWINT1",
                interrupt: "EXTINT1_SWINT1",
            },
            PeripheralInterrupt {
                signal: "EXTINT2_SWINT2",
                interrupt: "EXTINT2_SWINT2",
            },
            PeripheralInterrupt {
                signal: "EXTINT3_SWINT3",
                interrupt: "EXTINT3_SWINT3",
            },
            PeripheralInterrupt {
                signal: "EXTINT4_SWINT4",
                interrupt: "EXTINT4_SWINT4",
            },
            PeripheralInterrupt {
                signal: "EXTINT5_SWINT5",
                interrupt: "EXTINT5_SWINT5",
            },
            PeripheralInterrupt {
                signal: "EXTINT6_SWINT6",
                interrupt: "EXTINT6_SWINT6",
            },
            PeripheralInterrupt {
                signal: "EXTINT7_SWINT7",
                interrupt: "EXTINT7_SWINT7",
            },
            PeripheralInterrupt {
                signal: "FPU_SWINT19",
                interrupt: "FPU_SWINT19",
            },
        ],
    },
    Peripheral {
        name: "MAU",
        address: 0x40054800,
        registers: Some(
            PeripheralRegisters {
                kind: "mau",
                version: "v4",
                block: "MAU",
                ir: &mau_v4::REGISTERS,
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
                    89,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "MAU_SWINT20",
                interrupt: "MAU_SWINT20",
            },
        ],
    },
    Peripheral {
        name: "PERIC",
        address: 0x40055400,
        registers: Some(
            PeripheralRegisters {
                kind: "peric",
                version: "v11",
                block: "PERIC",
                ir: &peric_v11::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWC",
        address: 0x40054400,
        registers: Some(
            PeripheralRegisters {
                kind: "pwc",
                version: "v15",
                block: "PWC",
                ir: &pwc_v15::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "LVD1_SWINT12",
                interrupt: "LVD1_SWINT12",
            },
            PeripheralInterrupt {
                signal: "LVD2_SWINT13",
                interrupt: "LVD2_SWINT13",
            },
        ],
    },
    Peripheral {
        name: "RMU",
        address: 0x40054500,
        registers: Some(
            PeripheralRegisters {
                kind: "rmu",
                version: "v11",
                block: "RMU",
                ir: &rmu_v11::REGISTERS,
            },
        ),
        rcc: None,
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
                version: "v14",
                block: "SPI",
                ir: &spi_v14::REGISTERS,
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
                    314,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    315,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    316,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    317,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    318,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI1_RF_SE",
                interrupt: "SPI1_RF_SE",
            },
            PeripheralInterrupt {
                signal: "SPI1_Idle_Err",
                interrupt: "SPI1_IDLE_ERR",
            },
        ],
    },
    Peripheral {
        name: "SPI2",
        address: 0x4001c000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v14",
                block: "SPI",
                ir: &spi_v14::REGISTERS,
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
                    319,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    320,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEND",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    321,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    322,
                ),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    323,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI2_RF_SE",
                interrupt: "SPI2_RF_SE",
            },
            PeripheralInterrupt {
                signal: "SPI2_Idle_Err",
                interrupt: "SPI2_IDLE_ERR",
            },
        ],
    },
    Peripheral {
        name: "SRAMC",
        address: 0x40050800,
        registers: Some(
            PeripheralRegisters {
                kind: "sramc",
                version: "v9",
                block: "SRAMC",
                ir: &sramc_v9::REGISTERS,
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
                signal: "SWDT_SWINT11",
                interrupt: "SWDT_SWINT11",
            },
        ],
    },
    Peripheral {
        name: "TMR01",
        address: 0x40024000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v39",
                block: "TMR",
                ir: &tmr_v39::REGISTERS,
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
                    16,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    17,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    18,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    19,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR0_1_SWINT8",
                interrupt: "TMR0_1_SWINT8",
            },
        ],
    },
    Peripheral {
        name: "TMR02",
        address: 0x40024000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v39",
                block: "TMR",
                ir: &tmr_v39::REGISTERS,
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
                    20,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    21,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    22,
                ),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    23,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR0_2_SWINT9",
                interrupt: "TMR0_2_SWINT9",
            },
        ],
    },
    Peripheral {
        name: "TMR41",
        address: 0x40038000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v40",
                block: "TMR",
                ir: &tmr_v40::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMR4_1",
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
                    96,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    97,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADU",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    98,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADV",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    99,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADW",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    100,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    102,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    103,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    104,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    105,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    106,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    107,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    110,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    111,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    112,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    113,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    114,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    115,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR41_OVF_SWINT21",
                interrupt: "TMR41_OVF_SWINT21",
            },
            PeripheralInterrupt {
                signal: "TMR41_UDF_SWINT22",
                interrupt: "TMR41_UDF_SWINT22",
            },
            PeripheralInterrupt {
                signal: "TMR41_RELOAD_SWINT23",
                interrupt: "TMR41_RELOAD_SWINT23",
            },
            PeripheralInterrupt {
                signal: "TMR41_GCM_SWINT24",
                interrupt: "TMR41_GCM_SWINT24",
            },
            PeripheralInterrupt {
                signal: "TMR41_SCM_SWINT25",
                interrupt: "TMR41_SCM_SWINT25",
            },
        ],
    },
    Peripheral {
        name: "TMR42",
        address: 0x40038400,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v41",
                block: "TMR",
                ir: &tmr_v41::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMR4_2",
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
                    128,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    129,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADU",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    130,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADV",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    131,
                ),
            },
            PeripheralDmaChannel {
                signal: "RELOADW",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    132,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    134,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPUL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    135,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    136,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPVL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    137,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWH",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    138,
                ),
            },
            PeripheralDmaChannel {
                signal: "GCMPWL",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    139,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    142,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    143,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    144,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    145,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    146,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    147,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMR42_OVF",
                interrupt: "TMR42_OVF",
            },
            PeripheralInterrupt {
                signal: "TMR42_UDF",
                interrupt: "TMR42_UDF",
            },
            PeripheralInterrupt {
                signal: "TMR42_RELOAD",
                interrupt: "TMR42_RELOAD",
            },
            PeripheralInterrupt {
                signal: "TMR42_GCM",
                interrupt: "TMR42_GCM",
            },
            PeripheralInterrupt {
                signal: "TMR42_SCM",
                interrupt: "TMR42_SCM",
            },
        ],
    },
    Peripheral {
        name: "TMRA1",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v17",
                block: "TMRA",
                ir: &tmra_v17::REGISTERS,
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
                    118,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    119,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    120,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    184,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMRA1_OVF_SWINT26",
                interrupt: "TMRA1_OVF_SWINT26",
            },
            PeripheralInterrupt {
                signal: "TMRA1_UDF_SWINT27",
                interrupt: "TMRA1_UDF_SWINT27",
            },
            PeripheralInterrupt {
                signal: "TMRA1_CMP_SWINT28",
                interrupt: "TMRA1_CMP_SWINT28",
            },
        ],
    },
    Peripheral {
        name: "TMRA2",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v17",
                block: "TMRA",
                ir: &tmra_v17::REGISTERS,
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
                    121,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    122,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    123,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    185,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMRA2_OVF_SWINT29",
                interrupt: "TMRA2_OVF_SWINT29",
            },
            PeripheralInterrupt {
                signal: "TMRA2_UDF_SWINT30",
                interrupt: "TMRA2_UDF_SWINT30",
            },
            PeripheralInterrupt {
                signal: "TMRA2_CMP_SWINT31",
                interrupt: "TMRA2_CMP_SWINT31",
            },
        ],
    },
    Peripheral {
        name: "TMRA3",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v17",
                block: "TMRA",
                ir: &tmra_v17::REGISTERS,
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
                    150,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    151,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    152,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    186,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMRA3_OVF",
                interrupt: "TMRA3_OVF",
            },
            PeripheralInterrupt {
                signal: "TMRA3_UDF",
                interrupt: "TMRA3_UDF",
            },
            PeripheralInterrupt {
                signal: "TMRA3_CMP",
                interrupt: "TMRA3_CMP",
            },
        ],
    },
    Peripheral {
        name: "TMRA4",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v17",
                block: "TMRA",
                ir: &tmra_v17::REGISTERS,
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
                    153,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    154,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    155,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    187,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMRA4_OVF",
                interrupt: "TMRA4_OVF",
            },
            PeripheralInterrupt {
                signal: "TMRA4_UDF",
                interrupt: "TMRA4_UDF",
            },
            PeripheralInterrupt {
                signal: "TMRA4_CMP",
                interrupt: "TMRA4_CMP",
            },
        ],
    },
    Peripheral {
        name: "TMRA5",
        address: 0x4003a000,
        registers: Some(
            PeripheralRegisters {
                kind: "tmra",
                version: "v17",
                block: "TMRA",
                ir: &tmra_v17::REGISTERS,
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
                    252,
                ),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    253,
                ),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    254,
                ),
            },
            PeripheralDmaChannel {
                signal: "SCMP",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    262,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TMRA5_OVF",
                interrupt: "TMRA5_OVF",
            },
            PeripheralInterrupt {
                signal: "TMRA5_UDF",
                interrupt: "TMRA5_UDF",
            },
            PeripheralInterrupt {
                signal: "TMRA5_CMP",
                interrupt: "TMRA5_CMP",
            },
        ],
    },
    Peripheral {
        name: "TRNG",
        address: 0x40042000,
        registers: Some(
            PeripheralRegisters {
                kind: "trng",
                version: "v4",
                block: "TRNG",
                ir: &trng_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TRNG",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "END",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    495,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TRNG",
                interrupt: "TRNG",
            },
        ],
    },
    Peripheral {
        name: "USART1",
        address: 0x4001cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v35",
                block: "USART",
                ir: &usart_v35::REGISTERS,
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
                    272,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    273,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    274,
                ),
            },
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    275,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    276,
                ),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    277,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    278,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART1_RF_SE",
                interrupt: "USART1_RF_SE",
            },
            PeripheralInterrupt {
                signal: "USART1_Err_Tend_Lin_Timeout",
                interrupt: "USART1_ERR_TEND_LIN_TIMEOUT",
            },
            PeripheralInterrupt {
                signal: "USART1_TCI",
                interrupt: "USART1_TCI",
            },
            PeripheralInterrupt {
                signal: "USART1_WKUP",
                interrupt: "USART1_WKUP",
            },
        ],
    },
    Peripheral {
        name: "USART2",
        address: 0x4001cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v35",
                block: "USART",
                ir: &usart_v35::REGISTERS,
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
                    279,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    280,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    281,
                ),
            },
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    282,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    283,
                ),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    284,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    285,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART2_RF_SE",
                interrupt: "USART2_RF_SE",
            },
            PeripheralInterrupt {
                signal: "USART2_Err_Tend_Lin_Timeout",
                interrupt: "USART2_ERR_TEND_LIN_TIMEOUT",
            },
            PeripheralInterrupt {
                signal: "USART2_TCI",
                interrupt: "USART2_TCI",
            },
        ],
    },
    Peripheral {
        name: "USART3",
        address: 0x4001cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v35",
                block: "USART",
                ir: &usart_v35::REGISTERS,
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
                    286,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    287,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    288,
                ),
            },
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    289,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    290,
                ),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    291,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    292,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART3_RF_SE",
                interrupt: "USART3_RF_SE",
            },
            PeripheralInterrupt {
                signal: "USART3_Err_Tend_Lin_Timeout",
                interrupt: "USART3_ERR_TEND_LIN_TIMEOUT",
            },
            PeripheralInterrupt {
                signal: "USART3_TCI",
                interrupt: "USART3_TCI",
            },
        ],
    },
    Peripheral {
        name: "USART4",
        address: 0x4001cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v35",
                block: "USART",
                ir: &usart_v35::REGISTERS,
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
                    293,
                ),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    294,
                ),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    295,
                ),
            },
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    296,
                ),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    297,
                ),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    298,
                ),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some(
                    "DMA",
                ),
                channel: None,
                request: Some(
                    299,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART4_RF_SE",
                interrupt: "USART4_RF_SE",
            },
            PeripheralInterrupt {
                signal: "USART4_Err_Tend_Lin_Timeout",
                interrupt: "USART4_ERR_TEND_LIN_TIMEOUT",
            },
            PeripheralInterrupt {
                signal: "USART4_TCI",
                interrupt: "USART4_TCI",
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
                    409,
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
        name: "EXTINT_PORT_EIRQ0",
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
        name: "EXTINT_PORT_EIRQ1",
        number: 1,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ2",
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
        name: "EXTINT_PORT_EIRQ3",
        number: 3,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ4",
        number: 4,
    },
    Interrupt {
        name: "SWINT4",
        number: 4,
    },
    Interrupt {
        name: "SWINT5",
        number: 5,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ5",
        number: 5,
    },
    Interrupt {
        name: "SWINT6",
        number: 6,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ6",
        number: 6,
    },
    Interrupt {
        name: "SWINT7",
        number: 7,
    },
    Interrupt {
        name: "EXTINT_PORT_EIRQ7",
        number: 7,
    },
    Interrupt {
        name: "SWINT8",
        number: 8,
    },
    Interrupt {
        name: "TMR0_1",
        number: 8,
    },
    Interrupt {
        name: "TMR0_2",
        number: 9,
    },
    Interrupt {
        name: "SWINT9",
        number: 9,
    },
    Interrupt {
        name: "SWINT10",
        number: 10,
    },
    Interrupt {
        name: "XTAL",
        number: 10,
    },
    Interrupt {
        name: "SWDT",
        number: 11,
    },
    Interrupt {
        name: "SWINT11",
        number: 11,
    },
    Interrupt {
        name: "SWINT12",
        number: 12,
    },
    Interrupt {
        name: "LVD1",
        number: 12,
    },
    Interrupt {
        name: "LVD2",
        number: 13,
    },
    Interrupt {
        name: "SWINT13",
        number: 13,
    },
    Interrupt {
        name: "DMA_TC0_BTC0",
        number: 14,
    },
    Interrupt {
        name: "SWINT14",
        number: 14,
    },
    Interrupt {
        name: "DMA_TC1_BTC1",
        number: 15,
    },
    Interrupt {
        name: "SWINT15",
        number: 15,
    },
    Interrupt {
        name: "DMA_ERR",
        number: 16,
    },
    Interrupt {
        name: "SWINT16",
        number: 16,
    },
    Interrupt {
        name: "EFM",
        number: 17,
    },
    Interrupt {
        name: "SWINT17",
        number: 17,
    },
    Interrupt {
        name: "DFM",
        number: 18,
    },
    Interrupt {
        name: "SWINT18",
        number: 18,
    },
    Interrupt {
        name: "FPU",
        number: 19,
    },
    Interrupt {
        name: "SWINT19",
        number: 19,
    },
    Interrupt {
        name: "MAU",
        number: 20,
    },
    Interrupt {
        name: "SWINT20",
        number: 20,
    },
    Interrupt {
        name: "TMR4_1_OVF",
        number: 21,
    },
    Interrupt {
        name: "SWINT21",
        number: 21,
    },
    Interrupt {
        name: "SWINT22",
        number: 22,
    },
    Interrupt {
        name: "TMR4_1_UDF",
        number: 22,
    },
    Interrupt {
        name: "SWINT23",
        number: 23,
    },
    Interrupt {
        name: "TMR4_1_RELOAD",
        number: 23,
    },
    Interrupt {
        name: "SWINT24",
        number: 24,
    },
    Interrupt {
        name: "TMR4_1_GCMP",
        number: 24,
    },
    Interrupt {
        name: "SWINT25",
        number: 25,
    },
    Interrupt {
        name: "TMR4_1_SCMP",
        number: 25,
    },
    Interrupt {
        name: "SWINT26",
        number: 26,
    },
    Interrupt {
        name: "TMRA_1_OVF",
        number: 26,
    },
    Interrupt {
        name: "TMRA_1_UDF",
        number: 27,
    },
    Interrupt {
        name: "SWINT27",
        number: 27,
    },
    Interrupt {
        name: "TMRA_1_CMP",
        number: 28,
    },
    Interrupt {
        name: "SWINT28",
        number: 28,
    },
    Interrupt {
        name: "TMRA_2_OVF",
        number: 29,
    },
    Interrupt {
        name: "SWINT29",
        number: 29,
    },
    Interrupt {
        name: "TMRA_2_UDF",
        number: 30,
    },
    Interrupt {
        name: "SWINT30",
        number: 30,
    },
    Interrupt {
        name: "TMRA_2_CMP",
        number: 31,
    },
    Interrupt {
        name: "SWINT31",
        number: 31,
    },
    Interrupt {
        name: "TMR4_2_OVF",
        number: 32,
    },
    Interrupt {
        name: "TMR4_2_UDF",
        number: 33,
    },
    Interrupt {
        name: "TMR4_2_RELOAD",
        number: 34,
    },
    Interrupt {
        name: "TMR4_2_GCMP",
        number: 35,
    },
    Interrupt {
        name: "TMR4_2_SCMP",
        number: 36,
    },
    Interrupt {
        name: "TMRA_3_OVF",
        number: 37,
    },
    Interrupt {
        name: "TMRA_3_UDF",
        number: 38,
    },
    Interrupt {
        name: "TMRA_3_CMP",
        number: 39,
    },
    Interrupt {
        name: "TMRA_4_OVF",
        number: 40,
    },
    Interrupt {
        name: "TMRA_4_UDF",
        number: 41,
    },
    Interrupt {
        name: "TMRA_4_CMP",
        number: 42,
    },
    Interrupt {
        name: "CMP1",
        number: 43,
    },
    Interrupt {
        name: "CMP2",
        number: 44,
    },
    Interrupt {
        name: "EMB",
        number: 45,
    },
    Interrupt {
        name: "I2C_TX_RX",
        number: 46,
    },
    Interrupt {
        name: "I2C_TE_EE",
        number: 47,
    },
    Interrupt {
        name: "TMRA_5_OVF",
        number: 48,
    },
    Interrupt {
        name: "TMRA_5_UDF",
        number: 49,
    },
    Interrupt {
        name: "TMRA_5_CMP",
        number: 50,
    },
    Interrupt {
        name: "USART1_TX_RX",
        number: 51,
    },
    Interrupt {
        name: "USART1_EI_TE_RTO_BW",
        number: 52,
    },
    Interrupt {
        name: "USART1_TCI",
        number: 53,
    },
    Interrupt {
        name: "USART2_TX_RX",
        number: 54,
    },
    Interrupt {
        name: "USART2_EI_TE_RTO_BW",
        number: 55,
    },
    Interrupt {
        name: "USART2_TCI",
        number: 56,
    },
    Interrupt {
        name: "USART3_TX_RX",
        number: 57,
    },
    Interrupt {
        name: "USART3_EI_TE_RTO_BW",
        number: 58,
    },
    Interrupt {
        name: "USART3_TCI",
        number: 59,
    },
    Interrupt {
        name: "USART4_TX_RX",
        number: 60,
    },
    Interrupt {
        name: "USART4_EI_TE_RTO_BW",
        number: 61,
    },
    Interrupt {
        name: "USART4_TCI",
        number: 62,
    },
    Interrupt {
        name: "SPI1_TX_RX",
        number: 63,
    },
    Interrupt {
        name: "SPI1_IDLE_ERR",
        number: 64,
    },
    Interrupt {
        name: "SPI2_TX_RX",
        number: 65,
    },
    Interrupt {
        name: "SPI2_IDLE_ERR",
        number: 66,
    },
    Interrupt {
        name: "FCM",
        number: 67,
    },
    Interrupt {
        name: "WDT",
        number: 68,
    },
    Interrupt {
        name: "USART1_WUPI",
        number: 69,
    },
    Interrupt {
        name: "ADC1_ECOA",
        number: 70,
    },
    Interrupt {
        name: "ADC1_ECOB",
        number: 71,
    },
    Interrupt {
        name: "ADC1_ECOC",
        number: 72,
    },
    Interrupt {
        name: "ADC1_CMP",
        number: 73,
    },
    Interrupt {
        name: "ADC2_ECOA",
        number: 74,
    },
    Interrupt {
        name: "ADC2_ECOB",
        number: 75,
    },
    Interrupt {
        name: "ADC2_ECOC",
        number: 76,
    },
    Interrupt {
        name: "ADC2_CMP",
        number: 77,
    },
    Interrupt {
        name: "TRNG",
        number: 78,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v30.rs"] pub mod adc_v30;
#[path="../registers/adc_v31.rs"] pub mod adc_v31;
#[path="../registers/aos_v13.rs"] pub mod aos_v13;
#[path="../registers/cmp_v19.rs"] pub mod cmp_v19;
#[path="../registers/cmu_v14.rs"] pub mod cmu_v14;
#[path="../registers/crc_v9.rs"] pub mod crc_v9;
#[path="../registers/dbgc_v15.rs"] pub mod dbgc_v15;
#[path="../registers/dfm_v2.rs"] pub mod dfm_v2;
#[path="../registers/dma_v12.rs"] pub mod dma_v12;
#[path="../registers/efm_v14.rs"] pub mod efm_v14;
#[path="../registers/efm_v15.rs"] pub mod efm_v15;
#[path="../registers/emb_v17.rs"] pub mod emb_v17;
#[path="../registers/emb_v18.rs"] pub mod emb_v18;
#[path="../registers/fcm_v4.rs"] pub mod fcm_v4;
#[path="../registers/gpio_v15.rs"] pub mod gpio_v15;
#[path="../registers/i_v17.rs"] pub mod i_v17;
#[path="../registers/icg_v11.rs"] pub mod icg_v11;
#[path="../registers/intc_v13.rs"] pub mod intc_v13;
#[path="../registers/mau_v4.rs"] pub mod mau_v4;
#[path="../registers/peric_v11.rs"] pub mod peric_v11;
#[path="../registers/pwc_v15.rs"] pub mod pwc_v15;
#[path="../registers/rmu_v11.rs"] pub mod rmu_v11;
#[path="../registers/spi_v14.rs"] pub mod spi_v14;
#[path="../registers/sramc_v9.rs"] pub mod sramc_v9;
#[path="../registers/swdt_v1.rs"] pub mod swdt_v1;
#[path="../registers/tmr_v39.rs"] pub mod tmr_v39;
#[path="../registers/tmr_v40.rs"] pub mod tmr_v40;
#[path="../registers/tmr_v41.rs"] pub mod tmr_v41;
#[path="../registers/tmra_v17.rs"] pub mod tmra_v17;
#[path="../registers/trng_v4.rs"] pub mod trng_v4;
#[path="../registers/usart_v35.rs"] pub mod usart_v35;
#[path="../registers/wdt_common.rs"] pub mod wdt_common;
