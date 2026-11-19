
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x4000b800,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ll_v1",
            block: "ADC",
            ir: &adc_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_ADC",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "AOS",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "aos",
            version: "ll_v1",
            block: "AOS",
            ir: &aos_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_AOS",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CMU",
        address: 0x40014400,
        registers: Some(PeripheralRegisters {
            kind: "cmu",
            version: "ll_v1_2",
            block: "CMU",
            ir: &cmu_ll_v1_2::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 0x40015400,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "ll_v1_1",
            block: "CRC",
            ir: &crc_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_CRC",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CTC",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "ctc",
            version: "ll_v1",
            block: "CTC",
            ir: &ctc_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_CTC",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGC",
        address: 0x40015000,
        registers: Some(PeripheralRegisters {
            kind: "dbgc",
            version: "ll_v1_1",
            block: "DBGC",
            ir: &dbgc_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EFM",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "efm",
            version: "ll_v1",
            block: "EFM",
            ir: &efm_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40013800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ll_v1_1",
            block: "GPIO",
            ir: &gpio_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ll_v1_1",
            block: "I2C",
            ir: &i2c_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_I2C",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ICG",
        address: 0xc0,
        registers: Some(PeripheralRegisters {
            kind: "icg",
            version: "ll_v1_8",
            block: "ICG",
            ir: &icg_ll_v1_8::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "INTC",
        address: 0x40011000,
        registers: Some(PeripheralRegisters {
            kind: "intc",
            version: "ll_v2_1",
            block: "INTC",
            ir: &intc_ll_v2_1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWC",
        address: 0x40014000,
        registers: Some(PeripheralRegisters {
            kind: "pwc",
            version: "ll_v1",
            block: "PWC",
            ir: &pwc_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RMU",
        address: 0x40014100,
        registers: Some(PeripheralRegisters {
            kind: "rmu",
            version: "ll_v1",
            block: "RMU",
            ir: &rmu_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0x4000d400,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "ll_v1_1",
            block: "RTC",
            ir: &rtc_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_RTC",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ll_v1",
            block: "SPI",
            ir: &spi_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_SPI",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SWDT",
        address: 0x4000cc00,
        registers: Some(PeripheralRegisters {
            kind: "swdt",
            version: "ll_v1_1",
            block: "SWDT",
            ir: &swdt_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: None,
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR0",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "tmr0",
            version: "ll_v1_1",
            block: "TMR0",
            ir: &tmr0_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMR0",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB1",
        address: 0x40007800,
        registers: Some(PeripheralRegisters {
            kind: "tmrb",
            version: "ll_v1_1",
            block: "TMRB",
            ir: &tmrb_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMRB1",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB2",
        address: 0x40007c00,
        registers: Some(PeripheralRegisters {
            kind: "tmrb",
            version: "ll_v1_1",
            block: "TMRB",
            ir: &tmrb_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMRB2",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB3",
        address: 0x40008000,
        registers: Some(PeripheralRegisters {
            kind: "tmrb",
            version: "ll_v1_1",
            block: "TMRB",
            ir: &tmrb_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMRB3",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB4",
        address: 0x40008400,
        registers: Some(PeripheralRegisters {
            kind: "tmrb",
            version: "ll_v1_1",
            block: "TMRB",
            ir: &tmrb_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMRB4",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB5",
        address: 0x40008800,
        registers: Some(PeripheralRegisters {
            kind: "tmrb",
            version: "ll_v1_1",
            block: "TMRB",
            ir: &tmrb_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMRB5",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB6",
        address: 0x40008c00,
        registers: Some(PeripheralRegisters {
            kind: "tmrb",
            version: "ll_v1_1",
            block: "TMRB",
            ir: &tmrb_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMRB6",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB7",
        address: 0x40009000,
        registers: Some(PeripheralRegisters {
            kind: "tmrb",
            version: "ll_v1_1",
            block: "TMRB",
            ir: &tmrb_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMRB7",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB8",
        address: 0x40009400,
        registers: Some(PeripheralRegisters {
            kind: "tmrb",
            version: "ll_v1_1",
            block: "TMRB",
            ir: &tmrb_ll_v1_1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_TMRB8",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART1",
        address: 0x40001800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v1",
            block: "USART",
            ir: &usart_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_USART1",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART2",
        address: 0x40001c00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v1",
            block: "USART",
            ir: &usart_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_USART2",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART3",
        address: 0x40002000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v1",
            block: "USART",
            ir: &usart_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_USART3",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART4",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "ll_v1",
            block: "USART",
            ir: &usart_ll_v1::REGISTERS,
        }),
        views: &[],
        rcc: Some(PeripheralRcc {
            enable: Some(PeripheralRccRegister {
                register: "FCG0",
                field: "CMU_FCG_USART4",
            }),
            polarity: None,
            bus: None,
        }),
        kernel_clock: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "EXTINT00",
        number: 0,
    },
    Interrupt {
        name: "EXTINT01",
        number: 1,
    },
    Interrupt {
        name: "EXTINT02",
        number: 2,
    },
    Interrupt {
        name: "EXTINT03",
        number: 3,
    },
    Interrupt {
        name: "EXTINT04",
        number: 4,
    },
    Interrupt {
        name: "EXTINT05",
        number: 5,
    },
    Interrupt {
        name: "EXTINT06",
        number: 6,
    },
    Interrupt {
        name: "EXTINT07",
        number: 7,
    },
    Interrupt {
        name: "INT008",
        number: 8,
    },
    Interrupt {
        name: "INT009",
        number: 9,
    },
    Interrupt {
        name: "INT010",
        number: 10,
    },
    Interrupt {
        name: "INT011",
        number: 11,
    },
    Interrupt {
        name: "INT012",
        number: 12,
    },
    Interrupt {
        name: "INT013",
        number: 13,
    },
    Interrupt {
        name: "INT014",
        number: 14,
    },
    Interrupt {
        name: "INT015",
        number: 15,
    },
    Interrupt {
        name: "INT016",
        number: 16,
    },
    Interrupt {
        name: "INT017",
        number: 17,
    },
    Interrupt {
        name: "INT018",
        number: 18,
    },
    Interrupt {
        name: "INT019",
        number: 19,
    },
    Interrupt {
        name: "INT020",
        number: 20,
    },
    Interrupt {
        name: "INT021",
        number: 21,
    },
    Interrupt {
        name: "INT022",
        number: 22,
    },
    Interrupt {
        name: "INT023",
        number: 23,
    },
    Interrupt {
        name: "INT024",
        number: 24,
    },
    Interrupt {
        name: "INT025",
        number: 25,
    },
    Interrupt {
        name: "INT026",
        number: 26,
    },
    Interrupt {
        name: "INT027",
        number: 27,
    },
    Interrupt {
        name: "INT028",
        number: 28,
    },
    Interrupt {
        name: "INT029",
        number: 29,
    },
    Interrupt {
        name: "INT030",
        number: 30,
    },
    Interrupt {
        name: "INT031",
        number: 31,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/adc_ll_v1.rs"]
pub mod adc_ll_v1;
#[path = "../registers/aos_ll_v1.rs"]
pub mod aos_ll_v1;
#[path = "../registers/cmu_ll_v1_2.rs"]
pub mod cmu_ll_v1_2;
#[path = "../registers/crc_ll_v1_1.rs"]
pub mod crc_ll_v1_1;
#[path = "../registers/ctc_ll_v1.rs"]
pub mod ctc_ll_v1;
#[path = "../registers/dbgc_ll_v1_1.rs"]
pub mod dbgc_ll_v1_1;
#[path = "../registers/efm_ll_v1.rs"]
pub mod efm_ll_v1;
#[path = "../registers/gpio_ll_v1_1.rs"]
pub mod gpio_ll_v1_1;
#[path = "../registers/i2c_ll_v1_1.rs"]
pub mod i2c_ll_v1_1;
#[path = "../registers/icg_ll_v1_8.rs"]
pub mod icg_ll_v1_8;
#[path = "../registers/intc_ll_v2_1.rs"]
pub mod intc_ll_v2_1;
#[path = "../registers/pwc_ll_v1.rs"]
pub mod pwc_ll_v1;
#[path = "../registers/rmu_ll_v1.rs"]
pub mod rmu_ll_v1;
#[path = "../registers/rtc_ll_v1_1.rs"]
pub mod rtc_ll_v1_1;
#[path = "../registers/spi_ll_v1.rs"]
pub mod spi_ll_v1;
#[path = "../registers/swdt_ll_v1_1.rs"]
pub mod swdt_ll_v1_1;
#[path = "../registers/tmr0_ll_v1_1.rs"]
pub mod tmr0_ll_v1_1;
#[path = "../registers/tmrb_ll_v1_1.rs"]
pub mod tmrb_ll_v1_1;
#[path = "../registers/usart_ll_v1.rs"]
pub mod usart_ll_v1;
