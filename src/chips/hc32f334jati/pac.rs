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
    #[doc = "32 - DMA_ERR"]
    DMA_ERR = 32,
    #[doc = "33 - DMA_TC0_BTC0"]
    DMA_TC0_BTC0 = 33,
    #[doc = "34 - DMA_TC1_BTC1"]
    DMA_TC1_BTC1 = 34,
    #[doc = "35 - DMA_TC2_BTC2"]
    DMA_TC2_BTC2 = 35,
    #[doc = "36 - DMA_TC3_BTC3"]
    DMA_TC3_BTC3 = 36,
    #[doc = "37 - DMA_TC4_BTC4"]
    DMA_TC4_BTC4 = 37,
    #[doc = "38 - DMA_TC5_BTC5"]
    DMA_TC5_BTC5 = 38,
    #[doc = "39 - DMA_TC6_BTC6"]
    DMA_TC6_BTC6 = 39,
    #[doc = "40 - DMA_TC7_BTC7"]
    DMA_TC7_BTC7 = 40,
    #[doc = "41 - EFM_PEERR_RDCOL"]
    EFM_PEERR_RDCOL = 41,
    #[doc = "42 - EFM_OPTEND"]
    EFM_OPTEND = 42,
    #[doc = "43 - FPU_ERR"]
    FPU_ERR = 43,
    #[doc = "44 - TMR0_1"]
    TMR0_1 = 44,
    #[doc = "45 - TMR0_2"]
    TMR0_2 = 45,
    #[doc = "46 - RTC"]
    RTC = 46,
    #[doc = "47 - XTAL"]
    XTAL = 47,
    #[doc = "48 - WKTM"]
    WKTM = 48,
    #[doc = "49 - SWDT"]
    SWDT = 49,
    #[doc = "50 - TMR6_1_GCMP"]
    TMR6_1_GCMP = 50,
    #[doc = "51 - TMR6_1_OVF_UDF"]
    TMR6_1_OVF_UDF = 51,
    #[doc = "52 - TMR6_1_DTE"]
    TMR6_1_DTE = 52,
    #[doc = "53 - TMR6_1_SCMP"]
    TMR6_1_SCMP = 53,
    #[doc = "54 - TMRA_1_OVF_UDF"]
    TMRA_1_OVF_UDF = 54,
    #[doc = "55 - TMRA_1_CMP"]
    TMRA_1_CMP = 55,
    #[doc = "56 - TMR6_2_GCMP"]
    TMR6_2_GCMP = 56,
    #[doc = "57 - TMR6_2_OVF_UDF"]
    TMR6_2_OVF_UDF = 57,
    #[doc = "58 - TMR6_2_DTE"]
    TMR6_2_DTE = 58,
    #[doc = "59 - TMR6_2_SCMP"]
    TMR6_2_SCMP = 59,
    #[doc = "60 - TMRA_2_OVF_UDF"]
    TMRA_2_OVF_UDF = 60,
    #[doc = "61 - TMRA_2_CMP"]
    TMRA_2_CMP = 61,
    #[doc = "62 - TMR6_3_GCMP"]
    TMR6_3_GCMP = 62,
    #[doc = "63 - TMR6_3_OVF_UDF"]
    TMR6_3_OVF_UDF = 63,
    #[doc = "64 - TMR6_3_DTE"]
    TMR6_3_DTE = 64,
    #[doc = "65 - TMR6_3_SCMP"]
    TMR6_3_SCMP = 65,
    #[doc = "66 - TMRA_3_OVF_UDF"]
    TMRA_3_OVF_UDF = 66,
    #[doc = "67 - TMRA_3_CMP"]
    TMRA_3_CMP = 67,
    #[doc = "68 - TMR6_4_GCMP"]
    TMR6_4_GCMP = 68,
    #[doc = "69 - TMR6_4_OVF_UDF"]
    TMR6_4_OVF_UDF = 69,
    #[doc = "70 - TMR6_4_DTE"]
    TMR6_4_DTE = 70,
    #[doc = "71 - TMR6_4_SCMP"]
    TMR6_4_SCMP = 71,
    #[doc = "72 - TMRA_4_OVF_UDF"]
    TMRA_4_OVF_UDF = 72,
    #[doc = "73 - TMRA_4_CMP"]
    TMRA_4_CMP = 73,
    #[doc = "74 - HRPWM_1_GCMP"]
    HRPWM_1_GCMP = 74,
    #[doc = "75 - HRPWM_1_OVF_UDF"]
    HRPWM_1_OVF_UDF = 75,
    #[doc = "76 - HRPWM_1_SCMP"]
    HRPWM_1_SCMP = 76,
    #[doc = "77 - HRPWM_1_GCAP"]
    HRPWM_1_GCAP = 77,
    #[doc = "78 - HRPWM_2_GCMP"]
    HRPWM_2_GCMP = 78,
    #[doc = "79 - HRPWM_2_OVF_UDF"]
    HRPWM_2_OVF_UDF = 79,
    #[doc = "80 - HRPWM_2_SCMP"]
    HRPWM_2_SCMP = 80,
    #[doc = "81 - HRPWM_2_GCAP"]
    HRPWM_2_GCAP = 81,
    #[doc = "82 - HRPWM"]
    HRPWM = 82,
    #[doc = "83 - EMB_GR0"]
    EMB_GR0 = 83,
    #[doc = "84 - EMB_GR1"]
    EMB_GR1 = 84,
    #[doc = "85 - EMB_GR2"]
    EMB_GR2 = 85,
    #[doc = "86 - EMB_GR3"]
    EMB_GR3 = 86,
    #[doc = "87 - HRPWM_3_GCMP"]
    HRPWM_3_GCMP = 87,
    #[doc = "88 - HRPWM_3_OVF_UDF"]
    HRPWM_3_OVF_UDF = 88,
    #[doc = "89 - HRPWM_3_SCMP"]
    HRPWM_3_SCMP = 89,
    #[doc = "90 - HRPWM_3_GCAP"]
    HRPWM_3_GCAP = 90,
    #[doc = "91 - HRPWM_4_GCMP"]
    HRPWM_4_GCMP = 91,
    #[doc = "92 - HRPWM_4_OVF_UDF"]
    HRPWM_4_OVF_UDF = 92,
    #[doc = "93 - HRPWM_4_SCMP"]
    HRPWM_4_SCMP = 93,
    #[doc = "94 - HRPWM_4_GCAP"]
    HRPWM_4_GCAP = 94,
    #[doc = "95 - EMB_GR4"]
    EMB_GR4 = 95,
    #[doc = "96 - EMB_GR5"]
    EMB_GR5 = 96,
    #[doc = "97 - EMB_GR6"]
    EMB_GR6 = 97,
    #[doc = "98 - EMB_GR7"]
    EMB_GR7 = 98,
    #[doc = "99 - EMB_GR8"]
    EMB_GR8 = 99,
    #[doc = "100 - HRPWM_5_GCMP"]
    HRPWM_5_GCMP = 100,
    #[doc = "101 - HRPWM_5_OVF_UDF"]
    HRPWM_5_OVF_UDF = 101,
    #[doc = "102 - HRPWM_5_SCMP"]
    HRPWM_5_SCMP = 102,
    #[doc = "103 - HRPWM_5_GCAP"]
    HRPWM_5_GCAP = 103,
    #[doc = "104 - HRPWM_6_GCMP"]
    HRPWM_6_GCMP = 104,
    #[doc = "105 - HRPWM_6_OVF_UDF"]
    HRPWM_6_OVF_UDF = 105,
    #[doc = "106 - HRPWM_6_SCMP"]
    HRPWM_6_SCMP = 106,
    #[doc = "107 - HRPWM_6_GCAP"]
    HRPWM_6_GCAP = 107,
    #[doc = "108 - MCANRAM_ECCER"]
    MCANRAM_ECCER = 108,
    #[doc = "109 - MCAN1_INT0"]
    MCAN1_INT0 = 109,
    #[doc = "110 - MCAN1_INT1"]
    MCAN1_INT1 = 110,
    #[doc = "111 - MCAN2_INT0"]
    MCAN2_INT0 = 111,
    #[doc = "112 - MCAN2_INT1"]
    MCAN2_INT1 = 112,
    #[doc = "113 - TMR4_GCMP"]
    TMR4_GCMP = 113,
    #[doc = "114 - TMR4_OVF_UDF"]
    TMR4_OVF_UDF = 114,
    #[doc = "115 - TMR4_RELOAD"]
    TMR4_RELOAD = 115,
    #[doc = "116 - TMR4_SCMP"]
    TMR4_SCMP = 116,
    #[doc = "117 - CMP1"]
    CMP1 = 117,
    #[doc = "118 - CMP2"]
    CMP2 = 118,
    #[doc = "119 - CMP3"]
    CMP3 = 119,
    #[doc = "120 - I2C"]
    I2C = 120,
    #[doc = "121 - USART1"]
    USART1 = 121,
    #[doc = "122 - USART1_TCI"]
    USART1_TCI = 122,
    #[doc = "123 - SPI"]
    SPI = 123,
    #[doc = "124 - TMRA_5_OVF_UDF"]
    TMRA_5_OVF_UDF = 124,
    #[doc = "125 - TMRA_5_CMP"]
    TMRA_5_CMP = 125,
    #[doc = "126 - EVENT_PORT1"]
    EVENT_PORT1 = 126,
    #[doc = "127 - EVENT_PORT2"]
    EVENT_PORT2 = 127,
    #[doc = "128 - EVENT_PORT3"]
    EVENT_PORT3 = 128,
    #[doc = "129 - EVENT_PORT4"]
    EVENT_PORT4 = 129,
    #[doc = "130 - USART2"]
    USART2 = 130,
    #[doc = "131 - USART2_TCI"]
    USART2_TCI = 131,
    #[doc = "132 - USART3"]
    USART3 = 132,
    #[doc = "133 - USART3_TCI"]
    USART3_TCI = 133,
    #[doc = "134 - USART4"]
    USART4 = 134,
    #[doc = "135 - USART4_TCI"]
    USART4_TCI = 135,
    #[doc = "137 - USART1_WUPI"]
    USART1_WUPI = 137,
    #[doc = "138 - LVD1"]
    LVD1 = 138,
    #[doc = "139 - LVD2"]
    LVD2 = 139,
    #[doc = "140 - FCM"]
    FCM = 140,
    #[doc = "141 - WDT"]
    WDT = 141,
    #[doc = "142 - CTC"]
    CTC = 142,
    #[doc = "143 - ADC1"]
    ADC1 = 143,
    #[doc = "144 - ADC2"]
    ADC2 = 144,
    #[doc = "145 - ADC3"]
    ADC3 = 145,
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
        fn DMA_ERR();
        fn DMA_TC0_BTC0();
        fn DMA_TC1_BTC1();
        fn DMA_TC2_BTC2();
        fn DMA_TC3_BTC3();
        fn DMA_TC4_BTC4();
        fn DMA_TC5_BTC5();
        fn DMA_TC6_BTC6();
        fn DMA_TC7_BTC7();
        fn EFM_PEERR_RDCOL();
        fn EFM_OPTEND();
        fn FPU_ERR();
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
        fn TMR6_3_GCMP();
        fn TMR6_3_OVF_UDF();
        fn TMR6_3_DTE();
        fn TMR6_3_SCMP();
        fn TMRA_3_OVF_UDF();
        fn TMRA_3_CMP();
        fn TMR6_4_GCMP();
        fn TMR6_4_OVF_UDF();
        fn TMR6_4_DTE();
        fn TMR6_4_SCMP();
        fn TMRA_4_OVF_UDF();
        fn TMRA_4_CMP();
        fn HRPWM_1_GCMP();
        fn HRPWM_1_OVF_UDF();
        fn HRPWM_1_SCMP();
        fn HRPWM_1_GCAP();
        fn HRPWM_2_GCMP();
        fn HRPWM_2_OVF_UDF();
        fn HRPWM_2_SCMP();
        fn HRPWM_2_GCAP();
        fn HRPWM();
        fn EMB_GR0();
        fn EMB_GR1();
        fn EMB_GR2();
        fn EMB_GR3();
        fn HRPWM_3_GCMP();
        fn HRPWM_3_OVF_UDF();
        fn HRPWM_3_SCMP();
        fn HRPWM_3_GCAP();
        fn HRPWM_4_GCMP();
        fn HRPWM_4_OVF_UDF();
        fn HRPWM_4_SCMP();
        fn HRPWM_4_GCAP();
        fn EMB_GR4();
        fn EMB_GR5();
        fn EMB_GR6();
        fn EMB_GR7();
        fn EMB_GR8();
        fn HRPWM_5_GCMP();
        fn HRPWM_5_OVF_UDF();
        fn HRPWM_5_SCMP();
        fn HRPWM_5_GCAP();
        fn HRPWM_6_GCMP();
        fn HRPWM_6_OVF_UDF();
        fn HRPWM_6_SCMP();
        fn HRPWM_6_GCAP();
        fn MCANRAM_ECCER();
        fn MCAN1_INT0();
        fn MCAN1_INT1();
        fn MCAN2_INT0();
        fn MCAN2_INT1();
        fn TMR4_GCMP();
        fn TMR4_OVF_UDF();
        fn TMR4_RELOAD();
        fn TMR4_SCMP();
        fn CMP1();
        fn CMP2();
        fn CMP3();
        fn I2C();
        fn USART1();
        fn USART1_TCI();
        fn SPI();
        fn TMRA_5_OVF_UDF();
        fn TMRA_5_CMP();
        fn EVENT_PORT1();
        fn EVENT_PORT2();
        fn EVENT_PORT3();
        fn EVENT_PORT4();
        fn USART2();
        fn USART2_TCI();
        fn USART3();
        fn USART3_TCI();
        fn USART4();
        fn USART4_TCI();
        fn USART1_WUPI();
        fn LVD1();
        fn LVD2();
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
    pub static __INTERRUPTS: [Vector; 146] = [
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
        Vector { _handler: DMA_ERR },
        Vector {
            _handler: DMA_TC0_BTC0,
        },
        Vector {
            _handler: DMA_TC1_BTC1,
        },
        Vector {
            _handler: DMA_TC2_BTC2,
        },
        Vector {
            _handler: DMA_TC3_BTC3,
        },
        Vector {
            _handler: DMA_TC4_BTC4,
        },
        Vector {
            _handler: DMA_TC5_BTC5,
        },
        Vector {
            _handler: DMA_TC6_BTC6,
        },
        Vector {
            _handler: DMA_TC7_BTC7,
        },
        Vector {
            _handler: EFM_PEERR_RDCOL,
        },
        Vector {
            _handler: EFM_OPTEND,
        },
        Vector { _handler: FPU_ERR },
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
            _handler: TMR6_3_GCMP,
        },
        Vector {
            _handler: TMR6_3_OVF_UDF,
        },
        Vector {
            _handler: TMR6_3_DTE,
        },
        Vector {
            _handler: TMR6_3_SCMP,
        },
        Vector {
            _handler: TMRA_3_OVF_UDF,
        },
        Vector {
            _handler: TMRA_3_CMP,
        },
        Vector {
            _handler: TMR6_4_GCMP,
        },
        Vector {
            _handler: TMR6_4_OVF_UDF,
        },
        Vector {
            _handler: TMR6_4_DTE,
        },
        Vector {
            _handler: TMR6_4_SCMP,
        },
        Vector {
            _handler: TMRA_4_OVF_UDF,
        },
        Vector {
            _handler: TMRA_4_CMP,
        },
        Vector {
            _handler: HRPWM_1_GCMP,
        },
        Vector {
            _handler: HRPWM_1_OVF_UDF,
        },
        Vector {
            _handler: HRPWM_1_SCMP,
        },
        Vector {
            _handler: HRPWM_1_GCAP,
        },
        Vector {
            _handler: HRPWM_2_GCMP,
        },
        Vector {
            _handler: HRPWM_2_OVF_UDF,
        },
        Vector {
            _handler: HRPWM_2_SCMP,
        },
        Vector {
            _handler: HRPWM_2_GCAP,
        },
        Vector { _handler: HRPWM },
        Vector { _handler: EMB_GR0 },
        Vector { _handler: EMB_GR1 },
        Vector { _handler: EMB_GR2 },
        Vector { _handler: EMB_GR3 },
        Vector {
            _handler: HRPWM_3_GCMP,
        },
        Vector {
            _handler: HRPWM_3_OVF_UDF,
        },
        Vector {
            _handler: HRPWM_3_SCMP,
        },
        Vector {
            _handler: HRPWM_3_GCAP,
        },
        Vector {
            _handler: HRPWM_4_GCMP,
        },
        Vector {
            _handler: HRPWM_4_OVF_UDF,
        },
        Vector {
            _handler: HRPWM_4_SCMP,
        },
        Vector {
            _handler: HRPWM_4_GCAP,
        },
        Vector { _handler: EMB_GR4 },
        Vector { _handler: EMB_GR5 },
        Vector { _handler: EMB_GR6 },
        Vector { _handler: EMB_GR7 },
        Vector { _handler: EMB_GR8 },
        Vector {
            _handler: HRPWM_5_GCMP,
        },
        Vector {
            _handler: HRPWM_5_OVF_UDF,
        },
        Vector {
            _handler: HRPWM_5_SCMP,
        },
        Vector {
            _handler: HRPWM_5_GCAP,
        },
        Vector {
            _handler: HRPWM_6_GCMP,
        },
        Vector {
            _handler: HRPWM_6_OVF_UDF,
        },
        Vector {
            _handler: HRPWM_6_SCMP,
        },
        Vector {
            _handler: HRPWM_6_GCAP,
        },
        Vector {
            _handler: MCANRAM_ECCER,
        },
        Vector {
            _handler: MCAN1_INT0,
        },
        Vector {
            _handler: MCAN1_INT1,
        },
        Vector {
            _handler: MCAN2_INT0,
        },
        Vector {
            _handler: MCAN2_INT1,
        },
        Vector {
            _handler: TMR4_GCMP,
        },
        Vector {
            _handler: TMR4_OVF_UDF,
        },
        Vector {
            _handler: TMR4_RELOAD,
        },
        Vector {
            _handler: TMR4_SCMP,
        },
        Vector { _handler: CMP1 },
        Vector { _handler: CMP2 },
        Vector { _handler: CMP3 },
        Vector { _handler: I2C },
        Vector { _handler: USART1 },
        Vector {
            _handler: USART1_TCI,
        },
        Vector { _handler: SPI },
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
        Vector { _handler: USART2 },
        Vector {
            _handler: USART2_TCI,
        },
        Vector { _handler: USART3 },
        Vector {
            _handler: USART3_TCI,
        },
        Vector { _handler: USART4 },
        Vector {
            _handler: USART4_TCI,
        },
        Vector { _reserved: 0 },
        Vector {
            _handler: USART1_WUPI,
        },
        Vector { _handler: LVD1 },
        Vector { _handler: LVD2 },
        Vector { _handler: FCM },
        Vector { _handler: WDT },
        Vector { _handler: CTC },
        Vector { _handler: ADC1 },
        Vector { _handler: ADC2 },
        Vector { _handler: ADC3 },
    ];
}
pub const ICG: icg_ll_v1_1::Icg = unsafe { icg_ll_v1_1::Icg::from_ptr(0x0400usize as _) };
pub const CRC: crc_ll_v2::Crc = unsafe { crc_ll_v2::Crc::from_ptr(0x4000_8c00usize as _) };
pub const EFM: efm_ll_v2_2::Efm = unsafe { efm_ll_v2_2::Efm::from_ptr(0x4001_0400usize as _) };
pub const AOS: aos_ll_v7::Aos = unsafe { aos_ll_v7::Aos::from_ptr(0x4001_0800usize as _) };
pub const SPI: spi_ll_v2_5::Spi = unsafe { spi_ll_v2_5::Spi::from_ptr(0x4001_c000usize as _) };
pub const USART1: usart_ll_v3_8::Usart =
    unsafe { usart_ll_v3_8::Usart::from_ptr(0x4001_cc00usize as _) };
