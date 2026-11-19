#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - INT000"]
    INT000 = 0,
    #[doc = "1 - INT001"]
    INT001 = 1,
    #[doc = "2 - INT002"]
    INT002 = 2,
    #[doc = "3 - INT003"]
    INT003 = 3,
    #[doc = "4 - INT004"]
    INT004 = 4,
    #[doc = "5 - INT005"]
    INT005 = 5,
    #[doc = "6 - INT006"]
    INT006 = 6,
    #[doc = "7 - INT007"]
    INT007 = 7,
    #[doc = "8 - INT008"]
    INT008 = 8,
    #[doc = "9 - INT009"]
    INT009 = 9,
    #[doc = "10 - INT010"]
    INT010 = 10,
    #[doc = "11 - INT011"]
    INT011 = 11,
    #[doc = "12 - INT012"]
    INT012 = 12,
    #[doc = "13 - INT013"]
    INT013 = 13,
    #[doc = "14 - INT014"]
    INT014 = 14,
    #[doc = "15 - INT015"]
    INT015 = 15,
    #[doc = "16 - EXTINT_PORT_EIRQ0"]
    EXTINT_PORT_EIRQ0 = 16,
    #[doc = "17 - EXTINT_PORT_EIRQ1"]
    EXTINT_PORT_EIRQ1 = 17,
    #[doc = "18 - EXTINT_PORT_EIRQ2"]
    EXTINT_PORT_EIRQ2 = 18,
    #[doc = "19 - EXTINT_PORT_EIRQ3"]
    EXTINT_PORT_EIRQ3 = 19,
    #[doc = "20 - EXTINT_PORT_EIRQ4"]
    EXTINT_PORT_EIRQ4 = 20,
    #[doc = "21 - EXTINT_PORT_EIRQ5"]
    EXTINT_PORT_EIRQ5 = 21,
    #[doc = "22 - EXTINT_PORT_EIRQ6"]
    EXTINT_PORT_EIRQ6 = 22,
    #[doc = "23 - EXTINT_PORT_EIRQ7"]
    EXTINT_PORT_EIRQ7 = 23,
    #[doc = "24 - EXTINT_PORT_EIRQ8"]
    EXTINT_PORT_EIRQ8 = 24,
    #[doc = "25 - EXTINT_PORT_EIRQ9"]
    EXTINT_PORT_EIRQ9 = 25,
    #[doc = "26 - EXTINT_PORT_EIRQ10"]
    EXTINT_PORT_EIRQ10 = 26,
    #[doc = "27 - EXTINT_PORT_EIRQ11"]
    EXTINT_PORT_EIRQ11 = 27,
    #[doc = "28 - EXTINT_PORT_EIRQ12"]
    EXTINT_PORT_EIRQ12 = 28,
    #[doc = "29 - EXTINT_PORT_EIRQ13"]
    EXTINT_PORT_EIRQ13 = 29,
    #[doc = "30 - EXTINT_PORT_EIRQ14"]
    EXTINT_PORT_EIRQ14 = 30,
    #[doc = "31 - EXTINT_PORT_EIRQ15"]
    EXTINT_PORT_EIRQ15 = 31,
    #[doc = "32 - DMA1_ERR"]
    DMA1_ERR = 32,
    #[doc = "33 - DMA1_TC0_BTC0"]
    DMA1_TC0_BTC0 = 33,
    #[doc = "34 - DMA1_TC1_BTC1"]
    DMA1_TC1_BTC1 = 34,
    #[doc = "35 - DMA1_TC2_BTC2"]
    DMA1_TC2_BTC2 = 35,
    #[doc = "36 - DMA1_TC3_BTC3"]
    DMA1_TC3_BTC3 = 36,
    #[doc = "37 - DMA1_TC4_BTC4"]
    DMA1_TC4_BTC4 = 37,
    #[doc = "38 - DMA1_TC5_BTC5"]
    DMA1_TC5_BTC5 = 38,
    #[doc = "39 - EFM_PEERR_RDCOL"]
    EFM_PEERR_RDCOL = 39,
    #[doc = "40 - EFM_OPTEND"]
    EFM_OPTEND = 40,
    #[doc = "42 - DCU1"]
    DCU1 = 42,
    #[doc = "43 - DCU2"]
    DCU2 = 43,
    #[doc = "44 - DCU3"]
    DCU3 = 44,
    #[doc = "45 - DCU4"]
    DCU4 = 45,
    #[doc = "46 - DMA2_ERR"]
    DMA2_ERR = 46,
    #[doc = "47 - DMA2_TC0_BTC0"]
    DMA2_TC0_BTC0 = 47,
    #[doc = "48 - DMA2_TC1_BTC1"]
    DMA2_TC1_BTC1 = 48,
    #[doc = "49 - DMA2_TC2_BTC2"]
    DMA2_TC2_BTC2 = 49,
    #[doc = "50 - DMA2_TC3_BTC3"]
    DMA2_TC3_BTC3 = 50,
    #[doc = "51 - DMA2_TC4_BTC4"]
    DMA2_TC4_BTC4 = 51,
    #[doc = "52 - DMA2_TC5_BTC5"]
    DMA2_TC5_BTC5 = 52,
    #[doc = "53 - TMR0_1"]
    TMR0_1 = 53,
    #[doc = "54 - TMR0_2"]
    TMR0_2 = 54,
    #[doc = "55 - RTC"]
    RTC = 55,
    #[doc = "56 - XTAL"]
    XTAL = 56,
    #[doc = "57 - WKTM"]
    WKTM = 57,
    #[doc = "58 - SWDT"]
    SWDT = 58,
    #[doc = "59 - TMR6_1_GCMP"]
    TMR6_1_GCMP = 59,
    #[doc = "60 - TMR6_1_OVF_UDF"]
    TMR6_1_OVF_UDF = 60,
    #[doc = "61 - TMR6_1_DTE"]
    TMR6_1_DTE = 61,
    #[doc = "62 - TMR6_1_SCMP"]
    TMR6_1_SCMP = 62,
    #[doc = "63 - TMRA_1_OVF_UDF"]
    TMRA_1_OVF_UDF = 63,
    #[doc = "64 - TMRA_1_CMP"]
    TMRA_1_CMP = 64,
    #[doc = "65 - TMR6_2_GCMP"]
    TMR6_2_GCMP = 65,
    #[doc = "66 - TMR6_2_OVF_UDF"]
    TMR6_2_OVF_UDF = 66,
    #[doc = "67 - TMR6_2_DTE"]
    TMR6_2_DTE = 67,
    #[doc = "68 - TMR6_2_SCMP"]
    TMR6_2_SCMP = 68,
    #[doc = "69 - TMRA_2_OVF_UDF"]
    TMRA_2_OVF_UDF = 69,
    #[doc = "70 - TMRA_2_CMP"]
    TMRA_2_CMP = 70,
    #[doc = "71 - TMRA_3_OVF_UDF"]
    TMRA_3_OVF_UDF = 71,
    #[doc = "72 - TMRA_3_CMP"]
    TMRA_3_CMP = 72,
    #[doc = "73 - TMRA_4_OVF_UDF"]
    TMRA_4_OVF_UDF = 73,
    #[doc = "74 - TMRA_4_CMP"]
    TMRA_4_CMP = 74,
    #[doc = "75 - TMR4_1_GCMP"]
    TMR4_1_GCMP = 75,
    #[doc = "76 - TMR4_1_OVF_UDF"]
    TMR4_1_OVF_UDF = 76,
    #[doc = "77 - TMR4_1_RELOAD"]
    TMR4_1_RELOAD = 77,
    #[doc = "78 - TMR4_1_SCMP"]
    TMR4_1_SCMP = 78,
    #[doc = "79 - TMR4_2_GCMP"]
    TMR4_2_GCMP = 79,
    #[doc = "80 - TMR4_2_OVF_UDF"]
    TMR4_2_OVF_UDF = 80,
    #[doc = "81 - TMR4_2_RELOAD"]
    TMR4_2_RELOAD = 81,
    #[doc = "82 - TMR4_2_SCMP"]
    TMR4_2_SCMP = 82,
    #[doc = "83 - TMR4_3_GCMP"]
    TMR4_3_GCMP = 83,
    #[doc = "84 - TMR4_3_OVF_UDF"]
    TMR4_3_OVF_UDF = 84,
    #[doc = "85 - TMR4_3_RELOAD"]
    TMR4_3_RELOAD = 85,
    #[doc = "86 - TMR4_3_SCMP"]
    TMR4_3_SCMP = 86,
    #[doc = "87 - I2C1"]
    I2C1 = 87,
    #[doc = "88 - I2C2"]
    I2C2 = 88,
    #[doc = "89 - CMP1"]
    CMP1 = 89,
    #[doc = "90 - CMP2"]
    CMP2 = 90,
    #[doc = "91 - CMP3"]
    CMP3 = 91,
    #[doc = "92 - CMP4"]
    CMP4 = 92,
    #[doc = "93 - USART1"]
    USART1 = 93,
    #[doc = "94 - USART1_TCI"]
    USART1_TCI = 94,
    #[doc = "95 - USART2"]
    USART2 = 95,
    #[doc = "96 - USART2_TCI"]
    USART2_TCI = 96,
    #[doc = "97 - SPI1"]
    SPI1 = 97,
    #[doc = "98 - TMRA_5_OVF_UDF"]
    TMRA_5_OVF_UDF = 98,
    #[doc = "99 - TMRA_5_CMP"]
    TMRA_5_CMP = 99,
    #[doc = "100 - EVENT_PORT1"]
    EVENT_PORT1 = 100,
    #[doc = "101 - EVENT_PORT2"]
    EVENT_PORT2 = 101,
    #[doc = "102 - EVENT_PORT3"]
    EVENT_PORT3 = 102,
    #[doc = "103 - EVENT_PORT4"]
    EVENT_PORT4 = 103,
    #[doc = "104 - USART3"]
    USART3 = 104,
    #[doc = "105 - USART3_TCI"]
    USART3_TCI = 105,
    #[doc = "106 - USART4"]
    USART4 = 106,
    #[doc = "107 - USART4_TCI"]
    USART4_TCI = 107,
    #[doc = "108 - SPI2"]
    SPI2 = 108,
    #[doc = "109 - SPI3"]
    SPI3 = 109,
    #[doc = "110 - EMB_GR0"]
    EMB_GR0 = 110,
    #[doc = "111 - EMB_GR1"]
    EMB_GR1 = 111,
    #[doc = "112 - EMB_GR2"]
    EMB_GR2 = 112,
    #[doc = "113 - EMB_GR3"]
    EMB_GR3 = 113,
    #[doc = "114 - USART5"]
    USART5 = 114,
    #[doc = "115 - USART5_TCI"]
    USART5_TCI = 115,
    #[doc = "116 - USART6"]
    USART6 = 116,
    #[doc = "117 - USART6_TCI"]
    USART6_TCI = 117,
    #[doc = "122 - USART1_WUPI"]
    USART1_WUPI = 122,
    #[doc = "125 - FCM"]
    FCM = 125,
    #[doc = "126 - WDT"]
    WDT = 126,
    #[doc = "127 - CTC"]
    CTC = 127,
    #[doc = "128 - ADC1"]
    ADC1 = 128,
    #[doc = "129 - ADC2"]
    ADC2 = 129,
    #[doc = "130 - ADC3"]
    ADC3 = 130,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn INT000();
        fn INT001();
        fn INT002();
        fn INT003();
        fn INT004();
        fn INT005();
        fn INT006();
        fn INT007();
        fn INT008();
        fn INT009();
        fn INT010();
        fn INT011();
        fn INT012();
        fn INT013();
        fn INT014();
        fn INT015();
        fn EXTINT_PORT_EIRQ0();
        fn EXTINT_PORT_EIRQ1();
        fn EXTINT_PORT_EIRQ2();
        fn EXTINT_PORT_EIRQ3();
        fn EXTINT_PORT_EIRQ4();
        fn EXTINT_PORT_EIRQ5();
        fn EXTINT_PORT_EIRQ6();
        fn EXTINT_PORT_EIRQ7();
        fn EXTINT_PORT_EIRQ8();
        fn EXTINT_PORT_EIRQ9();
        fn EXTINT_PORT_EIRQ10();
        fn EXTINT_PORT_EIRQ11();
        fn EXTINT_PORT_EIRQ12();
        fn EXTINT_PORT_EIRQ13();
        fn EXTINT_PORT_EIRQ14();
        fn EXTINT_PORT_EIRQ15();
        fn DMA1_ERR();
        fn DMA1_TC0_BTC0();
        fn DMA1_TC1_BTC1();
        fn DMA1_TC2_BTC2();
        fn DMA1_TC3_BTC3();
        fn DMA1_TC4_BTC4();
        fn DMA1_TC5_BTC5();
        fn EFM_PEERR_RDCOL();
        fn EFM_OPTEND();
        fn DCU1();
        fn DCU2();
        fn DCU3();
        fn DCU4();
        fn DMA2_ERR();
        fn DMA2_TC0_BTC0();
        fn DMA2_TC1_BTC1();
        fn DMA2_TC2_BTC2();
        fn DMA2_TC3_BTC3();
        fn DMA2_TC4_BTC4();
        fn DMA2_TC5_BTC5();
        fn TMR0_1();
        fn TMR0_2();
        fn RTC();
        fn XTAL();
        fn WKTM();
        fn SWDT();
        fn TMR6_1_GCMP();
        fn TMR6_1_OVF_UDF();
        fn TMR6_1_DTE();
        fn TMR6_1_SCMP();
        fn TMRA_1_OVF_UDF();
        fn TMRA_1_CMP();
        fn TMR6_2_GCMP();
        fn TMR6_2_OVF_UDF();
        fn TMR6_2_DTE();
        fn TMR6_2_SCMP();
        fn TMRA_2_OVF_UDF();
        fn TMRA_2_CMP();
        fn TMRA_3_OVF_UDF();
        fn TMRA_3_CMP();
        fn TMRA_4_OVF_UDF();
        fn TMRA_4_CMP();
        fn TMR4_1_GCMP();
        fn TMR4_1_OVF_UDF();
        fn TMR4_1_RELOAD();
        fn TMR4_1_SCMP();
        fn TMR4_2_GCMP();
        fn TMR4_2_OVF_UDF();
        fn TMR4_2_RELOAD();
        fn TMR4_2_SCMP();
        fn TMR4_3_GCMP();
        fn TMR4_3_OVF_UDF();
        fn TMR4_3_RELOAD();
        fn TMR4_3_SCMP();
        fn I2C1();
        fn I2C2();
        fn CMP1();
        fn CMP2();
        fn CMP3();
        fn CMP4();
        fn USART1();
        fn USART1_TCI();
        fn USART2();
        fn USART2_TCI();
        fn SPI1();
        fn TMRA_5_OVF_UDF();
        fn TMRA_5_CMP();
        fn EVENT_PORT1();
        fn EVENT_PORT2();
        fn EVENT_PORT3();
        fn EVENT_PORT4();
        fn USART3();
        fn USART3_TCI();
        fn USART4();
        fn USART4_TCI();
        fn SPI2();
        fn SPI3();
        fn EMB_GR0();
        fn EMB_GR1();
        fn EMB_GR2();
        fn EMB_GR3();
        fn USART5();
        fn USART5_TCI();
        fn USART6();
        fn USART6_TCI();
        fn USART1_WUPI();
        fn FCM();
        fn WDT();
        fn CTC();
        fn ADC1();
        fn ADC2();
        fn ADC3();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 131] = [
        Vector { _handler: INT000 },
        Vector { _handler: INT001 },
        Vector { _handler: INT002 },
        Vector { _handler: INT003 },
        Vector { _handler: INT004 },
        Vector { _handler: INT005 },
        Vector { _handler: INT006 },
        Vector { _handler: INT007 },
        Vector { _handler: INT008 },
        Vector { _handler: INT009 },
        Vector { _handler: INT010 },
        Vector { _handler: INT011 },
        Vector { _handler: INT012 },
        Vector { _handler: INT013 },
        Vector { _handler: INT014 },
        Vector { _handler: INT015 },
        Vector {
            _handler: EXTINT_PORT_EIRQ0,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ1,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ2,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ3,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ4,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ5,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ6,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ7,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ8,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ9,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ10,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ11,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ12,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ13,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ14,
        },
        Vector {
            _handler: EXTINT_PORT_EIRQ15,
        },
        Vector { _handler: DMA1_ERR },
        Vector {
            _handler: DMA1_TC0_BTC0,
        },
        Vector {
            _handler: DMA1_TC1_BTC1,
        },
        Vector {
            _handler: DMA1_TC2_BTC2,
        },
        Vector {
            _handler: DMA1_TC3_BTC3,
        },
        Vector {
            _handler: DMA1_TC4_BTC4,
        },
        Vector {
            _handler: DMA1_TC5_BTC5,
        },
        Vector {
            _handler: EFM_PEERR_RDCOL,
        },
        Vector {
            _handler: EFM_OPTEND,
        },
        Vector { _reserved: 0 },
        Vector { _handler: DCU1 },
        Vector { _handler: DCU2 },
        Vector { _handler: DCU3 },
        Vector { _handler: DCU4 },
        Vector { _handler: DMA2_ERR },
        Vector {
            _handler: DMA2_TC0_BTC0,
        },
        Vector {
            _handler: DMA2_TC1_BTC1,
        },
        Vector {
            _handler: DMA2_TC2_BTC2,
        },
        Vector {
            _handler: DMA2_TC3_BTC3,
        },
        Vector {
            _handler: DMA2_TC4_BTC4,
        },
        Vector {
            _handler: DMA2_TC5_BTC5,
        },
        Vector { _handler: TMR0_1 },
        Vector { _handler: TMR0_2 },
        Vector { _handler: RTC },
        Vector { _handler: XTAL },
        Vector { _handler: WKTM },
        Vector { _handler: SWDT },
        Vector {
            _handler: TMR6_1_GCMP,
        },
        Vector {
            _handler: TMR6_1_OVF_UDF,
        },
        Vector {
            _handler: TMR6_1_DTE,
        },
        Vector {
            _handler: TMR6_1_SCMP,
        },
        Vector {
            _handler: TMRA_1_OVF_UDF,
        },
        Vector {
            _handler: TMRA_1_CMP,
        },
        Vector {
            _handler: TMR6_2_GCMP,
        },
        Vector {
            _handler: TMR6_2_OVF_UDF,
        },
        Vector {
            _handler: TMR6_2_DTE,
        },
        Vector {
            _handler: TMR6_2_SCMP,
        },
        Vector {
            _handler: TMRA_2_OVF_UDF,
        },
        Vector {
            _handler: TMRA_2_CMP,
        },
        Vector {
            _handler: TMRA_3_OVF_UDF,
        },
        Vector {
            _handler: TMRA_3_CMP,
        },
        Vector {
            _handler: TMRA_4_OVF_UDF,
        },
        Vector {
            _handler: TMRA_4_CMP,
        },
        Vector {
            _handler: TMR4_1_GCMP,
        },
        Vector {
            _handler: TMR4_1_OVF_UDF,
        },
        Vector {
            _handler: TMR4_1_RELOAD,
        },
        Vector {
            _handler: TMR4_1_SCMP,
        },
        Vector {
            _handler: TMR4_2_GCMP,
        },
        Vector {
            _handler: TMR4_2_OVF_UDF,
        },
        Vector {
            _handler: TMR4_2_RELOAD,
        },
        Vector {
            _handler: TMR4_2_SCMP,
        },
        Vector {
            _handler: TMR4_3_GCMP,
        },
        Vector {
            _handler: TMR4_3_OVF_UDF,
        },
        Vector {
            _handler: TMR4_3_RELOAD,
        },
        Vector {
            _handler: TMR4_3_SCMP,
        },
        Vector { _handler: I2C1 },
        Vector { _handler: I2C2 },
        Vector { _handler: CMP1 },
        Vector { _handler: CMP2 },
        Vector { _handler: CMP3 },
        Vector { _handler: CMP4 },
        Vector { _handler: USART1 },
        Vector {
            _handler: USART1_TCI,
        },
        Vector { _handler: USART2 },
        Vector {
            _handler: USART2_TCI,
        },
        Vector { _handler: SPI1 },
        Vector {
            _handler: TMRA_5_OVF_UDF,
        },
        Vector {
            _handler: TMRA_5_CMP,
        },
        Vector {
            _handler: EVENT_PORT1,
        },
        Vector {
            _handler: EVENT_PORT2,
        },
        Vector {
            _handler: EVENT_PORT3,
        },
        Vector {
            _handler: EVENT_PORT4,
        },
        Vector { _handler: USART3 },
        Vector {
            _handler: USART3_TCI,
        },
        Vector { _handler: USART4 },
        Vector {
            _handler: USART4_TCI,
        },
        Vector { _handler: SPI2 },
        Vector { _handler: SPI3 },
        Vector { _handler: EMB_GR0 },
        Vector { _handler: EMB_GR1 },
        Vector { _handler: EMB_GR2 },
        Vector { _handler: EMB_GR3 },
        Vector { _handler: USART5 },
        Vector {
            _handler: USART5_TCI,
        },
        Vector { _handler: USART6 },
        Vector {
            _handler: USART6_TCI,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: USART1_WUPI,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FCM },
        Vector { _handler: WDT },
        Vector { _handler: CTC },
        Vector { _handler: ADC1 },
        Vector { _handler: ADC2 },
        Vector { _handler: ADC3 },
    ];
}
pub const ICG: icg_ll_v1::Icg = unsafe { icg_ll_v1::Icg::from_ptr(0x0400usize as _) };
pub const CRC: crc_ll_v1::Crc = unsafe { crc_ll_v1::Crc::from_ptr(0x4000_8c00usize as _) };
pub const EFM: efm_ll_v2_3::Efm = unsafe { efm_ll_v2_3::Efm::from_ptr(0x4001_0400usize as _) };
pub const AOS: aos_ll_v9::Aos = unsafe { aos_ll_v9::Aos::from_ptr(0x4001_0800usize as _) };
pub const EMB0: emb_ll_v1_2::Emb = unsafe { emb_ll_v1_2::Emb::from_ptr(0x4001_7c00usize as _) };
pub const EMB1: emb_ll_v2_6::Emb = unsafe { emb_ll_v2_6::Emb::from_ptr(0x4001_7c20usize as _) };
pub const EMB2: emb_ll_v2_6::Emb = unsafe { emb_ll_v2_6::Emb::from_ptr(0x4001_7c40usize as _) };
pub const EMB3: emb_ll_v2_6::Emb = unsafe { emb_ll_v2_6::Emb::from_ptr(0x4001_7c60usize as _) };
pub const SPI1: spi_ll_v1_3::Spi = unsafe { spi_ll_v1_3::Spi::from_ptr(0x4001_c000usize as _) };
pub const SPI2: spi_ll_v1_3::Spi = unsafe { spi_ll_v1_3::Spi::from_ptr(0x4001_c400usize as _) };
pub const USART1: usart_ll_v2_4::Usart =
    unsafe { usart_ll_v2_4::Usart::from_ptr(0x4001_cc00usize as _) };
