
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v2",
                block: "ADC",
                ir: &adc_v2::REGISTERS,
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "JQR",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    36,
                ),
            },
            PeripheralDmaChannel {
                signal: "SQR",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    37,
                ),
            },
        ],
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
                version: "v1",
                block: "ADTIM",
                ir: &adtim_v1::REGISTERS,
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
                version: "v1",
                block: "ADTIM",
                ir: &adtim_v1::REGISTERS,
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
                version: "v1",
                block: "ADTIM",
                ir: &adtim_v1::REGISTERS,
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
        name: "AES",
        address: 0x40021400,
        registers: Some(
            PeripheralRegisters {
                kind: "aes",
                version: "v1",
                block: "AES",
                ir: &aes_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralAes",
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
                version: "v1",
                block: "CLK",
                ir: &clk_v1::REGISTERS,
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
                version: "v2",
                block: "CRC",
                ir: &crc_v2::REGISTERS,
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
        name: "DMAC",
        address: 0x40021000,
        registers: Some(
            PeripheralRegisters {
                kind: "dmac",
                version: "v2",
                block: "DMAC",
                ir: &dmac_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralDma",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "DMAC",
                interrupt: "DMAC",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40020000,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "v2",
                block: "FLASH",
                ir: &flash_v2::REGISTERS,
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
                version: "v1",
                block: "GPIO",
                ir: &gpio_v1::REGISTERS,
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
        name: "HDIV",
        address: 0x40021800,
        registers: Some(
            PeripheralRegisters {
                kind: "hdiv",
                version: "common",
                block: "HDIV",
                ir: &hdiv_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralDiv",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C0",
        address: 0x40000400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v3",
                block: "I",
                ir: &i_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralI2c0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "I2C0",
                interrupt: "I2C0",
            },
        ],
    },
    Peripheral {
        name: "I2C1",
        address: 0x40004400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v3",
                block: "I",
                ir: &i_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralI2c1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "I2C1",
                interrupt: "I2C1",
            },
        ],
    },
    Peripheral {
        name: "LVD",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "lvd",
                version: "v2",
                block: "LVD",
                ir: &lvd_v2::REGISTERS,
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
        name: "OPA",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "opa",
                version: "v2",
                block: "OPA",
                ir: &opa_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralOpa",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PCA",
        address: 0x40001000,
        registers: Some(
            PeripheralRegisters {
                kind: "pca",
                version: "v1",
                block: "PCA",
                ir: &pca_v1::REGISTERS,
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
                version: "v2",
                block: "RAM",
                ir: &ram_v2::REGISTERS,
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
                version: "v1",
                block: "RESET",
                ir: &reset_v1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI0",
        address: 0x40000800,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v2",
                block: "SPI",
                ir: &spi_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralSpi0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    32,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    33,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI0",
                interrupt: "SPI0",
            },
        ],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v2",
                block: "SPI",
                ir: &spi_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralSpi1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    34,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    35,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI1",
                interrupt: "SPI1",
            },
        ],
    },
    Peripheral {
        name: "SYSCTRL",
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "sysctrl",
                version: "v5",
                block: "SYSCTRL",
                ir: &sysctrl_v5::REGISTERS,
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
        registers: None,
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralBTim",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    50,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    51,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TIM0",
                interrupt: "TIM0",
            },
        ],
    },
    Peripheral {
        name: "TIM1",
        address: 0x40000d00,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralBTim",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    52,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    53,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TIM1",
                interrupt: "TIM1",
            },
        ],
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000e00,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralBTim",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    54,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    55,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TIM2",
                interrupt: "TIM2",
            },
        ],
    },
    Peripheral {
        name: "TIM3",
        address: 0x40005800,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralTim3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    56,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    57,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TIM3",
                interrupt: "TIM3",
            },
        ],
    },
    Peripheral {
        name: "TRNG",
        address: 0x40004c00,
        registers: Some(
            PeripheralRegisters {
                kind: "trng",
                version: "v1",
                block: "TRNG",
                ir: &trng_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN",
                        field: "SysctrlPeripheralRng",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "UART0",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "uart",
                version: "v1",
                block: "UART",
                ir: &uart_v1::REGISTERS,
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    40,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    41,
                ),
            },
        ],
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
                version: "v1",
                block: "UART",
                ir: &uart_v1::REGISTERS,
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    42,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    43,
                ),
            },
        ],
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
                version: "v2",
                block: "VC",
                ir: &vc_v2::REGISTERS,
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
        address: 0x40000f00,
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
        name: "TIM3",
        number: 5,
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
            #[path="../registers/adc_v2.rs"] pub mod adc_v2;
#[path="../registers/adtim_v1.rs"] pub mod adtim_v1;
#[path="../registers/aes_v1.rs"] pub mod aes_v1;
#[path="../registers/bgr_v1.rs"] pub mod bgr_v1;
#[path="../registers/clk_v1.rs"] pub mod clk_v1;
#[path="../registers/crc_v2.rs"] pub mod crc_v2;
#[path="../registers/debug_v2.rs"] pub mod debug_v2;
#[path="../registers/dmac_v2.rs"] pub mod dmac_v2;
#[path="../registers/flash_v2.rs"] pub mod flash_v2;
#[path="../registers/gpio_v1.rs"] pub mod gpio_v1;
#[path="../registers/hdiv_common.rs"] pub mod hdiv_common;
#[path="../registers/i_v3.rs"] pub mod i_v3;
#[path="../registers/lvd_v2.rs"] pub mod lvd_v2;
#[path="../registers/opa_v2.rs"] pub mod opa_v2;
#[path="../registers/pca_v1.rs"] pub mod pca_v1;
#[path="../registers/ram_v2.rs"] pub mod ram_v2;
#[path="../registers/reset_v1.rs"] pub mod reset_v1;
#[path="../registers/spi_v2.rs"] pub mod spi_v2;
#[path="../registers/sysctrl_v5.rs"] pub mod sysctrl_v5;
#[path="../registers/trng_v1.rs"] pub mod trng_v1;
#[path="../registers/uart_v1.rs"] pub mod uart_v1;
#[path="../registers/vc_v2.rs"] pub mod vc_v2;
#[path="../registers/wdt_v1.rs"] pub mod wdt_v1;
