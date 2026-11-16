
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v11",
                block: "ADC",
                ir: &adc_v11::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
                    68,
                ),
            },
            PeripheralDmaChannel {
                signal: "SQR",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    69,
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
        name: "AES",
        address: 0x40021400,
        registers: Some(
            PeripheralRegisters {
                kind: "aes",
                version: "v3",
                block: "AES",
                ir: &aes_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
        name: "CRC",
        address: 0x40020800,
        registers: Some(
            PeripheralRegisters {
                kind: "crc",
                version: "v5",
                block: "CRC",
                ir: &crc_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
        name: "CTRIM",
        address: 0x40009000,
        registers: Some(
            PeripheralRegisters {
                kind: "ctrim",
                version: "v4",
                block: "CTRIM",
                ir: &ctrim_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN1",
                        field: "SysctrlPeripheralCtrim",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CTRIM_CLKDET",
                interrupt: "CTRIM_CLKDET",
            },
        ],
    },
    Peripheral {
        name: "DEBUG_ACTIVE",
        address: 0x40021838,
        registers: Some(
            PeripheralRegisters {
                kind: "debug",
                version: "v4",
                block: "DEBUG",
                ir: &debug_v4::REGISTERS,
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
                version: "v5",
                block: "DMAC",
                ir: &dmac_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
                version: "v11",
                block: "FLASH",
                ir: &flash_v11::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
                version: "v5",
                block: "GPIO",
                ir: &gpio_v5::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
        name: "HSI2C",
        address: 0x4000bc00,
        registers: Some(
            PeripheralRegisters {
                kind: "hsi",
                version: "v2",
                block: "HSI",
                ir: &hsi_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralHsi2c",
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
                    90,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    91,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HSI2C",
                interrupt: "HSI2C",
            },
        ],
    },
    Peripheral {
        name: "I2C0",
        address: 0x40000400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v9",
                block: "I",
                ir: &i_v9::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
        address: 0x40008400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v9",
                block: "I",
                ir: &i_v9::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
        name: "IWDT",
        address: 0x40001080,
        registers: Some(
            PeripheralRegisters {
                kind: "iwdt",
                version: "v4",
                block: "IWDT",
                ir: &iwdt_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralIwdt",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "IWDT",
                interrupt: "IWDT",
            },
        ],
    },
    Peripheral {
        name: "LCD",
        address: 0x40009c00,
        registers: Some(
            PeripheralRegisters {
                kind: "lcd",
                version: "v4",
                block: "LCD",
                ir: &lcd_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralLcd",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    70,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "LCD",
                interrupt: "LCD",
            },
        ],
    },
    Peripheral {
        name: "LPTIMER0",
        address: 0x40000c00,
        registers: Some(
            PeripheralRegisters {
                kind: "lptimer",
                version: "v4",
                block: "LPTIMER",
                ir: &lptimer_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralLpTim0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPTIMER1",
        address: 0x40003c00,
        registers: Some(
            PeripheralRegisters {
                kind: "lptimer",
                version: "v4",
                block: "LPTIMER",
                ir: &lptimer_v4::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN1",
                        field: "SysctrlPeripheralLpTim1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPUART0",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "lpuart",
                version: "v7",
                block: "LPUART",
                ir: &lpuart_v7::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralLpUart0",
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
                    76,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    77,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "LPUART0",
                interrupt: "LPUART0",
            },
        ],
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40008000,
        registers: Some(
            PeripheralRegisters {
                kind: "lpuart",
                version: "v7",
                block: "LPUART",
                ir: &lpuart_v7::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralLpUart1",
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
                    78,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    79,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "LPUART1",
                interrupt: "LPUART1",
            },
        ],
    },
    Peripheral {
        name: "LPUART2",
        address: 0x4000b400,
        registers: Some(
            PeripheralRegisters {
                kind: "lpuart",
                version: "v7",
                block: "LPUART",
                ir: &lpuart_v7::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN1",
                        field: "SysctrlPeripheralLpuart2",
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
                    80,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    81,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "LPUART2",
                interrupt: "LPUART2",
            },
        ],
    },
    Peripheral {
        name: "LVD",
        address: 0x40002c00,
        registers: Some(
            PeripheralRegisters {
                kind: "lvd",
                version: "v7",
                block: "LVD",
                ir: &lvd_v7::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
        name: "RAM",
        address: 0x40020400,
        registers: Some(
            PeripheralRegisters {
                kind: "ram",
                version: "v4",
                block: "RAM",
                ir: &ram_v4::REGISTERS,
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
        address: 0x4002181c,
        registers: Some(
            PeripheralRegisters {
                kind: "reset",
                version: "v7",
                block: "RESET",
                ir: &reset_v7::REGISTERS,
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
                version: "v3",
                block: "RTC",
                ir: &rtc_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralRtc",
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
        name: "SPI0",
        address: 0x40000800,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v1",
                block: "SPI",
                ir: &spi_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
                    64,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    65,
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
        address: 0x40008800,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v1",
                block: "SPI",
                ir: &spi_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
                    66,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    67,
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
        address: 0x40021800,
        registers: Some(
            PeripheralRegisters {
                kind: "sysctrl",
                version: "v11",
                block: "SYSCTRL",
                ir: &sysctrl_v11::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM0",
        address: 0x40003000,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralBaseTim",
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
                    82,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    83,
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
        address: 0x40003400,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralBaseTim",
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
                    84,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    85,
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
        address: 0x40003800,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralBaseTim",
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
                    86,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    87,
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
        address: 0x40009800,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralBaseTim;SysctrlPeripheralTim3",
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
                    88,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    89,
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
        address: 0x40008c00,
        registers: Some(
            PeripheralRegisters {
                kind: "trng",
                version: "v2",
                block: "TRNG",
                ir: &trng_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralTrng",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART0",
        address: 0x4000a000,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "common",
                block: "USART",
                ir: &usart_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralUsart0",
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
                    72,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    73,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART0",
                interrupt: "USART0",
            },
        ],
    },
    Peripheral {
        name: "USART1",
        address: 0x4000a400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "common",
                block: "USART",
                ir: &usart_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralUsart1",
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
                    74,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    75,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART1",
                interrupt: "USART1",
            },
        ],
    },
    Peripheral {
        name: "VC0",
        address: 0x40002800,
        registers: Some(
            PeripheralRegisters {
                kind: "vc",
                version: "v8",
                block: "VC",
                ir: &vc_v8::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
        ],
    },
    Peripheral {
        name: "VC1",
        address: 0x40002810,
        registers: Some(
            PeripheralRegisters {
                kind: "vc",
                version: "v8",
                block: "VC",
                ir: &vc_v8::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralVcLvd",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "VC1",
                interrupt: "VC1",
            },
        ],
    },
    Peripheral {
        name: "VCTRIM",
        address: 0x40002820,
        registers: Some(
            PeripheralRegisters {
                kind: "vctrim",
                version: "common",
                block: "VCTRIM",
                ir: &vctrim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralVcLvd",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "WWDT",
        address: 0x4000b800,
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
                        register: "PERI_CLKEN1",
                        field: "SysctrlPeripheralWwdt",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "WWDT",
                interrupt: "WWDT",
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
        name: "USART0",
        number: 6,
    },
    Interrupt {
        name: "USART1",
        number: 7,
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
        name: "LPTIM_0_1",
        number: 17,
    },
    Interrupt {
        name: "HSI2C",
        number: 18,
    },
    Interrupt {
        name: "LPUART2",
        number: 19,
    },
    Interrupt {
        name: "WWDT",
        number: 21,
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
        name: "LCD",
        number: 29,
    },
    Interrupt {
        name: "FLASH_RAM",
        number: 30,
    },
    Interrupt {
        name: "CTRIM_CLKDET",
        number: 31,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v11.rs"] pub mod adc_v11;
#[path="../registers/aes_v3.rs"] pub mod aes_v3;
#[path="../registers/crc_v5.rs"] pub mod crc_v5;
#[path="../registers/ctrim_v4.rs"] pub mod ctrim_v4;
#[path="../registers/debug_v4.rs"] pub mod debug_v4;
#[path="../registers/dmac_v5.rs"] pub mod dmac_v5;
#[path="../registers/flash_v11.rs"] pub mod flash_v11;
#[path="../registers/gpio_v5.rs"] pub mod gpio_v5;
#[path="../registers/hsi_v2.rs"] pub mod hsi_v2;
#[path="../registers/i_v9.rs"] pub mod i_v9;
#[path="../registers/iwdt_v4.rs"] pub mod iwdt_v4;
#[path="../registers/lcd_v4.rs"] pub mod lcd_v4;
#[path="../registers/lptimer_v4.rs"] pub mod lptimer_v4;
#[path="../registers/lpuart_v7.rs"] pub mod lpuart_v7;
#[path="../registers/lvd_v7.rs"] pub mod lvd_v7;
#[path="../registers/ram_v4.rs"] pub mod ram_v4;
#[path="../registers/reset_v7.rs"] pub mod reset_v7;
#[path="../registers/rtc_v3.rs"] pub mod rtc_v3;
#[path="../registers/spi_v1.rs"] pub mod spi_v1;
#[path="../registers/sysctrl_v11.rs"] pub mod sysctrl_v11;
#[path="../registers/trng_v2.rs"] pub mod trng_v2;
#[path="../registers/usart_common.rs"] pub mod usart_common;
#[path="../registers/vc_v8.rs"] pub mod vc_v8;
#[path="../registers/vctrim_common.rs"] pub mod vctrim_common;
#[path="../registers/wwdt_common.rs"] pub mod wwdt_common;