pub const USART2: usart_ll_v2_4::Usart =
    unsafe { usart_ll_v2_4::Usart::from_ptr(0x4001_d000usize as _) };
pub const USART3: usart_ll_v1_1::Usart =
    unsafe { usart_ll_v1_1::Usart::from_ptr(0x4001_d400usize as _) };
pub const SPI3: spi_ll_v1_3::Spi = unsafe { spi_ll_v1_3::Spi::from_ptr(0x4002_0000usize as _) };
pub const USART4: usart_ll_v2_4::Usart =
    unsafe { usart_ll_v2_4::Usart::from_ptr(0x4002_0c00usize as _) };
pub const USART5: usart_ll_v2_4::Usart =
    unsafe { usart_ll_v2_4::Usart::from_ptr(0x4002_1000usize as _) };
pub const USART6: usart_ll_v1_1::Usart =
    unsafe { usart_ll_v1_1::Usart::from_ptr(0x4002_1400usize as _) };
pub const TMR01: tmr0_ll_v2_3::Tmr0 =
    unsafe { tmr0_ll_v2_3::Tmr0::from_ptr(0x4002_4000usize as _) };
pub const TMR02: tmr0_ll_v2_3::Tmr0 =
    unsafe { tmr0_ll_v2_3::Tmr0::from_ptr(0x4002_4400usize as _) };
