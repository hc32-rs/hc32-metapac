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
    #[doc = "39 - DMA1_TC6_BTC6"]
    DMA1_TC6_BTC6 = 39,
    #[doc = "40 - DMA1_TC7_BTC7"]
    DMA1_TC7_BTC7 = 40,
    #[doc = "41 - EFM_PEERR_RDCOL"]
    EFM_PEERR_RDCOL = 41,
    #[doc = "42 - EFM_OPTEND"]
    EFM_OPTEND = 42,
    #[doc = "43 - QSPI"]
    QSPI = 43,
    #[doc = "44 - DCU1"]
    DCU1 = 44,
    #[doc = "45 - DCU2"]
    DCU2 = 45,
    #[doc = "46 - DCU3"]
    DCU3 = 46,
    #[doc = "47 - DMA2_ERR"]
    DMA2_ERR = 47,
    #[doc = "48 - DMA2_TC0_BTC0"]
    DMA2_TC0_BTC0 = 48,
    #[doc = "49 - DMA2_TC1_BTC1"]
    DMA2_TC1_BTC1 = 49,
    #[doc = "50 - DMA2_TC2_BTC2"]
    DMA2_TC2_BTC2 = 50,
    #[doc = "51 - DMA2_TC3_BTC3"]
    DMA2_TC3_BTC3 = 51,
    #[doc = "52 - DMA2_TC4_BTC4"]
    DMA2_TC4_BTC4 = 52,
    #[doc = "53 - DMA2_TC5_BTC5"]
    DMA2_TC5_BTC5 = 53,
    #[doc = "54 - DMA2_TC6_BTC6"]
    DMA2_TC6_BTC6 = 54,
    #[doc = "55 - DMA2_TC7_BTC7"]
    DMA2_TC7_BTC7 = 55,
    #[doc = "56 - MAU"]
    MAU = 56,
    #[doc = "57 - FMAC1_FIR"]
    FMAC1_FIR = 57,
    #[doc = "58 - FMAC2_FIR"]
    FMAC2_FIR = 58,
    #[doc = "59 - FMAC3_FIR"]
    FMAC3_FIR = 59,
    #[doc = "60 - FMAC4_FIR"]
    FMAC4_FIR = 60,
    #[doc = "61 - TMR0_1_CMP"]
    TMR0_1_CMP = 61,
    #[doc = "62 - TMR0_2_CMP"]
    TMR0_2_CMP = 62,
    #[doc = "63 - TMR2_1_CMP"]
    TMR2_1_CMP = 63,
    #[doc = "64 - TMR2_1_OVF"]
    TMR2_1_OVF = 64,
    #[doc = "65 - TMR2_2_CMP"]
    TMR2_2_CMP = 65,
    #[doc = "66 - TMR2_2_OVF"]
    TMR2_2_OVF = 66,
    #[doc = "67 - TMR2_3_CMP"]
    TMR2_3_CMP = 67,
    #[doc = "68 - TMR2_3_OVF"]
    TMR2_3_OVF = 68,
    #[doc = "69 - TMR2_4_CMP"]
    TMR2_4_CMP = 69,
    #[doc = "70 - TMR2_4_OVF"]
    TMR2_4_OVF = 70,
    #[doc = "71 - RTC"]
    RTC = 71,
    #[doc = "72 - XTAL"]
    XTAL = 72,
    #[doc = "73 - WKTM"]
    WKTM = 73,
    #[doc = "74 - SWDT"]
    SWDT = 74,
    #[doc = "75 - TMR6_1_GCMP"]
    TMR6_1_GCMP = 75,
    #[doc = "76 - TMR6_1_OVF_UDF"]
    TMR6_1_OVF_UDF = 76,
    #[doc = "77 - TMR6_1_DTE"]
    TMR6_1_DTE = 77,
    #[doc = "78 - TMR6_1_SCMP"]
    TMR6_1_SCMP = 78,
    #[doc = "79 - TMRA_1_OVF_UDF"]
    TMRA_1_OVF_UDF = 79,
    #[doc = "80 - TMRA_1_CMP"]
    TMRA_1_CMP = 80,
    #[doc = "81 - TMR6_2_GCMP"]
    TMR6_2_GCMP = 81,
    #[doc = "82 - TMR6_2_OVF_UDF"]
    TMR6_2_OVF_UDF = 82,
    #[doc = "83 - TMR6_2_DTE"]
    TMR6_2_DTE = 83,
    #[doc = "84 - TMR6_2_SCMP"]
    TMR6_2_SCMP = 84,
    #[doc = "85 - TMRA_2_OVF_UDF"]
    TMRA_2_OVF_UDF = 85,
    #[doc = "86 - TMRA_2_CMP"]
    TMRA_2_CMP = 86,
    #[doc = "87 - TMR6_3_GCMP"]
    TMR6_3_GCMP = 87,
    #[doc = "88 - TMR6_3_OVF_UDF"]
    TMR6_3_OVF_UDF = 88,
    #[doc = "89 - TMR6_3_DTE"]
    TMR6_3_DTE = 89,
    #[doc = "90 - TMR6_3_SCMP"]
    TMR6_3_SCMP = 90,
    #[doc = "91 - TMRA_3_OVF_UDF"]
    TMRA_3_OVF_UDF = 91,
    #[doc = "92 - TMRA_3_CMP"]
    TMRA_3_CMP = 92,
    #[doc = "93 - TMR6_4_GCMP"]
    TMR6_4_GCMP = 93,
    #[doc = "94 - TMR6_4_OVF_UDF"]
    TMR6_4_OVF_UDF = 94,
    #[doc = "95 - TMR6_4_DTE"]
    TMR6_4_DTE = 95,
    #[doc = "96 - TMR6_4_SCMP"]
    TMR6_4_SCMP = 96,
    #[doc = "97 - TMRA_4_OVF_UDF"]
    TMRA_4_OVF_UDF = 97,
    #[doc = "98 - TMRA_4_CMP"]
    TMRA_4_CMP = 98,
    #[doc = "99 - TMR6_5_GCMP"]
    TMR6_5_GCMP = 99,
    #[doc = "100 - TMR6_5_OVF_UDF"]
    TMR6_5_OVF_UDF = 100,
    #[doc = "101 - TMR6_5_DTE"]
    TMR6_5_DTE = 101,
    #[doc = "102 - TMR6_5_SCMP"]
    TMR6_5_SCMP = 102,
    #[doc = "104 - TMR6_6_GCMP"]
    TMR6_6_GCMP = 104,
    #[doc = "105 - TMR6_6_OVF_UDF"]
    TMR6_6_OVF_UDF = 105,
    #[doc = "106 - TMR6_6_DTE"]
    TMR6_6_DTE = 106,
    #[doc = "107 - TMR6_6_SCMP"]
    TMR6_6_SCMP = 107,
    #[doc = "108 - TMR6_7_GCMP"]
    TMR6_7_GCMP = 108,
    #[doc = "109 - TMR6_7_OVF_UDF"]
    TMR6_7_OVF_UDF = 109,
    #[doc = "110 - TMR6_7_DTE"]
    TMR6_7_DTE = 110,
    #[doc = "111 - TMR6_7_SCMP"]
    TMR6_7_SCMP = 111,
    #[doc = "112 - TMR6_8_GCMP"]
    TMR6_8_GCMP = 112,
    #[doc = "113 - TMR6_8_OVF_UDF"]
    TMR6_8_OVF_UDF = 113,
    #[doc = "114 - TMR6_8_DTE"]
    TMR6_8_DTE = 114,
    #[doc = "115 - TMR6_8_SCMP"]
    TMR6_8_SCMP = 115,
    #[doc = "116 - TMR6_9_GCMP"]
    TMR6_9_GCMP = 116,
    #[doc = "117 - TMR6_9_OVF_UDF"]
    TMR6_9_OVF_UDF = 117,
    #[doc = "118 - TMR6_9_DTE"]
    TMR6_9_DTE = 118,
    #[doc = "119 - TMR6_9_SCMP"]
    TMR6_9_SCMP = 119,
    #[doc = "120 - TMR6_10_GCMP"]
    TMR6_10_GCMP = 120,
    #[doc = "121 - TMR6_10_OVF_UDF"]
    TMR6_10_OVF_UDF = 121,
    #[doc = "122 - TMR6_10_DTE"]
    TMR6_10_DTE = 122,
    #[doc = "123 - TMR6_10_SCMP"]
    TMR6_10_SCMP = 123,
    #[doc = "124 - TMR4_1_GCMP"]
    TMR4_1_GCMP = 124,
    #[doc = "125 - TMR4_1_OVF_UDF"]
    TMR4_1_OVF_UDF = 125,
    #[doc = "126 - TMR4_1_RELOAD"]
    TMR4_1_RELOAD = 126,
    #[doc = "127 - TMR4_1_SCMP"]
    TMR4_1_SCMP = 127,
    #[doc = "128 - CMP1"]
    CMP1 = 128,
    #[doc = "129 - CMP2"]
    CMP2 = 129,
    #[doc = "130 - CMP3"]
    CMP3 = 130,
    #[doc = "131 - CMP4"]
    CMP4 = 131,
    #[doc = "132 - CAN1_INT"]
    CAN1_INT = 132,
    #[doc = "133 - CAN2_INT"]
    CAN2_INT = 133,
    #[doc = "134 - CAN3_INT"]
    CAN3_INT = 134,
    #[doc = "135 - USART1"]
    USART1 = 135,
    #[doc = "136 - USART2"]
    USART2 = 136,
    #[doc = "137 - SPI1"]
    SPI1 = 137,
    #[doc = "138 - SPI2"]
    SPI2 = 138,
    #[doc = "139 - TMRA_5_OVF_UDF"]
    TMRA_5_OVF_UDF = 139,
    #[doc = "140 - TMRA_5_CMP"]
    TMRA_5_CMP = 140,
    #[doc = "141 - TMRA_6_OVF_UDF"]
    TMRA_6_OVF_UDF = 141,
    #[doc = "142 - TMRA_6_CMP"]
    TMRA_6_CMP = 142,
    #[doc = "143 - USBFS_GLB"]
    USBFS_GLB = 143,
    #[doc = "144 - EVENT_PORT1"]
    EVENT_PORT1 = 144,
    #[doc = "145 - EVENT_PORT2"]
    EVENT_PORT2 = 145,
    #[doc = "146 - EVENT_PORT3"]
    EVENT_PORT3 = 146,
    #[doc = "147 - EVENT_PORT4"]
    EVENT_PORT4 = 147,
    #[doc = "148 - USART3"]
    USART3 = 148,
    #[doc = "149 - USART4"]
    USART4 = 149,
    #[doc = "150 - SPI3"]
    SPI3 = 150,
    #[doc = "151 - SPI4"]
    SPI4 = 151,
    #[doc = "152 - EMB_GR0"]
    EMB_GR0 = 152,
    #[doc = "153 - EMB_GR1"]
    EMB_GR1 = 153,
    #[doc = "154 - EMB_GR2"]
    EMB_GR2 = 154,
    #[doc = "155 - EMB_GR3"]
    EMB_GR3 = 155,
    #[doc = "156 - EMB_GR4"]
    EMB_GR4 = 156,
    #[doc = "157 - EMB_GR5"]
    EMB_GR5 = 157,
    #[doc = "158 - EMB_GR6"]
    EMB_GR6 = 158,
    #[doc = "159 - HASH"]
    HASH = 159,
    #[doc = "160 - USART5"]
    USART5 = 160,
    #[doc = "161 - USART6"]
    USART6 = 161,
    #[doc = "162 - MDIO"]
    MDIO = 162,
    #[doc = "164 - I2C1"]
    I2C1 = 164,
    #[doc = "165 - I2C2"]
    I2C2 = 165,
    #[doc = "166 - I2C3"]
    I2C3 = 166,
    #[doc = "167 - USART1_WUPI"]
    USART1_WUPI = 167,
    #[doc = "168 - LVD1"]
    LVD1 = 168,
    #[doc = "169 - LVD2"]
    LVD2 = 169,
    #[doc = "170 - OTS"]
    OTS = 170,
    #[doc = "171 - FCM"]
    FCM = 171,
    #[doc = "172 - WDT"]
    WDT = 172,
    #[doc = "173 - CTC"]
    CTC = 173,
    #[doc = "174 - ADC1"]
    ADC1 = 174,
    #[doc = "175 - ADC2"]
    ADC2 = 175,
    #[doc = "176 - ADC3"]
    ADC3 = 176,
    #[doc = "177 - TRNG"]
    TRNG = 177,
    #[doc = "178 - USART1_TCI"]
    USART1_TCI = 178,
    #[doc = "179 - USART2_TCI"]
    USART2_TCI = 179,
    #[doc = "180 - USART3_TCI"]
    USART3_TCI = 180,
    #[doc = "181 - USART4_TCI"]
    USART4_TCI = 181,
    #[doc = "182 - USART5_TCI"]
    USART5_TCI = 182,
    #[doc = "183 - USART6_TCI"]
    USART6_TCI = 183,
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
        fn DMA1_TC6_BTC6();
        fn DMA1_TC7_BTC7();
        fn EFM_PEERR_RDCOL();
        fn EFM_OPTEND();
        fn QSPI();
        fn DCU1();
        fn DCU2();
        fn DCU3();
        fn DMA2_ERR();
        fn DMA2_TC0_BTC0();
        fn DMA2_TC1_BTC1();
        fn DMA2_TC2_BTC2();
        fn DMA2_TC3_BTC3();
        fn DMA2_TC4_BTC4();
        fn DMA2_TC5_BTC5();
        fn DMA2_TC6_BTC6();
        fn DMA2_TC7_BTC7();
        fn MAU();
        fn FMAC1_FIR();
        fn FMAC2_FIR();
        fn FMAC3_FIR();
        fn FMAC4_FIR();
        fn TMR0_1_CMP();
        fn TMR0_2_CMP();
        fn TMR2_1_CMP();
        fn TMR2_1_OVF();
        fn TMR2_2_CMP();
        fn TMR2_2_OVF();
        fn TMR2_3_CMP();
        fn TMR2_3_OVF();
        fn TMR2_4_CMP();
        fn TMR2_4_OVF();
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
        fn TMR6_5_GCMP();
        fn TMR6_5_OVF_UDF();
        fn TMR6_5_DTE();
        fn TMR6_5_SCMP();
        fn TMR6_6_GCMP();
        fn TMR6_6_OVF_UDF();
        fn TMR6_6_DTE();
        fn TMR6_6_SCMP();
        fn TMR6_7_GCMP();
        fn TMR6_7_OVF_UDF();
        fn TMR6_7_DTE();
        fn TMR6_7_SCMP();
        fn TMR6_8_GCMP();
        fn TMR6_8_OVF_UDF();
        fn TMR6_8_DTE();
        fn TMR6_8_SCMP();
        fn TMR6_9_GCMP();
        fn TMR6_9_OVF_UDF();
        fn TMR6_9_DTE();
        fn TMR6_9_SCMP();
        fn TMR6_10_GCMP();
        fn TMR6_10_OVF_UDF();
        fn TMR6_10_DTE();
        fn TMR6_10_SCMP();
        fn TMR4_1_GCMP();
        fn TMR4_1_OVF_UDF();
        fn TMR4_1_RELOAD();
        fn TMR4_1_SCMP();
        fn CMP1();
        fn CMP2();
        fn CMP3();
        fn CMP4();
        fn CAN1_INT();
        fn CAN2_INT();
        fn CAN3_INT();
        fn USART1();
        fn USART2();
        fn SPI1();
        fn SPI2();
        fn TMRA_5_OVF_UDF();
        fn TMRA_5_CMP();
        fn TMRA_6_OVF_UDF();
        fn TMRA_6_CMP();
        fn USBFS_GLB();
        fn EVENT_PORT1();
        fn EVENT_PORT2();
        fn EVENT_PORT3();
        fn EVENT_PORT4();
        fn USART3();
        fn USART4();
        fn SPI3();
        fn SPI4();
        fn EMB_GR0();
        fn EMB_GR1();
        fn EMB_GR2();
        fn EMB_GR3();
        fn EMB_GR4();
        fn EMB_GR5();
        fn EMB_GR6();
        fn HASH();
        fn USART5();
        fn USART6();
        fn MDIO();
        fn I2C1();
        fn I2C2();
        fn I2C3();
        fn USART1_WUPI();
        fn LVD1();
        fn LVD2();
        fn OTS();
        fn FCM();
        fn WDT();
        fn CTC();
        fn ADC1();
        fn ADC2();
        fn ADC3();
        fn TRNG();
        fn USART1_TCI();
        fn USART2_TCI();
        fn USART3_TCI();
        fn USART4_TCI();
        fn USART5_TCI();
        fn USART6_TCI();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 184] = [
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
            _handler: DMA1_TC6_BTC6,
        },
        Vector {
            _handler: DMA1_TC7_BTC7,
        },
        Vector {
            _handler: EFM_PEERR_RDCOL,
        },
        Vector {
            _handler: EFM_OPTEND,
        },
        Vector { _handler: QSPI },
        Vector { _handler: DCU1 },
        Vector { _handler: DCU2 },
        Vector { _handler: DCU3 },
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
        Vector {
            _handler: DMA2_TC6_BTC6,
        },
        Vector {
            _handler: DMA2_TC7_BTC7,
        },
        Vector { _handler: MAU },
        Vector {
            _handler: FMAC1_FIR,
        },
        Vector {
            _handler: FMAC2_FIR,
        },
        Vector {
            _handler: FMAC3_FIR,
        },
        Vector {
            _handler: FMAC4_FIR,
        },
        Vector {
            _handler: TMR0_1_CMP,
        },
        Vector {
            _handler: TMR0_2_CMP,
        },
        Vector {
            _handler: TMR2_1_CMP,
        },
        Vector {
            _handler: TMR2_1_OVF,
        },
        Vector {
            _handler: TMR2_2_CMP,
        },
        Vector {
            _handler: TMR2_2_OVF,
        },
        Vector {
            _handler: TMR2_3_CMP,
        },
        Vector {
            _handler: TMR2_3_OVF,
        },
        Vector {
            _handler: TMR2_4_CMP,
        },
        Vector {
            _handler: TMR2_4_OVF,
        },
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
            _handler: TMR6_5_GCMP,
        },
        Vector {
            _handler: TMR6_5_OVF_UDF,
        },
        Vector {
            _handler: TMR6_5_DTE,
        },
        Vector {
            _handler: TMR6_5_SCMP,
        },
        Vector { _reserved: 0 },
        Vector {
            _handler: TMR6_6_GCMP,
        },
        Vector {
            _handler: TMR6_6_OVF_UDF,
        },
        Vector {
            _handler: TMR6_6_DTE,
        },
        Vector {
            _handler: TMR6_6_SCMP,
        },
        Vector {
            _handler: TMR6_7_GCMP,
        },
        Vector {
            _handler: TMR6_7_OVF_UDF,
        },
        Vector {
            _handler: TMR6_7_DTE,
        },
        Vector {
            _handler: TMR6_7_SCMP,
        },
        Vector {
            _handler: TMR6_8_GCMP,
        },
        Vector {
            _handler: TMR6_8_OVF_UDF,
        },
        Vector {
            _handler: TMR6_8_DTE,
        },
        Vector {
            _handler: TMR6_8_SCMP,
        },
        Vector {
            _handler: TMR6_9_GCMP,
        },
        Vector {
            _handler: TMR6_9_OVF_UDF,
        },
        Vector {
            _handler: TMR6_9_DTE,
        },
        Vector {
            _handler: TMR6_9_SCMP,
        },
        Vector {
            _handler: TMR6_10_GCMP,
        },
        Vector {
            _handler: TMR6_10_OVF_UDF,
        },
        Vector {
            _handler: TMR6_10_DTE,
        },
        Vector {
            _handler: TMR6_10_SCMP,
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
        Vector { _handler: CMP1 },
        Vector { _handler: CMP2 },
        Vector { _handler: CMP3 },
        Vector { _handler: CMP4 },
        Vector { _handler: CAN1_INT },
        Vector { _handler: CAN2_INT },
        Vector { _handler: CAN3_INT },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector {
            _handler: TMRA_5_OVF_UDF,
        },
        Vector {
            _handler: TMRA_5_CMP,
        },
        Vector {
            _handler: TMRA_6_OVF_UDF,
        },
        Vector {
            _handler: TMRA_6_CMP,
        },
        Vector {
            _handler: USBFS_GLB,
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
        Vector { _handler: USART4 },
        Vector { _handler: SPI3 },
        Vector { _handler: SPI4 },
        Vector { _handler: EMB_GR0 },
        Vector { _handler: EMB_GR1 },
        Vector { _handler: EMB_GR2 },
        Vector { _handler: EMB_GR3 },
        Vector { _handler: EMB_GR4 },
        Vector { _handler: EMB_GR5 },
        Vector { _handler: EMB_GR6 },
        Vector { _handler: HASH },
        Vector { _handler: USART5 },
        Vector { _handler: USART6 },
        Vector { _handler: MDIO },
        Vector { _reserved: 0 },
        Vector { _handler: I2C1 },
        Vector { _handler: I2C2 },
        Vector { _handler: I2C3 },
        Vector {
            _handler: USART1_WUPI,
        },
        Vector { _handler: LVD1 },
        Vector { _handler: LVD2 },
        Vector { _handler: OTS },
        Vector { _handler: FCM },
        Vector { _handler: WDT },
        Vector { _handler: CTC },
        Vector { _handler: ADC1 },
        Vector { _handler: ADC2 },
        Vector { _handler: ADC3 },
        Vector { _handler: TRNG },
        Vector {
            _handler: USART1_TCI,
        },
        Vector {
            _handler: USART2_TCI,
        },
        Vector {
            _handler: USART3_TCI,
        },
        Vector {
            _handler: USART4_TCI,
        },
        Vector {
            _handler: USART5_TCI,
        },
        Vector {
            _handler: USART6_TCI,
        },
    ];
}
pub const EMB_CTL1: emb_ctl1_ll_v1::EmbCtl1 =
    unsafe { emb_ctl1_ll_v1::EmbCtl1::from_ptr(0x0usize as _) };
