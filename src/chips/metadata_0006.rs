
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ddl_v1",
            block: "ADC",
            ir: &adc_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralADC",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "ADC",
            interrupt: "ADC",
        }],
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
                    version: "ddl_v1_4",
                    block: "ATIM",
                    ir: &atim_ddl_v1_4::REGISTERS,
                },
            },
            PeripheralView {
                name: "ATIM3MODE1",
                mode: "MODE1",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v2",
                    block: "ATIM",
                    ir: &atim_ddl_v2::REGISTERS,
                },
            },
            PeripheralView {
                name: "ATIM3MODE23",
                mode: "MODE23",
                registers: PeripheralRegisters {
                    kind: "atim",
                    version: "ddl_v4_2",
                    block: "ATIM",
                    ir: &atim_ddl_v4_2::REGISTERS,
                },
            },
        ],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralATIM3",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "ATIM3",
            interrupt: "ATIM3",
        }],
    },
    Peripheral {
        name: "BTIM3",
        address: 0x40007400,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_1",
            block: "BTIM",
            ir: &btim_ddl_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralGTIM",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "BTIM3_4_5",
            interrupt: "BTIM3_4_5",
        }],
    },
    Peripheral {
        name: "BTIM4",
        address: 0x40007500,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_1",
            block: "BTIM",
            ir: &btim_ddl_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralGTIM",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "BTIM3_4_5",
            interrupt: "BTIM3_4_5",
        }],
    },
    Peripheral {
        name: "BTIM5",
        address: 0x40007600,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1_1",
            block: "BTIM",
            ir: &btim_ddl_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralGTIM",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "BTIM3_4_5",
            interrupt: "BTIM3_4_5",
        }],
    },
    Peripheral {
        name: "CTRIM",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "ctrim",
            version: "ddl_v1",
            block: "CTRIM",
            ir: &ctrim_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralCTRIM",
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
        name: "FLASH",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "ddl_v1_2",
            block: "FLASH",
            ir: &flash_ddl_v1_2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralFLASH",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "FLASH",
            interrupt: "FLASH",
        }],
    },
    Peripheral {
        name: "GPIOA",
        address: 0x40020c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3",
            block: "GPIO",
            ir: &gpio_ddl_v3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralPA",
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
        address: 0x40020ca0,
        registers: Some(PeripheralRegisters {
            kind: "gpioaux",
            version: "ddl_v1",
            block: "GPIOAUX",
            ir: &gpioaux_ddl_v1::REGISTERS,
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
        address: 0x40020d00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3",
            block: "GPIO",
            ir: &gpio_ddl_v3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralPB",
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
        address: 0x40020e00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3",
            block: "GPIO",
            ir: &gpio_ddl_v3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralPC",
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
        address: 0x40020f00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3",
            block: "GPIO",
            ir: &gpio_ddl_v3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralPD",
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
        name: "GTIM",
        address: 0x40007400,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ddl_v1",
            block: "GTIM",
            ir: &gtim_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralGTIM",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GTIM",
            interrupt: "BTIM3_4_5",
        }],
    },
    Peripheral {
        name: "I2C",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ddl_v2_1",
            block: "I2C",
            ir: &i2c_ddl_v2_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralI2C",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "I2C",
            interrupt: "I2C",
        }],
    },
    Peripheral {
        name: "IWDT",
        address: 0x40000f80,
        registers: Some(PeripheralRegisters {
            kind: "iwdt",
            version: "ddl_v1_1",
            block: "IWDT",
            ir: &iwdt_ddl_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralIWDT",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "IWDT_WWDT",
            interrupt: "IWDT_WWDT",
        }],
    },
    Peripheral {
        name: "LPUART0",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "lpuart",
            version: "ddl_v3",
            block: "LPUART",
            ir: &lpuart_ddl_v3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralLPUART0",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
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
            version: "ddl_v3",
            block: "LPUART",
            ir: &lpuart_ddl_v3::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralLPUART1",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "LPUART1",
            interrupt: "LPUART1",
        }],
    },
    Peripheral {
        name: "LVD",
        address: 0x40002680,
        registers: Some(PeripheralRegisters {
            kind: "lvd",
            version: "ddl_v1_4",
            block: "LVD",
            ir: &lvd_ddl_v1_4::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "LVD",
            interrupt: "LVD",
        }],
    },
    Peripheral {
        name: "SPI",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ddl_v2",
            block: "SPI",
            ir: &spi_ddl_v2::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERICLKEN0",
                field: "SysctrlPeripheralSPI",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "SPI0",
            interrupt: "SPI0",
        }],
    },
    Peripheral {
        name: "SYSCTRL",
        address: 0x40002000,
        registers: Some(PeripheralRegisters {
            kind: "sysctrl",
            version: "ddl_v1",
            block: "SYSCTRL",
            ir: &sysctrl_ddl_v1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
                field: "SysctrlPeripheralWWDT",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "IWDT_WWDT",
            interrupt: "IWDT_WWDT",
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
        name: "ATIM3",
        number: 5,
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
        name: "I2C",
        number: 12,
    },
    Interrupt {
        name: "BTIM3_4_5",
        number: 15,
    },
    Interrupt {
        name: "IWDT_WWDT",
        number: 22,
    },
    Interrupt {
        name: "ADC",
        number: 24,
    },
    Interrupt {
        name: "LVD",
        number: 28,
    },
    Interrupt {
        name: "FLASH",
        number: 30,
    },
    Interrupt {
        name: "CTRIM",
        number: 31,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/adc_ddl_v1.rs"]
pub mod adc_ddl_v1;
#[path = "../registers/atim_ddl_v1_4.rs"]
pub mod atim_ddl_v1_4;
#[path = "../registers/atim_ddl_v2.rs"]
pub mod atim_ddl_v2;
#[path = "../registers/atim_ddl_v4_2.rs"]
pub mod atim_ddl_v4_2;
#[path = "../registers/btim_ddl_v1_1.rs"]
pub mod btim_ddl_v1_1;
#[path = "../registers/ctrim_ddl_v1.rs"]
pub mod ctrim_ddl_v1;
#[path = "../registers/flash_ddl_v1_2.rs"]
pub mod flash_ddl_v1_2;
#[path = "../registers/gpio_ddl_v3.rs"]
pub mod gpio_ddl_v3;
#[path = "../registers/gpioaux_ddl_v1.rs"]
pub mod gpioaux_ddl_v1;
#[path = "../registers/gtim_ddl_v1.rs"]
pub mod gtim_ddl_v1;
#[path = "../registers/i2c_ddl_v2_1.rs"]
pub mod i2c_ddl_v2_1;
#[path = "../registers/iwdt_ddl_v1_1.rs"]
pub mod iwdt_ddl_v1_1;
#[path = "../registers/lpuart_ddl_v3.rs"]
pub mod lpuart_ddl_v3;
#[path = "../registers/lvd_ddl_v1_4.rs"]
pub mod lvd_ddl_v1_4;
#[path = "../registers/spi_ddl_v2.rs"]
pub mod spi_ddl_v2;
#[path = "../registers/sysctrl_ddl_v1.rs"]
pub mod sysctrl_ddl_v1;
#[path = "../registers/wwdt_ddl_v1.rs"]
pub mod wwdt_ddl_v1;