pub const TMRA5: tmra_ll_v4_2::Tmra =
    unsafe { tmra_ll_v4_2::Tmra::from_ptr(0x4002_6000usize as _) };
pub const TMR41: tmr4_ll_v2::Tmr4 = unsafe { tmr4_ll_v2::Tmr4::from_ptr(0x4003_8000usize as _) };
pub const TMR42: tmr4_ll_v2_2::Tmr4 =
    unsafe { tmr4_ll_v2_2::Tmr4::from_ptr(0x4003_8400usize as _) };
pub const CMP1: cmp_ll_v3::Cmp = unsafe { cmp_ll_v3::Cmp::from_ptr(0x4003_8800usize as _) };
pub const CMP2: cmp_ll_v3_2::Cmp = unsafe { cmp_ll_v3_2::Cmp::from_ptr(0x4003_8900usize as _) };
pub const CMP3: cmp_ll_v3_1::Cmp = unsafe { cmp_ll_v3_1::Cmp::from_ptr(0x4003_8c00usize as _) };
pub const CMP4: cmp_ll_v3_3::Cmp = unsafe { cmp_ll_v3_3::Cmp::from_ptr(0x4003_8d00usize as _) };
pub const TMR43: tmr4_ll_v2_3::Tmr4 =
    unsafe { tmr4_ll_v2_3::Tmr4::from_ptr(0x4003_8e00usize as _) };
