
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v21",
                block: "ADC",
                ir: &adc_v21::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_ADC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "JRQ",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    64,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOC",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    65,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "ATIM3",
        address: 0x4000e400,
        registers: Some(
            PeripheralRegisters {
                kind: "atim",
                version: "common",
                block: "ATIM",
                ir: &atim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "SYSCTRL_APBEN1_ATIM3",
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
        address: 0x40027800,
        registers: Some(
            PeripheralRegisters {
                kind: "crc",
                version: "v10",
                block: "CRC",
                ir: &crc_v10::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CSU0",
        address: 0x40025100,
        registers: Some(
            PeripheralRegisters {
                kind: "csu",
                version: "common",
                block: "CSU",
                ir: &csu_common::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CSU1",
        address: 0x40025100,
        registers: Some(
            PeripheralRegisters {
                kind: "csu",
                version: "common",
                block: "CSU",
                ir: &csu_common::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CSU2",
        address: 0x40025100,
        registers: Some(
            PeripheralRegisters {
                kind: "csu",
                version: "common",
                block: "CSU",
                ir: &csu_common::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CTRIM",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "ctrim",
                version: "common",
                block: "CTRIM",
                ir: &ctrim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_CTRIM",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGC",
        address: 0x40001c00,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgc",
                version: "v9",
                block: "DBGC",
                ir: &dbgc_v9::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_DBGC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DFM",
        address: 0x40026800,
        registers: Some(
            PeripheralRegisters {
                kind: "dfm",
                version: "v1",
                block: "DFM",
                ir: &dfm_v1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMAC",
        address: 0x40024800,
        registers: Some(
            PeripheralRegisters {
                kind: "dmac",
                version: "common",
                block: "DMAC",
                ir: &dmac_common::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EFM",
        address: 0x40026400,
        registers: Some(
            PeripheralRegisters {
                kind: "efm",
                version: "v8",
                block: "EFM",
                ir: &efm_v8::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EXTI",
        address: 0x40024000,
        registers: Some(
            PeripheralRegisters {
                kind: "exti",
                version: "common",
                block: "EXTI",
                ir: &exti_common::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40020000,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v21",
                block: "GPIO",
                ir: &gpio_v21::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM0",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "gtim",
                version: "common",
                block: "GTIM",
                ir: &gtim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_GTIM0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM1",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "gtim",
                version: "common",
                block: "GTIM",
                ir: &gtim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_GTIM1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM2",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "gtim",
                version: "common",
                block: "GTIM",
                ir: &gtim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_GTIM2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM3",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "gtim",
                version: "common",
                block: "GTIM",
                ir: &gtim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_GTIM3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM4",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "gtim",
                version: "common",
                block: "GTIM",
                ir: &gtim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_GTIM4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM5",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "gtim",
                version: "common",
                block: "GTIM",
                ir: &gtim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_GTIM5",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM6",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "gtim",
                version: "common",
                block: "GTIM",
                ir: &gtim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "SYSCTRL_APBEN1_GTIM6",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM7",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "gtim",
                version: "common",
                block: "GTIM",
                ir: &gtim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "SYSCTRL_APBEN1_GTIM7",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GTIM8",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "gtim",
                version: "common",
                block: "GTIM",
                ir: &gtim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "SYSCTRL_APBEN1_GTIM8",
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
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v13",
                block: "I",
                ir: &i_v13::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_I2C0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C1",
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v13",
                block: "I",
                ir: &i_v13::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "SYSCTRL_APBEN1_I2C1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "IWDT",
        address: 0x40000800,
        registers: Some(
            PeripheralRegisters {
                kind: "iwdt",
                version: "common",
                block: "IWDT",
                ir: &iwdt_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_IWDT",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPTIM0",
        address: 0x40005000,
        registers: Some(
            PeripheralRegisters {
                kind: "lptim",
                version: "v1",
                block: "LPTIM",
                ir: &lptim_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_LPTIM0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPTIM1",
        address: 0x40005000,
        registers: Some(
            PeripheralRegisters {
                kind: "lptim",
                version: "v1",
                block: "LPTIM",
                ir: &lptim_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "SYSCTRL_APBEN1_LPTIM1",
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
        address: 0x40003800,
        registers: Some(
            PeripheralRegisters {
                kind: "lpuart",
                version: "common",
                block: "LPUART",
                ir: &lpuart_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_LPUART0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    82,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    83,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40003800,
        registers: Some(
            PeripheralRegisters {
                kind: "lpuart",
                version: "common",
                block: "LPUART",
                ir: &lpuart_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "SYSCTRL_APBEN1_LPUART1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    84,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    85,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "LVD",
        address: 0x40007400,
        registers: Some(
            PeripheralRegisters {
                kind: "lvd",
                version: "common",
                block: "LVD",
                ir: &lvd_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_LVD",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0x40001000,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "v11",
                block: "RTC",
                ir: &rtc_v11::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_RTC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI0",
        address: 0x40004000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v11",
                block: "SPI",
                ir: &spi_v11::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_SPI0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    66,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    67,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40004000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v11",
                block: "SPI",
                ir: &spi_v11::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "SYSCTRL_APBEN1_SPI1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    68,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    69,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SRAM",
        address: 0x40024c00,
        registers: Some(
            PeripheralRegisters {
                kind: "sram",
                version: "common",
                block: "SRAM",
                ir: &sram_common::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SYSCTRL",
        address: 0x40025000,
        registers: Some(
            PeripheralRegisters {
                kind: "sysctrl",
                version: "common",
                block: "SYSCTRL",
                ir: &sysctrl_common::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART0",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v26",
                block: "USART",
                ir: &usart_v26::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_USART0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    70,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    71,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART1",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v26",
                block: "USART",
                ir: &usart_v26::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_USART1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    72,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    73,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART2",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v26",
                block: "USART",
                ir: &usart_v26::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_USART2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    74,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    75,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART3",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v26",
                block: "USART",
                ir: &usart_v26::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "SYSCTRL_APBEN1_USART3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    76,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    77,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART4",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v26",
                block: "USART",
                ir: &usart_v26::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "SYSCTRL_APBEN1_USART4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    78,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    79,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "USART5",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v26",
                block: "USART",
                ir: &usart_v26::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG1",
                        field: "SYSCTRL_APBEN1_USART5",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    80,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMAC",
                ),
                channel: None,
                request: Some(
                    81,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "WWDT",
        address: 0x40000c00,
        registers: Some(
            PeripheralRegisters {
                kind: "wwdt",
                version: "v1",
                block: "WWDT",
                ir: &wwdt_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "SYSCTRL_APBEN0_WWDT",
                    },
                ),
            },
        ),
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
            #[path="../registers/adc_v21.rs"] pub mod adc_v21;
#[path="../registers/atim_common.rs"] pub mod atim_common;
#[path="../registers/crc_v10.rs"] pub mod crc_v10;
#[path="../registers/csu_common.rs"] pub mod csu_common;
#[path="../registers/ctrim_common.rs"] pub mod ctrim_common;
#[path="../registers/dbgc_v9.rs"] pub mod dbgc_v9;
#[path="../registers/dfm_v1.rs"] pub mod dfm_v1;
#[path="../registers/dmac_common.rs"] pub mod dmac_common;
#[path="../registers/efm_v8.rs"] pub mod efm_v8;
#[path="../registers/exti_common.rs"] pub mod exti_common;
#[path="../registers/gpio_v21.rs"] pub mod gpio_v21;
#[path="../registers/gtim_common.rs"] pub mod gtim_common;
#[path="../registers/i_v13.rs"] pub mod i_v13;
#[path="../registers/iwdt_common.rs"] pub mod iwdt_common;
#[path="../registers/lptim_v1.rs"] pub mod lptim_v1;
#[path="../registers/lpuart_common.rs"] pub mod lpuart_common;
#[path="../registers/lvd_common.rs"] pub mod lvd_common;
#[path="../registers/rtc_v11.rs"] pub mod rtc_v11;
#[path="../registers/spi_v11.rs"] pub mod spi_v11;
#[path="../registers/sram_common.rs"] pub mod sram_common;
#[path="../registers/sysctrl_common.rs"] pub mod sysctrl_common;
#[path="../registers/usart_v26.rs"] pub mod usart_v26;
#[path="../registers/wwdt_v1.rs"] pub mod wwdt_v1;
