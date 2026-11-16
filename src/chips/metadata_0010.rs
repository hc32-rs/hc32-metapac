
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x4000b800,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v19",
                block: "ADC",
                ir: &adc_v19::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_ADC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "AOS",
        address: 0x40000c00,
        registers: Some(
            PeripheralRegisters {
                kind: "aos",
                version: "v6",
                block: "AOS",
                ir: &aos_v6::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_AOS",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CMU",
        address: 0x40014400,
        registers: Some(
            PeripheralRegisters {
                kind: "cmu",
                version: "v6",
                block: "CMU",
                ir: &cmu_v6::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 0x40015400,
        registers: Some(
            PeripheralRegisters {
                kind: "crc",
                version: "v8",
                block: "CRC",
                ir: &crc_v8::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_CRC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CTC",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "ctc",
                version: "v3",
                block: "CTC",
                ir: &ctc_v3::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_CTC",
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
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgc",
                version: "v6",
                block: "DBGC",
                ir: &dbgc_v6::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EFM",
        address: 0x40000800,
        registers: Some(
            PeripheralRegisters {
                kind: "efm",
                version: "v6",
                block: "EFM",
                ir: &efm_v6::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40013800,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v18",
                block: "GPIO",
                ir: &gpio_v18::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C",
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "i",
                version: "v25",
                block: "I",
                ir: &i_v25::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_I2C",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ICG",
        address: 0xc0,
        registers: Some(
            PeripheralRegisters {
                kind: "icg",
                version: "v5",
                block: "ICG",
                ir: &icg_v5::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "INTC",
        address: 0x40011000,
        registers: Some(
            PeripheralRegisters {
                kind: "intc",
                version: "v6",
                block: "INTC",
                ir: &intc_v6::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWC",
        address: 0x40014000,
        registers: Some(
            PeripheralRegisters {
                kind: "pwc",
                version: "v6",
                block: "PWC",
                ir: &pwc_v6::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RMU",
        address: 0x40014100,
        registers: Some(
            PeripheralRegisters {
                kind: "rmu",
                version: "v6",
                block: "RMU",
                ir: &rmu_v6::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0x4000d400,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "v10",
                block: "RTC",
                ir: &rtc_v10::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_RTC",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI",
        address: 0x40003800,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v10",
                block: "SPI",
                ir: &spi_v10::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_SPI",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SWDT",
        address: 0x4000cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "swdt",
                version: "v1",
                block: "SWDT",
                ir: &swdt_v1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR0",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "v27",
                block: "TMR",
                ir: &tmr_v27::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMR0",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB1",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmrb",
                version: "v1",
                block: "TMRB",
                ir: &tmrb_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRB1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB2",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmrb",
                version: "v1",
                block: "TMRB",
                ir: &tmrb_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRB2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB3",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmrb",
                version: "v1",
                block: "TMRB",
                ir: &tmrb_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRB3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB4",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmrb",
                version: "v1",
                block: "TMRB",
                ir: &tmrb_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRB4",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB5",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmrb",
                version: "v1",
                block: "TMRB",
                ir: &tmrb_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRB5",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB6",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmrb",
                version: "v1",
                block: "TMRB",
                ir: &tmrb_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRB6",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB7",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmrb",
                version: "v1",
                block: "TMRB",
                ir: &tmrb_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRB7",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMRB8",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "tmrb",
                version: "v1",
                block: "TMRB",
                ir: &tmrb_v1::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_TMRB8",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART1",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v25",
                block: "USART",
                ir: &usart_v25::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_USART1",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART2",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v25",
                block: "USART",
                ir: &usart_v25::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_USART2",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART3",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v25",
                block: "USART",
                ir: &usart_v25::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_USART3",
                    },
                ),
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART4",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v25",
                block: "USART",
                ir: &usart_v25::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                enable: Some(
                    PeripheralRccRegister {
                        register: "FCG0",
                        field: "CMU_FCG_USART4",
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
            #[path="../registers/adc_v19.rs"] pub mod adc_v19;
#[path="../registers/aos_v6.rs"] pub mod aos_v6;
#[path="../registers/cmu_v6.rs"] pub mod cmu_v6;
#[path="../registers/crc_v8.rs"] pub mod crc_v8;
#[path="../registers/ctc_v3.rs"] pub mod ctc_v3;
#[path="../registers/dbgc_v6.rs"] pub mod dbgc_v6;
#[path="../registers/efm_v6.rs"] pub mod efm_v6;
#[path="../registers/gpio_v18.rs"] pub mod gpio_v18;
#[path="../registers/i_v25.rs"] pub mod i_v25;
#[path="../registers/icg_v5.rs"] pub mod icg_v5;
#[path="../registers/intc_v6.rs"] pub mod intc_v6;
#[path="../registers/pwc_v6.rs"] pub mod pwc_v6;
#[path="../registers/rmu_v6.rs"] pub mod rmu_v6;
#[path="../registers/rtc_v10.rs"] pub mod rtc_v10;
#[path="../registers/spi_v10.rs"] pub mod spi_v10;
#[path="../registers/swdt_v1.rs"] pub mod swdt_v1;
#[path="../registers/tmr_v27.rs"] pub mod tmr_v27;
#[path="../registers/tmrb_v1.rs"] pub mod tmrb_v1;
#[path="../registers/usart_v25.rs"] pub mod usart_v25;
