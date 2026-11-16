
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40002500,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v1",
                block: "ADC",
                ir: &adc_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
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
                    76,
                ),
            },
            PeripheralDmaChannel {
                signal: "EOC",
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
                signal: "ADC",
                interrupt: "ADC",
            },
        ],
    },
    Peripheral {
        name: "ATIM0",
        address: 0x40003000,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralAtim0",
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
                signal: "ATIM0",
                interrupt: "ATIM0",
            },
            PeripheralInterrupt {
                signal: "ATIM_BK",
                interrupt: "ATIM_BK",
            },
        ],
    },
    Peripheral {
        name: "ATIM1",
        address: 0x40003400,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralAtim1",
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
                    90,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
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
                signal: "ATIM1",
                interrupt: "ATIM1",
            },
            PeripheralInterrupt {
                signal: "ATIM_BK",
                interrupt: "ATIM_BK",
            },
        ],
    },
    Peripheral {
        name: "ATIM2",
        address: 0x40003800,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralAtim2",
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
                    92,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    93,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ATIM2",
                interrupt: "ATIM2",
            },
            PeripheralInterrupt {
                signal: "ATIM_BK",
                interrupt: "ATIM_BK",
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
                    94,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    95,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ATIM3",
                interrupt: "ATIM3",
            },
            PeripheralInterrupt {
                signal: "ATIM_BK",
                interrupt: "ATIM_BK",
            },
        ],
    },
    Peripheral {
        name: "BTIM0",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "btim",
                version: "v1",
                block: "BTIM",
                ir: &btim_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralGtim0Btim012",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GTIM0_BTIM0_1_2",
                interrupt: "GTIM0_BTIM0_1_2",
            },
        ],
    },
    Peripheral {
        name: "BTIM1",
        address: 0x40001900,
        registers: Some(
            PeripheralRegisters {
                kind: "btim",
                version: "v1",
                block: "BTIM",
                ir: &btim_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralGtim0Btim012",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GTIM0_BTIM0_1_2",
                interrupt: "GTIM0_BTIM0_1_2",
            },
        ],
    },
    Peripheral {
        name: "BTIM2",
        address: 0x40001a00,
        registers: Some(
            PeripheralRegisters {
                kind: "btim",
                version: "v1",
                block: "BTIM",
                ir: &btim_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralGtim0Btim012",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GTIM0_BTIM0_1_2",
                interrupt: "GTIM0_BTIM0_1_2",
            },
        ],
    },
    Peripheral {
        name: "BTIM3",
        address: 0x40001c00,
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
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralGtim1Btim345",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GTIM1_BTIM3_4_5",
                interrupt: "GTIM1_BTIM3_4_5",
            },
        ],
    },
    Peripheral {
        name: "BTIM4",
        address: 0x40001d00,
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
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralGtim1Btim345",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GTIM1_BTIM3_4_5",
                interrupt: "GTIM1_BTIM3_4_5",
            },
        ],
    },
    Peripheral {
        name: "BTIM5",
        address: 0x40001e00,
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
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralGtim1Btim345",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GTIM1_BTIM3_4_5",
                interrupt: "GTIM1_BTIM3_4_5",
            },
        ],
    },
    Peripheral {
        name: "CAN",
        address: 0x40030000,
        registers: Some(
            PeripheralRegisters {
                kind: "can",
                version: "v1",
                block: "CAN",
                ir: &can_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralCAN",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CAN",
                interrupt: "CAN",
            },
        ],
    },
    Peripheral {
        name: "CRC",
        address: 0x40020800,
        registers: Some(
            PeripheralRegisters {
                kind: "crc",
                version: "v1",
                block: "CRC",
                ir: &crc_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
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
        address: 0x40005000,
        registers: Some(
            PeripheralRegisters {
                kind: "ctrim",
                version: "v1",
                block: "CTRIM",
                ir: &ctrim_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralCtrim",
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
        name: "DMAC",
        address: 0x40021000,
        registers: Some(
            PeripheralRegisters {
                kind: "dmac",
                version: "v1",
                block: "DMAC",
                ir: &dmac_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
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
        name: "EAU",
        address: 0x40021800,
        registers: Some(
            PeripheralRegisters {
                kind: "eau",
                version: "common",
                block: "EAU",
                ir: &eau_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralEau",
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
                version: "v1",
                block: "FLASH",
                ir: &flash_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
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
        name: "GPIOA",
        address: 0x40020c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gpioa",
                version: "v1",
                block: "GPIOA",
                ir: &gpioa_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralGpioA",
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
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "gpioaux",
                version: "v1",
                block: "GPIOAUX",
                ir: &gpioaux_v1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 0x40021c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gpioa",
                version: "v1",
                block: "GPIOA",
                ir: &gpioa_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralGpioB",
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
        address: 0x40022000,
        registers: Some(
            PeripheralRegisters {
                kind: "gpioa",
                version: "v1",
                block: "GPIOA",
                ir: &gpioa_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralGpioC",
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
        address: 0x40022400,
        registers: Some(
            PeripheralRegisters {
                kind: "gpioa",
                version: "v1",
                block: "GPIOA",
                ir: &gpioa_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralGpioD",
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
        name: "GTIM0",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "gtim",
                version: "v1",
                block: "GTIM",
                ir: &gtim_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralGtim0Btim012",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GTIM0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    72,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GTIM0_BTIM0_1_2",
                interrupt: "GTIM0_BTIM0_1_2",
            },
        ],
    },
    Peripheral {
        name: "GTIM1",
        address: 0x40001c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gtim",
                version: "v1",
                block: "GTIM",
                ir: &gtim_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN1",
                        field: "SysctrlPeripheralGtim1Btim345",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GTIM1",
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
                signal: "GTIM1_BTIM3_4_5",
                interrupt: "GTIM1_BTIM3_4_5",
            },
        ],
    },
    Peripheral {
        name: "I2C0",
        address: 0x40000400,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v1",
                block: "I",
                ir: &i_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
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
                version: "v1",
                block: "I",
                ir: &i_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
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
        name: "I2CSLV",
        address: 0x40004c00,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v2",
                block: "I",
                ir: &i_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralI2cslv",
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
                    84,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
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
                signal: "I2CSLV",
                interrupt: "I2CSLV",
            },
        ],
    },
    Peripheral {
        name: "IWDT",
        address: 0x40001080,
        registers: Some(
            PeripheralRegisters {
                kind: "iwdt",
                version: "v1",
                block: "IWDT",
                ir: &iwdt_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralIwt",
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
        name: "LPTIM",
        address: 0x40000c00,
        registers: Some(
            PeripheralRegisters {
                kind: "lptim",
                version: "common",
                block: "LPTIM",
                ir: &lptim_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralLpTim",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "LPTIM",
                interrupt: "LPTIM",
            },
        ],
    },
    Peripheral {
        name: "LPUART0",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "lpuart",
                version: "v1",
                block: "LPUART",
                ir: &lpuart_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralLpUart0",
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
                    68,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
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
                version: "v1",
                block: "LPUART",
                ir: &lpuart_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralLpUart1",
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
            PeripheralDmaChannel {
                signal: "RX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    71,
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
        name: "LVD",
        address: 0x40002480,
        registers: Some(
            PeripheralRegisters {
                kind: "lvd",
                version: "v1",
                block: "LVD",
                ir: &lvd_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralVcLvd",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "VC0_VC1_LVD",
                interrupt: "VC0_VC1_LVD",
            },
        ],
    },
    Peripheral {
        name: "OPA",
        address: 0x40002600,
        registers: Some(
            PeripheralRegisters {
                kind: "opa",
                version: "v1",
                block: "OPA",
                ir: &opa_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
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
        name: "RAM",
        address: 0x40020400,
        registers: Some(
            PeripheralRegisters {
                kind: "ram",
                version: "v1",
                block: "RAM",
                ir: &ram_v1::REGISTERS,
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
        name: "RTC",
        address: 0x40001400,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "v1",
                block: "RTC",
                ir: &rtc_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
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
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralSpi0",
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
                    80,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
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
                version: "v1",
                block: "SPI",
                ir: &spi_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralSpi1",
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
                    82,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
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
                version: "v1",
                block: "SYSCTRL",
                ir: &sysctrl_v1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART0",
        address: 0x40006000,
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
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralUsart0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART0",
                interrupt: "USART0",
            },
        ],
    },
    Peripheral {
        name: "USART1",
        address: 0x40006400,
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
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralUsart1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USART1",
                interrupt: "USART1",
            },
        ],
    },
    Peripheral {
        name: "VC0",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "vc",
                version: "v1",
                block: "VC",
                ir: &vc_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralVcLvd",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "VC0_VC1_LVD",
                interrupt: "VC0_VC1_LVD",
            },
        ],
    },
    Peripheral {
        name: "VC1",
        address: 0x40002410,
        registers: Some(
            PeripheralRegisters {
                kind: "vc",
                version: "v1",
                block: "VC",
                ir: &vc_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERICLKEN0",
                        field: "SysctrlPeripheralVcLvd",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "VC0_VC1_LVD",
                interrupt: "VC0_VC1_LVD",
            },
        ],
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
                        field: "SysctrlPeripheralWwt",
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
        name: "ATIM3",
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
        name: "ATIM0",
        number: 14,
    },
    Interrupt {
        name: "ATIM1",
        number: 15,
    },
    Interrupt {
        name: "ATIM2",
        number: 16,
    },
    Interrupt {
        name: "LPTIM",
        number: 17,
    },
    Interrupt {
        name: "GTIM0_BTIM0_1_2",
        number: 18,
    },
    Interrupt {
        name: "GTIM1_BTIM3_4_5",
        number: 19,
    },
    Interrupt {
        name: "I2CSLV",
        number: 20,
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
        name: "VC0_VC1_LVD",
        number: 26,
    },
    Interrupt {
        name: "CAN",
        number: 28,
    },
    Interrupt {
        name: "ATIM_BK",
        number: 29,
    },
    Interrupt {
        name: "FLASH_RAM",
        number: 30,
    },
    Interrupt {
        name: "CTRIM",
        number: 31,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v1.rs"] pub mod adc_v1;
#[path="../registers/btim_v1.rs"] pub mod btim_v1;
#[path="../registers/btim_v2.rs"] pub mod btim_v2;
#[path="../registers/can_v1.rs"] pub mod can_v1;
#[path="../registers/crc_v1.rs"] pub mod crc_v1;
#[path="../registers/ctrim_v1.rs"] pub mod ctrim_v1;
#[path="../registers/dmac_v1.rs"] pub mod dmac_v1;
#[path="../registers/eau_common.rs"] pub mod eau_common;
#[path="../registers/flash_v1.rs"] pub mod flash_v1;
#[path="../registers/gpioa_v1.rs"] pub mod gpioa_v1;
#[path="../registers/gpioaux_v1.rs"] pub mod gpioaux_v1;
#[path="../registers/gtim_v1.rs"] pub mod gtim_v1;
#[path="../registers/i_v1.rs"] pub mod i_v1;
#[path="../registers/i_v2.rs"] pub mod i_v2;
#[path="../registers/iwdt_v1.rs"] pub mod iwdt_v1;
#[path="../registers/lptim_common.rs"] pub mod lptim_common;
#[path="../registers/lpuart_v1.rs"] pub mod lpuart_v1;
#[path="../registers/lvd_v1.rs"] pub mod lvd_v1;
#[path="../registers/opa_v1.rs"] pub mod opa_v1;
#[path="../registers/ram_v1.rs"] pub mod ram_v1;
#[path="../registers/rtc_v1.rs"] pub mod rtc_v1;
#[path="../registers/spi_v1.rs"] pub mod spi_v1;
#[path="../registers/sysctrl_v1.rs"] pub mod sysctrl_v1;
#[path="../registers/usart_common.rs"] pub mod usart_common;
#[path="../registers/vc_v1.rs"] pub mod vc_v1;
#[path="../registers/wwdt_common.rs"] pub mod wwdt_common;