pub const TMRA1: tmra_ll_v4_2::Tmra =
    unsafe { tmra_ll_v4_2::Tmra::from_ptr(0x4003_a000usize as _) };
pub const TMRA2: tmra_ll_v4_2::Tmra =
    unsafe { tmra_ll_v4_2::Tmra::from_ptr(0x4003_a400usize as _) };
pub const TMRA3: tmra_ll_v4_2::Tmra =
    unsafe { tmra_ll_v4_2::Tmra::from_ptr(0x4003_a800usize as _) };
pub const TMRA4: tmra_ll_v4_2::Tmra =
    unsafe { tmra_ll_v4_2::Tmra::from_ptr(0x4003_ac00usize as _) };
pub const I2C1: i2c_ll_v3_1::I2c = unsafe { i2c_ll_v3_1::I2c::from_ptr(0x4003_b400usize as _) };
pub const I2C2: i2c_ll_v3_1::I2c = unsafe { i2c_ll_v3_1::I2c::from_ptr(0x4003_b800usize as _) };
pub const TMR61: tmr_v5::Tmr = unsafe { tmr_v5::Tmr::from_ptr(0x4003_c000usize as _) };
pub const TMR6_COMMON: tmr_v6::Tmr = unsafe { tmr_v6::Tmr::from_ptr(0x4003_c300usize as _) };
pub const TMR62: tmr_v5::Tmr = unsafe { tmr_v5::Tmr::from_ptr(0x4003_c400usize as _) };
pub const ADC1: adc_ll_v2_6::Adc = unsafe { adc_ll_v2_6::Adc::from_ptr(0x4004_0000usize as _) };
pub const ADC2: adc_ll_v2_1::Adc = unsafe { adc_ll_v2_1::Adc::from_ptr(0x4004_0400usize as _) };
pub const ADC3: adc_ll_v2_2::Adc = unsafe { adc_ll_v2_2::Adc::from_ptr(0x4004_0800usize as _) };
pub const DAC: dac_ll_v1_2::Dac = unsafe { dac_ll_v1_2::Dac::from_ptr(0x4004_1000usize as _) };
pub const CMU: cmu_ll_v3::Cmu = unsafe { cmu_ll_v3::Cmu::from_ptr(0x4004_8000usize as _) };
pub const PWC: pwc_ll_v3_4::Pwc = unsafe { pwc_ll_v3_4::Pwc::from_ptr(0x4004_8000usize as _) };
pub const FCM: fcm_ll_v1_1::Fcm = unsafe { fcm_ll_v1_1::Fcm::from_ptr(0x4004_8400usize as _) };
pub const WDT: wdt_ll_v1::Wdt = unsafe { wdt_ll_v1::Wdt::from_ptr(0x4004_9000usize as _) };
pub const SWDT: swdt_ll_v1_1::Swdt = unsafe { swdt_ll_v1_1::Swdt::from_ptr(0x4004_9400usize as _) };
pub const CTC: ctc_ll_v1_1::Ctc = unsafe { ctc_ll_v1_1::Ctc::from_ptr(0x4004_9c00usize as _) };
pub const RTC: rtc_ll_v1::Rtc = unsafe { rtc_ll_v1::Rtc::from_ptr(0x4004_c000usize as _) };
pub const RMU: rmu_ll_v3_1::Rmu = unsafe { rmu_ll_v3_1::Rmu::from_ptr(0x4004_cce0usize as _) };
pub const MPU: mpu_v7::Mpu = unsafe { mpu_v7::Mpu::from_ptr(0x4005_0000usize as _) };
pub const SRAMC: sramc_ll_v2_1::Sramc =
    unsafe { sramc_ll_v2_1::Sramc::from_ptr(0x4005_0800usize as _) };