pub const USART2: usart_ll_v3_13::Usart =
    unsafe { usart_ll_v3_13::Usart::from_ptr(0x4001_d000usize as _) };
pub const USART3: usart_ll_v3_13::Usart =
    unsafe { usart_ll_v3_13::Usart::from_ptr(0x4001_d000usize as _) };
pub const USART4: usart_ll_v3_13::Usart =
    unsafe { usart_ll_v3_13::Usart::from_ptr(0x4001_d000usize as _) };
pub const TMR01: tmr0_ll_v2_5::Tmr0 =
    unsafe { tmr0_ll_v2_5::Tmr0::from_ptr(0x4002_4000usize as _) };
pub const TMR02: tmr0_ll_v2_5::Tmr0 =
    unsafe { tmr0_ll_v2_5::Tmr0::from_ptr(0x4002_4000usize as _) };
pub const TMR4: tmr4_ll_v2_1::Tmr4 = unsafe { tmr4_ll_v2_1::Tmr4::from_ptr(0x4003_8000usize as _) };
pub const CMP1: cmp_ll_v3_5::Cmp = unsafe { cmp_ll_v3_5::Cmp::from_ptr(0x4003_8400usize as _) };
pub const CMP2: cmp_ll_v3_5::Cmp = unsafe { cmp_ll_v3_5::Cmp::from_ptr(0x4003_8400usize as _) };
pub const CMP3: cmp_ll_v3_5::Cmp = unsafe { cmp_ll_v3_5::Cmp::from_ptr(0x4003_8400usize as _) };
pub const EMB0: emb_ll_v2_11::Emb = unsafe { emb_ll_v2_11::Emb::from_ptr(0x4003_8600usize as _) };
pub const EMB1: emb_ll_v2_11::Emb = unsafe { emb_ll_v2_11::Emb::from_ptr(0x4003_8600usize as _) };
pub const EMB2: emb_ll_v2_11::Emb = unsafe { emb_ll_v2_11::Emb::from_ptr(0x4003_8600usize as _) };
pub const EMB3: emb_ll_v2_11::Emb = unsafe { emb_ll_v2_11::Emb::from_ptr(0x4003_8600usize as _) };
pub const EMB4: emb_ll_v2_11::Emb = unsafe { emb_ll_v2_11::Emb::from_ptr(0x4003_8600usize as _) };
pub const EMB5: emb_ll_v2_11::Emb = unsafe { emb_ll_v2_11::Emb::from_ptr(0x4003_8600usize as _) };
pub const EMB6: emb_ll_v2_10::Emb = unsafe { emb_ll_v2_10::Emb::from_ptr(0x4003_86c0usize as _) };
pub const EMB7: emb_ll_v2_10::Emb = unsafe { emb_ll_v2_10::Emb::from_ptr(0x4003_86c0usize as _) };
pub const EMB8: emb_ll_v2_10::Emb = unsafe { emb_ll_v2_10::Emb::from_ptr(0x4003_86c0usize as _) };
pub const TMR61: tmr_v30::Tmr = unsafe { tmr_v30::Tmr::from_ptr(0x4003_9000usize as _) };
pub const TMR62: tmr_v30::Tmr = unsafe { tmr_v30::Tmr::from_ptr(0x4003_9000usize as _) };
pub const TMR63: tmr_v30::Tmr = unsafe { tmr_v30::Tmr::from_ptr(0x4003_9000usize as _) };
pub const TMR64: tmr_v30::Tmr = unsafe { tmr_v30::Tmr::from_ptr(0x4003_9000usize as _) };
pub const TMR6_COMMON: tmr_v31::Tmr = unsafe { tmr_v31::Tmr::from_ptr(0x4003_9300usize as _) };
pub const TMRA1: tmra_ll_v2_8::Tmra =
    unsafe { tmra_ll_v2_8::Tmra::from_ptr(0x4003_a000usize as _) };
