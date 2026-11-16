
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v4",
                block: "ADC",
                ir: &adc_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralAdcBgr",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ADC",
                interrupt: "ADC",
            },
        ],
    },
    Peripheral {
        name: "ADT0",
        address: 0x40003000,
        registers: Some(
            PeripheralRegisters {
                kind: "adt",
                version: "common",
                block: "ADT",
                ir: &adt_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralAdt",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ADT1",
        address: 0x40003400,
        registers: Some(
            PeripheralRegisters {
                kind: "adt",
                version: "common",
                block: "ADT",
                ir: &adt_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralAdt",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ADT2",
        address: 0x40003800,
        registers: Some(
            PeripheralRegisters {
                kind: "adt",
                version: "common",
                block: "ADT",
                ir: &adt_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralAdt",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BGR",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "bgr",
                version: "v2",
                block: "BGR",
                ir: &bgr_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralAdcBgr",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BT0",
        address: 0x40000c00,
        registers: Some(
            PeripheralRegisters {
                kind: "bt",
                version: "common",
                block: "BT",
                ir: &bt_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralBt",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BT1",
        address: 0x40000c20,
        registers: Some(
            PeripheralRegisters {
                kind: "bt",
                version: "common",
                block: "BT",
                ir: &bt_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralBt",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BT2",
        address: 0x40000c40,
        registers: Some(
            PeripheralRegisters {
                kind: "bt",
                version: "common",
                block: "BT",
                ir: &bt_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralBt",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CLK_TRIM",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "clk",
                version: "v3",
                block: "CLK",
                ir: &clk_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralClkTrim",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CLK_TRIM",
                interrupt: "CLK_TRIM",
            },
        ],
    },
    Peripheral {
        name: "CLOCK",
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "clock",
                version: "common",
                block: "CLOCK",
                ir: &clock_common::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 0x40020900,
        registers: Some(
            PeripheralRegisters {
                kind: "crc",
                version: "v3",
                block: "CRC",
                ir: &crc_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralCrc",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40020000,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "v10",
                block: "FLASH",
                ir: &flash_v10::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralFlash",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FLASH_RAM",
                interrupt: "FLASH_RAM",
            },
        ],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40020c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v2",
                block: "GPIO",
                ir: &gpio_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralGpio",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C",
        address: 0x40000400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v8",
                block: "I",
                ir: &i_v8::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralI2c",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "I2C",
                interrupt: "I2C",
            },
        ],
    },
    Peripheral {
        name: "LPTIMER",
        address: 0x40000c00,
        registers: Some(
            PeripheralRegisters {
                kind: "lptimer",
                version: "v3",
                block: "LPTIMER",
                ir: &lptimer_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralLpTim",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPUART",
        address: 0x40000200,
        registers: Some(
            PeripheralRegisters {
                kind: "lpuart",
                version: "v6",
                block: "LPUART",
                ir: &lpuart_v6::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralLpUart",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "LPUART",
                interrupt: "LPUART",
            },
        ],
    },
    Peripheral {
        name: "LVD",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "lvd",
                version: "v6",
                block: "LVD",
                ir: &lvd_v6::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralVcLvd",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "LVD",
                interrupt: "LVD",
            },
        ],
    },
    Peripheral {
        name: "PCA",
        address: 0x40001000,
        registers: Some(
            PeripheralRegisters {
                kind: "pca",
                version: "v3",
                block: "PCA",
                ir: &pca_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralPca",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PCA",
                interrupt: "PCA",
            },
        ],
    },
    Peripheral {
        name: "RAM",
        address: 0x40020400,
        registers: Some(
            PeripheralRegisters {
                kind: "ram",
                version: "v3",
                block: "RAM",
                ir: &ram_v3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FLASH_RAM",
                interrupt: "FLASH_RAM",
            },
        ],
    },
    Peripheral {
        name: "RESET",
        address: 0x4000201c,
        registers: Some(
            PeripheralRegisters {
                kind: "reset",
                version: "v6",
                block: "RESET",
                ir: &reset_v6::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0x40001400,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "v5",
                block: "RTC",
                ir: &rtc_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralRtc",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RTC",
                interrupt: "RTC",
            },
        ],
    },
    Peripheral {
        name: "SPI",
        address: 0x40000800,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v6",
                block: "SPI",
                ir: &spi_v6::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralSpi",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI",
                interrupt: "SPI",
            },
        ],
    },
    Peripheral {
        name: "UART0",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "uart",
                version: "v2",
                block: "UART",
                ir: &uart_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralUart0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UART0",
                interrupt: "UART0",
            },
        ],
    },
    Peripheral {
        name: "UART1",
        address: 0x40000100,
        registers: Some(
            PeripheralRegisters {
                kind: "uart",
                version: "v2",
                block: "UART",
                ir: &uart_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralUart1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UART1",
                interrupt: "UART1",
            },
        ],
    },
    Peripheral {
        name: "VC",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "vc",
                version: "v7",
                block: "VC",
                ir: &vc_v7::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralVcLvd",
                    },
                ),
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "wdt",
                version: "v2",
                block: "WDT",
                ir: &wdt_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "ClkPeripheralWdt",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
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
            #[path="../registers/adc_v4.rs"] pub mod adc_v4;
#[path="../registers/adt_common.rs"] pub mod adt_common;
#[path="../registers/bgr_v2.rs"] pub mod bgr_v2;
#[path="../registers/bt_common.rs"] pub mod bt_common;
#[path="../registers/clk_v3.rs"] pub mod clk_v3;
#[path="../registers/clock_common.rs"] pub mod clock_common;
#[path="../registers/crc_v3.rs"] pub mod crc_v3;
#[path="../registers/flash_v10.rs"] pub mod flash_v10;
#[path="../registers/gpio_v2.rs"] pub mod gpio_v2;
#[path="../registers/i_v8.rs"] pub mod i_v8;
#[path="../registers/lptimer_v3.rs"] pub mod lptimer_v3;
#[path="../registers/lpuart_v6.rs"] pub mod lpuart_v6;
#[path="../registers/lvd_v6.rs"] pub mod lvd_v6;
#[path="../registers/pca_v3.rs"] pub mod pca_v3;
#[path="../registers/ram_v3.rs"] pub mod ram_v3;
#[path="../registers/reset_v6.rs"] pub mod reset_v6;
#[path="../registers/rtc_v5.rs"] pub mod rtc_v5;
#[path="../registers/spi_v6.rs"] pub mod spi_v6;
#[path="../registers/uart_v2.rs"] pub mod uart_v2;
#[path="../registers/vc_v7.rs"] pub mod vc_v7;
#[path="../registers/wdt_v2.rs"] pub mod wdt_v2;