pub const KEYSCAN: keyscan_common::Keyscan =
    unsafe { keyscan_common::Keyscan::from_ptr(0x4005_0c00usize as _) };
pub const INTC: intc_ll_v2_5::Intc = unsafe { intc_ll_v2_5::Intc::from_ptr(0x4005_1000usize as _) };
pub const DMA1: dma_ll_v2_3::Dma = unsafe { dma_ll_v2_3::Dma::from_ptr(0x4005_3000usize as _) };
pub const DMA2: dma_ll_v2_3::Dma = unsafe { dma_ll_v2_3::Dma::from_ptr(0x4005_3400usize as _) };
pub const GPIO: gpio_ll_v1_7::Gpio = unsafe { gpio_ll_v1_7::Gpio::from_ptr(0x4005_3800usize as _) };
pub const PERIC: peric_ll_v1_2::Peric =
    unsafe { peric_ll_v1_2::Peric::from_ptr(0x4005_5400usize as _) };
pub const DCU1: dcu_v1::Dcu = unsafe { dcu_v1::Dcu::from_ptr(0x4005_6000usize as _) };
pub const DCU2: dcu_v1::Dcu = unsafe { dcu_v1::Dcu::from_ptr(0x4005_6400usize as _) };
pub const DCU3: dcu_v1::Dcu = unsafe { dcu_v1::Dcu::from_ptr(0x4005_6800usize as _) };
pub const DCU4: dcu_v1::Dcu = unsafe { dcu_v1::Dcu::from_ptr(0x4005_6c00usize as _) };
pub const DBGC: dbgc_ll_v2_2::Dbgc = unsafe { dbgc_ll_v2_2::Dbgc::from_ptr(0xe004_2000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ll_v2_1.rs"]
pub mod adc_ll_v2_1;
#[path = "../../peripherals/adc_ll_v2_2.rs"]
pub mod adc_ll_v2_2;
#[path = "../../peripherals/adc_ll_v2_6.rs"]
pub mod adc_ll_v2_6;
#[path = "../../peripherals/aos_ll_v9.rs"]
pub mod aos_ll_v9;
#[path = "../../peripherals/cmp_ll_v3.rs"]
pub mod cmp_ll_v3;
#[path = "../../peripherals/cmp_ll_v3_1.rs"]
pub mod cmp_ll_v3_1;
#[path = "../../peripherals/cmp_ll_v3_2.rs"]
pub mod cmp_ll_v3_2;
#[path = "../../peripherals/cmp_ll_v3_3.rs"]
pub mod cmp_ll_v3_3;
#[path = "../../peripherals/cmu_ll_v3.rs"]
pub mod cmu_ll_v3;
#[path = "../../peripherals/crc_ll_v1.rs"]
pub mod crc_ll_v1;
#[path = "../../peripherals/ctc_ll_v1_1.rs"]
pub mod ctc_ll_v1_1;
#[path = "../../peripherals/dac_ll_v1_2.rs"]
pub mod dac_ll_v1_2;
#[path = "../../peripherals/dbgc_ll_v2_2.rs"]
pub mod dbgc_ll_v2_2;
#[path = "../../peripherals/dcu_v1.rs"]
pub mod dcu_v1;
#[path = "../../peripherals/dma_ll_v2_3.rs"]
pub mod dma_ll_v2_3;
#[path = "../../peripherals/efm_ll_v2_3.rs"]
pub mod efm_ll_v2_3;
#[path = "../../peripherals/emb_ll_v1_2.rs"]
pub mod emb_ll_v1_2;
#[path = "../../peripherals/emb_ll_v2_6.rs"]
pub mod emb_ll_v2_6;
#[path = "../../peripherals/fcm_ll_v1_1.rs"]
pub mod fcm_ll_v1_1;
#[path = "../../peripherals/gpio_ll_v1_7.rs"]
pub mod gpio_ll_v1_7;
#[path = "../../peripherals/i2c_ll_v3_1.rs"]
pub mod i2c_ll_v3_1;
#[path = "../../peripherals/icg_ll_v1.rs"]
pub mod icg_ll_v1;
#[path = "../../peripherals/intc_ll_v2_5.rs"]
pub mod intc_ll_v2_5;
#[path = "../../peripherals/keyscan_common.rs"]
pub mod keyscan_common;
#[path = "../../peripherals/mpu_v7.rs"]
pub mod mpu_v7;
#[path = "../../peripherals/peric_ll_v1_2.rs"]
pub mod peric_ll_v1_2;
#[path = "../../peripherals/pwc_ll_v3_4.rs"]
pub mod pwc_ll_v3_4;
#[path = "../../peripherals/rmu_ll_v3_1.rs"]
pub mod rmu_ll_v3_1;
#[path = "../../peripherals/rtc_ll_v1.rs"]
pub mod rtc_ll_v1;
#[path = "../../peripherals/spi_ll_v1_3.rs"]
pub mod spi_ll_v1_3;
#[path = "../../peripherals/sramc_ll_v2_1.rs"]
pub mod sramc_ll_v2_1;
#[path = "../../peripherals/swdt_ll_v1_1.rs"]
pub mod swdt_ll_v1_1;
#[path = "../../peripherals/tmr0_ll_v2_3.rs"]
pub mod tmr0_ll_v2_3;
#[path = "../../peripherals/tmr4_ll_v2.rs"]
pub mod tmr4_ll_v2;
#[path = "../../peripherals/tmr4_ll_v2_2.rs"]
pub mod tmr4_ll_v2_2;
#[path = "../../peripherals/tmr4_ll_v2_3.rs"]
pub mod tmr4_ll_v2_3;
#[path = "../../peripherals/tmr_v5.rs"]
pub mod tmr_v5;
#[path = "../../peripherals/tmr_v6.rs"]
pub mod tmr_v6;
#[path = "../../peripherals/tmra_ll_v4_2.rs"]
pub mod tmra_ll_v4_2;
#[path = "../../peripherals/usart_ll_v1_1.rs"]
pub mod usart_ll_v1_1;
#[path = "../../peripherals/usart_ll_v2_4.rs"]
pub mod usart_ll_v2_4;
#[path = "../../peripherals/wdt_ll_v1.rs"]
pub mod wdt_ll_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
