
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
                        field: "SysctrlPeripheralAdcBgr",
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
        name: "ADTIM4",
        address: 0x40003000,
        registers: Some(
            PeripheralRegisters {
                kind: "adtim",
                version: "v2",
                block: "ADTIM",
                ir: &adtim_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralAdvTim",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ADTIM4",
                interrupt: "ADTIM4",
            },
        ],
    },
    Peripheral {
        name: "ADTIM5",
        address: 0x40003400,
        registers: Some(
            PeripheralRegisters {
                kind: "adtim",
                version: "v2",
                block: "ADTIM",
                ir: &adtim_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralAdvTim",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ADTIM5",
                interrupt: "ADTIM5",
            },
        ],
    },
    Peripheral {
        name: "ADTIM6",
        address: 0x40003800,
        registers: Some(
            PeripheralRegisters {
                kind: "adtim",
                version: "v2",
                block: "ADTIM",
                ir: &adtim_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralAdvTim",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ADTIM6",
                interrupt: "ADTIM6",
            },
        ],
    },
    Peripheral {
        name: "BGR",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "bgr",
                version: "v1",
                block: "BGR",
                ir: &bgr_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralAdcBgr",
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
                version: "v2",
                block: "CLK",
                ir: &clk_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralTrim",
                    },
                ),
            },
        ),
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
                        field: "SysctrlPeripheralCrc",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DEBUG_ACTIVE",
        address: 0x40002038,
        registers: Some(
            PeripheralRegisters {
                kind: "debug",
                version: "v2",
                block: "DEBUG",
                ir: &debug_v2::REGISTERS,
            },
        ),
        rcc: None,
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
                version: "v4",
                block: "FLASH",
                ir: &flash_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralFlash",
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
                        field: "SysctrlPeripheralGpio",
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
                version: "v5",
                block: "I",
                ir: &i_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralI2c",
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
        name: "LVD",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "lvd",
                version: "v4",
                block: "LVD",
                ir: &lvd_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralVcLvd",
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
                version: "v2",
                block: "PCA",
                ir: &pca_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralPca",
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
                version: "v2",
                block: "RESET",
                ir: &reset_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI",
        address: 0x40000800,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v4",
                block: "SPI",
                ir: &spi_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralSpi",
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
        name: "SYSCTRL",
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "sysctrl",
                version: "v4",
                block: "SYSCTRL",
                ir: &sysctrl_v4::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM0",
        address: 0x40000c00,
        registers: Some(
            PeripheralRegisters {
                kind: "tim",
                version: "v13",
                block: "TIM",
                ir: &tim_v13::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralBt",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TIM0",
                interrupt: "TIM0",
            },
        ],
    },
    Peripheral {
        name: "TIM1",
        address: 0x40000c20,
        registers: Some(
            PeripheralRegisters {
                kind: "tim",
                version: "v13",
                block: "TIM",
                ir: &tim_v13::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralBt",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TIM1",
                interrupt: "TIM1",
            },
        ],
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000c40,
        registers: Some(
            PeripheralRegisters {
                kind: "tim",
                version: "v13",
                block: "TIM",
                ir: &tim_v13::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralBt",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TIM2",
                interrupt: "TIM2",
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
                        field: "SysctrlPeripheralUart0",
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
                        field: "SysctrlPeripheralUart1",
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
                version: "v3",
                block: "VC",
                ir: &vc_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralVcLvd",
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
                version: "v1",
                block: "WDT",
                ir: &wdt_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralWdt",
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
        name: "CLKTRIM",
        number: 31,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v4.rs"] pub mod adc_v4;
#[path="../registers/adtim_v2.rs"] pub mod adtim_v2;
#[path="../registers/bgr_v1.rs"] pub mod bgr_v1;
#[path="../registers/clk_v2.rs"] pub mod clk_v2;
#[path="../registers/crc_v3.rs"] pub mod crc_v3;
#[path="../registers/debug_v2.rs"] pub mod debug_v2;
#[path="../registers/flash_v4.rs"] pub mod flash_v4;
#[path="../registers/gpio_v2.rs"] pub mod gpio_v2;
#[path="../registers/i_v5.rs"] pub mod i_v5;
#[path="../registers/lvd_v4.rs"] pub mod lvd_v4;
#[path="../registers/pca_v2.rs"] pub mod pca_v2;
#[path="../registers/ram_v3.rs"] pub mod ram_v3;
#[path="../registers/reset_v2.rs"] pub mod reset_v2;
#[path="../registers/spi_v4.rs"] pub mod spi_v4;
#[path="../registers/sysctrl_v4.rs"] pub mod sysctrl_v4;
#[path="../registers/tim_v13.rs"] pub mod tim_v13;
#[path="../registers/uart_v2.rs"] pub mod uart_v2;
#[path="../registers/vc_v3.rs"] pub mod vc_v3;
#[path="../registers/wdt_v1.rs"] pub mod wdt_v1;
