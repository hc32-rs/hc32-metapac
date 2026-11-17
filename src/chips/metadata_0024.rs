
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ddl_v4",
            block: "ADC",
            ir: &adc_ddl_v4::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralAdcBgr",
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
        name: "ADT0",
        address: 0x40003000,
        registers: Some(PeripheralRegisters {
            kind: "adt",
            version: "ddl_v1",
            block: "ADT",
            ir: &adt_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralAdt",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ADT1",
        address: 0x40003400,
        registers: Some(PeripheralRegisters {
            kind: "adt",
            version: "ddl_v1",
            block: "ADT",
            ir: &adt_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralAdt",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ADT2",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "adt",
            version: "ddl_v1",
            block: "ADT",
            ir: &adt_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralAdt",
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
            version: "ddl_v1",
            block: "BGR",
            ir: &bgr_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralAdcBgr",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BT0",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "bt",
            version: "ddl_v1",
            block: "BT",
            ir: &bt_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralBt",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BT1",
        address: 0x40000c20,
        registers: Some(PeripheralRegisters {
            kind: "bt",
            version: "ddl_v1",
            block: "BT",
            ir: &bt_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralBt",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BT2",
        address: 0x40000c40,
        registers: Some(PeripheralRegisters {
            kind: "bt",
            version: "ddl_v1",
            block: "BT",
            ir: &bt_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralBt",
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
            version: "ddl_v1",
            block: "CLK_TRIM",
            ir: &clk_trim_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralClkTrim",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "CLK_TRIM",
            interrupt: "CLK_TRIM",
        }],
    },
    Peripheral {
        name: "CLOCK",
        address: 0x40002000,
        registers: Some(PeripheralRegisters {
            kind: "sysctrl",
            version: "ddl_v2_2",
            block: "SYSCTRL",
            ir: &sysctrl_ddl_v2_2::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 0x40020900,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "ddl_v1",
            block: "CRC",
            ir: &crc_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralCrc",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "ddl_v3",
            block: "FLASH",
            ir: &flash_ddl_v3::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralFlash",
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
            version: "ddl_v2",
            block: "GPIO",
            ir: &gpio_ddl_v2::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralGpio",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ddl_v1",
            block: "I2C",
            ir: &i2c_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralI2c",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "I2C",
            interrupt: "I2C",
        }],
    },
    Peripheral {
        name: "LPTIMER",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "ddl_v1",
            block: "LPTIM",
            ir: &lptim_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralLpTim",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPUART",
        address: 0x40000200,
        registers: Some(PeripheralRegisters {
            kind: "lpuart",
            version: "ddl_v1",
            block: "LPUART",
            ir: &lpuart_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralLpUart",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "LPUART",
            interrupt: "LPUART",
        }],
    },
    Peripheral {
        name: "LVD",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "lvd",
            version: "ddl_v1",
            block: "LVD",
            ir: &lvd_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralVcLvd",
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
        name: "PCA",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "pca",
            version: "ddl_v1",
            block: "PCA",
            ir: &pca_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralPca",
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
            version: "ddl_v1",
            block: "RAM",
            ir: &ram_ddl_v1::REGISTERS,
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
            version: "ddl_v1_1",
            block: "RESET",
            ir: &reset_ddl_v1_1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0x40001400,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "ddl_v1",
            block: "RTC",
            ir: &rtc_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralRtc",
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
            version: "ddl_v1",
            block: "SPI",
            ir: &spi_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralSpi",
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
        name: "UART0",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ddl_v1",
            block: "UART",
            ir: &uart_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralUart0",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "UART0",
            interrupt: "UART0",
        }],
    },
    Peripheral {
        name: "UART1",
        address: 0x40000100,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ddl_v1",
            block: "UART",
            ir: &uart_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralUart1",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "UART1",
            interrupt: "UART1",
        }],
    },
    Peripheral {
        name: "VC",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "vc",
            version: "ddl_v2_1",
            block: "VC",
            ir: &vc_ddl_v2_1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralVcLvd",
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
    Peripheral {
        name: "WDT",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "wdt",
            version: "ddl_v1",
            block: "WDT",
            ir: &wdt_ddl_v1::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "PERI_CLKEN",
                field: "ClkPeripheralWdt",
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
        name: "PORT0",
        number: 0,
    },
    Interrupt {
        name: "PORT1",
        number: 1,
    },
    Interrupt {
        name: "PORT2",
        number: 2,
    },
    Interrupt {
        name: "PORT3",
        number: 3,
    },
    Interrupt {
        name: "UART0",
        number: 6,
    },
    Interrupt {
        name: "UART1",
        number: 7,
    },
    Interrupt {
        name: "LPUART",
        number: 8,
    },
    Interrupt {
        name: "SPI",
        number: 10,
    },
    Interrupt {
        name: "I2C",
        number: 12,
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
        name: "LPTIM",
        number: 17,
    },
    Interrupt {
        name: "TIM4",
        number: 18,
    },
    Interrupt {
        name: "TIM5",
        number: 19,
    },
    Interrupt {
        name: "TIM6",
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
        name: "FLASH_RAM",
        number: 30,
    },
    Interrupt {
        name: "CLK_TRIM",
        number: 31,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/adc_ddl_v4.rs"]
pub mod adc_ddl_v4;
#[path = "../registers/adt_ddl_v1.rs"]
pub mod adt_ddl_v1;
#[path = "../registers/bgr_ddl_v1.rs"]
pub mod bgr_ddl_v1;
#[path = "../registers/bt_ddl_v1.rs"]
pub mod bt_ddl_v1;
#[path = "../registers/clk_trim_ddl_v1.rs"]
pub mod clk_trim_ddl_v1;
#[path = "../registers/crc_ddl_v1.rs"]
pub mod crc_ddl_v1;
#[path = "../registers/flash_ddl_v3.rs"]
pub mod flash_ddl_v3;
#[path = "../registers/gpio_ddl_v2.rs"]
pub mod gpio_ddl_v2;
#[path = "../registers/i2c_ddl_v1.rs"]
pub mod i2c_ddl_v1;
#[path = "../registers/lptim_ddl_v1.rs"]
pub mod lptim_ddl_v1;
#[path = "../registers/lpuart_ddl_v1.rs"]
pub mod lpuart_ddl_v1;
#[path = "../registers/lvd_ddl_v1.rs"]
pub mod lvd_ddl_v1;
#[path = "../registers/pca_ddl_v1.rs"]
pub mod pca_ddl_v1;
#[path = "../registers/ram_ddl_v1.rs"]
pub mod ram_ddl_v1;
#[path = "../registers/reset_ddl_v1_1.rs"]
pub mod reset_ddl_v1_1;
#[path = "../registers/rtc_ddl_v1.rs"]
pub mod rtc_ddl_v1;
#[path = "../registers/spi_ddl_v1.rs"]
pub mod spi_ddl_v1;
#[path = "../registers/sysctrl_ddl_v2_2.rs"]
pub mod sysctrl_ddl_v2_2;
#[path = "../registers/uart_ddl_v1.rs"]
pub mod uart_ddl_v1;
#[path = "../registers/vc_ddl_v2_1.rs"]
pub mod vc_ddl_v2_1;
#[path = "../registers/wdt_ddl_v1.rs"]
pub mod wdt_ddl_v1;
