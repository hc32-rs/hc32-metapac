
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40040000,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ll_v3_6",
            block: "ADC",
            ir: &adc_ll_v3_6::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG3",
                field: "PWC_FCG3_ADC1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EOCA",
                dma: Some("DMA"),
                channel: None,
                request: Some(480),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some("DMA"),
                channel: None,
                request: Some(481),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some("DMA"),
                channel: None,
                request: Some(482),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some("DMA"),
                channel: None,
                request: Some(483),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "ADC1",
            interrupt: "ADC1",
        }],
    },
    Peripheral {
        name: "ADC2",
        address: 0x40040400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ll_v3_5",
            block: "ADC",
            ir: &adc_ll_v3_5::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG3",
                field: "PWC_FCG3_ADC2",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EOCA",
                dma: Some("DMA"),
                channel: None,
                request: Some(484),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some("DMA"),
                channel: None,
                request: Some(485),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some("DMA"),
                channel: None,
                request: Some(486),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some("DMA"),
                channel: None,
                request: Some(487),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "ADC2",
            interrupt: "ADC2",
        }],
    },
    Peripheral {
        name: "ADC3",
        address: 0x40040800,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ll_v3_2",
            block: "ADC",
            ir: &adc_ll_v3_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG3",
                field: "PWC_FCG3_ADC3",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EOCA",
                dma: Some("DMA"),
                channel: None,
                request: Some(488),
            },
            PeripheralDmaChannel {
                signal: "EOCB",
                dma: Some("DMA"),
                channel: None,
                request: Some(489),
            },
            PeripheralDmaChannel {
                signal: "CMP0",
                dma: Some("DMA"),
                channel: None,
                request: Some(490),
            },
            PeripheralDmaChannel {
                signal: "CMP1",
                dma: Some("DMA"),
                channel: None,
                request: Some(491),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "ADC3",
            interrupt: "ADC3",
        }],
    },
    Peripheral {
        name: "AOS",
        address: 0x40010800,
        registers: Some(PeripheralRegisters {
            kind: "aos",
            version: "ll_v7",
            block: "AOS",
            ir: &aos_ll_v7::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "PWC_FCG0_AOS",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "STRG",
            dma: Some("DMA"),
            channel: None,
            request: Some(415),
        }],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP1",
        address: 0x40038400,
        registers: Some(PeripheralRegisters {
            kind: "cmp",
            version: "ll_v3_5",
            block: "CMP",
            ir: &cmp_ll_v3_5::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "CMP1",
            dma: Some("DMA"),
            channel: None,
            request: Some(310),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "CMP1",
            interrupt: "CMP1",
        }],
    },
    Peripheral {
        name: "CMP2",
        address: 0x40038400,
        registers: Some(PeripheralRegisters {
            kind: "cmp",
            version: "ll_v3_5",
            block: "CMP",
            ir: &cmp_ll_v3_5::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "CMP2",
            dma: Some("DMA"),
            channel: None,
            request: Some(311),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "CMP2",
            interrupt: "CMP2",
        }],
    },
    Peripheral {
        name: "CMP3",
        address: 0x40038400,
        registers: Some(PeripheralRegisters {
            kind: "cmp",
            version: "ll_v3_5",
            block: "CMP",
            ir: &cmp_ll_v3_5::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG3",
                field: "PWC_FCG3_CMP3",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "CMP3",
            dma: Some("DMA"),
            channel: None,
            request: Some(312),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "CMP3",
            interrupt: "CMP3",
        }],
    },
    Peripheral {
        name: "CMU",
        address: 0x40048000,
        registers: Some(PeripheralRegisters {
            kind: "cmu",
            version: "ll_v3_1",
            block: "CMU",
            ir: &cmu_ll_v3_1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "XTAL_STOP",
            interrupt: "XTAL_STOP",
        }],
    },
    Peripheral {
        name: "CRC",
        address: 0x40008c00,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "ll_v2",
            block: "CRC",
            ir: &crc_ll_v2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "PWC_FCG0_CRC",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CTC",
        address: 0x40049c00,
        registers: Some(PeripheralRegisters {
            kind: "ctc",
            version: "ll_v1_1",
            block: "CTC",
            ir: &ctc_ll_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "PWC_FCG0_CTC",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "CTC",
            interrupt: "CTC",
        }],
    },
    Peripheral {
        name: "DAC1",
        address: 0x40041000,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "ll_v2",
            block: "DAC",
            ir: &dac_ll_v2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG3",
                field: "PWC_FCG3_DAC1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC2",
        address: 0x40041000,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "ll_v2",
            block: "DAC",
            ir: &dac_ll_v2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG3",
                field: "PWC_FCG3_DAC2",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGC",
        address: 0xe0042000,
        registers: Some(PeripheralRegisters {
            kind: "dbgc",
            version: "ll_v3_3",
            block: "DBGC",
            ir: &dbgc_ll_v3_3::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA",
        address: 0x40053000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "ll_v2_4",
            block: "DMA",
            ir: &dma_ll_v2_4::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "PWC_FCG0_DMA",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TC0",
                dma: Some("DMA"),
                channel: None,
                request: Some(33),
            },
            PeripheralDmaChannel {
                signal: "BTC0",
                dma: Some("DMA"),
                channel: None,
                request: Some(34),
            },
            PeripheralDmaChannel {
                signal: "TC1",
                dma: Some("DMA"),
                channel: None,
                request: Some(35),
            },
            PeripheralDmaChannel {
                signal: "BTC1",
                dma: Some("DMA"),
                channel: None,
                request: Some(36),
            },
            PeripheralDmaChannel {
                signal: "TC2",
                dma: Some("DMA"),
                channel: None,
                request: Some(37),
            },
            PeripheralDmaChannel {
                signal: "BTC2",
                dma: Some("DMA"),
                channel: None,
                request: Some(38),
            },
            PeripheralDmaChannel {
                signal: "TC3",
                dma: Some("DMA"),
                channel: None,
                request: Some(39),
            },
            PeripheralDmaChannel {
                signal: "BTC3",
                dma: Some("DMA"),
                channel: None,
                request: Some(40),
            },
            PeripheralDmaChannel {
                signal: "TC4",
                dma: Some("DMA"),
                channel: None,
                request: Some(41),
            },
            PeripheralDmaChannel {
                signal: "BTC4",
                dma: Some("DMA"),
                channel: None,
                request: Some(42),
            },
            PeripheralDmaChannel {
                signal: "TC5",
                dma: Some("DMA"),
                channel: None,
                request: Some(43),
            },
            PeripheralDmaChannel {
                signal: "BTC5",
                dma: Some("DMA"),
                channel: None,
                request: Some(44),
            },
            PeripheralDmaChannel {
                signal: "TC6",
                dma: Some("DMA"),
                channel: None,
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "BTC6",
                dma: Some("DMA"),
                channel: None,
                request: Some(46),
            },
            PeripheralDmaChannel {
                signal: "TC7",
                dma: Some("DMA"),
                channel: None,
                request: Some(47),
            },
            PeripheralDmaChannel {
                signal: "BTC7",
                dma: Some("DMA"),
                channel: None,
                request: Some(48),
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
            PeripheralInterrupt {
                signal: "DMA_TC4_BTC4",
                interrupt: "DMA_TC4_BTC4",
            },
            PeripheralInterrupt {
                signal: "DMA_TC5_BTC5",
                interrupt: "DMA_TC5_BTC5",
            },
            PeripheralInterrupt {
                signal: "DMA_TC6_BTC6",
                interrupt: "DMA_TC6_BTC6",
            },
            PeripheralInterrupt {
                signal: "DMA_TC7_BTC7",
                interrupt: "DMA_TC7_BTC7",
            },
        ],
    },
    Peripheral {
        name: "EFM",
        address: 0x40010400,
        registers: Some(PeripheralRegisters {
            kind: "efm",
            version: "ll_v2_2",
            block: "EFM",
            ir: &efm_ll_v2_2::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "OPTEND",
            dma: Some("DMA"),
            channel: None,
            request: Some(51),
        }],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PEERR_RDCOL",
                interrupt: "PEERR_RDCOL",
            },
            PeripheralInterrupt {
                signal: "OPEND",
                interrupt: "OPEND",
            },
        ],
    },
    Peripheral {
        name: "EMB0",
        address: 0x40038600,
        registers: Some(PeripheralRegisters {
            kind: "emb",
            version: "ll_v2_11",
            block: "EMB",
            ir: &emb_ll_v2_11::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "EMB_GR0",
            interrupt: "EMB_GR0",
        }],
    },
    Peripheral {
        name: "EMB1",
        address: 0x40038600,
        registers: Some(PeripheralRegisters {
            kind: "emb",
            version: "ll_v2_11",
            block: "EMB",
            ir: &emb_ll_v2_11::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "EMB_GR1",
            interrupt: "EMB_GR1",
        }],
    },
    Peripheral {
        name: "EMB2",
        address: 0x40038600,
        registers: Some(PeripheralRegisters {
            kind: "emb",
            version: "ll_v2_11",
            block: "EMB",
            ir: &emb_ll_v2_11::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "EMB_GR2",
            interrupt: "EMB_GR2",
        }],
    },
    Peripheral {
        name: "EMB3",
        address: 0x40038600,
        registers: Some(PeripheralRegisters {
            kind: "emb",
            version: "ll_v2_11",
            block: "EMB",
            ir: &emb_ll_v2_11::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "EMB_GR3",
            interrupt: "EMB_GR3",
        }],
    },
    Peripheral {
        name: "EMB4",
        address: 0x40038600,
        registers: Some(PeripheralRegisters {
            kind: "emb",
            version: "ll_v2_11",
            block: "EMB",
            ir: &emb_ll_v2_11::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "EMB_GR4",
            interrupt: "EMB_GR4",
        }],
    },
    Peripheral {
        name: "EMB5",
        address: 0x40038600,
        registers: Some(PeripheralRegisters {
            kind: "emb",
            version: "ll_v2_11",
            block: "EMB",
            ir: &emb_ll_v2_11::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "EMB_GR5",
            interrupt: "EMB_GR5",
        }],
    },
    Peripheral {
        name: "EMB6",
        address: 0x400386c0,
        registers: Some(PeripheralRegisters {
            kind: "emb",
            version: "ll_v2_10",
            block: "EMB",
            ir: &emb_ll_v2_10::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "EMB_GR6",
            interrupt: "EMB_GR6",
        }],
    },
    Peripheral {
        name: "EMB7",
        address: 0x400386c0,
        registers: Some(PeripheralRegisters {
            kind: "emb",
            version: "ll_v2_10",
            block: "EMB",
            ir: &emb_ll_v2_10::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "EMB_GR7",
            interrupt: "EMB_GR7",
        }],
    },
    Peripheral {
        name: "EMB8",
        address: 0x400386c0,
        registers: Some(PeripheralRegisters {
            kind: "emb",
            version: "ll_v2_10",
            block: "EMB",
            ir: &emb_ll_v2_10::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "EMB_GR8",
            interrupt: "EMB_GR8",
        }],
    },
    Peripheral {
        name: "FCM",
        address: 0x40048400,
        registers: Some(PeripheralRegisters {
            kind: "fcm",
            version: "ll_v1_1",
            block: "FCM",
            ir: &fcm_ll_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "PWC_FCG0_FCM",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "FCM",
            interrupt: "FCM",
        }],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40053800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ll_v2_5",
            block: "GPIO",
            ir: &gpio_ll_v2_5::REGISTERS,
        }),
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
        name: "HRPWM1",
        address: 0x4003c000,
        registers: Some(PeripheralRegisters {
            kind: "hrpwm",
            version: "v2",
            block: "HRPWM",
            ir: &hrpwm_v2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_HRPWM_1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(192),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(193),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some("DMA"),
                channel: None,
                request: Some(194),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some("DMA"),
                channel: None,
                request: Some(195),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some("DMA"),
                channel: None,
                request: Some(196),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some("DMA"),
                channel: None,
                request: Some(197),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(198),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(199),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(200),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(201),
            },
            PeripheralDmaChannel {
                signal: "GCAPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(202),
            },
            PeripheralDmaChannel {
                signal: "GCAPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(203),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HRPWM1_GCMP",
                interrupt: "HRPWM1_GCMP",
            },
            PeripheralInterrupt {
                signal: "HRPWM1_OVF_UDF",
                interrupt: "HRPWM1_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "HRPWM1_SCM",
                interrupt: "HRPWM1_SCM",
            },
            PeripheralInterrupt {
                signal: "HRPWM1_GCAP",
                interrupt: "HRPWM1_GCAP",
            },
            PeripheralInterrupt {
                signal: "HRPWM_IRQ",
                interrupt: "HRPWM_IRQ",
            },
        ],
    },
    Peripheral {
        name: "HRPWM2",
        address: 0x4003c400,
        registers: Some(PeripheralRegisters {
            kind: "hrpwm",
            version: "v3",
            block: "HRPWM",
            ir: &hrpwm_v3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_HRPWM_2",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(204),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(205),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some("DMA"),
                channel: None,
                request: Some(206),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some("DMA"),
                channel: None,
                request: Some(207),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some("DMA"),
                channel: None,
                request: Some(208),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some("DMA"),
                channel: None,
                request: Some(209),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(210),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(211),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(212),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(213),
            },
            PeripheralDmaChannel {
                signal: "GCAPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(214),
            },
            PeripheralDmaChannel {
                signal: "GCAPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(215),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HRPWM2_GCMP",
                interrupt: "HRPWM2_GCMP",
            },
            PeripheralInterrupt {
                signal: "HRPWM2_OVF_UDF",
                interrupt: "HRPWM2_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "HRPWM2_SCM",
                interrupt: "HRPWM2_SCM",
            },
            PeripheralInterrupt {
                signal: "HRPWM2_GCAP",
                interrupt: "HRPWM2_GCAP",
            },
        ],
    },
    Peripheral {
        name: "HRPWM3",
        address: 0x4003c400,
        registers: Some(PeripheralRegisters {
            kind: "hrpwm",
            version: "v3",
            block: "HRPWM",
            ir: &hrpwm_v3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_HRPWM_3",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(224),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(225),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some("DMA"),
                channel: None,
                request: Some(226),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some("DMA"),
                channel: None,
                request: Some(227),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some("DMA"),
                channel: None,
                request: Some(228),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some("DMA"),
                channel: None,
                request: Some(229),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(230),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(231),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(232),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(233),
            },
            PeripheralDmaChannel {
                signal: "GCAPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(234),
            },
            PeripheralDmaChannel {
                signal: "GCAPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(235),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HRPWM3_GCMP",
                interrupt: "HRPWM3_GCMP",
            },
            PeripheralInterrupt {
                signal: "HRPWM3_OVF_UDF",
                interrupt: "HRPWM3_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "HRPWM3_SCM",
                interrupt: "HRPWM3_SCM",
            },
            PeripheralInterrupt {
                signal: "HRPWM3_GCAP",
                interrupt: "HRPWM3_GCAP",
            },
        ],
    },
    Peripheral {
        name: "HRPWM4",
        address: 0x4003c400,
        registers: Some(PeripheralRegisters {
            kind: "hrpwm",
            version: "v3",
            block: "HRPWM",
            ir: &hrpwm_v3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_HRPWM_4",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(236),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(237),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some("DMA"),
                channel: None,
                request: Some(238),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some("DMA"),
                channel: None,
                request: Some(239),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some("DMA"),
                channel: None,
                request: Some(240),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some("DMA"),
                channel: None,
                request: Some(241),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(242),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(243),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(244),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(245),
            },
            PeripheralDmaChannel {
                signal: "GCAPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(246),
            },
            PeripheralDmaChannel {
                signal: "GCAPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(247),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HRPWM4_GCMP",
                interrupt: "HRPWM4_GCMP",
            },
            PeripheralInterrupt {
                signal: "HRPWM4_OVF_UDF",
                interrupt: "HRPWM4_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "HRPWM4_SCM",
                interrupt: "HRPWM4_SCM",
            },
            PeripheralInterrupt {
                signal: "HRPWM4_GCAP",
                interrupt: "HRPWM4_GCAP",
            },
        ],
    },
    Peripheral {
        name: "HRPWM5",
        address: 0x4003c400,
        registers: Some(PeripheralRegisters {
            kind: "hrpwm",
            version: "v3",
            block: "HRPWM",
            ir: &hrpwm_v3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_HRPWM_5",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(256),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(257),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some("DMA"),
                channel: None,
                request: Some(258),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some("DMA"),
                channel: None,
                request: Some(259),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some("DMA"),
                channel: None,
                request: Some(260),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some("DMA"),
                channel: None,
                request: Some(261),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(262),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(263),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(264),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(265),
            },
            PeripheralDmaChannel {
                signal: "GCAPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(266),
            },
            PeripheralDmaChannel {
                signal: "GCAPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(267),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HRPWM5_GCMP",
                interrupt: "HRPWM5_GCMP",
            },
            PeripheralInterrupt {
                signal: "HRPWM5_OVF_UDF",
                interrupt: "HRPWM5_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "HRPWM5_SCM",
                interrupt: "HRPWM5_SCM",
            },
            PeripheralInterrupt {
                signal: "HRPWM5_GCAP",
                interrupt: "HRPWM5_GCAP",
            },
        ],
    },
    Peripheral {
        name: "HRPWM6",
        address: 0x4003c400,
        registers: Some(PeripheralRegisters {
            kind: "hrpwm",
            version: "v3",
            block: "HRPWM",
            ir: &hrpwm_v3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_HRPWM_6",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(268),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(269),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some("DMA"),
                channel: None,
                request: Some(270),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some("DMA"),
                channel: None,
                request: Some(271),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some("DMA"),
                channel: None,
                request: Some(272),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some("DMA"),
                channel: None,
                request: Some(273),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(274),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(275),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(276),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(277),
            },
            PeripheralDmaChannel {
                signal: "GCAPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(278),
            },
            PeripheralDmaChannel {
                signal: "GCAPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(279),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HRPWM6_GCMP",
                interrupt: "HRPWM6_GCMP",
            },
            PeripheralInterrupt {
                signal: "HRPWM6_OVF_UDF",
                interrupt: "HRPWM6_OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "HRPWM6_SCM",
                interrupt: "HRPWM6_SCM",
            },
            PeripheralInterrupt {
                signal: "HRPWM6_GCAP",
                interrupt: "HRPWM6_GCAP",
            },
        ],
    },
    Peripheral {
        name: "HRPWM_COMMON",
        address: 0x4003c2f0,
        registers: Some(PeripheralRegisters {
            kind: "hrpwm",
            version: "v4",
            block: "HRPWM",
            ir: &hrpwm_v4::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C",
        address: 0x4003b400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ll_v4_2",
            block: "I2C",
            ir: &i2c_ll_v4_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "PWC_FCG1_I2C",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RXI",
                dma: Some("DMA"),
                channel: None,
                request: Some(316),
            },
            PeripheralDmaChannel {
                signal: "TXI",
                dma: Some("DMA"),
                channel: None,
                request: Some(317),
            },
            PeripheralDmaChannel {
                signal: "TEI",
                dma: Some("DMA"),
                channel: None,
                request: Some(318),
            },
            PeripheralDmaChannel {
                signal: "EEI",
                dma: Some("DMA"),
                channel: None,
                request: Some(319),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "I2C",
            interrupt: "I2C",
        }],
    },
    Peripheral {
        name: "ICG",
        address: 0x400,
        registers: Some(PeripheralRegisters {
            kind: "icg",
            version: "ll_v1_1",
            block: "ICG",
            ir: &icg_ll_v1_1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "INTC",
        address: 0x40051000,
        registers: Some(PeripheralRegisters {
            kind: "intc",
            version: "ll_v2_7",
            block: "INTC",
            ir: &intc_ll_v2_7::REGISTERS,
        }),
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
                signal: "FPU_ERR",
                interrupt: "FPU_ERR",
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
        name: "MCAN1",
        address: 0x40071000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "ll_v1_1",
            block: "MCAN",
            ir: &mcan_ll_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "PWC_FCG1_MCAN1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CAN1_INT0",
                interrupt: "CAN1_INT0",
            },
            PeripheralInterrupt {
                signal: "CAN1_INT1",
                interrupt: "CAN1_INT1",
            },
        ],
    },
    Peripheral {
        name: "MCAN2",
        address: 0x40071000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "ll_v1_1",
            block: "MCAN",
            ir: &mcan_ll_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "PWC_FCG1_MCAN2",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CAN2_INT0",
                interrupt: "CAN2_INT0",
            },
            PeripheralInterrupt {
                signal: "CAN2_INT1",
                interrupt: "CAN2_INT1",
            },
        ],
    },
    Peripheral {
        name: "MCANRAM",
        address: 0x40073800,
        registers: Some(PeripheralRegisters {
            kind: "mcanram",
            version: "ll_v1_1",
            block: "MCANRAM",
            ir: &mcanram_ll_v1_1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "CANRAM_ECCER",
            interrupt: "CANRAM_ECCER",
        }],
    },
    Peripheral {
        name: "MPU",
        address: 0x40050000,
        registers: Some(PeripheralRegisters {
            kind: "mpu",
            version: "v6",
            block: "MPU",
            ir: &mpu_v6::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PERIC",
        address: 0x40055400,
        registers: Some(PeripheralRegisters {
            kind: "peric",
            version: "ll_v1_3",
            block: "PERIC",
            ir: &peric_ll_v1_3::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PLA",
        address: 0x4004fc00,
        registers: Some(PeripheralRegisters {
            kind: "pla",
            version: "v2",
            block: "PLA",
            ir: &pla_v2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "PWC_FCG0_PLA",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "PLAINT",
            interrupt: "PLAINT",
        }],
    },
    Peripheral {
        name: "PWC",
        address: 0x40048000,
        registers: Some(PeripheralRegisters {
            kind: "pwc",
            version: "ll_v3_1",
            block: "PWC",
            ir: &pwc_ll_v3_1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "WKTM",
                interrupt: "WKTM",
            },
            PeripheralInterrupt {
                signal: "PVD1",
                interrupt: "PVD1",
            },
            PeripheralInterrupt {
                signal: "PVD2",
                interrupt: "PVD2",
            },
        ],
    },
    Peripheral {
        name: "RMU",
        address: 0x4004cce0,
        registers: Some(PeripheralRegisters {
            kind: "rmu",
            version: "ll_v3_2",
            block: "RMU",
            ir: &rmu_ll_v3_2::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0x4004c000,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "ll_v1",
            block: "RTC",
            ir: &rtc_ll_v1::REGISTERS,
        }),
        rcc: None,
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
        interrupts: &[PeripheralInterrupt {
            signal: "RTC",
            interrupt: "RTC",
        }],
    },
    Peripheral {
        name: "SPI",
        address: 0x4001c000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ll_v2_5",
            block: "SPI",
            ir: &spi_ll_v2_5::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "PWC_FCG1_SPI",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SPRI",
                dma: Some("DMA"),
                channel: None,
                request: Some(330),
            },
            PeripheralDmaChannel {
                signal: "SPTI",
                dma: Some("DMA"),
                channel: None,
                request: Some(331),
            },
            PeripheralDmaChannel {
                signal: "SPII",
                dma: Some("DMA"),
                channel: None,
                request: Some(332),
            },
            PeripheralDmaChannel {
                signal: "SPEI",
                dma: Some("DMA"),
                channel: None,
                request: Some(333),
            },
            PeripheralDmaChannel {
                signal: "SPTEND",
                dma: Some("DMA"),
                channel: None,
                request: Some(333),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "SPI",
            interrupt: "SPI",
        }],
    },
    Peripheral {
        name: "SRAMC",
        address: 0x40050800,
        registers: Some(PeripheralRegisters {
            kind: "sramc",
            version: "ll_v3_3",
            block: "SRAMC",
            ir: &sramc_ll_v3_3::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SWDT",
        address: 0x40049400,
        registers: Some(PeripheralRegisters {
            kind: "swdt",
            version: "ll_v1_1",
            block: "SWDT",
            ir: &swdt_ll_v1_1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "SWDT",
            interrupt: "SWDT",
        }],
    },
    Peripheral {
        name: "TMR01",
        address: 0x40024000,
        registers: Some(PeripheralRegisters {
            kind: "tmr0",
            version: "ll_v2_5",
            block: "TMR0",
            ir: &tmr0_ll_v2_5::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_TMR0_1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(96),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(97),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some("DMA"),
                channel: None,
                request: Some(98),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some("DMA"),
                channel: None,
                request: Some(99),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "TMR0_1",
            interrupt: "TMR0_1",
        }],
    },
    Peripheral {
        name: "TMR02",
        address: 0x40024000,
        registers: Some(PeripheralRegisters {
            kind: "tmr0",
            version: "ll_v2_5",
            block: "TMR0",
            ir: &tmr0_ll_v2_5::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_TMR0_2",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(100),
            },
            PeripheralDmaChannel {
                signal: "CMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(101),
            },
            PeripheralDmaChannel {
                signal: "OVFA",
                dma: Some("DMA"),
                channel: None,
                request: Some(102),
            },
            PeripheralDmaChannel {
                signal: "OVFB",
                dma: Some("DMA"),
                channel: None,
                request: Some(103),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "TMR0_2",
            interrupt: "TMR0_2",
        }],
    },
    Peripheral {
        name: "TMR4",
        address: 0x40038000,
        registers: Some(PeripheralRegisters {
            kind: "tmr4",
            version: "ll_v2_1",
            block: "TMR4",
            ir: &tmr4_ll_v2_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_TMR4",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPUH",
                dma: Some("DMA"),
                channel: None,
                request: Some(288),
            },
            PeripheralDmaChannel {
                signal: "GCMPUL",
                dma: Some("DMA"),
                channel: None,
                request: Some(289),
            },
            PeripheralDmaChannel {
                signal: "GCMPVH",
                dma: Some("DMA"),
                channel: None,
                request: Some(290),
            },
            PeripheralDmaChannel {
                signal: "GCMPVL",
                dma: Some("DMA"),
                channel: None,
                request: Some(291),
            },
            PeripheralDmaChannel {
                signal: "GCMPWH",
                dma: Some("DMA"),
                channel: None,
                request: Some(292),
            },
            PeripheralDmaChannel {
                signal: "GCMPWL",
                dma: Some("DMA"),
                channel: None,
                request: Some(293),
            },
            PeripheralDmaChannel {
                signal: "GCMPXH",
                dma: Some("DMA"),
                channel: None,
                request: Some(294),
            },
            PeripheralDmaChannel {
                signal: "GCMPXL",
                dma: Some("DMA"),
                channel: None,
                request: Some(295),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(296),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(297),
            },
            PeripheralDmaChannel {
                signal: "RELOADU",
                dma: Some("DMA"),
                channel: None,
                request: Some(298),
            },
            PeripheralDmaChannel {
                signal: "RELOADV",
                dma: Some("DMA"),
                channel: None,
                request: Some(299),
            },
            PeripheralDmaChannel {
                signal: "RELOADW",
                dma: Some("DMA"),
                channel: None,
                request: Some(300),
            },
            PeripheralDmaChannel {
                signal: "RELOADX",
                dma: Some("DMA"),
                channel: None,
                request: Some(301),
            },
            PeripheralDmaChannel {
                signal: "SCMP0",
                dma: Some("DMA"),
                channel: None,
                request: Some(302),
            },
            PeripheralDmaChannel {
                signal: "SCMP1",
                dma: Some("DMA"),
                channel: None,
                request: Some(303),
            },
            PeripheralDmaChannel {
                signal: "SCMP2",
                dma: Some("DMA"),
                channel: None,
                request: Some(304),
            },
            PeripheralDmaChannel {
                signal: "SCMP3",
                dma: Some("DMA"),
                channel: None,
                request: Some(305),
            },
            PeripheralDmaChannel {
                signal: "SCMP4",
                dma: Some("DMA"),
                channel: None,
                request: Some(306),
            },
            PeripheralDmaChannel {
                signal: "SCMP5",
                dma: Some("DMA"),
                channel: None,
                request: Some(307),
            },
            PeripheralDmaChannel {
                signal: "SCMP6",
                dma: Some("DMA"),
                channel: None,
                request: Some(308),
            },
            PeripheralDmaChannel {
                signal: "SCMP7",
                dma: Some("DMA"),
                channel: None,
                request: Some(309),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GCMP",
                interrupt: "GCMP",
            },
            PeripheralInterrupt {
                signal: "OVF_UDF",
                interrupt: "OVF_UDF",
            },
            PeripheralInterrupt {
                signal: "RELOAD",
                interrupt: "RELOAD",
            },
            PeripheralInterrupt {
                signal: "SCM",
                interrupt: "SCM",
            },
        ],
    },
    Peripheral {
        name: "TMR61",
        address: 0x40039000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v30",
            block: "TMR",
            ir: &tmr_v30::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_TMR6_1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(128),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(129),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some("DMA"),
                channel: None,
                request: Some(130),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some("DMA"),
                channel: None,
                request: Some(131),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some("DMA"),
                channel: None,
                request: Some(132),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some("DMA"),
                channel: None,
                request: Some(133),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(134),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(135),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(137),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(138),
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
                signal: "TMR6_1_DTE",
                interrupt: "TMR6_1_DTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_1_SCMP",
                interrupt: "TMR6_1_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR62",
        address: 0x40039000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v30",
            block: "TMR",
            ir: &tmr_v30::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_TMR6_2",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(144),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(145),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some("DMA"),
                channel: None,
                request: Some(146),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some("DMA"),
                channel: None,
                request: Some(147),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some("DMA"),
                channel: None,
                request: Some(148),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some("DMA"),
                channel: None,
                request: Some(149),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(150),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(151),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(153),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(154),
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
                signal: "TMR6_2_DTE",
                interrupt: "TMR6_2_DTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_2_SCMP",
                interrupt: "TMR6_2_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR63",
        address: 0x40039000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v30",
            block: "TMR",
            ir: &tmr_v30::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_TMR6_3",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(160),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(161),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some("DMA"),
                channel: None,
                request: Some(162),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some("DMA"),
                channel: None,
                request: Some(163),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some("DMA"),
                channel: None,
                request: Some(164),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some("DMA"),
                channel: None,
                request: Some(165),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(166),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(167),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(169),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(170),
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
                signal: "TMR6_3_DTE",
                interrupt: "TMR6_3_DTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_3_SCMP",
                interrupt: "TMR6_3_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR64",
        address: 0x40039000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v30",
            block: "TMR",
            ir: &tmr_v30::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_TMR6_4",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(176),
            },
            PeripheralDmaChannel {
                signal: "GCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(177),
            },
            PeripheralDmaChannel {
                signal: "GCMPC",
                dma: Some("DMA"),
                channel: None,
                request: Some(178),
            },
            PeripheralDmaChannel {
                signal: "GCMPD",
                dma: Some("DMA"),
                channel: None,
                request: Some(179),
            },
            PeripheralDmaChannel {
                signal: "GCMPE",
                dma: Some("DMA"),
                channel: None,
                request: Some(180),
            },
            PeripheralDmaChannel {
                signal: "GCMPF",
                dma: Some("DMA"),
                channel: None,
                request: Some(181),
            },
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(182),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(183),
            },
            PeripheralDmaChannel {
                signal: "SCMPA",
                dma: Some("DMA"),
                channel: None,
                request: Some(185),
            },
            PeripheralDmaChannel {
                signal: "SCMPB",
                dma: Some("DMA"),
                channel: None,
                request: Some(186),
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
                signal: "TMR6_4_DTE",
                interrupt: "TMR6_4_DTE",
            },
            PeripheralInterrupt {
                signal: "TMR6_4_SCMP",
                interrupt: "TMR6_4_SCMP",
            },
        ],
    },
    Peripheral {
        name: "TMR6_COMMON",
        address: 0x40039300,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v31",
            block: "TMR",
            ir: &tmr_v31::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRA1",
        address: 0x4003a000,
        registers: Some(PeripheralRegisters {
            kind: "tmra",
            version: "ll_v2_8",
            block: "TMRA",
            ir: &tmra_ll_v2_8::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_TMRA_1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(139),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(140),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some("DMA"),
                channel: None,
                request: Some(141),
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
        registers: Some(PeripheralRegisters {
            kind: "tmra",
            version: "ll_v2_9",
            block: "TMRA",
            ir: &tmra_ll_v2_9::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_TMRA_2",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(155),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(156),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some("DMA"),
                channel: None,
                request: Some(157),
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
        address: 0x4003a400,
        registers: Some(PeripheralRegisters {
            kind: "tmra",
            version: "ll_v2_9",
            block: "TMRA",
            ir: &tmra_ll_v2_9::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_TMRA_3",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(171),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(172),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some("DMA"),
                channel: None,
                request: Some(173),
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
        address: 0x4003a400,
        registers: Some(PeripheralRegisters {
            kind: "tmra",
            version: "ll_v2_9",
            block: "TMRA",
            ir: &tmra_ll_v2_9::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_TMRA_4",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(187),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(188),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some("DMA"),
                channel: None,
                request: Some(189),
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
        address: 0x4003a400,
        registers: Some(PeripheralRegisters {
            kind: "tmra",
            version: "ll_v2_9",
            block: "TMRA",
            ir: &tmra_ll_v2_9::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG2",
                field: "PWC_FCG2_TMRA_5",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OVF",
                dma: Some("DMA"),
                channel: None,
                request: Some(340),
            },
            PeripheralDmaChannel {
                signal: "UDF",
                dma: Some("DMA"),
                channel: None,
                request: Some(341),
            },
            PeripheralDmaChannel {
                signal: "CMP",
                dma: Some("DMA"),
                channel: None,
                request: Some(342),
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
        address: 0x4001cc00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v3_8",
            block: "USART",
            ir: &usart_ll_v3_8::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG3",
                field: "PWC_FCG3_USART1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some("DMA"),
                channel: None,
                request: Some(321),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some("DMA"),
                channel: None,
                request: Some(322),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some("DMA"),
                channel: None,
                request: Some(323),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some("DMA"),
                channel: None,
                request: Some(324),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some("DMA"),
                channel: None,
                request: Some(325),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some("DMA"),
                channel: None,
                request: Some(326),
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
        address: 0x4001d000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v3_13",
            block: "USART",
            ir: &usart_ll_v3_13::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG3",
                field: "PWC_FCG3_USART2",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some("DMA"),
                channel: None,
                request: Some(352),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some("DMA"),
                channel: None,
                request: Some(353),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some("DMA"),
                channel: None,
                request: Some(354),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some("DMA"),
                channel: None,
                request: Some(355),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some("DMA"),
                channel: None,
                request: Some(356),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some("DMA"),
                channel: None,
                request: Some(357),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some("DMA"),
                channel: None,
                request: Some(358),
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
        address: 0x4001d000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v3_13",
            block: "USART",
            ir: &usart_ll_v3_13::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG3",
                field: "PWC_FCG3_USART3",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some("DMA"),
                channel: None,
                request: Some(384),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some("DMA"),
                channel: None,
                request: Some(385),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some("DMA"),
                channel: None,
                request: Some(386),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some("DMA"),
                channel: None,
                request: Some(387),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some("DMA"),
                channel: None,
                request: Some(388),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some("DMA"),
                channel: None,
                request: Some(389),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some("DMA"),
                channel: None,
                request: Some(390),
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
        address: 0x4001d000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v3_13",
            block: "USART",
            ir: &usart_ll_v3_13::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG3",
                field: "PWC_FCG3_USART4",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "BRKWKPI",
                dma: Some("DMA"),
                channel: None,
                request: Some(391),
            },
            PeripheralDmaChannel {
                signal: "EI",
                dma: Some("DMA"),
                channel: None,
                request: Some(392),
            },
            PeripheralDmaChannel {
                signal: "RI",
                dma: Some("DMA"),
                channel: None,
                request: Some(393),
            },
            PeripheralDmaChannel {
                signal: "TI",
                dma: Some("DMA"),
                channel: None,
                request: Some(394),
            },
            PeripheralDmaChannel {
                signal: "RTO",
                dma: Some("DMA"),
                channel: None,
                request: Some(395),
            },
            PeripheralDmaChannel {
                signal: "TENDI",
                dma: Some("DMA"),
                channel: None,
                request: Some(396),
            },
            PeripheralDmaChannel {
                signal: "TCI",
                dma: Some("DMA"),
                channel: None,
                request: Some(397),
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
        registers: Some(PeripheralRegisters {
            kind: "wdt",
            version: "ll_v1",
            block: "WDT",
            ir: &wdt_ll_v1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "REFUDF",
            dma: Some("DMA"),
            channel: None,
            request: Some(471),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "WDT",
            interrupt: "WDT",
        }],
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
        name: "DMA_ERR",
        number: 32,
    },
    Interrupt {
        name: "DMA_TC0_BTC0",
        number: 33,
    },
    Interrupt {
        name: "DMA_TC1_BTC1",
        number: 34,
    },
    Interrupt {
        name: "DMA_TC2_BTC2",
        number: 35,
    },
    Interrupt {
        name: "DMA_TC3_BTC3",
        number: 36,
    },
    Interrupt {
        name: "DMA_TC4_BTC4",
        number: 37,
    },
    Interrupt {
        name: "DMA_TC5_BTC5",
        number: 38,
    },
    Interrupt {
        name: "DMA_TC6_BTC6",
        number: 39,
    },
    Interrupt {
        name: "DMA_TC7_BTC7",
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
        name: "FPU_ERR",
        number: 43,
    },
    Interrupt {
        name: "TMR0_1",
        number: 44,
    },
    Interrupt {
        name: "TMR0_2",
        number: 45,
    },
    Interrupt {
        name: "RTC",
        number: 46,
    },
    Interrupt {
        name: "XTAL",
        number: 47,
    },
    Interrupt {
        name: "WKTM",
        number: 48,
    },
    Interrupt {
        name: "SWDT",
        number: 49,
    },
    Interrupt {
        name: "TMR6_1_GCMP",
        number: 50,
    },
    Interrupt {
        name: "TMR6_1_OVF_UDF",
        number: 51,
    },
    Interrupt {
        name: "TMR6_1_DTE",
        number: 52,
    },
    Interrupt {
        name: "TMR6_1_SCMP",
        number: 53,
    },
    Interrupt {
        name: "TMRA_1_OVF_UDF",
        number: 54,
    },
    Interrupt {
        name: "TMRA_1_CMP",
        number: 55,
    },
    Interrupt {
        name: "TMR6_2_GCMP",
        number: 56,
    },
    Interrupt {
        name: "TMR6_2_OVF_UDF",
        number: 57,
    },
    Interrupt {
        name: "TMR6_2_DTE",
        number: 58,
    },
    Interrupt {
        name: "TMR6_2_SCMP",
        number: 59,
    },
    Interrupt {
        name: "TMRA_2_OVF_UDF",
        number: 60,
    },
    Interrupt {
        name: "TMRA_2_CMP",
        number: 61,
    },
    Interrupt {
        name: "TMR6_3_GCMP",
        number: 62,
    },
    Interrupt {
        name: "TMR6_3_OVF_UDF",
        number: 63,
    },
    Interrupt {
        name: "TMR6_3_DTE",
        number: 64,
    },
    Interrupt {
        name: "TMR6_3_SCMP",
        number: 65,
    },
    Interrupt {
        name: "TMRA_3_OVF_UDF",
        number: 66,
    },
    Interrupt {
        name: "TMRA_3_CMP",
        number: 67,
    },
    Interrupt {
        name: "TMR6_4_GCMP",
        number: 68,
    },
    Interrupt {
        name: "TMR6_4_OVF_UDF",
        number: 69,
    },
    Interrupt {
        name: "TMR6_4_DTE",
        number: 70,
    },
    Interrupt {
        name: "TMR6_4_SCMP",
        number: 71,
    },
    Interrupt {
        name: "TMRA_4_OVF_UDF",
        number: 72,
    },
    Interrupt {
        name: "TMRA_4_CMP",
        number: 73,
    },
    Interrupt {
        name: "HRPWM_1_GCMP",
        number: 74,
    },
    Interrupt {
        name: "HRPWM_1_OVF_UDF",
        number: 75,
    },
    Interrupt {
        name: "HRPWM_1_SCMP",
        number: 76,
    },
    Interrupt {
        name: "HRPWM_1_GCAP",
        number: 77,
    },
    Interrupt {
        name: "HRPWM_2_GCMP",
        number: 78,
    },
    Interrupt {
        name: "HRPWM_2_OVF_UDF",
        number: 79,
    },
    Interrupt {
        name: "HRPWM_2_SCMP",
        number: 80,
    },
    Interrupt {
        name: "HRPWM_2_GCAP",
        number: 81,
    },
    Interrupt {
        name: "HRPWM",
        number: 82,
    },
    Interrupt {
        name: "EMB_GR0",
        number: 83,
    },
    Interrupt {
        name: "EMB_GR1",
        number: 84,
    },
    Interrupt {
        name: "EMB_GR2",
        number: 85,
    },
    Interrupt {
        name: "EMB_GR3",
        number: 86,
    },
    Interrupt {
        name: "HRPWM_3_GCMP",
        number: 87,
    },
    Interrupt {
        name: "HRPWM_3_OVF_UDF",
        number: 88,
    },
    Interrupt {
        name: "HRPWM_3_SCMP",
        number: 89,
    },
    Interrupt {
        name: "HRPWM_3_GCAP",
        number: 90,
    },
    Interrupt {
        name: "HRPWM_4_GCMP",
        number: 91,
    },
    Interrupt {
        name: "HRPWM_4_OVF_UDF",
        number: 92,
    },
    Interrupt {
        name: "HRPWM_4_SCMP",
        number: 93,
    },
    Interrupt {
        name: "HRPWM_4_GCAP",
        number: 94,
    },
    Interrupt {
        name: "EMB_GR4",
        number: 95,
    },
    Interrupt {
        name: "EMB_GR5",
        number: 96,
    },
    Interrupt {
        name: "EMB_GR6",
        number: 97,
    },
    Interrupt {
        name: "EMB_GR7",
        number: 98,
    },
    Interrupt {
        name: "EMB_GR8",
        number: 99,
    },
    Interrupt {
        name: "HRPWM_5_GCMP",
        number: 100,
    },
    Interrupt {
        name: "HRPWM_5_OVF_UDF",
        number: 101,
    },
    Interrupt {
        name: "HRPWM_5_SCMP",
        number: 102,
    },
    Interrupt {
        name: "HRPWM_5_GCAP",
        number: 103,
    },
    Interrupt {
        name: "HRPWM_6_GCMP",
        number: 104,
    },
    Interrupt {
        name: "HRPWM_6_OVF_UDF",
        number: 105,
    },
    Interrupt {
        name: "HRPWM_6_SCMP",
        number: 106,
    },
    Interrupt {
        name: "HRPWM_6_GCAP",
        number: 107,
    },
    Interrupt {
        name: "MCANRAM_ECCER",
        number: 108,
    },
    Interrupt {
        name: "MCAN1_INT0",
        number: 109,
    },
    Interrupt {
        name: "MCAN1_INT1",
        number: 110,
    },
    Interrupt {
        name: "MCAN2_INT0",
        number: 111,
    },
    Interrupt {
        name: "MCAN2_INT1",
        number: 112,
    },
    Interrupt {
        name: "TMR4_GCMP",
        number: 113,
    },
    Interrupt {
        name: "TMR4_OVF_UDF",
        number: 114,
    },
    Interrupt {
        name: "TMR4_RELOAD",
        number: 115,
    },
    Interrupt {
        name: "TMR4_SCMP",
        number: 116,
    },
    Interrupt {
        name: "CMP1",
        number: 117,
    },
    Interrupt {
        name: "CMP2",
        number: 118,
    },
    Interrupt {
        name: "CMP3",
        number: 119,
    },
    Interrupt {
        name: "I2C",
        number: 120,
    },
    Interrupt {
        name: "USART1",
        number: 121,
    },
    Interrupt {
        name: "USART1_TCI",
        number: 122,
    },
    Interrupt {
        name: "SPI",
        number: 123,
    },
    Interrupt {
        name: "TMRA_5_OVF_UDF",
        number: 124,
    },
    Interrupt {
        name: "TMRA_5_CMP",
        number: 125,
    },
    Interrupt {
        name: "EVENT_PORT1",
        number: 126,
    },
    Interrupt {
        name: "EVENT_PORT2",
        number: 127,
    },
    Interrupt {
        name: "EVENT_PORT3",
        number: 128,
    },
    Interrupt {
        name: "EVENT_PORT4",
        number: 129,
    },
    Interrupt {
        name: "USART2",
        number: 130,
    },
    Interrupt {
        name: "USART2_TCI",
        number: 131,
    },
    Interrupt {
        name: "USART3",
        number: 132,
    },
    Interrupt {
        name: "USART3_TCI",
        number: 133,
    },
    Interrupt {
        name: "USART4",
        number: 134,
    },
    Interrupt {
        name: "USART4_TCI",
        number: 135,
    },
    Interrupt {
        name: "USART1_WUPI",
        number: 137,
    },
    Interrupt {
        name: "LVD1",
        number: 138,
    },
    Interrupt {
        name: "LVD2",
        number: 139,
    },
    Interrupt {
        name: "FCM",
        number: 140,
    },
    Interrupt {
        name: "WDT",
        number: 141,
    },
    Interrupt {
        name: "CTC",
        number: 142,
    },
    Interrupt {
        name: "ADC1",
        number: 143,
    },
    Interrupt {
        name: "ADC2",
        number: 144,
    },
    Interrupt {
        name: "ADC3",
        number: 145,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/adc_ll_v3_2.rs"]
pub mod adc_ll_v3_2;
#[path = "../registers/adc_ll_v3_5.rs"]
pub mod adc_ll_v3_5;
#[path = "../registers/adc_ll_v3_6.rs"]
pub mod adc_ll_v3_6;
#[path = "../registers/aos_ll_v7.rs"]
pub mod aos_ll_v7;
#[path = "../registers/cmp_ll_v3_5.rs"]
pub mod cmp_ll_v3_5;
#[path = "../registers/cmu_ll_v3_1.rs"]
pub mod cmu_ll_v3_1;
#[path = "../registers/crc_ll_v2.rs"]
pub mod crc_ll_v2;
#[path = "../registers/ctc_ll_v1_1.rs"]
pub mod ctc_ll_v1_1;
#[path = "../registers/dac_ll_v2.rs"]
pub mod dac_ll_v2;
#[path = "../registers/dbgc_ll_v3_3.rs"]
pub mod dbgc_ll_v3_3;
#[path = "../registers/dma_ll_v2_4.rs"]
pub mod dma_ll_v2_4;
#[path = "../registers/efm_ll_v2_2.rs"]
pub mod efm_ll_v2_2;
#[path = "../registers/emb_ll_v2_10.rs"]
pub mod emb_ll_v2_10;
#[path = "../registers/emb_ll_v2_11.rs"]
pub mod emb_ll_v2_11;
#[path = "../registers/fcm_ll_v1_1.rs"]
pub mod fcm_ll_v1_1;
#[path = "../registers/gpio_ll_v2_5.rs"]
pub mod gpio_ll_v2_5;
#[path = "../registers/hrpwm_v2.rs"]
pub mod hrpwm_v2;
#[path = "../registers/hrpwm_v3.rs"]
pub mod hrpwm_v3;
#[path = "../registers/hrpwm_v4.rs"]
pub mod hrpwm_v4;
#[path = "../registers/i2c_ll_v4_2.rs"]
pub mod i2c_ll_v4_2;
#[path = "../registers/icg_ll_v1_1.rs"]
pub mod icg_ll_v1_1;
#[path = "../registers/intc_ll_v2_7.rs"]
pub mod intc_ll_v2_7;
#[path = "../registers/mcan_ll_v1_1.rs"]
pub mod mcan_ll_v1_1;
#[path = "../registers/mcanram_ll_v1_1.rs"]
pub mod mcanram_ll_v1_1;
#[path = "../registers/mpu_v6.rs"]
pub mod mpu_v6;
#[path = "../registers/peric_ll_v1_3.rs"]
pub mod peric_ll_v1_3;
#[path = "../registers/pla_v2.rs"]
pub mod pla_v2;
#[path = "../registers/pwc_ll_v3_1.rs"]
pub mod pwc_ll_v3_1;
#[path = "../registers/rmu_ll_v3_2.rs"]
pub mod rmu_ll_v3_2;
#[path = "../registers/rtc_ll_v1.rs"]
pub mod rtc_ll_v1;
#[path = "../registers/spi_ll_v2_5.rs"]
pub mod spi_ll_v2_5;
#[path = "../registers/sramc_ll_v3_3.rs"]
pub mod sramc_ll_v3_3;
#[path = "../registers/swdt_ll_v1_1.rs"]
pub mod swdt_ll_v1_1;
#[path = "../registers/tmr0_ll_v2_5.rs"]
pub mod tmr0_ll_v2_5;
#[path = "../registers/tmr4_ll_v2_1.rs"]
pub mod tmr4_ll_v2_1;
#[path = "../registers/tmr_v30.rs"]
pub mod tmr_v30;
#[path = "../registers/tmr_v31.rs"]
pub mod tmr_v31;
#[path = "../registers/tmra_ll_v2_8.rs"]
pub mod tmra_ll_v2_8;
#[path = "../registers/tmra_ll_v2_9.rs"]
pub mod tmra_ll_v2_9;
#[path = "../registers/usart_ll_v3_13.rs"]
pub mod usart_ll_v3_13;
#[path = "../registers/usart_ll_v3_8.rs"]
pub mod usart_ll_v3_8;
#[path = "../registers/wdt_ll_v1.rs"]
pub mod wdt_ll_v1;
