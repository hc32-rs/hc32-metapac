
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x4000a000,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ddl_v3",
            block: "ADC",
            ir: &adc_ddl_v3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "APB1EN",
                field: "SysctrlPeripheralAdc0",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SQR",
            dma: Some("DMA1"),
            channel: None,
            request: Some(77),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "ADC0_ADC1",
            interrupt: "ADC0_ADC1",
        }],
    },
    Peripheral {
        name: "ADC1",
        address: 0x4000a400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ddl_v3_1",
            block: "ADC",
            ir: &adc_ddl_v3_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "APB1EN",
                field: "SysctrlPeripheralAdc1",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SQR",
            dma: Some("DMA1"),
            channel: None,
            request: Some(79),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "ADC0_ADC1",
            interrupt: "ADC0_ADC1",
        }],
    },
    Peripheral {
        name: "ATIM0",
        address: 0x40006000,
        registers: None,
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralATim0",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CHA",
                dma: Some("DMA1"),
                channel: None,
                request: Some(88),
            },
            PeripheralDmaChannel {
                signal: "CHB",
                dma: Some("DMA1"),
                channel: None,
                request: Some(89),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ATIM0_UEV",
                interrupt: "ATIM0_UEV",
            },
            PeripheralInterrupt {
                signal: "ATIM0_CHA",
                interrupt: "ATIM0_CHA",
            },
            PeripheralInterrupt {
                signal: "ATIM0_CHB",
                interrupt: "ATIM0_CHB",
            },
            PeripheralInterrupt {
                signal: "ATIM0_BKETR",
                interrupt: "ATIM0_BKETR",
            },
        ],
    },
    Peripheral {
        name: "ATIM3",
        address: 0x40006c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralATim3",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CHA",
                dma: Some("DMA1"),
                channel: None,
                request: Some(94),
            },
            PeripheralDmaChannel {
                signal: "CHB",
                dma: Some("DMA1"),
                channel: None,
                request: Some(95),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ATIM3_UEV",
                interrupt: "ATIM3_UEV",
            },
            PeripheralInterrupt {
                signal: "ATIM3_CHA",
                interrupt: "ATIM3_CHA",
            },
            PeripheralInterrupt {
                signal: "ATIM3_CHB",
                interrupt: "ATIM3_CHB",
            },
            PeripheralInterrupt {
                signal: "ATIM3_BK_ETR_OV_UD_CMP3",
                interrupt: "ATIM3_BK_ETR_OV_UD_CMP3",
            },
        ],
    },
    Peripheral {
        name: "BTIM0",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_2",
            block: "BTIM",
            ir: &btim_ddl_v1_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCTim0",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BTIM1",
        address: 0x40005100,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_2",
            block: "BTIM",
            ir: &btim_ddl_v1_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCTim0",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BTIM2",
        address: 0x40005200,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_2",
            block: "BTIM",
            ir: &btim_ddl_v1_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCTim0",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BTIM3",
        address: 0x40005400,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_1",
            block: "BTIM",
            ir: &btim_ddl_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCTim1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BTIM4",
        address: 0x40005500,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_1",
            block: "BTIM",
            ir: &btim_ddl_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCTim1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BTIM5",
        address: 0x40005600,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_1",
            block: "BTIM",
            ir: &btim_ddl_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCTim1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BTIM6",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_3",
            block: "BTIM",
            ir: &btim_ddl_v1_3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCTim2",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BTIM7",
        address: 0x40005900,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_3",
            block: "BTIM",
            ir: &btim_ddl_v1_3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCTim2",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BTIM8",
        address: 0x40005a00,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_3",
            block: "BTIM",
            ir: &btim_ddl_v1_3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCTim2",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 0x40020800,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "ddl_v2_2",
            block: "CRC",
            ir: &crc_ddl_v2_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCrc",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CTRIM",
        address: 0x40008000,
        registers: Some(PeripheralRegisters {
            kind: "ctrim",
            version: "ddl_v1_1",
            block: "CTRIM",
            ir: &ctrim_ddl_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "APB1EN",
                field: "SysctrlPeripheralCtrim",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "CTRIM",
            interrupt: "CTRIM",
        }],
    },
    Peripheral {
        name: "DMAC",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "dmac",
            version: "ddl_v2_1",
            block: "DMAC",
            ir: &dmac_ddl_v2_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralDma",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "DMA_CH0",
                interrupt: "DMA_CH0",
            },
            PeripheralInterrupt {
                signal: "DMA_CH1",
                interrupt: "DMA_CH1",
            },
            PeripheralInterrupt {
                signal: "DMA_CH2",
                interrupt: "DMA_CH2",
            },
            PeripheralInterrupt {
                signal: "DMA_CH3",
                interrupt: "DMA_CH3",
            },
            PeripheralInterrupt {
                signal: "DMA_CH4",
                interrupt: "DMA_CH4",
            },
            PeripheralInterrupt {
                signal: "DMA_CH5",
                interrupt: "DMA_CH5",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40021400,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "ddl_v2",
            block: "FLASH",
            ir: &flash_ddl_v2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralFlash",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "FLASH",
            interrupt: "FLASH",
        }],
    },
    Peripheral {
        name: "GPIOA",
        address: 0x40024000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3_1",
            block: "GPIO",
            ir: &gpio_ddl_v3_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB1EN",
                field: "SysctrlPeripheralGpioa",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "PORTA",
            interrupt: "PORTA",
        }],
    },
    Peripheral {
        name: "GPIOAUX",
        address: 0x4000b800,
        registers: Some(PeripheralRegisters {
            kind: "gpioaux",
            version: "ddl_v1_2",
            block: "GPIOAUX",
            ir: &gpioaux_ddl_v1_2::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 0x40024400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3_1",
            block: "GPIO",
            ir: &gpio_ddl_v3_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB1EN",
                field: "SysctrlPeripheralGpiob",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "PORTB",
            interrupt: "PORTB",
        }],
    },
    Peripheral {
        name: "GPIOC",
        address: 0x40024800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3_1",
            block: "GPIO",
            ir: &gpio_ddl_v3_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB1EN",
                field: "SysctrlPeripheralGpioc",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "PORTC",
            interrupt: "PORTC",
        }],
    },
    Peripheral {
        name: "GPIOD",
        address: 0x40024c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3_1",
            block: "GPIO",
            ir: &gpio_ddl_v3_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB1EN",
                field: "SysctrlPeripheralGpiod",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "PORTD",
            interrupt: "PORTD",
        }],
    },
    Peripheral {
        name: "GTIM0",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ddl_v1_3",
            block: "GTIM",
            ir: &gtim_ddl_v1_3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCTim0",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GTIM0",
            dma: Some("DMA1"),
            channel: None,
            request: Some(72),
        }],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GTIM0_UEV_ETR_BTIMA",
                interrupt: "GTIM0_UEV_ETR_BTIMA",
            },
            PeripheralInterrupt {
                signal: "GTIM0_CH01_BTIMB",
                interrupt: "GTIM0_CH01_BTIMB",
            },
            PeripheralInterrupt {
                signal: "GTIM0_CH23_BTIMC",
                interrupt: "GTIM0_CH23_BTIMC",
            },
        ],
    },
    Peripheral {
        name: "GTIM1",
        address: 0x40005400,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ddl_v1_3",
            block: "GTIM",
            ir: &gtim_ddl_v1_3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCTim1",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GTIM1",
            dma: Some("DMA1"),
            channel: None,
            request: Some(73),
        }],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GTIM1_UEV_ETR_BTIMA",
                interrupt: "GTIM1_UEV_ETR_BTIMA",
            },
            PeripheralInterrupt {
                signal: "GTIM1_CH01_BTIMB",
                interrupt: "GTIM1_CH01_BTIMB",
            },
            PeripheralInterrupt {
                signal: "GTIM1_CH23_BTIMC",
                interrupt: "GTIM1_CH23_BTIMC",
            },
        ],
    },
    Peripheral {
        name: "GTIM2",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ddl_v1_3",
            block: "GTIM",
            ir: &gtim_ddl_v1_3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralCTim2",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GTIM2",
            dma: Some("DMA1"),
            channel: None,
            request: Some(74),
        }],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GTIM2_UEV_ETR_BTIMA",
                interrupt: "GTIM2_UEV_ETR_BTIMA",
            },
            PeripheralInterrupt {
                signal: "GTIM2_CH01_BTIMB",
                interrupt: "GTIM2_CH01_BTIMB",
            },
            PeripheralInterrupt {
                signal: "GTIM2_CH23_BTIMC",
                interrupt: "GTIM2_CH23_BTIMC",
            },
        ],
    },
    Peripheral {
        name: "I2C0",
        address: 0x40002800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ddl_v2_2",
            block: "I2C",
            ir: &i2c_ddl_v2_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
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
        address: 0x40002c00,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ddl_v2_2",
            block: "I2C",
            ir: &i2c_ddl_v2_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
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
        name: "IWDT",
        address: 0x40008c80,
        registers: Some(PeripheralRegisters {
            kind: "iwdt",
            version: "ddl_v1",
            block: "IWDT",
            ir: &iwdt_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "APB1EN",
                field: "SysctrlPeripheralIwt",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "IWDT",
            interrupt: "IWDT",
        }],
    },
    Peripheral {
        name: "LVD",
        address: 0x4000aa00,
        registers: Some(PeripheralRegisters {
            kind: "lvd",
            version: "ddl_v1_4",
            block: "LVD",
            ir: &lvd_ddl_v1_4::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "APB1EN",
                field: "SysctrlPeripherallvd",
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
        name: "SPI0",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ddl_v2_2",
            block: "SPI",
            ir: &spi_ddl_v2_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralSpi0",
            }),
        }),
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI0_TX",
                interrupt: "SPI0_TX",
            },
            PeripheralInterrupt {
                signal: "SPI0_RX",
                interrupt: "SPI0_RX",
            },
            PeripheralInterrupt {
                signal: "SPI0_SSR",
                interrupt: "SPI0_SSR",
            },
            PeripheralInterrupt {
                signal: "SPI0_ERR",
                interrupt: "SPI0_ERR",
            },
        ],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40003c00,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ddl_v2_2",
            block: "SPI",
            ir: &spi_ddl_v2_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralSpi1",
            }),
        }),
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI1_TX",
                interrupt: "SPI1_TX",
            },
            PeripheralInterrupt {
                signal: "SPI1_RX",
                interrupt: "SPI1_RX",
            },
            PeripheralInterrupt {
                signal: "SPI1_SSR",
                interrupt: "SPI1_SSR",
            },
            PeripheralInterrupt {
                signal: "SPI1_ERR",
                interrupt: "SPI1_ERR",
            },
        ],
    },
    Peripheral {
        name: "SYSCTRL",
        address: 0x4000b800,
        registers: Some(PeripheralRegisters {
            kind: "sysctrl",
            version: "ddl_v4",
            block: "SYSCTRL",
            ir: &sysctrl_ddl_v4::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART0",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ddl_v1",
            block: "USART",
            ir: &usart_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralUsart0",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(65),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART0_RX",
                interrupt: "USART0_RX",
            },
            PeripheralInterrupt {
                signal: "USART0_TX",
                interrupt: "USART0_TX",
            },
            PeripheralInterrupt {
                signal: "USART0_FLAG",
                interrupt: "USART0_FLAG",
            },
        ],
    },
    Peripheral {
        name: "USART1",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ddl_v1",
            block: "USART",
            ir: &usart_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralUsart1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMA1"),
                channel: None,
                request: Some(67),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART1_RX",
                interrupt: "USART1_RX",
            },
            PeripheralInterrupt {
                signal: "USART1_TX",
                interrupt: "USART1_TX",
            },
            PeripheralInterrupt {
                signal: "USART1_FLAG",
                interrupt: "USART1_FLAG",
            },
        ],
    },
    Peripheral {
        name: "USART2",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ddl_v1",
            block: "USART",
            ir: &usart_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralUsart2",
            }),
        }),
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART2_RX",
                interrupt: "USART2_RX",
            },
            PeripheralInterrupt {
                signal: "USART2_TX",
                interrupt: "USART2_TX",
            },
            PeripheralInterrupt {
                signal: "USART2_FLAG",
                interrupt: "USART2_FLAG",
            },
        ],
    },
    Peripheral {
        name: "USART3",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ddl_v1",
            block: "USART",
            ir: &usart_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "AHB0EN",
                field: "SysctrlPeripheralUsart3",
            }),
        }),
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART3_RX",
                interrupt: "USART3_RX",
            },
            PeripheralInterrupt {
                signal: "USART3_TX",
                interrupt: "USART3_TX",
            },
            PeripheralInterrupt {
                signal: "USART3_FLAG",
                interrupt: "USART3_FLAG",
            },
        ],
    },
    Peripheral {
        name: "VC0",
        address: 0x4000a800,
        registers: Some(PeripheralRegisters {
            kind: "vc",
            version: "ddl_v1",
            block: "VC",
            ir: &vc_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "APB1EN",
                field: "SysctrlPeripheralVc",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "VC",
            interrupt: "VC",
        }],
    },
    Peripheral {
        name: "VC1",
        address: 0x4000a810,
        registers: Some(PeripheralRegisters {
            kind: "vc",
            version: "ddl_v1",
            block: "VC",
            ir: &vc_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "APB1EN",
                field: "SysctrlPeripheralVc",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "VC",
            interrupt: "VC",
        }],
    },
    Peripheral {
        name: "WWDT",
        address: 0x40008800,
        registers: Some(PeripheralRegisters {
            kind: "wwdt",
            version: "ddl_v1",
            block: "WWDT",
            ir: &wwdt_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "APB1EN",
                field: "SysctrlPeripheralWwt",
            }),
        }),
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
        name: "CTRIM",
        number: 0,
    },
    Interrupt {
        name: "VC",
        number: 1,
    },
    Interrupt {
        name: "LVD",
        number: 2,
    },
    Interrupt {
        name: "PORTA",
        number: 3,
    },
    Interrupt {
        name: "PORTB",
        number: 4,
    },
    Interrupt {
        name: "PORTC",
        number: 5,
    },
    Interrupt {
        name: "PORTD",
        number: 6,
    },
    Interrupt {
        name: "IWDT",
        number: 7,
    },
    Interrupt {
        name: "WWDT",
        number: 8,
    },
    Interrupt {
        name: "CLK_FAULT",
        number: 9,
    },
    Interrupt {
        name: "ADC0_ADC1",
        number: 10,
    },
    Interrupt {
        name: "FLASH",
        number: 11,
    },
    Interrupt {
        name: "USART0_RX",
        number: 12,
    },
    Interrupt {
        name: "USART0_TX",
        number: 13,
    },
    Interrupt {
        name: "USART0_FLAG",
        number: 14,
    },
    Interrupt {
        name: "USART1_RX",
        number: 15,
    },
    Interrupt {
        name: "USART1_TX",
        number: 16,
    },
    Interrupt {
        name: "USART1_FLAG",
        number: 17,
    },
    Interrupt {
        name: "USART2_RX",
        number: 18,
    },
    Interrupt {
        name: "USART2_TX",
        number: 19,
    },
    Interrupt {
        name: "USART2_FLAG",
        number: 20,
    },
    Interrupt {
        name: "USART3_RX",
        number: 21,
    },
    Interrupt {
        name: "USART3_TX",
        number: 22,
    },
    Interrupt {
        name: "USART3_FLAG",
        number: 23,
    },
    Interrupt {
        name: "I2C0",
        number: 24,
    },
    Interrupt {
        name: "I2C1",
        number: 25,
    },
    Interrupt {
        name: "SPI0_TX",
        number: 26,
    },
    Interrupt {
        name: "SPI0_RX",
        number: 27,
    },
    Interrupt {
        name: "SPI0_SSR",
        number: 28,
    },
    Interrupt {
        name: "SPI0_ERR",
        number: 29,
    },
    Interrupt {
        name: "SPI1_TX",
        number: 30,
    },
    Interrupt {
        name: "SPI1_RX",
        number: 31,
    },
    Interrupt {
        name: "SPI1_SSR",
        number: 32,
    },
    Interrupt {
        name: "SPI1_ERR",
        number: 33,
    },
    Interrupt {
        name: "GTIM0_UEV_ETR_BTIMA",
        number: 34,
    },
    Interrupt {
        name: "GTIM0_CH01_BTIMB",
        number: 35,
    },
    Interrupt {
        name: "GTIM0_CH23_BTIMC",
        number: 36,
    },
    Interrupt {
        name: "GTIM1_UEV_ETR_BTIMA",
        number: 37,
    },
    Interrupt {
        name: "GTIM1_CH01_BTIMB",
        number: 38,
    },
    Interrupt {
        name: "GTIM1_CH23_BTIMC",
        number: 39,
    },
    Interrupt {
        name: "GTIM2_UEV_ETR_BTIMA",
        number: 40,
    },
    Interrupt {
        name: "GTIM2_CH01_BTIMB",
        number: 41,
    },
    Interrupt {
        name: "GTIM2_CH23_BTIMC",
        number: 42,
    },
    Interrupt {
        name: "ATIM0_UEV",
        number: 43,
    },
    Interrupt {
        name: "ATIM0_CHA",
        number: 44,
    },
    Interrupt {
        name: "ATIM0_CHB",
        number: 45,
    },
    Interrupt {
        name: "ATIM0_BKETR",
        number: 46,
    },
    Interrupt {
        name: "ATIM3_UEV",
        number: 47,
    },
    Interrupt {
        name: "ATIM3_CHA",
        number: 48,
    },
    Interrupt {
        name: "ATIM3_CHB",
        number: 49,
    },
    Interrupt {
        name: "ATIM3_BK_ETR_OV_UD_CMP3",
        number: 50,
    },
    Interrupt {
        name: "DMA_CH0",
        number: 51,
    },
    Interrupt {
        name: "DMA_CH1",
        number: 52,
    },
    Interrupt {
        name: "DMA_CH2",
        number: 53,
    },
    Interrupt {
        name: "DMA_CH3",
        number: 54,
    },
    Interrupt {
        name: "DMA_CH4",
        number: 55,
    },
    Interrupt {
        name: "DMA_CH5",
        number: 56,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/adc_ddl_v3.rs"]
pub mod adc_ddl_v3;
#[path = "../registers/adc_ddl_v3_1.rs"]
pub mod adc_ddl_v3_1;
#[path = "../registers/btim_ddl_v1_1.rs"]
pub mod btim_ddl_v1_1;
#[path = "../registers/btim_ddl_v1_2.rs"]
pub mod btim_ddl_v1_2;
#[path = "../registers/btim_ddl_v1_3.rs"]
pub mod btim_ddl_v1_3;
#[path = "../registers/crc_ddl_v2_2.rs"]
pub mod crc_ddl_v2_2;
#[path = "../registers/ctrim_ddl_v1_1.rs"]
pub mod ctrim_ddl_v1_1;
#[path = "../registers/dmac_ddl_v2_1.rs"]
pub mod dmac_ddl_v2_1;
#[path = "../registers/flash_ddl_v2.rs"]
pub mod flash_ddl_v2;
#[path = "../registers/gpio_ddl_v3_1.rs"]
pub mod gpio_ddl_v3_1;
#[path = "../registers/gpioaux_ddl_v1_2.rs"]
pub mod gpioaux_ddl_v1_2;
#[path = "../registers/gtim_ddl_v1_3.rs"]
pub mod gtim_ddl_v1_3;
#[path = "../registers/i2c_ddl_v2_2.rs"]
pub mod i2c_ddl_v2_2;
#[path = "../registers/iwdt_ddl_v1.rs"]
pub mod iwdt_ddl_v1;
#[path = "../registers/lvd_ddl_v1_4.rs"]
pub mod lvd_ddl_v1_4;
#[path = "../registers/spi_ddl_v2_2.rs"]
pub mod spi_ddl_v2_2;
#[path = "../registers/sysctrl_ddl_v4.rs"]
pub mod sysctrl_ddl_v4;
#[path = "../registers/usart_ddl_v1.rs"]
pub mod usart_ddl_v1;
#[path = "../registers/vc_ddl_v1.rs"]
pub mod vc_ddl_v1;
#[path = "../registers/wwdt_ddl_v1.rs"]
pub mod wwdt_ddl_v1;