pub const TMRA2: tmra_ll_v2_9::Tmra =
    unsafe { tmra_ll_v2_9::Tmra::from_ptr(0x4003_a400usize as _) };
pub const TMRA3: tmra_ll_v2_9::Tmra =
    unsafe { tmra_ll_v2_9::Tmra::from_ptr(0x4003_a400usize as _) };
pub const TMRA4: tmra_ll_v2_9::Tmra =
    unsafe { tmra_ll_v2_9::Tmra::from_ptr(0x4003_a400usize as _) };
pub const TMRA5: tmra_ll_v2_9::Tmra =
    unsafe { tmra_ll_v2_9::Tmra::from_ptr(0x4003_a400usize as _) };
pub const I2C: i2c_ll_v4_2::I2c = unsafe { i2c_ll_v4_2::I2c::from_ptr(0x4003_b400usize as _) };
pub const HRPWM1: hrpwm_v2::Hrpwm = unsafe { hrpwm_v2::Hrpwm::from_ptr(0x4003_c000usize as _) };
pub const HRPWM_COMMON: hrpwm_v4::Hrpwm =
    unsafe { hrpwm_v4::Hrpwm::from_ptr(0x4003_c2f0usize as _) };
pub const HRPWM2: hrpwm_v3::Hrpwm = unsafe { hrpwm_v3::Hrpwm::from_ptr(0x4003_c400usize as _) };
pub const HRPWM3: hrpwm_v3::Hrpwm = unsafe { hrpwm_v3::Hrpwm::from_ptr(0x4003_c400usize as _) };
pub const HRPWM4: hrpwm_v3::Hrpwm = unsafe { hrpwm_v3::Hrpwm::from_ptr(0x4003_c400usize as _) };
pub const HRPWM5: hrpwm_v3::Hrpwm = unsafe { hrpwm_v3::Hrpwm::from_ptr(0x4003_c400usize as _) };
pub const HRPWM6: hrpwm_v3::Hrpwm = unsafe { hrpwm_v3::Hrpwm::from_ptr(0x4003_c400usize as _) };
pub const ADC1: adc_ll_v3_6::Adc = unsafe { adc_ll_v3_6::Adc::from_ptr(0x4004_0000usize as _) };
pub const ADC2: adc_ll_v3_5::Adc = unsafe { adc_ll_v3_5::Adc::from_ptr(0x4004_0400usize as _) };
pub const ADC3: adc_ll_v3_2::Adc = unsafe { adc_ll_v3_2::Adc::from_ptr(0x4004_0800usize as _) };
pub const DAC1: dac_ll_v2::Dac = unsafe { dac_ll_v2::Dac::from_ptr(0x4004_1000usize as _) };
pub const DAC2: dac_ll_v2::Dac = unsafe { dac_ll_v2::Dac::from_ptr(0x4004_1000usize as _) };
pub const CMU: cmu_ll_v3_1::Cmu = unsafe { cmu_ll_v3_1::Cmu::from_ptr(0x4004_8000usize as _) };
pub const PWC: pwc_ll_v3_1::Pwc = unsafe { pwc_ll_v3_1::Pwc::from_ptr(0x4004_8000usize as _) };
pub const FCM: fcm_ll_v1_1::Fcm = unsafe { fcm_ll_v1_1::Fcm::from_ptr(0x4004_8400usize as _) };
pub const WDT: wdt_ll_v1::Wdt = unsafe { wdt_ll_v1::Wdt::from_ptr(0x4004_9000usize as _) };
pub const SWDT: swdt_ll_v1_1::Swdt = unsafe { swdt_ll_v1_1::Swdt::from_ptr(0x4004_9400usize as _) };
pub const CTC: ctc_ll_v1_1::Ctc = unsafe { ctc_ll_v1_1::Ctc::from_ptr(0x4004_9c00usize as _) };
pub const RTC: rtc_ll_v1::Rtc = unsafe { rtc_ll_v1::Rtc::from_ptr(0x4004_c000usize as _) };
pub const RMU: rmu_ll_v3_2::Rmu = unsafe { rmu_ll_v3_2::Rmu::from_ptr(0x4004_cce0usize as _) };
pub const PLA: pla_v2::Pla = unsafe { pla_v2::Pla::from_ptr(0x4004_fc00usize as _) };
pub const MPU: mpu_v6::Mpu = unsafe { mpu_v6::Mpu::from_ptr(0x4005_0000usize as _) };
pub const SRAMC: sramc_ll_v3_3::Sramc =
    unsafe { sramc_ll_v3_3::Sramc::from_ptr(0x4005_0800usize as _) };