pub const ICG: icg_ll_v1_3::Icg = unsafe { icg_ll_v1_3::Icg::from_ptr(0x0400usize as _) };
pub const AES: aes_ll_v2::Aes = unsafe { aes_ll_v2::Aes::from_ptr(0x4000_8000usize as _) };
pub const HASH: hash_v2::Hash = unsafe { hash_v2::Hash::from_ptr(0x4000_8400usize as _) };
pub const CRC: crc_ll_v2_1::Crc = unsafe { crc_ll_v2_1::Crc::from_ptr(0x4000_8c00usize as _) };
pub const EFM: efm_ll_v2_4::Efm = unsafe { efm_ll_v2_4::Efm::from_ptr(0x4001_0400usize as _) };
pub const OTS: ots_v2::Ots = unsafe { ots_v2::Ots::from_ptr(0x4001_0600usize as _) };
pub const AOS: aos_ll_v8::Aos = unsafe { aos_ll_v8::Aos::from_ptr(0x4001_0800usize as _) };
pub const EMB0: emb_ll_v2_9::Emb = unsafe { emb_ll_v2_9::Emb::from_ptr(0x4001_7c00usize as _) };
pub const EMB1: emb_ll_v2_9::Emb = unsafe { emb_ll_v2_9::Emb::from_ptr(0x4001_7c00usize as _) };
pub const EMB2: emb_ll_v2_9::Emb = unsafe { emb_ll_v2_9::Emb::from_ptr(0x4001_7c00usize as _) };
pub const EMB3: emb_ll_v2_9::Emb = unsafe { emb_ll_v2_9::Emb::from_ptr(0x4001_7c00usize as _) };
pub const EMB4: emb_ll_v2_9::Emb = unsafe { emb_ll_v2_9::Emb::from_ptr(0x4001_7c00usize as _) };
pub const EMB5: emb_ll_v2_9::Emb = unsafe { emb_ll_v2_9::Emb::from_ptr(0x4001_7c00usize as _) };
pub const EMB6: emb_ll_v2_9::Emb = unsafe { emb_ll_v2_9::Emb::from_ptr(0x4001_7c00usize as _) };
pub const SPI1: spi_ll_v2_3::Spi = unsafe { spi_ll_v2_3::Spi::from_ptr(0x4001_c000usize as _) };
pub const SPI2: spi_ll_v2_3::Spi = unsafe { spi_ll_v2_3::Spi::from_ptr(0x4001_c000usize as _) };
pub const SPI3: spi_ll_v2_3::Spi = unsafe { spi_ll_v2_3::Spi::from_ptr(0x4001_c000usize as _) };
pub const SPI4: spi_ll_v2_3::Spi = unsafe { spi_ll_v2_3::Spi::from_ptr(0x4001_c000usize as _) };
pub const USART1: usart_ll_v2_5::Usart =
    unsafe { usart_ll_v2_5::Usart::from_ptr(0x4001_cc00usize as _) };
