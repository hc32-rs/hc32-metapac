
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ddl_v7_3",
            block: "ADC",
            ir: &adc_ddl_v7_3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralAdcBgr",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "JQR",
                dma: Some("DMA1"),
                channel: None,
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "SQR",
                dma: Some("DMA1"),
                channel: None,
                request: Some(69),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "ADC_DAC",
            interrupt: "ADC_DAC",
        }],
    },
    Peripheral {
        name: "ADTIM4",
        address: 0x40003000,
        registers: Some(PeripheralRegisters {
            kind: "adt",
            version: "ddl_v2_1",
            block: "ADT",
            ir: &adt_ddl_v2_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralAdvTim",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "ADTIM4",
            interrupt: "ADTIM4",
        }],
    },
    Peripheral {
        name: "ADTIM5",
        address: 0x40003400,
        registers: Some(PeripheralRegisters {
            kind: "adt",
            version: "ddl_v2_1",
            block: "ADT",
            ir: &adt_ddl_v2_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralAdvTim",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "ADTIM5",
            interrupt: "ADTIM5",
        }],
    },
    Peripheral {
        name: "ADTIM6",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "adt",
            version: "ddl_v2_1",
            block: "ADT",
            ir: &adt_ddl_v2_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralAdvTim",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "ADTIM6",
            interrupt: "ADTIM6",
        }],
    },
    Peripheral {
        name: "AES",
        address: 0x40021400,
        registers: Some(PeripheralRegisters {
            kind: "aes",
            version: "ddl_v2",
            block: "AES",
            ir: &aes_ddl_v2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralAes",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BGR",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "bgr",
            version: "ddl_v1_1",
            block: "BGR",
            ir: &bgr_ddl_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralAdcBgr",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CLK_TRIM",
        address: 0x40001800,
        registers: Some(PeripheralRegisters {
            kind: "clk_trim",
            version: "ddl_v1_2",
            block: "CLK_TRIM",
            ir: &clk_trim_ddl_v1_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralTrim",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 0x40020900,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "ddl_v2_1",
            block: "CRC",
            ir: &crc_ddl_v2_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralCrc",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC",
        address: 0x40002500,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "ddl_v1",
            block: "DAC",
            ir: &dac_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN1",
                field: "SysctrlPeripheralDac",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "0",
                dma: Some("DMA1"),
                channel: None,
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "1",
                dma: Some("DMA1"),
                channel: None,
                request: Some(81),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "ADC_DAC",
            interrupt: "ADC_DAC",
        }],
    },
    Peripheral {
        name: "DEBUG_ACTIVE",
        address: 0x40002038,
        registers: Some(PeripheralRegisters {
            kind: "debug_active",
            version: "ddl_v1_3",
            block: "DEBUG_ACTIVE",
            ir: &debug_active_ddl_v1_3::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMAC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "dmac",
            version: "ddl_v1_3",
            block: "DMAC",
            ir: &dmac_ddl_v1_3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralDma",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "DMAC",
            interrupt: "DMAC",
        }],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "ddl_v3_4",
            block: "FLASH",
            ir: &flash_ddl_v3_4::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralFlash",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "FLASH_RAM",
            interrupt: "FLASH_RAM",
        }],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40020c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3_7",
            block: "GPIO",
            ir: &gpio_ddl_v3_7::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralGpio",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C0",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ddl_v1_1",
            block: "I2C",
            ir: &i2c_ddl_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralI2c0",
            }),
        }),
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
            version: "ddl_v1_1",
            block: "I2C",
            ir: &i2c_ddl_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralI2c1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "I2C1",
            interrupt: "I2C1",
        }],
    },
    Peripheral {
        name: "LCD",
        address: 0x40005c00,
        registers: Some(PeripheralRegisters {
            kind: "lcd",
            version: "ddl_v1_1",
            block: "LCD",
            ir: &lcd_ddl_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralLcd",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "TX",
            dma: Some("DMA1"),
            channel: None,
            request: Some(70),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "LCD",
            interrupt: "LCD",
        }],
    },
    Peripheral {
        name: "LVD",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "lvd",
            version: "ddl_v1_3",
            block: "LVD",
            ir: &lvd_ddl_v1_3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralVcLvd",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "LVD",
            interrupt: "LVD",
        }],
    },
    Peripheral {
        name: "OPA",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "opa",
            version: "ddl_v2",
            block: "OPA",
            ir: &opa_ddl_v2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralOpa",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PCA",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "pca",
            version: "ddl_v1_2",
            block: "PCA",
            ir: &pca_ddl_v1_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralPca",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "PCA",
            interrupt: "PCA",
        }],
    },
    Peripheral {
        name: "RAM",
        address: 0x40020400,
        registers: Some(PeripheralRegisters {
            kind: "ram",
            version: "ddl_v3_1",
            block: "RAM",
            ir: &ram_ddl_v3_1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "FLASH_RAM",
            interrupt: "FLASH_RAM",
        }],
    },
    Peripheral {
        name: "RESET",
        address: 0x4000201c,
        registers: Some(PeripheralRegisters {
            kind: "reset",
            version: "ddl_v1_4",
            block: "RESET",
            ir: &reset_ddl_v1_4::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI0",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ddl_v1_2",
            block: "SPI",
            ir: &spi_ddl_v1_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralSpi0",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(65),
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
            version: "ddl_v1_2",
            block: "SPI",
            ir: &spi_ddl_v1_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralSpi1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(67),
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
            version: "ddl_v3_2",
            block: "SYSCTRL",
            ir: &sysctrl_ddl_v3_2::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM0",
        address: 0x40000c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralBaseTim;SysctrlPeripheralLpTim0",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                dma: Some("DMA1"),
                channel: None,
                request: Some(82),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some("DMA1"),
                channel: None,
                request: Some(83),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "TIM0",
            interrupt: "TIM0",
        }],
    },
    Peripheral {
        name: "TIM1",
        address: 0x40000d00,
        registers: None,
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralBaseTim;SysctrlPeripheralLpTim1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                dma: Some("DMA1"),
                channel: None,
                request: Some(84),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some("DMA1"),
                channel: None,
                request: Some(85),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "TIM1",
            interrupt: "TIM1",
        }],
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000e00,
        registers: None,
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralBaseTim",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                dma: Some("DMA1"),
                channel: None,
                request: Some(86),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some("DMA1"),
                channel: None,
                request: Some(87),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "TIM2",
            interrupt: "TIM2",
        }],
    },
    Peripheral {
        name: "TIM3",
        address: 0x40005800,
        registers: None,
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralBaseTim;SysctrlPeripheralTim3",
            }),
        }),
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
        interrupts: &[PeripheralInterrupt {
            signal: "TIM3",
            interrupt: "TIM3",
        }],
    },
    Peripheral {
        name: "TRNG",
        address: 0x40004c00,
        registers: Some(PeripheralRegisters {
            kind: "trng",
            version: "ddl_v1",
            block: "TRNG",
            ir: &trng_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralRng",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "UART0",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ddl_v3",
            block: "UART",
            ir: &uart_ddl_v3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralUart0;SysctrlPeripheralLpUart0",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(73),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "UART0_2",
            interrupt: "UART0_2",
        }],
    },
    Peripheral {
        name: "UART1",
        address: 0x40000100,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ddl_v3",
            block: "UART",
            ir: &uart_ddl_v3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralUart1;SysctrlPeripheralLpUart1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(75),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "UART1_3",
            interrupt: "UART1_3",
        }],
    },
    Peripheral {
        name: "UART2",
        address: 0x40006000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ddl_v3",
            block: "UART",
            ir: &uart_ddl_v3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN1",
                field: "SysctrlPeripheralUart2",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(96),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(97),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "UART0_2",
            interrupt: "UART0_2",
        }],
    },
    Peripheral {
        name: "UART3",
        address: 0x40006400,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ddl_v3",
            block: "UART",
            ir: &uart_ddl_v3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN1",
                field: "SysctrlPeripheralUart3",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(98),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(99),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "UART1_3",
            interrupt: "UART1_3",
        }],
    },
    Peripheral {
        name: "VC",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "vc",
            version: "ddl_v3",
            block: "VC",
            ir: &vc_ddl_v3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralVcLvd",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "VC0",
                interrupt: "VC0",
            },
            PeripheralInterrupt {
                signal: "VC1_2",
                interrupt: "VC1_2",
            },
        ],
    },
    Peripheral {
        name: "WDT",
        address: 0x40000f00,
        registers: Some(PeripheralRegisters {
            kind: "wdt",
            version: "ddl_v1_1",
            block: "WDT",
            ir: &wdt_ddl_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "SysctrlPeripheralWdt",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WDT",
            interrupt: "WDT",
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
        name: "PORTC_E",
        number: 2,
    },
    Interrupt {
        name: "PORTD_F",
        number: 3,
    },
    Interrupt {
        name: "DMAC",
        number: 4,
    },
    Interrupt {
        name: "TIM3",
        number: 5,
    },
    Interrupt {
        name: "UART0_2",
        number: 6,
    },
    Interrupt {
        name: "UART1_3",
        number: 7,
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
        name: "TIM0",
        number: 14,
    },
    Interrupt {
        name: "TIM1",
        number: 15,
    },
    Interrupt {
        name: "TIM2",
        number: 16,
    },
    Interrupt {
        name: "ADTIM4",
        number: 18,
    },
    Interrupt {
        name: "ADTIM5",
        number: 19,
    },
    Interrupt {
        name: "ADTIM6",
        number: 20,
    },
    Interrupt {
        name: "PCA",
        number: 21,
    },
    Interrupt {
        name: "WDT",
        number: 22,
    },
    Interrupt {
        name: "ADC_DAC",
        number: 24,
    },
    Interrupt {
        name: "VC0",
        number: 26,
    },
    Interrupt {
        name: "VC1_2",
        number: 27,
    },
    Interrupt {
        name: "LVD",
        number: 28,
    },
    Interrupt {
        name: "LCD",
        number: 29,
    },
    Interrupt {
        name: "FLASH_RAM",
        number: 30,
    },
    Interrupt {
        name: "CLKTRIM",
        number: 31,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/adc_ddl_v7_3.rs"]
pub mod adc_ddl_v7_3;
#[path = "../registers/adt_ddl_v2_1.rs"]
pub mod adt_ddl_v2_1;
#[path = "../registers/aes_ddl_v2.rs"]
pub mod aes_ddl_v2;
#[path = "../registers/bgr_ddl_v1_1.rs"]
pub mod bgr_ddl_v1_1;
#[path = "../registers/clk_trim_ddl_v1_2.rs"]
pub mod clk_trim_ddl_v1_2;
#[path = "../registers/crc_ddl_v2_1.rs"]
pub mod crc_ddl_v2_1;
#[path = "../registers/dac_ddl_v1.rs"]
pub mod dac_ddl_v1;
#[path = "../registers/debug_active_ddl_v1_3.rs"]
pub mod debug_active_ddl_v1_3;
#[path = "../registers/dmac_ddl_v1_3.rs"]
pub mod dmac_ddl_v1_3;
#[path = "../registers/flash_ddl_v3_4.rs"]
pub mod flash_ddl_v3_4;
#[path = "../registers/gpio_ddl_v3_7.rs"]
pub mod gpio_ddl_v3_7;
#[path = "../registers/i2c_ddl_v1_1.rs"]
pub mod i2c_ddl_v1_1;
#[path = "../registers/lcd_ddl_v1_1.rs"]
pub mod lcd_ddl_v1_1;
#[path = "../registers/lvd_ddl_v1_3.rs"]
pub mod lvd_ddl_v1_3;
#[path = "../registers/opa_ddl_v2.rs"]
pub mod opa_ddl_v2;
#[path = "../registers/pca_ddl_v1_2.rs"]
pub mod pca_ddl_v1_2;
#[path = "../registers/ram_ddl_v3_1.rs"]
pub mod ram_ddl_v3_1;
#[path = "../registers/reset_ddl_v1_4.rs"]
pub mod reset_ddl_v1_4;
#[path = "../registers/spi_ddl_v1_2.rs"]
pub mod spi_ddl_v1_2;
#[path = "../registers/sysctrl_ddl_v3_2.rs"]
pub mod sysctrl_ddl_v3_2;
#[path = "../registers/trng_ddl_v1.rs"]
pub mod trng_ddl_v1;
#[path = "../registers/uart_ddl_v3.rs"]
pub mod uart_ddl_v3;
#[path = "../registers/vc_ddl_v3.rs"]
pub mod vc_ddl_v3;
#[path = "../registers/wdt_ddl_v1_1.rs"]
pub mod wdt_ddl_v1_1;
