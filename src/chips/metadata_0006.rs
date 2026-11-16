
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v3",
                block: "ADC",
                ir: &adc_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralADC",
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
        name: "ATIM3",
        address: 0x40005800,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralATIM3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ATIM3",
                interrupt: "ATIM3",
            },
        ],
    },
    Peripheral {
        name: "BTIM3",
        address: 0x40007400,
        registers: Some(
            PeripheralRegisters {
                kind: "btim",
                version: "v2",
                block: "BTIM",
                ir: &btim_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralGTIM",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BTIM3_4_5",
                interrupt: "BTIM3_4_5",
            },
        ],
    },
    Peripheral {
        name: "BTIM4",
        address: 0x40007500,
        registers: Some(
            PeripheralRegisters {
                kind: "btim",
                version: "v2",
                block: "BTIM",
                ir: &btim_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralGTIM",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BTIM3_4_5",
                interrupt: "BTIM3_4_5",
            },
        ],
    },
    Peripheral {
        name: "BTIM5",
        address: 0x40007600,
        registers: Some(
            PeripheralRegisters {
                kind: "btim",
                version: "v2",
                block: "BTIM",
                ir: &btim_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralGTIM",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BTIM3_4_5",
                interrupt: "BTIM3_4_5",
            },
        ],
    },
    Peripheral {
        name: "CTRIM",
        address: 0x40005000,
        registers: Some(
            PeripheralRegisters {
                kind: "ctrim",
                version: "v2",
                block: "CTRIM",
                ir: &ctrim_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralCTRIM",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CTRIM",
                interrupt: "CTRIM",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40020000,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "v3",
                block: "FLASH",
                ir: &flash_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralFLASH",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FLASH",
                interrupt: "FLASH",
            },
        ],
    },
    Peripheral {
        name: "GPIOA",
        address: 0x40020c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gpioa",
                version: "v2",
                block: "GPIOA",
                ir: &gpioa_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralPA",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PORTA",
                interrupt: "PORTA",
            },
        ],
    },
    Peripheral {
        name: "GPIOAUX",
        address: 0x40020ca0,
        registers: Some(
            PeripheralRegisters {
                kind: "gpioaux",
                version: "v2",
                block: "GPIOAUX",
                ir: &gpioaux_v2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 0x40020d00,
        registers: Some(
            PeripheralRegisters {
                kind: "gpioa",
                version: "v2",
                block: "GPIOA",
                ir: &gpioa_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralPB",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PORTB",
                interrupt: "PORTB",
            },
        ],
    },
    Peripheral {
        name: "GPIOC",
        address: 0x40020e00,
        registers: Some(
            PeripheralRegisters {
                kind: "gpioa",
                version: "v2",
                block: "GPIOA",
                ir: &gpioa_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralPC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PORTC",
                interrupt: "PORTC",
            },
        ],
    },
    Peripheral {
        name: "GPIOD",
        address: 0x40020f00,
        registers: Some(
            PeripheralRegisters {
                kind: "gpioa",
                version: "v2",
                block: "GPIOA",
                ir: &gpioa_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralPD",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PORTD",
                interrupt: "PORTD",
            },
        ],
    },
    Peripheral {
        name: "GTIM",
        address: 0x40007400,
        registers: Some(
            PeripheralRegisters {
                kind: "gtim",
                version: "v2",
                block: "GTIM",
                ir: &gtim_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralGTIM",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GTIM",
                interrupt: "GTIM",
            },
        ],
    },
    Peripheral {
        name: "I2C",
        address: 0x40000400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v4",
                block: "I",
                ir: &i_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralI2C",
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
        name: "IWDT",
        address: 0x40000f80,
        registers: Some(
            PeripheralRegisters {
                kind: "iwdt",
                version: "v2",
                block: "IWDT",
                ir: &iwdt_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralIWDT",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "IWDT_WWDT",
                interrupt: "IWDT_WWDT",
            },
        ],
    },
    Peripheral {
        name: "LPUART0",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "lpuart",
                version: "v3",
                block: "LPUART",
                ir: &lpuart_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralLPUART0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "LPUART0",
                interrupt: "LPUART0",
            },
        ],
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40004000,
        registers: Some(
            PeripheralRegisters {
                kind: "lpuart",
                version: "v3",
                block: "LPUART",
                ir: &lpuart_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralLPUART1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "LPUART1",
                interrupt: "LPUART1",
            },
        ],
    },
    Peripheral {
        name: "LVD",
        address: 0x40002680,
        registers: Some(
            PeripheralRegisters {
                kind: "lvd",
                version: "v3",
                block: "LVD",
                ir: &lvd_v3::REGISTERS,
            },
        ),
        rcc: None,
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
        name: "SPI",
        address: 0x40000800,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v3",
                block: "SPI",
                ir: &spi_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralSPI",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI0",
                interrupt: "SPI0",
            },
        ],
    },
    Peripheral {
        name: "SYSCTRL",
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "sysctrl",
                version: "v3",
                block: "SYSCTRL",
                ir: &sysctrl_v3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "WWDT",
        address: 0x40006800,
        registers: Some(
            PeripheralRegisters {
                kind: "wwdt",
                version: "common",
                block: "WWDT",
                ir: &wwdt_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralWWDT",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "IWDT_WWDT",
                interrupt: "IWDT_WWDT",
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
            #[path="../registers/adc_v3.rs"] pub mod adc_v3;
#[path="../registers/btim_v2.rs"] pub mod btim_v2;
#[path="../registers/ctrim_v2.rs"] pub mod ctrim_v2;
#[path="../registers/flash_v3.rs"] pub mod flash_v3;
#[path="../registers/gpioa_v2.rs"] pub mod gpioa_v2;
#[path="../registers/gpioaux_v2.rs"] pub mod gpioaux_v2;
#[path="../registers/gtim_v2.rs"] pub mod gtim_v2;
#[path="../registers/i_v4.rs"] pub mod i_v4;
#[path="../registers/iwdt_v2.rs"] pub mod iwdt_v2;
#[path="../registers/lpuart_v3.rs"] pub mod lpuart_v3;
#[path="../registers/lvd_v3.rs"] pub mod lvd_v3;
#[path="../registers/spi_v3.rs"] pub mod spi_v3;
#[path="../registers/sysctrl_v3.rs"] pub mod sysctrl_v3;
#[path="../registers/wwdt_common.rs"] pub mod wwdt_common;
