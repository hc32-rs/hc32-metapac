
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v5",
                block: "ADC",
                ir: &adc_v5::REGISTERS,
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
                signal: "ADC_DAC",
                interrupt: "ADC_DAC",
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
                        register: "PERI_CLKEN0",
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
                        register: "PERI_CLKEN0",
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
                        register: "PERI_CLKEN0",
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
                version: "v2",
                block: "AES",
                ir: &aes_v2::REGISTERS,
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
                        register: "PERI_CLKEN0",
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
        name: "CAN",
        address: 0x40030000,
        registers: Some(
            PeripheralRegisters {
                kind: "can",
                version: "v2",
                block: "CAN",
                ir: &can_v2::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN1",
                        field: "SysctrlPeripheralCan",
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
                        register: "PERI_CLKEN0",
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
        name: "CTS",
        address: 0x40005000,
        registers: Some(
            PeripheralRegisters {
                kind: "cts",
                version: "common",
                block: "CTS",
                ir: &cts_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN1",
                        field: "SysctrlPeripheralCts",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CLKTRIM_CTS",
                interrupt: "CLKTRIM_CTS",
            },
        ],
    },
    Peripheral {
        name: "DAC",
        address: 0x40002500,
        registers: Some(
            PeripheralRegisters {
                kind: "dac",
                version: "v1",
                block: "DAC",
                ir: &dac_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN1",
                        field: "SysctrlPeripheralDac",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "0",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    80,
                ),
            },
            PeripheralDmaChannel {
                signal: "1",
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
                signal: "ADC_DAC",
                interrupt: "ADC_DAC",
            },
        ],
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
                version: "v3",
                block: "DMAC",
                ir: &dmac_v3::REGISTERS,
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
                version: "v5",
                block: "FLASH",
                ir: &flash_v5::REGISTERS,
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
                version: "v3",
                block: "GPIO",
                ir: &gpio_v3::REGISTERS,
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
                        register: "PERI_CLKEN0",
                        field: "SysctrlPeripheralHdiv",
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
        name: "I2S0",
        address: 0x40002800,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v6",
                block: "I",
                ir: &i_v6::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN1",
                        field: "SysctrlPeripheralI2s0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "LEFT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    100,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI0_I2S0",
                interrupt: "SPI0_I2S0",
            },
        ],
    },
    Peripheral {
        name: "I2S1",
        address: 0x40002c00,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v6",
                block: "I",
                ir: &i_v6::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN1",
                        field: "SysctrlPeripheralI2s1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "LEFT",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    102,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "SPI1_I2S1",
                interrupt: "SPI1_I2S1",
            },
        ],
    },
    Peripheral {
        name: "LPUART0",
        address: 0x40000200,
        registers: Some(
            PeripheralRegisters {
                kind: "lpuart",
                version: "v4",
                block: "LPUART",
                ir: &lpuart_v4::REGISTERS,
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
        address: 0x40004000,
        registers: Some(
            PeripheralRegisters {
                kind: "lpuart",
                version: "v4",
                block: "LPUART",
                ir: &lpuart_v4::REGISTERS,
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
                signal: "VC0_1_2_LVD",
                interrupt: "VC0_1_2_LVD",
            },
        ],
    },
    Peripheral {
        name: "OPA",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "opa",
                version: "v3",
                block: "OPA",
                ir: &opa_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
                        register: "PERI_CLKEN0",
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
        address: 0x4000201c,
        registers: Some(
            PeripheralRegisters {
                kind: "reset",
                version: "v3",
                block: "RESET",
                ir: &reset_v3::REGISTERS,
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
                signal: "SPI0_I2S0",
                interrupt: "SPI0_I2S0",
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
                signal: "SPI1_I2S1",
                interrupt: "SPI1_I2S1",
            },
        ],
    },
    Peripheral {
        name: "SYSCTRL",
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "sysctrl",
                version: "v6",
                block: "SYSCTRL",
                ir: &sysctrl_v6::REGISTERS,
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
        address: 0x40000d00,
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
        address: 0x40000e00,
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
        address: 0x40005800,
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
                        register: "PERI_CLKEN0",
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
                version: "v3",
                block: "UART",
                ir: &uart_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
                signal: "UART0_2",
                interrupt: "UART0_2",
            },
        ],
    },
    Peripheral {
        name: "UART1",
        address: 0x40000100,
        registers: Some(
            PeripheralRegisters {
                kind: "uart",
                version: "v3",
                block: "UART",
                ir: &uart_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN0",
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
                signal: "UART1_3",
                interrupt: "UART1_3",
            },
        ],
    },
    Peripheral {
        name: "UART2",
        address: 0x40006000,
        registers: Some(
            PeripheralRegisters {
                kind: "uart",
                version: "v3",
                block: "UART",
                ir: &uart_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN1",
                        field: "SysctrlPeripheralUart2",
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
                    96,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    97,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UART0_2",
                interrupt: "UART0_2",
            },
        ],
    },
    Peripheral {
        name: "UART3",
        address: 0x40006400,
        registers: Some(
            PeripheralRegisters {
                kind: "uart",
                version: "v3",
                block: "UART",
                ir: &uart_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN1",
                        field: "SysctrlPeripheralUart3",
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
                    98,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dma: Some(
                    "DMA1",
                ),
                channel: None,
                request: Some(
                    99,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UART1_3",
                interrupt: "UART1_3",
            },
        ],
    },
    Peripheral {
        name: "USBFS",
        address: 0x40040000,
        registers: Some(
            PeripheralRegisters {
                kind: "usbfs",
                version: "common",
                block: "USBFS",
                ir: &usbfs_common::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "PERI_CLKEN1",
                        field: "SysctrlPeripheralUsb",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "USBFS",
                interrupt: "USBFS",
            },
        ],
    },
    Peripheral {
        name: "VC",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "vc",
                version: "v4",
                block: "VC",
                ir: &vc_v4::REGISTERS,
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
                signal: "VC0_1_2_LVD",
                interrupt: "VC0_1_2_LVD",
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
                        register: "PERI_CLKEN0",
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
        name: "PORTC_E",
        number: 2,
    },
    Interrupt {
        name: "PORTD_F",
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
        name: "UART0_2",
        number: 6,
    },
    Interrupt {
        name: "UART1_3",
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
        name: "SPI0_I2S0",
        number: 10,
    },
    Interrupt {
        name: "SPI1_I2S1",
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
        name: "ADC_DAC",
        number: 24,
    },
    Interrupt {
        name: "VC0_1_2_LVD",
        number: 26,
    },
    Interrupt {
        name: "USBFS",
        number: 27,
    },
    Interrupt {
        name: "CAN",
        number: 28,
    },
    Interrupt {
        name: "FLASH_RAM",
        number: 30,
    },
    Interrupt {
        name: "CLKTRIM_CTS",
        number: 31,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v5.rs"] pub mod adc_v5;
#[path="../registers/adtim_v1.rs"] pub mod adtim_v1;
#[path="../registers/aes_v2.rs"] pub mod aes_v2;
#[path="../registers/bgr_v1.rs"] pub mod bgr_v1;
#[path="../registers/can_v2.rs"] pub mod can_v2;
#[path="../registers/clk_v1.rs"] pub mod clk_v1;
#[path="../registers/crc_v2.rs"] pub mod crc_v2;
#[path="../registers/cts_common.rs"] pub mod cts_common;
#[path="../registers/dac_v1.rs"] pub mod dac_v1;
#[path="../registers/debug_v2.rs"] pub mod debug_v2;
#[path="../registers/dmac_v3.rs"] pub mod dmac_v3;
#[path="../registers/flash_v5.rs"] pub mod flash_v5;
#[path="../registers/gpio_v3.rs"] pub mod gpio_v3;
#[path="../registers/hdiv_common.rs"] pub mod hdiv_common;
#[path="../registers/i_v3.rs"] pub mod i_v3;
#[path="../registers/i_v6.rs"] pub mod i_v6;
#[path="../registers/lpuart_v4.rs"] pub mod lpuart_v4;
#[path="../registers/lvd_v2.rs"] pub mod lvd_v2;
#[path="../registers/opa_v3.rs"] pub mod opa_v3;
#[path="../registers/pca_v1.rs"] pub mod pca_v1;
#[path="../registers/ram_v4.rs"] pub mod ram_v4;
#[path="../registers/reset_v3.rs"] pub mod reset_v3;
#[path="../registers/spi_v2.rs"] pub mod spi_v2;
#[path="../registers/sysctrl_v6.rs"] pub mod sysctrl_v6;
#[path="../registers/trng_v1.rs"] pub mod trng_v1;
#[path="../registers/uart_v3.rs"] pub mod uart_v3;
#[path="../registers/usbfs_common.rs"] pub mod usbfs_common;
#[path="../registers/vc_v4.rs"] pub mod vc_v4;
#[path="../registers/wdt_v1.rs"] pub mod wdt_v1;