pub const USART2: usart_ll_v2_6::Usart =
    unsafe { usart_ll_v2_6::Usart::from_ptr(0x4001_d000usize as _) };
pub const USART3: usart_ll_v2_12::Usart =
    unsafe { usart_ll_v2_12::Usart::from_ptr(0x4001_d400usize as _) };
pub const USART4: usart_ll_v2_7::Usart =
    unsafe { usart_ll_v2_7::Usart::from_ptr(0x4002_0c00usize as _) };
pub const USART5: usart_ll_v2_8::Usart =
    unsafe { usart_ll_v2_8::Usart::from_ptr(0x4002_1000usize as _) };
pub const USART6: usart_ll_v2_13::Usart =
    unsafe { usart_ll_v2_13::Usart::from_ptr(0x4002_1400usize as _) };
pub const TMR01: tmr0_ll_v2_2::Tmr0 =
    unsafe { tmr0_ll_v2_2::Tmr0::from_ptr(0x4002_4000usize as _) };
pub const TMR02: tmr0_ll_v2_2::Tmr0 =
    unsafe { tmr0_ll_v2_2::Tmr0::from_ptr(0x4002_4000usize as _) };
pub const TMR21: tmr2_ll_v2_1::Tmr2 =
    unsafe { tmr2_ll_v2_1::Tmr2::from_ptr(0x4002_4800usize as _) };
