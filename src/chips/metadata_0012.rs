
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40007800,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ll_v1",
            block: "ADC",
            ir: &adc_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_ADC",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "JRQ",
                dma: Some("DMAC"),
                channel: None,
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "EOC",
                dma: Some("DMAC"),
                channel: None,
                request: Some(65),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "ATIM3",
        address: 0x4000e400,
        registers: Some(PeripheralRegisters {
            kind: "atim",
            version: "ll_v1",
            block: "ATIM",
            ir: &atim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "SYSCTRL_APBEN1_ATIM3",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 0x40027800,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "ll_v1",
            block: "CRC",
            ir: &crc_ll_v1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CSU0",
        address: 0x40025100,
        registers: Some(PeripheralRegisters {
            kind: "csu",
            version: "ll_v1",
            block: "CSU",
            ir: &csu_ll_v1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CSU1",
        address: 0x40025100,
        registers: Some(PeripheralRegisters {
            kind: "csu",
            version: "ll_v1",
            block: "CSU",
            ir: &csu_ll_v1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CSU2",
        address: 0x40025100,
        registers: Some(PeripheralRegisters {
            kind: "csu",
            version: "ll_v1",
            block: "CSU",
            ir: &csu_ll_v1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CTRIM",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "ctrim",
            version: "ll_v1",
            block: "CTRIM",
            ir: &ctrim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_CTRIM",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGC",
        address: 0x40001c00,
        registers: Some(PeripheralRegisters {
            kind: "dbgc",
            version: "ll_v1",
            block: "DBGC",
            ir: &dbgc_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_DBGC",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DFM",
        address: 0x40026800,
        registers: Some(PeripheralRegisters {
            kind: "dfm",
            version: "ll_v1",
            block: "DFM",
            ir: &dfm_ll_v1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMAC",
        address: 0x40024800,
        registers: Some(PeripheralRegisters {
            kind: "dmac",
            version: "ll_v1",
            block: "DMAC",
            ir: &dmac_ll_v1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EFM",
        address: 0x40026400,
        registers: Some(PeripheralRegisters {
            kind: "efm",
            version: "ll_v3",
            block: "EFM",
            ir: &efm_ll_v3::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EXTI",
        address: 0x40024000,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "ll_v1",
            block: "EXTI",
            ir: &exti_ll_v1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ll_v1",
            block: "GPIO",
            ir: &gpio_ll_v1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM0",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ll_v1",
            block: "GTIM",
            ir: &gtim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_GTIM0",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM1",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ll_v1",
            block: "GTIM",
            ir: &gtim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_GTIM1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM2",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ll_v1",
            block: "GTIM",
            ir: &gtim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_GTIM2",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM3",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ll_v1",
            block: "GTIM",
            ir: &gtim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_GTIM3",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM4",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ll_v1",
            block: "GTIM",
            ir: &gtim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_GTIM4",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM5",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ll_v1",
            block: "GTIM",
            ir: &gtim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_GTIM5",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM6",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ll_v1",
            block: "GTIM",
            ir: &gtim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "SYSCTRL_APBEN1_GTIM6",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM7",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ll_v1",
            block: "GTIM",
            ir: &gtim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "SYSCTRL_APBEN1_GTIM7",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM8",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ll_v1",
            block: "GTIM",
            ir: &gtim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "SYSCTRL_APBEN1_GTIM8",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C0",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ll_v1",
            block: "I2C",
            ir: &i2c_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_I2C0",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C1",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ll_v1",
            block: "I2C",
            ir: &i2c_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "SYSCTRL_APBEN1_I2C1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "IWDT",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "iwdt",
            version: "ll_v1",
            block: "IWDT",
            ir: &iwdt_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_IWDT",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPTIM0",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "ll_v1",
            block: "LPTIM",
            ir: &lptim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_LPTIM0",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPTIM1",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "ll_v1",
            block: "LPTIM",
            ir: &lptim_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "SYSCTRL_APBEN1_LPTIM1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPUART0",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "lpuart",
            version: "ll_v1",
            block: "LPUART",
            ir: &lpuart_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_LPUART0",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(82),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(83),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "lpuart",
            version: "ll_v1",
            block: "LPUART",
            ir: &lpuart_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "SYSCTRL_APBEN1_LPUART1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(84),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(85),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "LVD",
        address: 0x40007400,
        registers: Some(PeripheralRegisters {
            kind: "lvd",
            version: "ll_v1",
            block: "LVD",
            ir: &lvd_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_LVD",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "ll_v2",
            block: "RTC",
            ir: &rtc_ll_v2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_RTC",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI0",
        address: 0x40004000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ll_v1",
            block: "SPI",
            ir: &spi_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_SPI0",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(67),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40004000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ll_v1",
            block: "SPI",
            ir: &spi_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "SYSCTRL_APBEN1_SPI1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(69),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SRAM",
        address: 0x40024c00,
        registers: Some(PeripheralRegisters {
            kind: "sram",
            version: "ll_v1",
            block: "SRAM",
            ir: &sram_ll_v1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SYSCTRL",
        address: 0x40025000,
        registers: Some(PeripheralRegisters {
            kind: "sysctrl",
            version: "ll_v1",
            block: "SYSCTRL",
            ir: &sysctrl_ll_v1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART0",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v2_15",
            block: "USART",
            ir: &usart_ll_v2_15::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_USART0",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(71),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART1",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v2_15",
            block: "USART",
            ir: &usart_ll_v2_15::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_USART1",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(73),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART2",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v2_15",
            block: "USART",
            ir: &usart_ll_v2_15::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_USART2",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(75),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART3",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v2_15",
            block: "USART",
            ir: &usart_ll_v2_15::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "SYSCTRL_APBEN1_USART3",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(77),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART4",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v2_15",
            block: "USART",
            ir: &usart_ll_v2_15::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "SYSCTRL_APBEN1_USART4",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(78),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(79),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART5",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v2_15",
            block: "USART",
            ir: &usart_ll_v2_15::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG1",
                field: "SYSCTRL_APBEN1_USART5",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some("DMAC"),
                channel: None,
                request: Some(81),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "WWDT",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "wwdt",
            version: "ll_v1",
            block: "WWDT",
            ir: &wwdt_ll_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "SYSCTRL_APBEN0_WWDT",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDT",
        number: 0,
    },
    Interrupt {
        name: "IWDT",
        number: 1,
    },
    Interrupt {
        name: "LVD",
        number: 2,
    },
    Interrupt {
        name: "INPUT0_3",
        number: 3,
    },
    Interrupt {
        name: "INPUT4_7",
        number: 4,
    },
    Interrupt {
        name: "INPUT8_11",
        number: 5,
    },
    Interrupt {
        name: "INPUT12_15",
        number: 6,
    },
    Interrupt {
        name: "DMAC",
        number: 7,
    },
    Interrupt {
        name: "ATIM3",
        number: 8,
    },
    Interrupt {
        name: "USART0",
        number: 9,
    },
    Interrupt {
        name: "USART1",
        number: 10,
    },
    Interrupt {
        name: "LPUART0",
        number: 11,
    },
    Interrupt {
        name: "LPUART1",
        number: 12,
    },
    Interrupt {
        name: "SPI0",
        number: 13,
    },
    Interrupt {
        name: "SPI1",
        number: 14,
    },
    Interrupt {
        name: "I2C0",
        number: 15,
    },
    Interrupt {
        name: "I2C1",
        number: 16,
    },
    Interrupt {
        name: "GTIM0_1",
        number: 17,
    },
    Interrupt {
        name: "GTIM2_3",
        number: 18,
    },
    Interrupt {
        name: "GTIM4_5",
        number: 19,
    },
    Interrupt {
        name: "GTIM6_7_8",
        number: 20,
    },
    Interrupt {
        name: "LPTIM0_1",
        number: 21,
    },
    Interrupt {
        name: "USART2",
        number: 22,
    },
    Interrupt {
        name: "USART3",
        number: 23,
    },
    Interrupt {
        name: "ADC",
        number: 24,
    },
    Interrupt {
        name: "USART4",
        number: 25,
    },
    Interrupt {
        name: "USART5",
        number: 26,
    },
    Interrupt {
        name: "WKUPIO",
        number: 27,
    },
    Interrupt {
        name: "RTC",
        number: 28,
    },
    Interrupt {
        name: "EFM_DFM",
        number: 30,
    },
    Interrupt {
        name: "CTRIM",
        number: 31,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/adc_ll_v1.rs"]
pub mod adc_ll_v1;
#[path = "../registers/atim_ll_v1.rs"]
pub mod atim_ll_v1;
#[path = "../registers/crc_ll_v1.rs"]
pub mod crc_ll_v1;
#[path = "../registers/csu_ll_v1.rs"]
pub mod csu_ll_v1;
#[path = "../registers/ctrim_ll_v1.rs"]
pub mod ctrim_ll_v1;
#[path = "../registers/dbgc_ll_v1.rs"]
pub mod dbgc_ll_v1;
#[path = "../registers/dfm_ll_v1.rs"]
pub mod dfm_ll_v1;
#[path = "../registers/dmac_ll_v1.rs"]
pub mod dmac_ll_v1;
#[path = "../registers/efm_ll_v3.rs"]
pub mod efm_ll_v3;
#[path = "../registers/exti_ll_v1.rs"]
pub mod exti_ll_v1;
#[path = "../registers/gpio_ll_v1.rs"]
pub mod gpio_ll_v1;
#[path = "../registers/gtim_ll_v1.rs"]
pub mod gtim_ll_v1;
#[path = "../registers/i2c_ll_v1.rs"]
pub mod i2c_ll_v1;
#[path = "../registers/iwdt_ll_v1.rs"]
pub mod iwdt_ll_v1;
#[path = "../registers/lptim_ll_v1.rs"]
pub mod lptim_ll_v1;
#[path = "../registers/lpuart_ll_v1.rs"]
pub mod lpuart_ll_v1;
#[path = "../registers/lvd_ll_v1.rs"]
pub mod lvd_ll_v1;
#[path = "../registers/rtc_ll_v2.rs"]
pub mod rtc_ll_v2;
#[path = "../registers/spi_ll_v1.rs"]
pub mod spi_ll_v1;
#[path = "../registers/sram_ll_v1.rs"]
pub mod sram_ll_v1;
#[path = "../registers/sysctrl_ll_v1.rs"]
pub mod sysctrl_ll_v1;
#[path = "../registers/usart_ll_v2_15.rs"]
pub mod usart_ll_v2_15;
#[path = "../registers/wwdt_ll_v1.rs"]
pub mod wwdt_ll_v1;
