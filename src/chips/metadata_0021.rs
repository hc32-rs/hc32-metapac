
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ddl_v2",
            block: "ADC",
            ir: &adc_ddl_v2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockAdc",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "ADC",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "ATIM3",
        address: 0x40009800,
        registers: None,
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockAtim3",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "ATIM3",
            interrupt: "ATIM3",
        }],
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
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN1",
                field: "PeriphClockCtim0",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN1",
                field: "PeriphClockCtim0",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN1",
                field: "PeriphClockCtim0",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BTIM3",
        address: 0x40001c00,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1",
            block: "BTIM",
            ir: &btim_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN1",
                field: "PeriphClockCtim1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BTIM4",
        address: 0x40001d00,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1",
            block: "BTIM",
            ir: &btim_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN1",
                field: "PeriphClockCtim1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BTIM5",
        address: 0x40001e00,
        registers: Some(PeripheralRegisters {
            kind: "btim",
            version: "ddl_v1",
            block: "BTIM",
            ir: &btim_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN1",
                field: "PeriphClockCtim1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CTRIM",
        address: 0x40009000,
        registers: Some(PeripheralRegisters {
            kind: "ctrim",
            version: "ddl_v1_2",
            block: "CTRIM",
            ir: &ctrim_ddl_v1_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN1",
                field: "PeriphClockCtrim",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "CTRIM_CLKDET",
            interrupt: "CTRIM_CLKDET",
        }],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "ddl_v1",
            block: "FLASH",
            ir: &flash_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockFlash",
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
        address: 0x40020c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v3_2",
            block: "GPIO",
            ir: &gpio_ddl_v3_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockGpio",
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
        address: 0x40020c00,
        registers: Some(PeripheralRegisters {
            kind: "gpioaux",
            version: "ddl_v1_3",
            block: "GPIOAUX",
            ir: &gpioaux_ddl_v1_3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockGpio",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 0x40020c40,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ddl_v1",
            block: "GPIO",
            ir: &gpio_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockGpio",
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
        name: "GTIM0",
        address: 0x40001800,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ddl_v1_1",
            block: "GTIM",
            ir: &gtim_ddl_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN1",
                field: "PeriphClockCtim0",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM1",
        address: 0x40001c00,
        registers: Some(PeripheralRegisters {
            kind: "gtim",
            version: "ddl_v1_1",
            block: "GTIM",
            ir: &gtim_ddl_v1_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN1",
                field: "PeriphClockCtim1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "HSI2C",
        address: 0x4000bc00,
        registers: Some(PeripheralRegisters {
            kind: "hsi2c",
            version: "ddl_v1",
            block: "HSI2C",
            ir: &hsi2c_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockHsi2c",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "HSI2C",
            interrupt: "HSI2C",
        }],
    },
    Peripheral {
        name: "IWDT",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "iwdt",
            version: "ddl_v1_2",
            block: "IWDT",
            ir: &iwdt_ddl_v1_2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockIwdt",
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
        name: "LPUART0",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "lpuart",
            version: "ddl_v4",
            block: "LPUART",
            ir: &lpuart_ddl_v4::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockLpuart0",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "LPUART0",
            interrupt: "LPUART0",
        }],
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40008000,
        registers: Some(PeripheralRegisters {
            kind: "lpuart",
            version: "ddl_v4",
            block: "LPUART",
            ir: &lpuart_ddl_v4::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockLpuart1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "LPUART1",
            interrupt: "LPUART1",
        }],
    },
    Peripheral {
        name: "LVD",
        address: 0x40002c00,
        registers: Some(PeripheralRegisters {
            kind: "lvd",
            version: "ddl_v1_5",
            block: "LVD",
            ir: &lvd_ddl_v1_5::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "LVD",
            interrupt: "LVD",
        }],
    },
    Peripheral {
        name: "RTC",
        address: 0x40001400,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "ddl_v2_1",
            block: "RTC",
            ir: &rtc_ddl_v2_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockRtc",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "RTC",
            interrupt: "RTC",
        }],
    },
    Peripheral {
        name: "SPI",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ddl_v2_1",
            block: "SPI",
            ir: &spi_ddl_v2_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockSpi",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "SPI",
            interrupt: "SPI",
        }],
    },
    Peripheral {
        name: "SYSCTRL",
        address: 0x40021800,
        registers: Some(PeripheralRegisters {
            kind: "sysctrl",
            version: "ddl_v2_3",
            block: "SYSCTRL",
            ir: &sysctrl_ddl_v2_3::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TRNG",
        address: 0x40008c00,
        registers: Some(PeripheralRegisters {
            kind: "trng",
            version: "ddl_v1",
            block: "TRNG",
            ir: &trng_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockTrng",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "VC",
        address: 0x40002800,
        registers: Some(PeripheralRegisters {
            kind: "vc",
            version: "ddl_v2_3",
            block: "VC",
            ir: &vc_ddl_v2_3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN0",
                field: "PeriphClockVc",
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
                signal: "VC1",
                interrupt: "VC1",
            },
        ],
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
        name: "SPI",
        number: 10,
    },
    Interrupt {
        name: "CTIM0",
        number: 14,
    },
    Interrupt {
        name: "CTIM1",
        number: 15,
    },
    Interrupt {
        name: "HSI2C",
        number: 18,
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
        name: "VC0",
        number: 26,
    },
    Interrupt {
        name: "VC1",
        number: 27,
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
        name: "CTRIM_CLKDET",
        number: 31,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/adc_ddl_v2.rs"]
pub mod adc_ddl_v2;
#[path = "../registers/btim_ddl_v1.rs"]
pub mod btim_ddl_v1;
#[path = "../registers/ctrim_ddl_v1_2.rs"]
pub mod ctrim_ddl_v1_2;
#[path = "../registers/flash_ddl_v1.rs"]
pub mod flash_ddl_v1;
#[path = "../registers/gpio_ddl_v1.rs"]
pub mod gpio_ddl_v1;
#[path = "../registers/gpio_ddl_v3_2.rs"]
pub mod gpio_ddl_v3_2;
#[path = "../registers/gpioaux_ddl_v1_3.rs"]
pub mod gpioaux_ddl_v1_3;
#[path = "../registers/gtim_ddl_v1_1.rs"]
pub mod gtim_ddl_v1_1;
#[path = "../registers/hsi2c_ddl_v1.rs"]
pub mod hsi2c_ddl_v1;
#[path = "../registers/iwdt_ddl_v1_2.rs"]
pub mod iwdt_ddl_v1_2;
#[path = "../registers/lpuart_ddl_v4.rs"]
pub mod lpuart_ddl_v4;
#[path = "../registers/lvd_ddl_v1_5.rs"]
pub mod lvd_ddl_v1_5;
#[path = "../registers/rtc_ddl_v2_1.rs"]
pub mod rtc_ddl_v2_1;
#[path = "../registers/spi_ddl_v2_1.rs"]
pub mod spi_ddl_v2_1;
#[path = "../registers/sysctrl_ddl_v2_3.rs"]
pub mod sysctrl_ddl_v2_3;
#[path = "../registers/trng_ddl_v1.rs"]
pub mod trng_ddl_v1;
#[path = "../registers/vc_ddl_v2_3.rs"]
pub mod vc_ddl_v2_3;