pub const TMR22: tmr2_ll_v2_1::Tmr2 =
    unsafe { tmr2_ll_v2_1::Tmr2::from_ptr(0x4002_4800usize as _) };
pub const TMR23: tmr2_ll_v2_1::Tmr2 =
    unsafe { tmr2_ll_v2_1::Tmr2::from_ptr(0x4002_4800usize as _) };
pub const TMR24: tmr2_ll_v2_1::Tmr2 =
    unsafe { tmr2_ll_v2_1::Tmr2::from_ptr(0x4002_4800usize as _) };
pub const TMRA5: tmra_ll_v2_4::Tmra =
    unsafe { tmra_ll_v2_4::Tmra::from_ptr(0x4002_6000usize as _) };
pub const TMRA6: tmra_ll_v2_5::Tmra =
    unsafe { tmra_ll_v2_5::Tmra::from_ptr(0x4002_6400usize as _) };
pub const MDIO: mdio_common::Mdio = unsafe { mdio_common::Mdio::from_ptr(0x4002_8400usize as _) };
pub const TMR4: tmr4_ll_v1_4::Tmr4 = unsafe { tmr4_ll_v1_4::Tmr4::from_ptr(0x4003_8000usize as _) };
pub const CMP1: cmp_ll_v2_6::Cmp = unsafe { cmp_ll_v2_6::Cmp::from_ptr(0x4003_8400usize as _) };
pub const CMP3: cmp_ll_v2_6::Cmp = unsafe { cmp_ll_v2_6::Cmp::from_ptr(0x4003_8400usize as _) };
pub const CMP2: cmp_ll_v2_7::Cmp = unsafe { cmp_ll_v2_7::Cmp::from_ptr(0x4003_8500usize as _) };
pub const CMP4: cmp_ll_v2_7::Cmp = unsafe { cmp_ll_v2_7::Cmp::from_ptr(0x4003_8500usize as _) };
pub const TMRA1: tmra_ll_v2::Tmra = unsafe { tmra_ll_v2::Tmra::from_ptr(0x4003_a000usize as _) };
pub const TMRA2: tmra_ll_v2_1::Tmra =
    unsafe { tmra_ll_v2_1::Tmra::from_ptr(0x4003_a400usize as _) };
