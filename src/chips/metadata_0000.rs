
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40002500,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ddl_v5",
            block: "ADC",
            ir: &adc_ddl_v5::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralAdcBgr",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "JQR",
                dma: Some("DMA1"),
                channel: None,
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "EOC",
                dma: Some("DMA1"),
                channel: None,
                request: Some(77),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "ADC",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "ATIM0",
        address: 0x40003000,
        registers: None,
        views: &[
            PeripheralView {
                name: "ATIM0MODE0",
                mode: "MODE0",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v1",
                    block: "ATIM",
                    ir: &atim_ddl_v1::REGISTERS,
                },
            },
            PeripheralView {
                name: "ATIM0MODE1",
                mode: "MODE1",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v2_4",
                    block: "ATIM",
                    ir: &atim_ddl_v2_4::REGISTERS,
                },
            },
            PeripheralView {
                name: "ATIM0MODE23",
                mode: "MODE23",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v3",
                    block: "ATIM",
                    ir: &atim_ddl_v3::REGISTERS,
                },
            },
        ],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralAtim0",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                dma: Some("DMA1"),
                channel: None,
                request: Some(88),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some("DMA1"),
                channel: None,
                request: Some(89),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ATIM0",
                interrupt: "ATIM0",
            },
            PeripheralInterrupt {
                signal: "ATIM_BK",
                interrupt: "ATIM_BK",
            },
        ],
    },
    Peripheral {
        name: "ATIM1",
        address: 0x40003400,
        registers: None,
        views: &[
            PeripheralView {
                name: "ATIM1MODE0",
                mode: "MODE0",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v1",
                    block: "ATIM",
                    ir: &atim_ddl_v1::REGISTERS,
                },
            },
            PeripheralView {
                name: "ATIM1MODE1",
                mode: "MODE1",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v2_4",
                    block: "ATIM",
                    ir: &atim_ddl_v2_4::REGISTERS,
                },
            },
            PeripheralView {
                name: "ATIM1MODE23",
                mode: "MODE23",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v3",
                    block: "ATIM",
                    ir: &atim_ddl_v3::REGISTERS,
                },
            },
        ],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralAtim1",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                dma: Some("DMA1"),
                channel: None,
                request: Some(90),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some("DMA1"),
                channel: None,
                request: Some(91),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ATIM1",
                interrupt: "ATIM1",
            },
            PeripheralInterrupt {
                signal: "ATIM_BK",
                interrupt: "ATIM_BK",
            },
        ],
    },
    Peripheral {
        name: "ATIM2",
        address: 0x40003800,
        registers: None,
        views: &[
            PeripheralView {
                name: "ATIM2MODE0",
                mode: "MODE0",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v1",
                    block: "ATIM",
                    ir: &atim_ddl_v1::REGISTERS,
                },
            },
            PeripheralView {
                name: "ATIM2MODE1",
                mode: "MODE1",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v2_4",
                    block: "ATIM",
                    ir: &atim_ddl_v2_4::REGISTERS,
                },
            },
            PeripheralView {
                name: "ATIM2MODE23",
                mode: "MODE23",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v3",
                    block: "ATIM",
                    ir: &atim_ddl_v3::REGISTERS,
                },
            },
        ],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralAtim2",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                dma: Some("DMA1"),
                channel: None,
                request: Some(92),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some("DMA1"),
                channel: None,
                request: Some(93),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ATIM2",
                interrupt: "ATIM2",
            },
            PeripheralInterrupt {
                signal: "ATIM_BK",
                interrupt: "ATIM_BK",
            },
        ],
    },
    Peripheral {
        name: "ATIM3",
        address: 0x40005800,
        registers: None,
        views: &[
            PeripheralView {
                name: "ATIM3MODE0",
                mode: "MODE0",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v1_3",
                    block: "ATIM",
                    ir: &atim_ddl_v1_3::REGISTERS,
                },
            },
            PeripheralView {
                name: "ATIM3MODE1",
                mode: "MODE1",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v2_5",
                    block: "ATIM",
                    ir: &atim_ddl_v2_5::REGISTERS,
                },
            },
            PeripheralView {
                name: "ATIM3MODE23",
                mode: "MODE23",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v4_3",
                    block: "ATIM",
                    ir: &atim_ddl_v4_3::REGISTERS,
                },
            },
        ],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralTim3",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                dma: Some("DMA1"),
                channel: None,
                request: Some(94),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some("DMA1"),
                channel: None,
                request: Some(95),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ATIM3",
                interrupt: "ATIM3",
            },
            PeripheralInterrupt {
                signal: "ATIM_BK",
                interrupt: "ATIM_BK",
            },
        ],
    },
    Peripheral {
        name: "BTIM0",
        address: 0x40001800,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1",
            block: "BTIM",
            ir: &btim_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralGtim0Btim012",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GTIM0_BTIM0_1_2",
            interrupt: "GTIM0_BTIM0_1_2",
        }],
    },
    Peripheral {
        name: "BTIM1",
        address: 0x40001900,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1",
            block: "BTIM",
            ir: &btim_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralGtim0Btim012",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GTIM0_BTIM0_1_2",
            interrupt: "GTIM0_BTIM0_1_2",
        }],
    },
    Peripheral {
        name: "BTIM2",
        address: 0x40001a00,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1",
            block: "BTIM",
            ir: &btim_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralGtim0Btim012",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GTIM0_BTIM0_1_2",
            interrupt: "GTIM0_BTIM0_1_2",
        }],
    },
    Peripheral {
        name: "BTIM3",
        address: 0x40001c00,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_1",
            block: "BTIM",
            ir: &btim_ddl_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralGtim1Btim345",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GTIM1_BTIM3_4_5",
            interrupt: "GTIM1_BTIM3_4_5",
        }],
    },
    Peripheral {
        name: "BTIM4",
        address: 0x40001d00,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_1",
            block: "BTIM",
            ir: &btim_ddl_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralGtim1Btim345",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GTIM1_BTIM3_4_5",
            interrupt: "GTIM1_BTIM3_4_5",
        }],
    },
    Peripheral {
        name: "BTIM5",
        address: 0x40001e00,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_1",
            block: "BTIM",
            ir: &btim_ddl_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralGtim1Btim345",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GTIM1_BTIM3_4_5",
            interrupt: "GTIM1_BTIM3_4_5",
        }],
    },
    Peripheral {
        name: "CAN",
        address: 0x40030000,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "ddl_v1",
            block: "CAN",
            ir: &can_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralCAN",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "CAN",
            interrupt: "CAN",
        }],
    },
    Peripheral {
        name: "CRC",
        address: 0x40020800,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "ddl_v2",
            block: "CRC",
            ir: &crc_ddl_v2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralCrc",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CTRIM",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "ctrim",
            version: "ddl_v1_3",
            block: "CTRIM",
            ir: &ctrim_ddl_v1_3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralCtrim",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "CTRIM",
            interrupt: "CTRIM",
        }],
    },
    Peripheral {
        name: "DMAC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "dmac",
            version: "ddl_v2",
            block: "DMAC",
            ir: &dmac_ddl_v2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralDma",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "DMAC",
            interrupt: "DMAC",
        }],
    },
    Peripheral {
        name: "EAU",
        address: 0x40021800,
        registers: Some(PeripheralRegisters {
            kind: "eau",
            version: "ddl_v1",
            block: "EAU",
            ir: &eau_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralEau",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "ddl_v1_3",
            block: "FLASH",
            ir: &flash_ddl_v1_3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralFlash",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "FLASH_RAM",
            interrupt: "FLASH_RAM",
        }],
    },
    Peripheral {
        name: "GPIOA",
        address: 0x40020c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3_1",
            block: "GPIO",
            ir: &gpio_ddl_v3_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralGpioA",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "PORTA",
            interrupt: "PORTA",
        }],
    },
    Peripheral {
        name: "GPIOAUX",
        address: 0x40002000,
        registers: Some(PeripheralRegisters {
            kind: "gpioaux",
            version: "ddl_v1_1",
            block: "GPIOAUX",
            ir: &gpioaux_ddl_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 0x40021c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3_1",
            block: "GPIO",
            ir: &gpio_ddl_v3_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralGpioB",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "PORTB",
            interrupt: "PORTB",
        }],
    },
    Peripheral {
        name: "GPIOC",
        address: 0x40022000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3_1",
            block: "GPIO",
            ir: &gpio_ddl_v3_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralGpioC",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "PORTC",
            interrupt: "PORTC",
        }],
    },
    Peripheral {
        name: "GPIOD",
        address: 0x40022400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3_1",
            block: "GPIO",
            ir: &gpio_ddl_v3_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralGpioD",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "PORTD",
            interrupt: "PORTD",
        }],
    },
    Peripheral {
        name: "GTIM0",
        address: 0x40001800,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ddl_v1_2",
            block: "GTIM",
            ir: &gtim_ddl_v1_2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralGtim0Btim012",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GTIM0",
            dma: Some("DMA1"),
            channel: None,
            request: Some(72),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GTIM0_BTIM0_1_2",
            interrupt: "GTIM0_BTIM0_1_2",
        }],
    },
    Peripheral {
        name: "GTIM1",
        address: 0x40001c00,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ddl_v1_2",
            block: "GTIM",
            ir: &gtim_ddl_v1_2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN1",
                field: "SysctrlPeripheralGtim1Btim345",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GTIM1",
            dma: Some("DMA1"),
            channel: None,
            request: Some(73),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GTIM1_BTIM3_4_5",
            interrupt: "GTIM1_BTIM3_4_5",
        }],
    },
    Peripheral {
        name: "I2C0",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ddl_v2",
            block: "I2C",
            ir: &i2c_ddl_v2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralI2c0",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "I2C0",
            interrupt: "I2C0",
        }],
    },
    Peripheral {
        name: "I2C1",
        address: 0x40004400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ddl_v2",
            block: "I2C",
            ir: &i2c_ddl_v2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralI2c1",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "I2C1",
            interrupt: "I2C1",
        }],
    },
    Peripheral {
        name: "I2CSLV",
        address: 0x40004c00,
        registers: Some(PeripheralRegisters {
            kind: "i2cslv",
            version: "ddl_v1",
            block: "I2CSLV",
            ir: &i2cslv_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralI2cslv",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(84),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(85),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "I2CSLV",
            interrupt: "I2CSLV",
        }],
    },
    Peripheral {
        name: "IWDT",
        address: 0x40001080,
        registers: Some(PeripheralRegisters {
            kind: "iwdt",
            version: "ddl_v1",
            block: "IWDT",
            ir: &iwdt_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralIwt",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "IWDT",
            interrupt: "IWDT",
        }],
    },
    Peripheral {
        name: "LPTIM",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "ddl_v1_2",
            block: "LPTIM",
            ir: &lptim_ddl_v1_2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralLpTim",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "LPTIM",
            interrupt: "LPTIM",
        }],
    },
    Peripheral {
        name: "LPUART0",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "lpuart",
            version: "ddl_v3_1",
            block: "LPUART",
            ir: &lpuart_ddl_v3_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralLpUart0",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(69),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "LPUART0",
            interrupt: "LPUART0",
        }],
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40004000,
        registers: Some(PeripheralRegisters {
            kind: "lpuart",
            version: "ddl_v3_1",
            block: "LPUART",
            ir: &lpuart_ddl_v3_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralLpUart1",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(71),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "LPUART1",
            interrupt: "LPUART1",
        }],
    },
    Peripheral {
        name: "LVD",
        address: 0x40002480,
        registers: Some(PeripheralRegisters {
            kind: "lvd",
            version: "ddl_v1_2",
            block: "LVD",
            ir: &lvd_ddl_v1_2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralVcLvd",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "VC0_VC1_LVD",
            interrupt: "VC0_VC1_LVD",
        }],
    },
    Peripheral {
        name: "OPA",
        address: 0x40002600,
        registers: Some(PeripheralRegisters {
            kind: "opa",
            version: "ddl_v3",
            block: "OPA",
            ir: &opa_ddl_v3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralOpa",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RAM",
        address: 0x40020400,
        registers: Some(PeripheralRegisters {
            kind: "ram",
            version: "ddl_v3",
            block: "RAM",
            ir: &ram_ddl_v3::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "FLASH_RAM",
            interrupt: "FLASH_RAM",
        }],
    },
    Peripheral {
        name: "RTC",
        address: 0x40001400,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "ddl_v2",
            block: "RTC",
            ir: &rtc_ddl_v2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralRtc",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "RTC",
            interrupt: "RTC",
        }],
    },
    Peripheral {
        name: "SPI0",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ddl_v2_2",
            block: "SPI",
            ir: &spi_ddl_v2_2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralSpi0",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(81),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "SPI0",
            interrupt: "SPI0",
        }],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ddl_v2_2",
            block: "SPI",
            ir: &spi_ddl_v2_2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralSpi1",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(82),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(83),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "SPI1",
            interrupt: "SPI1",
        }],
    },
    Peripheral {
        name: "SYSCTRL",
        address: 0x40002000,
        registers: Some(PeripheralRegisters {
            kind: "sysctrl",
            version: "ddl_v3_6",
            block: "SYSCTRL",
            ir: &sysctrl_ddl_v3_6::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART0",
        address: 0x40006000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ddl_v1",
            block: "USART",
            ir: &usart_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralUsart0",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UART0_TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "UART0_RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(65),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "USART0",
            interrupt: "USART0",
        }],
    },
    Peripheral {
        name: "USART1",
        address: 0x40006400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ddl_v1",
            block: "USART",
            ir: &usart_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralUsart1",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UART1_TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "UART1_RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(67),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "USART1",
            interrupt: "USART1",
        }],
    },
    Peripheral {
        name: "VC0",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "vc",
            version: "ddl_v1_1",
            block: "VC",
            ir: &vc_ddl_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralVcLvd",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "VC0_VC1_LVD",
            interrupt: "VC0_VC1_LVD",
        }],
    },
    Peripheral {
        name: "VC1",
        address: 0x40002410,
        registers: Some(PeripheralRegisters {
            kind: "vc",
            version: "ddl_v1_1",
            block: "VC",
            ir: &vc_ddl_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralVcLvd",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "VC0_VC1_LVD",
            interrupt: "VC0_VC1_LVD",
        }],
    },
    Peripheral {
        name: "WWDT",
        address: 0x40006800,
        registers: Some(PeripheralRegisters {
            kind: "wwdt",
            version: "ddl_v1",
            block: "WWDT",
            ir: &wwdt_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralWwt",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WWDT",
            interrupt: "WWDT",
        }],
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "PORTA",
        number: 0,
    },
    Interrupt {
        name: "PORTB",
        number: 1,
    },
    Interrupt {
        name: "PORTC",
        number: 2,
    },
    Interrupt {
        name: "PORTD",
        number: 3,
    },
    Interrupt {
        name: "DMAC",
        number: 4,
    },
    Interrupt {
        name: "ATIM3",
        number: 5,
    },
    Interrupt {
        name: "USART0",
        number: 6,
    },
    Interrupt {
        name: "USART1",
        number: 7,
    },
    Interrupt {
        name: "LPUART0",
        number: 8,
    },
    Interrupt {
        name: "LPUART1",
        number: 9,
    },
    Interrupt {
        name: "SPI0",
        number: 10,
    },
    Interrupt {
        name: "SPI1",
        number: 11,
    },
    Interrupt {
        name: "I2C0",
        number: 12,
    },
    Interrupt {
        name: "I2C1",
        number: 13,
    },
    Interrupt {
        name: "ATIM0",
        number: 14,
    },
    Interrupt {
        name: "ATIM1",
        number: 15,
    },
    Interrupt {
        name: "ATIM2",
        number: 16,
    },
    Interrupt {
        name: "LPTIM",
        number: 17,
    },
    Interrupt {
        name: "GTIM0_BTIM0_1_2",
        number: 18,
    },
    Interrupt {
        name: "GTIM1_BTIM3_4_5",
        number: 19,
    },
    Interrupt {
        name: "I2CSLV",
        number: 20,
    },
    Interrupt {
        name: "WWDT",
        number: 21,
    },
    Interrupt {
        name: "IWDT",
        number: 22,
    },
    Interrupt {
        name: "RTC",
        number: 23,
    },
    Interrupt {
        name: "ADC",
        number: 24,
    },
    Interrupt {
        name: "VC0_VC1_LVD",
        number: 26,
    },
    Interrupt {
        name: "CAN",
        number: 28,
    },
    Interrupt {
        name: "ATIM_BK",
        number: 29,
    },
    Interrupt {
        name: "FLASH_RAM",
        number: 30,
    },
    Interrupt {
        name: "CTRIM",
        number: 31,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH0",
        dma: "DMA1",
        channel: 0,
    },
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 1,
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 2,
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 3,
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 4,
    },
];
#[path = "../registers/adc_ddl_v5.rs"]
pub mod adc_ddl_v5;
#[path = "../registers/atim_ddl_v1.rs"]
pub mod atim_ddl_v1;
#[path = "../registers/atim_ddl_v1_3.rs"]
pub mod atim_ddl_v1_3;
#[path = "../registers/atim_ddl_v2_4.rs"]
pub mod atim_ddl_v2_4;
#[path = "../registers/atim_ddl_v2_5.rs"]
pub mod atim_ddl_v2_5;
#[path = "../registers/atim_ddl_v3.rs"]
pub mod atim_ddl_v3;
#[path = "../registers/atim_ddl_v4_3.rs"]
pub mod atim_ddl_v4_3;
#[path = "../registers/btim_ddl_v1.rs"]
pub mod btim_ddl_v1;
#[path = "../registers/btim_ddl_v1_1.rs"]
pub mod btim_ddl_v1_1;
#[path = "../registers/can_ddl_v1.rs"]
pub mod can_ddl_v1;
#[path = "../registers/crc_ddl_v2.rs"]
pub mod crc_ddl_v2;
#[path = "../registers/ctrim_ddl_v1_3.rs"]
pub mod ctrim_ddl_v1_3;
#[path = "../registers/dmac_ddl_v2.rs"]
pub mod dmac_ddl_v2;
#[path = "../registers/eau_ddl_v1.rs"]
pub mod eau_ddl_v1;
#[path = "../registers/flash_ddl_v1_3.rs"]
pub mod flash_ddl_v1_3;
#[path = "../registers/gpio_ddl_v3_1.rs"]
pub mod gpio_ddl_v3_1;
#[path = "../registers/gpioaux_ddl_v1_1.rs"]
pub mod gpioaux_ddl_v1_1;
#[path = "../registers/gtim_ddl_v1_2.rs"]
pub mod gtim_ddl_v1_2;
#[path = "../registers/i2c_ddl_v2.rs"]
pub mod i2c_ddl_v2;
#[path = "../registers/i2cslv_ddl_v1.rs"]
pub mod i2cslv_ddl_v1;
#[path = "../registers/iwdt_ddl_v1.rs"]
pub mod iwdt_ddl_v1;
#[path = "../registers/lptim_ddl_v1_2.rs"]
pub mod lptim_ddl_v1_2;
#[path = "../registers/lpuart_ddl_v3_1.rs"]
pub mod lpuart_ddl_v3_1;
#[path = "../registers/lvd_ddl_v1_2.rs"]
pub mod lvd_ddl_v1_2;
#[path = "../registers/opa_ddl_v3.rs"]
pub mod opa_ddl_v3;
#[path = "../registers/ram_ddl_v3.rs"]
pub mod ram_ddl_v3;
#[path = "../registers/rtc_ddl_v2.rs"]
pub mod rtc_ddl_v2;
#[path = "../registers/spi_ddl_v2_2.rs"]
pub mod spi_ddl_v2_2;
#[path = "../registers/sysctrl_ddl_v3_6.rs"]
pub mod sysctrl_ddl_v3_6;
#[path = "../registers/usart_ddl_v1.rs"]
pub mod usart_ddl_v1;
#[path = "../registers/vc_ddl_v1_1.rs"]
pub mod vc_ddl_v1_1;
#[path = "../registers/wwdt_ddl_v1.rs"]
pub mod wwdt_ddl_v1;
