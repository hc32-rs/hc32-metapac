
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x4000b800,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ll_v1_3",
            block: "ADC",
            ir: &adc_ll_v1_3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_ADC",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EOCA",
                dma: Some("DMA"),
                channel: None,
                request: Some(113),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some("DMA"),
                channel: None,
                request: Some(114),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some("DMA"),
                channel: None,
                request: Some(115),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some("DMA"),
                channel: None,
                request: Some(116),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "ADC",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "AOS",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "aos",
            version: "ll_v5",
            block: "AOS",
            ir: &aos_ll_v5::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_AOS",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "STRG",
            dma: Some("DMA"),
            channel: None,
            request: Some(124),
        }],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP",
        address: 0x4000c800,
        registers: Some(PeripheralRegisters {
            kind: "cmp",
            version: "ll_v4",
            block: "CMP",
            ir: &cmp_ll_v4::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_CMP",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "IRQ",
            dma: Some("DMA"),
            channel: None,
            request: Some(112),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "CMP",
            interrupt: "CMP",
        }],
    },
    Peripheral {
        name: "CMU",
        address: 0x40014400,
        registers: Some(PeripheralRegisters {
            kind: "cmu",
            version: "ll_v1_4",
            block: "CMU",
            ir: &cmu_ll_v1_4::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "FCM_CMU",
            interrupt: "FCM_XTAL",
        }],
    },
    Peripheral {
        name: "CRC",
        address: 0x40015400,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "ll_v1",
            block: "CRC",
            ir: &crc_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_CRC",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CTC",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "ctc",
            version: "ll_v1_2",
            block: "CTC",
            ir: &ctc_ll_v1_2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_CTC",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGC",
        address: 0x40015000,
        registers: Some(PeripheralRegisters {
            kind: "dbgc",
            version: "ll_v1",
            block: "DBGC",
            ir: &dbgc_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGC_T",
        address: 0xe0042000,
        registers: Some(PeripheralRegisters {
            kind: "dbgc_t",
            version: "ll_v1_1",
            block: "DBGC_T",
            ir: &dbgc_t_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA",
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "ll_v1_2",
            block: "DMA",
            ir: &dma_ll_v1_2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_DMA",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TC0",
                dma: Some("DMA"),
                channel: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some("DMA"),
                channel: None,
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some("DMA"),
                channel: None,
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some("DMA"),
                channel: None,
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some("DMA"),
                channel: None,
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
                dma: Some("DMA"),
                channel: None,
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "TC3",
                dma: Some("DMA"),
                channel: None,
                request: Some(14),
            },
            PeripheralDmaChannel {
                signal: "BTC3",
                dma: Some("DMA"),
                channel: None,
                request: Some(15),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "DMA",
            interrupt: "DMA",
        }],
    },
    Peripheral {
        name: "EFM",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "efm",
            version: "ll_v4",
            block: "EFM",
            ir: &efm_ll_v4::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OPTEND",
                dma: Some("DMA"),
                channel: None,
                request: Some(20),
            },
            PeripheralDmaChannel {
                signal: "DOPTEND",
                dma: Some("DMA"),
                channel: None,
                request: Some(24),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "EFM",
            interrupt: "EFM",
        }],
    },
    Peripheral {
        name: "EMB",
        address: 0x40006c00,
        registers: Some(PeripheralRegisters {
            kind: "emb",
            version: "ll_v2_4",
            block: "EMB",
            ir: &emb_ll_v2_4::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_EMB",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "EMB",
            interrupt: "EMB",
        }],
    },
    Peripheral {
        name: "FCM",
        address: 0x40005400,
        registers: Some(PeripheralRegisters {
            kind: "fcm",
            version: "ll_v1_2",
            block: "FCM",
            ir: &fcm_ll_v1_2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_FCM",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40013800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ll_v1_6",
            block: "GPIO",
            ir: &gpio_ll_v1_6::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EXTINT0_1",
                interrupt: "EXTINT00_EXTINT01",
            },
            PeripheralInterrupt {
                signal: "EXTINT2_3",
                interrupt: "EXTINT02_EXTINT03",
            },
            PeripheralInterrupt {
                signal: "EXTINT4_5",
                interrupt: "EXTINT04_EXTINT05",
            },
            PeripheralInterrupt {
                signal: "EXTINT6_7",
                interrupt: "EXTINT06_EXTINT07",
            },
        ],
    },
    Peripheral {
        name: "I2C",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ll_v2",
            block: "I2C",
            ir: &i2c_ll_v2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_I2C",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some("DMA"),
                channel: None,
                request: Some(96),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some("DMA"),
                channel: None,
                request: Some(97),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some("DMA"),
                channel: None,
                request: Some(98),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some("DMA"),
                channel: None,
                request: Some(99),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "I2C_RXFULL_TXEMPTY",
                interrupt: "I2C_RXI_TXI",
            },
            PeripheralInterrupt {
                signal: "I2C_IDLE_ERROR",
                interrupt: "I2C_TEI_EEI",
            },
        ],
    },
    Peripheral {
        name: "ICGA",
        address: 0x1000a00,
        registers: Some(PeripheralRegisters {
            kind: "icga",
            version: "ll_v1",
            block: "ICGA",
            ir: &icga_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ICGB",
        address: 0x40017800,
        registers: Some(PeripheralRegisters {
            kind: "icgb",
            version: "ll_v1",
            block: "ICGB",
            ir: &icgb_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "INTC",
        address: 0x40011000,
        registers: Some(PeripheralRegisters {
            kind: "intc",
            version: "ll_v1_1",
            block: "INTC",
            ir: &intc_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "MCAN",
        address: 0x40011400,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "ll_v1",
            block: "MCAN",
            ir: &mcan_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_MCAN",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "MCAN",
            interrupt: "MCAN",
        }],
    },
    Peripheral {
        name: "MCANRAM",
        address: 0x40011800,
        registers: Some(PeripheralRegisters {
            kind: "mcanram",
            version: "ll_v1_2",
            block: "MCANRAM",
            ir: &mcanram_ll_v1_2::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PERIC",
        address: 0x40013400,
        registers: Some(PeripheralRegisters {
            kind: "peric",
            version: "ll_v1",
            block: "PERIC",
            ir: &peric_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWC",
        address: 0x40014000,
        registers: Some(PeripheralRegisters {
            kind: "pwc",
            version: "ll_v2",
            block: "PWC",
            ir: &pwc_ll_v2::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RMU",
        address: 0x40014100,
        registers: Some(PeripheralRegisters {
            kind: "rmu",
            version: "ll_v1_1",
            block: "RMU",
            ir: &rmu_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0x4000d400,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "ll_v1_3",
            block: "RTC",
            ir: &rtc_ll_v1_3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_RTC",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ALM",
                dma: Some("DMA"),
                channel: None,
                request: Some(121),
            },
            PeripheralDmaChannel {
                signal: "PRD",
                dma: Some("DMA"),
                channel: None,
                request: Some(122),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ll_v1_1",
            block: "SPI",
            ir: &spi_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_SPI1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some("DMA"),
                channel: None,
                request: Some(86),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some("DMA"),
                channel: None,
                request: Some(87),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some("DMA"),
                channel: None,
                request: Some(88),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some("DMA"),
                channel: None,
                request: Some(89),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some("DMA"),
                channel: None,
                request: Some(90),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI1_RXFULL_TXEMPTY",
                interrupt: "SPI1_SPRI_SPTI_SPTEND",
            },
            PeripheralInterrupt {
                signal: "SPI1_IDLE_ERROR",
                interrupt: "SPI1_SPII_SPEI",
            },
        ],
    },
    Peripheral {
        name: "SPI2",
        address: 0x40003c00,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ll_v1_1",
            block: "SPI",
            ir: &spi_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_SPI2",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some("DMA"),
                channel: None,
                request: Some(91),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some("DMA"),
                channel: None,
                request: Some(92),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some("DMA"),
                channel: None,
                request: Some(93),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some("DMA"),
                channel: None,
                request: Some(94),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some("DMA"),
                channel: None,
                request: Some(95),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI2_RXFULL_TXEMPTY",
                interrupt: "SPI2_SPRI_SPTI_SPTEND",
            },
            PeripheralInterrupt {
                signal: "SPI2_IDLE_ERROR",
                interrupt: "SPI2_SPII_SPEI",
            },
        ],
    },
    Peripheral {
        name: "SRAMC",
        address: 0x40010c00,
        registers: Some(PeripheralRegisters {
            kind: "sramc",
            version: "ll_v2",
            block: "SRAMC",
            ir: &sramc_ll_v2::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SWDT",
        address: 0x4000cc00,
        registers: Some(PeripheralRegisters {
            kind: "swdt",
            version: "ll_v1_1",
            block: "SWDT",
            ir: &swdt_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR0",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "tmr0",
            version: "ll_v1_2",
            block: "TMR0",
            ir: &tmr0_ll_v1_2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMR0",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some("DMA"),
                channel: None,
                request: Some(27),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(28),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "TMR0",
            interrupt: "TMR0",
        }],
    },
    Peripheral {
        name: "TMR21",
        address: 0x40005c00,
        registers: Some(PeripheralRegisters {
            kind: "tmr2",
            version: "ll_v2_3",
            block: "TMR2",
            ir: &tmr2_ll_v2_3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMR2_1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(29),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(30),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some("DMA"),
                channel: None,
                request: Some(31),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some("DMA"),
                channel: None,
                request: Some(32),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "TMR2",
            interrupt: "TMR2",
        }],
    },
    Peripheral {
        name: "TMR22",
        address: 0x40006000,
        registers: Some(PeripheralRegisters {
            kind: "tmr2",
            version: "ll_v2_3",
            block: "TMR2",
            ir: &tmr2_ll_v2_3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMR2_2",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(33),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(34),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some("DMA"),
                channel: None,
                request: Some(35),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some("DMA"),
                channel: None,
                request: Some(36),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR4",
        address: 0x40006800,
        registers: Some(PeripheralRegisters {
            kind: "tmr4",
            version: "ll_v1_4",
            block: "TMR4",
            ir: &tmr4_ll_v1_4::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMR4",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMPUH",
                dma: Some("DMA"),
                channel: None,
                request: Some(37),
            },
            PeripheralDmaChannel {
                signal: "CMPUL",
                dma: Some("DMA"),
                channel: None,
                request: Some(38),
            },
            PeripheralDmaChannel {
                signal: "CMPVH",
                dma: Some("DMA"),
                channel: None,
                request: Some(39),
            },
            PeripheralDmaChannel {
                signal: "CMPVL",
                dma: Some("DMA"),
                channel: None,
                request: Some(40),
            },
            PeripheralDmaChannel {
                signal: "CMPWH",
                dma: Some("DMA"),
                channel: None,
                request: Some(41),
            },
            PeripheralDmaChannel {
                signal: "CMPWL",
                dma: Some("DMA"),
                channel: None,
                request: Some(42),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(43),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(44),
            },
            PeripheralDmaChannel {
                signal: "RELOADU",
                dma: Some("DMA"),
                channel: None,
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "RELOADV",
                dma: Some("DMA"),
                channel: None,
                request: Some(46),
            },
            PeripheralDmaChannel {
                signal: "RELOADW",
                dma: Some("DMA"),
                channel: None,
                request: Some(47),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some("DMA"),
                channel: None,
                request: Some(48),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some("DMA"),
                channel: None,
                request: Some(49),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some("DMA"),
                channel: None,
                request: Some(50),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some("DMA"),
                channel: None,
                request: Some(51),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some("DMA"),
                channel: None,
                request: Some(52),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some("DMA"),
                channel: None,
                request: Some(53),
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
        registers: Some(PeripheralRegisters {
            kind: "tmra",
            version: "ll_v4_1",
            block: "TMRA",
            ir: &tmra_ll_v4_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMRA",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(54),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(55),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some("DMA"),
                channel: None,
                request: Some(56),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "TMRA",
            interrupt: "TMRA",
        }],
    },
    Peripheral {
        name: "USART1",
        address: 0x40001800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v3_9",
            block: "USART",
            ir: &usart_ll_v3_9::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_USART1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some("DMA"),
                channel: None,
                request: Some(60),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some("DMA"),
                channel: None,
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some("DMA"),
                channel: None,
                request: Some(62),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some("DMA"),
                channel: None,
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some("DMA"),
                channel: None,
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some("DMA"),
                channel: None,
                request: Some(65),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART1_WKUP_RXFULL_TXEMPT",
                interrupt: "USART1_WKUP_RI_TI",
            },
            PeripheralInterrupt {
                signal: "USART1_ERR_TXEND_TXCMPL_RTO",
                interrupt: "USART1_EI_TEDNI_TCI_RTO",
            },
        ],
    },
    Peripheral {
        name: "USART2",
        address: 0x40001c00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v3_9",
            block: "USART",
            ir: &usart_ll_v3_9::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_USART2",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some("DMA"),
                channel: None,
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some("DMA"),
                channel: None,
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some("DMA"),
                channel: None,
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some("DMA"),
                channel: None,
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some("DMA"),
                channel: None,
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some("DMA"),
                channel: None,
                request: Some(71),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART2_WKUP_RXFULL_TXEMPT",
                interrupt: "USART2_RI_TI",
            },
            PeripheralInterrupt {
                signal: "USART2_ERR_TXEND_TXCMPL_RTO",
                interrupt: "USART2_EI_TEDNI_TCI_RTO",
            },
        ],
    },
    Peripheral {
        name: "USART3",
        address: 0x40002000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v3_14",
            block: "USART",
            ir: &usart_ll_v3_14::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_USART3",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some("DMA"),
                channel: None,
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some("DMA"),
                channel: None,
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some("DMA"),
                channel: None,
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some("DMA"),
                channel: None,
                request: Some(75),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some("DMA"),
                channel: None,
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some("DMA"),
                channel: None,
                request: Some(77),
            },
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some("DMA"),
                channel: None,
                request: Some(78),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART3_WKUP_RXFULL_TXEMPT",
                interrupt: "USART3_RI_TI",
            },
            PeripheralInterrupt {
                signal: "USART3_ERR_TXEND_TXCMPL_RTO",
                interrupt: "USART3_EI_TEDNI_TCI_RTO_BRKWKP",
            },
        ],
    },
    Peripheral {
        name: "USART4",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v3_14",
            block: "USART",
            ir: &usart_ll_v3_14::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_USART4",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some("DMA"),
                channel: None,
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some("DMA"),
                channel: None,
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some("DMA"),
                channel: None,
                request: Some(81),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some("DMA"),
                channel: None,
                request: Some(82),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some("DMA"),
                channel: None,
                request: Some(83),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some("DMA"),
                channel: None,
                request: Some(84),
            },
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some("DMA"),
                channel: None,
                request: Some(85),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART4_WKUP_RXFULL_TXEMPT",
                interrupt: "USART4_RI_TI",
            },
            PeripheralInterrupt {
                signal: "USART4_ERR_TXEND_TXCMPL_RTO",
                interrupt: "USART4_EI_TEDNI_TCI_RTO_BRKWKP",
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
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA_CH0",
        dma: "DMA",
        channel: 0,
    },
    DmaChannel {
        name: "DMA_CH1",
        dma: "DMA",
        channel: 1,
    },
    DmaChannel {
        name: "DMA_CH2",
        dma: "DMA",
        channel: 2,
    },
    DmaChannel {
        name: "DMA_CH3",
        dma: "DMA",
        channel: 3,
    },
];
#[path = "../registers/adc_ll_v1_3.rs"]
pub mod adc_ll_v1_3;
#[path = "../registers/aos_ll_v5.rs"]
pub mod aos_ll_v5;
#[path = "../registers/cmp_ll_v4.rs"]
pub mod cmp_ll_v4;
#[path = "../registers/cmu_ll_v1_4.rs"]
pub mod cmu_ll_v1_4;
#[path = "../registers/crc_ll_v1.rs"]
pub mod crc_ll_v1;
#[path = "../registers/ctc_ll_v1_2.rs"]
pub mod ctc_ll_v1_2;
#[path = "../registers/dbgc_ll_v1.rs"]
pub mod dbgc_ll_v1;
#[path = "../registers/dbgc_t_ll_v1_1.rs"]
pub mod dbgc_t_ll_v1_1;
#[path = "../registers/dma_ll_v1_2.rs"]
pub mod dma_ll_v1_2;
#[path = "../registers/efm_ll_v4.rs"]
pub mod efm_ll_v4;
#[path = "../registers/emb_ll_v2_4.rs"]
pub mod emb_ll_v2_4;
#[path = "../registers/fcm_ll_v1_2.rs"]
pub mod fcm_ll_v1_2;
#[path = "../registers/gpio_ll_v1_6.rs"]
pub mod gpio_ll_v1_6;
#[path = "../registers/i2c_ll_v2.rs"]
pub mod i2c_ll_v2;
#[path = "../registers/icga_ll_v1.rs"]
pub mod icga_ll_v1;
#[path = "../registers/icgb_ll_v1.rs"]
pub mod icgb_ll_v1;
#[path = "../registers/intc_ll_v1_1.rs"]
pub mod intc_ll_v1_1;
#[path = "../registers/mcan_ll_v1.rs"]
pub mod mcan_ll_v1;
#[path = "../registers/mcanram_ll_v1_2.rs"]
pub mod mcanram_ll_v1_2;
#[path = "../registers/peric_ll_v1.rs"]
pub mod peric_ll_v1;
#[path = "../registers/pwc_ll_v2.rs"]
pub mod pwc_ll_v2;
#[path = "../registers/rmu_ll_v1_1.rs"]
pub mod rmu_ll_v1_1;
#[path = "../registers/rtc_ll_v1_3.rs"]
pub mod rtc_ll_v1_3;
#[path = "../registers/spi_ll_v1_1.rs"]
pub mod spi_ll_v1_1;
#[path = "../registers/sramc_ll_v2.rs"]
pub mod sramc_ll_v2;
#[path = "../registers/swdt_ll_v1_1.rs"]
pub mod swdt_ll_v1_1;
#[path = "../registers/tmr0_ll_v1_2.rs"]
pub mod tmr0_ll_v1_2;
#[path = "../registers/tmr2_ll_v2_3.rs"]
pub mod tmr2_ll_v2_3;
#[path = "../registers/tmr4_ll_v1_4.rs"]
pub mod tmr4_ll_v1_4;
#[path = "../registers/tmra_ll_v4_1.rs"]
pub mod tmra_ll_v4_1;
#[path = "../registers/usart_ll_v3_14.rs"]
pub mod usart_ll_v3_14;
#[path = "../registers/usart_ll_v3_9.rs"]
pub mod usart_ll_v3_9;