pub const TMRA3: tmra_ll_v2_2::Tmra =
    unsafe { tmra_ll_v2_2::Tmra::from_ptr(0x4003_a800usize as _) };
pub const TMRA4: tmra_ll_v2_3::Tmra =
    unsafe { tmra_ll_v2_3::Tmra::from_ptr(0x4003_ac00usize as _) };
pub const I2C1: i2c_ll_v4_5::I2c = unsafe { i2c_ll_v4_5::I2c::from_ptr(0x4003_b400usize as _) };
pub const I2C2: i2c_ll_v4_5::I2c = unsafe { i2c_ll_v4_5::I2c::from_ptr(0x4003_b400usize as _) };
pub const I2C3: i2c_ll_v4_5::I2c = unsafe { i2c_ll_v4_5::I2c::from_ptr(0x4003_b400usize as _) };
pub const TMR61: tmr_v15::Tmr = unsafe { tmr_v15::Tmr::from_ptr(0x4003_c000usize as _) };
pub const TMR610: tmr_v15::Tmr = unsafe { tmr_v15::Tmr::from_ptr(0x4003_c000usize as _) };
pub const TMR62: tmr_v15::Tmr = unsafe { tmr_v15::Tmr::from_ptr(0x4003_c000usize as _) };
pub const TMR63: tmr_v15::Tmr = unsafe { tmr_v15::Tmr::from_ptr(0x4003_c000usize as _) };
pub const TMR64: tmr_v15::Tmr = unsafe { tmr_v15::Tmr::from_ptr(0x4003_c000usize as _) };
pub const TMR65: tmr_v15::Tmr = unsafe { tmr_v15::Tmr::from_ptr(0x4003_c000usize as _) };
pub const TMR66: tmr_v15::Tmr = unsafe { tmr_v15::Tmr::from_ptr(0x4003_c000usize as _) };
pub const TMR67: tmr_v15::Tmr = unsafe { tmr_v15::Tmr::from_ptr(0x4003_c000usize as _) };
pub const TMR68: tmr_v15::Tmr = unsafe { tmr_v15::Tmr::from_ptr(0x4003_c000usize as _) };
pub const TMR69: tmr_v15::Tmr = unsafe { tmr_v15::Tmr::from_ptr(0x4003_c000usize as _) };
pub const TMR6_COMMON: tmr_v16::Tmr = unsafe { tmr_v16::Tmr::from_ptr(0x4003_c300usize as _) };
pub const ADC1: adc_ll_v4_7::Adc = unsafe { adc_ll_v4_7::Adc::from_ptr(0x4004_0000usize as _) };
pub const ADC2: adc_ll_v4_8::Adc = unsafe { adc_ll_v4_8::Adc::from_ptr(0x4004_0400usize as _) };
pub const ADC3: adc_ll_v3_9::Adc = unsafe { adc_ll_v3_9::Adc::from_ptr(0x4004_0800usize as _) };
pub const DAC1: dac_ll_v1::Dac = unsafe { dac_ll_v1::Dac::from_ptr(0x4004_1000usize as _) };
pub const DAC2: dac_ll_v1::Dac = unsafe { dac_ll_v1::Dac::from_ptr(0x4004_1000usize as _) };
pub const DAC3: dac_ll_v1::Dac = unsafe { dac_ll_v1::Dac::from_ptr(0x4004_1000usize as _) };
pub const DAC4: dac_ll_v1::Dac = unsafe { dac_ll_v1::Dac::from_ptr(0x4004_1000usize as _) };
pub const TRNG: trng_ll_v1::Trng = unsafe { trng_ll_v1::Trng::from_ptr(0x4004_2000usize as _) };
pub const CMU: cmu_ll_v3_3::Cmu = unsafe { cmu_ll_v3_3::Cmu::from_ptr(0x4004_8000usize as _) };
pub const PWC: pwc_ll_v3::Pwc = unsafe { pwc_ll_v3::Pwc::from_ptr(0x4004_8000usize as _) };
pub const FCM: fcm_ll_v1_1::Fcm = unsafe { fcm_ll_v1_1::Fcm::from_ptr(0x4004_8400usize as _) };
pub const WDT: wdt_ll_v1::Wdt = unsafe { wdt_ll_v1::Wdt::from_ptr(0x4004_9000usize as _) };
pub const SWDT: swdt_ll_v1_1::Swdt = unsafe { swdt_ll_v1_1::Swdt::from_ptr(0x4004_9400usize as _) };
pub const CTC: ctc_ll_v1_3::Ctc = unsafe { ctc_ll_v1_3::Ctc::from_ptr(0x4004_9c00usize as _) };
pub const RTC: rtc_ll_v1::Rtc = unsafe { rtc_ll_v1::Rtc::from_ptr(0x4004_c000usize as _) };
pub const RMU: rmu_ll_v3_4::Rmu = unsafe { rmu_ll_v3_4::Rmu::from_ptr(0x4004_cce0usize as _) };
pub const VREF: vref_common::Vref = unsafe { vref_common::Vref::from_ptr(0x4004_f800usize as _) };
pub const PLA: pla_v1::Pla = unsafe { pla_v1::Pla::from_ptr(0x4004_fc00usize as _) };
pub const MPU: mpu_v3::Mpu = unsafe { mpu_v3::Mpu::from_ptr(0x4005_0000usize as _) };
pub const SRAMC: sramc_ll_v3_1::Sramc =
    unsafe { sramc_ll_v3_1::Sramc::from_ptr(0x4005_0800usize as _) };