pub const INTC: intc_ll_v2_7::Intc = unsafe { intc_ll_v2_7::Intc::from_ptr(0x4005_1000usize as _) };
pub const DMA: dma_ll_v2_4::Dma = unsafe { dma_ll_v2_4::Dma::from_ptr(0x4005_3000usize as _) };
pub const GPIO: gpio_ll_v2_5::Gpio = unsafe { gpio_ll_v2_5::Gpio::from_ptr(0x4005_3800usize as _) };
pub const PERIC: peric_ll_v1_3::Peric =
    unsafe { peric_ll_v1_3::Peric::from_ptr(0x4005_5400usize as _) };
pub const MCAN1: mcan_ll_v1_1::Mcan =
    unsafe { mcan_ll_v1_1::Mcan::from_ptr(0x4007_1000usize as _) };
pub const MCAN2: mcan_ll_v1_1::Mcan =
    unsafe { mcan_ll_v1_1::Mcan::from_ptr(0x4007_1000usize as _) };
pub const MCANRAM: mcanram_ll_v1_1::Mcanram =
    unsafe { mcanram_ll_v1_1::Mcanram::from_ptr(0x4007_3800usize as _) };
pub const DBGC: dbgc_ll_v3_3::Dbgc = unsafe { dbgc_ll_v3_3::Dbgc::from_ptr(0xe004_2000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ll_v3_2.rs"]
pub mod adc_ll_v3_2;
#[path = "../../peripherals/adc_ll_v3_5.rs"]
pub mod adc_ll_v3_5;
#[path = "../../peripherals/adc_ll_v3_6.rs"]
pub mod adc_ll_v3_6;
#[path = "../../peripherals/aos_ll_v7.rs"]
pub mod aos_ll_v7;
#[path = "../../peripherals/cmp_ll_v3_5.rs"]
pub mod cmp_ll_v3_5;
#[path = "../../peripherals/cmu_ll_v3_1.rs"]
pub mod cmu_ll_v3_1;
#[path = "../../peripherals/crc_ll_v2.rs"]
pub mod crc_ll_v2;
#[path = "../../peripherals/ctc_ll_v1_1.rs"]
pub mod ctc_ll_v1_1;
#[path = "../../peripherals/dac_ll_v2.rs"]
pub mod dac_ll_v2;
#[path = "../../peripherals/dbgc_ll_v3_3.rs"]
pub mod dbgc_ll_v3_3;
#[path = "../../peripherals/dma_ll_v2_4.rs"]
pub mod dma_ll_v2_4;
#[path = "../../peripherals/efm_ll_v2_2.rs"]
pub mod efm_ll_v2_2;
#[path = "../../peripherals/emb_ll_v2_10.rs"]
pub mod emb_ll_v2_10;
#[path = "../../peripherals/emb_ll_v2_11.rs"]
pub mod emb_ll_v2_11;
#[path = "../../peripherals/fcm_ll_v1_1.rs"]
pub mod fcm_ll_v1_1;
#[path = "../../peripherals/gpio_ll_v2_5.rs"]
pub mod gpio_ll_v2_5;
#[path = "../../peripherals/hrpwm_v2.rs"]
pub mod hrpwm_v2;
#[path = "../../peripherals/hrpwm_v3.rs"]
pub mod hrpwm_v3;
#[path = "../../peripherals/hrpwm_v4.rs"]
pub mod hrpwm_v4;
#[path = "../../peripherals/i2c_ll_v4_2.rs"]
pub mod i2c_ll_v4_2;
#[path = "../../peripherals/icg_ll_v1_1.rs"]
pub mod icg_ll_v1_1;
#[path = "../../peripherals/intc_ll_v2_7.rs"]
pub mod intc_ll_v2_7;
#[path = "../../peripherals/mcan_ll_v1_1.rs"]
pub mod mcan_ll_v1_1;
#[path = "../../peripherals/mcanram_ll_v1_1.rs"]
pub mod mcanram_ll_v1_1;
#[path = "../../peripherals/mpu_v6.rs"]
pub mod mpu_v6;
#[path = "../../peripherals/peric_ll_v1_3.rs"]
pub mod peric_ll_v1_3;
#[path = "../../peripherals/pla_v2.rs"]
pub mod pla_v2;
#[path = "../../peripherals/pwc_ll_v3_1.rs"]
pub mod pwc_ll_v3_1;
#[path = "../../peripherals/rmu_ll_v3_2.rs"]
pub mod rmu_ll_v3_2;
#[path = "../../peripherals/rtc_ll_v1.rs"]
pub mod rtc_ll_v1;
#[path = "../../peripherals/spi_ll_v2_5.rs"]
pub mod spi_ll_v2_5;
#[path = "../../peripherals/sramc_ll_v3_3.rs"]
pub mod sramc_ll_v3_3;
#[path = "../../peripherals/swdt_ll_v1_1.rs"]
pub mod swdt_ll_v1_1;
#[path = "../../peripherals/tmr0_ll_v2_5.rs"]
pub mod tmr0_ll_v2_5;
#[path = "../../peripherals/tmr4_ll_v2_1.rs"]
pub mod tmr4_ll_v2_1;
#[path = "../../peripherals/tmr_v30.rs"]
pub mod tmr_v30;
#[path = "../../peripherals/tmr_v31.rs"]
pub mod tmr_v31;
#[path = "../../peripherals/tmra_ll_v2_8.rs"]
pub mod tmra_ll_v2_8;
#[path = "../../peripherals/tmra_ll_v2_9.rs"]
pub mod tmra_ll_v2_9;
#[path = "../../peripherals/usart_ll_v3_13.rs"]
pub mod usart_ll_v3_13;
#[path = "../../peripherals/usart_ll_v3_8.rs"]
pub mod usart_ll_v3_8;
#[path = "../../peripherals/wdt_ll_v1.rs"]
pub mod wdt_ll_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