pub const KEYSCAN: keyscan_common::Keyscan =
    unsafe { keyscan_common::Keyscan::from_ptr(0x4005_0c00usize as _) };
pub const INTC: intc_ll_v2_6::Intc = unsafe { intc_ll_v2_6::Intc::from_ptr(0x4005_1000usize as _) };
pub const DMA1: dma_ll_v2_6::Dma = unsafe { dma_ll_v2_6::Dma::from_ptr(0x4005_3000usize as _) };
pub const DMA2: dma_ll_v2_6::Dma = unsafe { dma_ll_v2_6::Dma::from_ptr(0x4005_3000usize as _) };
pub const GPIO: gpio_ll_v2_10::Gpio =
    unsafe { gpio_ll_v2_10::Gpio::from_ptr(0x4005_3800usize as _) };
pub const MAU: mau_v1::Mau = unsafe { mau_v1::Mau::from_ptr(0x4005_5000usize as _) };
pub const PERIC: peric_ll_v2_1::Peric =
    unsafe { peric_ll_v2_1::Peric::from_ptr(0x4005_5400usize as _) };
pub const DCU1: dcu_v3::Dcu = unsafe { dcu_v3::Dcu::from_ptr(0x4005_6000usize as _) };
pub const DCU2: dcu_v3::Dcu = unsafe { dcu_v3::Dcu::from_ptr(0x4005_6000usize as _) };
pub const DCU3: dcu_v3::Dcu = unsafe { dcu_v3::Dcu::from_ptr(0x4005_6000usize as _) };
pub const FMAC1: fmac_common::Fmac = unsafe { fmac_common::Fmac::from_ptr(0x4005_8000usize as _) };
pub const FMAC2: fmac_common::Fmac = unsafe { fmac_common::Fmac::from_ptr(0x4005_8000usize as _) };
pub const FMAC3: fmac_common::Fmac = unsafe { fmac_common::Fmac::from_ptr(0x4005_8000usize as _) };
pub const FMAC4: fmac_common::Fmac = unsafe { fmac_common::Fmac::from_ptr(0x4005_8000usize as _) };
pub const CAN1: can_ll_v2::Can = unsafe { can_ll_v2::Can::from_ptr(0x4007_8000usize as _) };
pub const CAN2: can_ll_v2::Can = unsafe { can_ll_v2::Can::from_ptr(0x4007_8000usize as _) };
pub const CAN3: can_ll_v2::Can = unsafe { can_ll_v2::Can::from_ptr(0x4007_8000usize as _) };
pub const USBFS: usbfs_v2::Usbfs = unsafe { usbfs_v2::Usbfs::from_ptr(0x4008_0000usize as _) };
pub const SMC: smc_v2::Smc = unsafe { smc_v2::Smc::from_ptr(0x8800_0000usize as _) };
pub const QSPI: qspi_v3::Qspi = unsafe { qspi_v3::Qspi::from_ptr(0x9c00_0000usize as _) };
pub const DBGC: dbgc_ll_v3_4::Dbgc = unsafe { dbgc_ll_v3_4::Dbgc::from_ptr(0xe004_2000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ll_v3_9.rs"]
pub mod adc_ll_v3_9;
#[path = "../../peripherals/adc_ll_v4_7.rs"]
pub mod adc_ll_v4_7;
#[path = "../../peripherals/adc_ll_v4_8.rs"]
pub mod adc_ll_v4_8;
#[path = "../../peripherals/aes_ll_v2.rs"]
pub mod aes_ll_v2;
#[path = "../../peripherals/aos_ll_v8.rs"]
pub mod aos_ll_v8;
#[path = "../../peripherals/can_ll_v2.rs"]
pub mod can_ll_v2;
#[path = "../../peripherals/cmp_ll_v2_6.rs"]
pub mod cmp_ll_v2_6;
#[path = "../../peripherals/cmp_ll_v2_7.rs"]
pub mod cmp_ll_v2_7;
#[path = "../../peripherals/cmu_ll_v3_3.rs"]
pub mod cmu_ll_v3_3;
#[path = "../../peripherals/crc_ll_v2_1.rs"]
pub mod crc_ll_v2_1;
#[path = "../../peripherals/ctc_ll_v1_3.rs"]
pub mod ctc_ll_v1_3;
#[path = "../../peripherals/dac_ll_v1.rs"]
pub mod dac_ll_v1;
#[path = "../../peripherals/dbgc_ll_v3_4.rs"]
pub mod dbgc_ll_v3_4;
#[path = "../../peripherals/dcu_v3.rs"]
pub mod dcu_v3;
#[path = "../../peripherals/dma_ll_v2_6.rs"]
pub mod dma_ll_v2_6;
#[path = "../../peripherals/efm_ll_v2_4.rs"]
pub mod efm_ll_v2_4;
#[path = "../../peripherals/emb_ctl1_ll_v1.rs"]
pub mod emb_ctl1_ll_v1;
#[path = "../../peripherals/emb_ll_v2_9.rs"]
pub mod emb_ll_v2_9;
#[path = "../../peripherals/fcm_ll_v1_1.rs"]
pub mod fcm_ll_v1_1;
#[path = "../../peripherals/fmac_common.rs"]
pub mod fmac_common;
#[path = "../../peripherals/gpio_ll_v2_10.rs"]
pub mod gpio_ll_v2_10;
#[path = "../../peripherals/hash_v2.rs"]
pub mod hash_v2;
#[path = "../../peripherals/i2c_ll_v4_5.rs"]
pub mod i2c_ll_v4_5;
#[path = "../../peripherals/icg_ll_v1_3.rs"]
pub mod icg_ll_v1_3;
#[path = "../../peripherals/intc_ll_v2_6.rs"]
pub mod intc_ll_v2_6;
#[path = "../../peripherals/keyscan_common.rs"]
pub mod keyscan_common;
#[path = "../../peripherals/mau_v1.rs"]
pub mod mau_v1;
#[path = "../../peripherals/mdio_common.rs"]
pub mod mdio_common;
#[path = "../../peripherals/mpu_v3.rs"]
pub mod mpu_v3;
#[path = "../../peripherals/ots_v2.rs"]
pub mod ots_v2;
#[path = "../../peripherals/peric_ll_v2_1.rs"]
pub mod peric_ll_v2_1;
#[path = "../../peripherals/pla_v1.rs"]
pub mod pla_v1;
#[path = "../../peripherals/pwc_ll_v3.rs"]
pub mod pwc_ll_v3;
#[path = "../../peripherals/qspi_v3.rs"]
pub mod qspi_v3;
#[path = "../../peripherals/rmu_ll_v3_4.rs"]
pub mod rmu_ll_v3_4;
#[path = "../../peripherals/rtc_ll_v1.rs"]
pub mod rtc_ll_v1;
#[path = "../../peripherals/smc_v2.rs"]
pub mod smc_v2;
#[path = "../../peripherals/spi_ll_v2_3.rs"]
pub mod spi_ll_v2_3;
#[path = "../../peripherals/sramc_ll_v3_1.rs"]
pub mod sramc_ll_v3_1;
#[path = "../../peripherals/swdt_ll_v1_1.rs"]
pub mod swdt_ll_v1_1;
#[path = "../../peripherals/tmr0_ll_v2_2.rs"]
pub mod tmr0_ll_v2_2;
#[path = "../../peripherals/tmr2_ll_v2_1.rs"]
pub mod tmr2_ll_v2_1;
#[path = "../../peripherals/tmr4_ll_v1_4.rs"]
pub mod tmr4_ll_v1_4;
#[path = "../../peripherals/tmr_v15.rs"]
pub mod tmr_v15;
#[path = "../../peripherals/tmr_v16.rs"]
pub mod tmr_v16;
#[path = "../../peripherals/tmra_ll_v2.rs"]
pub mod tmra_ll_v2;
#[path = "../../peripherals/tmra_ll_v2_1.rs"]
pub mod tmra_ll_v2_1;
#[path = "../../peripherals/tmra_ll_v2_2.rs"]
pub mod tmra_ll_v2_2;
#[path = "../../peripherals/tmra_ll_v2_3.rs"]
pub mod tmra_ll_v2_3;
#[path = "../../peripherals/tmra_ll_v2_4.rs"]
pub mod tmra_ll_v2_4;
#[path = "../../peripherals/tmra_ll_v2_5.rs"]
pub mod tmra_ll_v2_5;
#[path = "../../peripherals/trng_ll_v1.rs"]
pub mod trng_ll_v1;
#[path = "../../peripherals/usart_ll_v2_12.rs"]
pub mod usart_ll_v2_12;
#[path = "../../peripherals/usart_ll_v2_13.rs"]
pub mod usart_ll_v2_13;
#[path = "../../peripherals/usart_ll_v2_5.rs"]
pub mod usart_ll_v2_5;
#[path = "../../peripherals/usart_ll_v2_6.rs"]
pub mod usart_ll_v2_6;
#[path = "../../peripherals/usart_ll_v2_7.rs"]
pub mod usart_ll_v2_7;
#[path = "../../peripherals/usart_ll_v2_8.rs"]
pub mod usart_ll_v2_8;
#[path = "../../peripherals/usbfs_v2.rs"]
pub mod usbfs_v2;
#[path = "../../peripherals/vref_common.rs"]
pub mod vref_common;
#[path = "../../peripherals/wdt_ll_v1.rs"]
pub mod wdt_ll_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
