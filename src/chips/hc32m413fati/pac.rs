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
    #[doc = "24 - DMA_ERR"]
    DMA_ERR = 24,
    #[doc = "25 - DMA_TC0_BTC0"]
    DMA_TC0_BTC0 = 25,
    #[doc = "26 - DMA_TC1_BTC1"]
    DMA_TC1_BTC1 = 26,
    #[doc = "27 - DMA_TC2_BTC2"]
    DMA_TC2_BTC2 = 27,
    #[doc = "28 - DMA_TC3_BTC3"]
    DMA_TC3_BTC3 = 28,
    #[doc = "29 - EFM_PEERR_RDCOL"]
    EFM_PEERR_RDCOL = 29,
    #[doc = "30 - EFM_OPTEND"]
    EFM_OPTEND = 30,
    #[doc = "31 - DCU"]
    DCU = 31,
    #[doc = "32 - FPU"]
    FPU = 32,
    #[doc = "33 - MAU"]
    MAU = 33,
    #[doc = "34 - TMR0_1"]
    TMR0_1 = 34,
    #[doc = "35 - TMR0_2"]
    TMR0_2 = 35,
    #[doc = "36 - XTAL"]
    XTAL = 36,
    #[doc = "37 - SWDT"]
    SWDT = 37,
    #[doc = "38 - TMRA_1_OVF_UDF"]
    TMRA_1_OVF_UDF = 38,
    #[doc = "39 - TMRA_1_CMP"]
    TMRA_1_CMP = 39,
    #[doc = "40 - TMRA_2_OVF_UDF"]
    TMRA_2_OVF_UDF = 40,
    #[doc = "41 - TMRA_2_CMP"]
    TMRA_2_CMP = 41,
    #[doc = "42 - TMRA_3_OVF_UDF"]
    TMRA_3_OVF_UDF = 42,
    #[doc = "43 - TMRA_3_CMP"]
    TMRA_3_CMP = 43,
    #[doc = "44 - TMRA_4_OVF_UDF"]
    TMRA_4_OVF_UDF = 44,
    #[doc = "45 - TMRA_4_CMP"]
    TMRA_4_CMP = 45,
    #[doc = "46 - EMB_GR0"]
    EMB_GR0 = 46,
    #[doc = "47 - EMB_GR1"]
    EMB_GR1 = 47,
    #[doc = "48 - TMR4_GCMP"]
    TMR4_GCMP = 48,
    #[doc = "49 - TMR4_OVF_UDF"]
    TMR4_OVF_UDF = 49,
    #[doc = "50 - TMR4_RELOAD"]
    TMR4_RELOAD = 50,
    #[doc = "51 - TMR4_SCMP"]
    TMR4_SCMP = 51,
    #[doc = "52 - USART1"]
    USART1 = 52,
    #[doc = "53 - USART1_TCI"]
    USART1_TCI = 53,
    #[doc = "54 - SPI"]
    SPI = 54,
    #[doc = "55 - TMRA_5_OVF_UDF"]
    TMRA_5_OVF_UDF = 55,
    #[doc = "56 - TMRA_5_CMP"]
    TMRA_5_CMP = 56,
    #[doc = "57 - USART2"]
    USART2 = 57,
    #[doc = "58 - USART2_TCI"]
    USART2_TCI = 58,
    #[doc = "59 - USART3"]
    USART3 = 59,
    #[doc = "60 - USART3_TCI"]
    USART3_TCI = 60,
    #[doc = "61 - USART4"]
    USART4 = 61,
    #[doc = "62 - USART4_TCI"]
    USART4_TCI = 62,
    #[doc = "63 - HALL"]
    HALL = 63,
    #[doc = "64 - CMP1"]
    CMP1 = 64,
    #[doc = "65 - CMP2"]
    CMP2 = 65,
    #[doc = "66 - CMP3"]
    CMP3 = 66,
    #[doc = "67 - I2C"]
    I2C = 67,
    #[doc = "68 - USART1_WUPI"]
    USART1_WUPI = 68,
    #[doc = "69 - LVD1"]
    LVD1 = 69,
    #[doc = "70 - LVD2"]
    LVD2 = 70,
    #[doc = "71 - FCM"]
    FCM = 71,
    #[doc = "72 - WDT"]
    WDT = 72,
    #[doc = "73 - ADC"]
    ADC = 73,
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
        fn DMA_ERR();
        fn DMA_TC0_BTC0();
        fn DMA_TC1_BTC1();
        fn DMA_TC2_BTC2();
        fn DMA_TC3_BTC3();
        fn EFM_PEERR_RDCOL();
        fn EFM_OPTEND();
        fn DCU();
        fn FPU();
        fn MAU();
        fn TMR0_1();
        fn TMR0_2();
        fn XTAL();
        fn SWDT();
        fn TMRA_1_OVF_UDF();
        fn TMRA_1_CMP();
        fn TMRA_2_OVF_UDF();
        fn TMRA_2_CMP();
        fn TMRA_3_OVF_UDF();
        fn TMRA_3_CMP();
        fn TMRA_4_OVF_UDF();
        fn TMRA_4_CMP();
        fn EMB_GR0();
        fn EMB_GR1();
        fn TMR4_GCMP();
        fn TMR4_OVF_UDF();
        fn TMR4_RELOAD();
        fn TMR4_SCMP();
        fn USART1();
        fn USART1_TCI();
        fn SPI();
        fn TMRA_5_OVF_UDF();
        fn TMRA_5_CMP();
        fn USART2();
        fn USART2_TCI();
        fn USART3();
        fn USART3_TCI();
        fn USART4();
        fn USART4_TCI();
        fn HALL();
        fn CMP1();
        fn CMP2();
        fn CMP3();
        fn I2C();
        fn USART1_WUPI();
        fn LVD1();
        fn LVD2();
        fn FCM();
        fn WDT();
        fn ADC();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 74] = [
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
            _handler: EFM_PEERR_RDCOL,
        },
        Vector {
            _handler: EFM_OPTEND,
        },
        Vector { _handler: DCU },
        Vector { _handler: FPU },
        Vector { _handler: MAU },
        Vector { _handler: TMR0_1 },
        Vector { _handler: TMR0_2 },
        Vector { _handler: XTAL },
        Vector { _handler: SWDT },
        Vector {
            _handler: TMRA_1_OVF_UDF,
        },
        Vector {
            _handler: TMRA_1_CMP,
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
        Vector { _handler: EMB_GR0 },
        Vector { _handler: EMB_GR1 },
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
        Vector { _handler: HALL },
        Vector { _handler: CMP1 },
        Vector { _handler: CMP2 },
        Vector { _handler: CMP3 },
        Vector { _handler: I2C },
        Vector {
            _handler: USART1_WUPI,
        },
        Vector { _handler: LVD1 },
        Vector { _handler: LVD2 },
        Vector { _handler: FCM },
        Vector { _handler: WDT },
        Vector { _handler: ADC },
    ];
}
pub const ICG: icg_ll_v1_4::Icg = unsafe { icg_ll_v1_4::Icg::from_ptr(0x0002_0000usize as _) };
pub const EFM: efm_ll_v2_1::Efm = unsafe { efm_ll_v2_1::Efm::from_ptr(0x4001_0400usize as _) };
pub const AOS: aos_ll_v4::Aos = unsafe { aos_ll_v4::Aos::from_ptr(0x4001_0800usize as _) };
pub const TMRA1: tmra_ll_v1_1::Tmra =
    unsafe { tmra_ll_v1_1::Tmra::from_ptr(0x4001_5000usize as _) };
pub const TMRA2: tmra_ll_v1_1::Tmra =
    unsafe { tmra_ll_v1_1::Tmra::from_ptr(0x4001_5000usize as _) };
pub const TMRA3: tmra_ll_v1_1::Tmra =
    unsafe { tmra_ll_v1_1::Tmra::from_ptr(0x4001_5000usize as _) };
pub const TMRA4: tmra_ll_v1_1::Tmra =
    unsafe { tmra_ll_v1_1::Tmra::from_ptr(0x4001_5000usize as _) };
pub const TMRA5: tmra_ll_v3::Tmra = unsafe { tmra_ll_v3::Tmra::from_ptr(0x4001_6000usize as _) };
pub const TMR4: tmr4_ll_v3_2::Tmr4 = unsafe { tmr4_ll_v3_2::Tmr4::from_ptr(0x4001_7000usize as _) };
pub const EMB0: emb_ll_v2::Emb = unsafe { emb_ll_v2::Emb::from_ptr(0x4001_7c00usize as _) };
pub const EMB1: emb_ll_v2::Emb = unsafe { emb_ll_v2::Emb::from_ptr(0x4001_7c00usize as _) };
pub const SPI: spi_ll_v2_5::Spi = unsafe { spi_ll_v2_5::Spi::from_ptr(0x4001_c000usize as _) };
pub const USART1: usart_ll_v3_10::Usart =
    unsafe { usart_ll_v3_10::Usart::from_ptr(0x4001_d000usize as _) };
pub const USART2: usart_ll_v3_15::Usart =
    unsafe { usart_ll_v3_15::Usart::from_ptr(0x4001_d400usize as _) };
pub const USART3: usart_ll_v3_15::Usart =
    unsafe { usart_ll_v3_15::Usart::from_ptr(0x4001_d400usize as _) };
pub const USART4: usart_ll_v3_15::Usart =
    unsafe { usart_ll_v3_15::Usart::from_ptr(0x4001_d400usize as _) };
pub const HALL: hall_common::Hall = unsafe { hall_common::Hall::from_ptr(0x4001_e800usize as _) };
pub const TMR01: tmr0_ll_v2_4::Tmr0 =
    unsafe { tmr0_ll_v2_4::Tmr0::from_ptr(0x4002_4000usize as _) };
pub const TMR02: tmr0_ll_v2_4::Tmr0 =
    unsafe { tmr0_ll_v2_4::Tmr0::from_ptr(0x4002_4000usize as _) };
pub const ADC: adc_ll_v4::Adc = unsafe { adc_ll_v4::Adc::from_ptr(0x4003_0000usize as _) };
pub const FCM: fcm_ll_v1_2::Fcm = unsafe { fcm_ll_v1_2::Fcm::from_ptr(0x4004_8400usize as _) };
pub const WDT: wdt_ll_v1::Wdt = unsafe { wdt_ll_v1::Wdt::from_ptr(0x4004_9000usize as _) };
pub const SWDT: swdt_ll_v1_1::Swdt = unsafe { swdt_ll_v1_1::Swdt::from_ptr(0x4004_9400usize as _) };
pub const CMP1: cmp_ll_v3_4::Cmp = unsafe { cmp_ll_v3_4::Cmp::from_ptr(0x4004_a000usize as _) };
pub const CMP2: cmp_ll_v3_4::Cmp = unsafe { cmp_ll_v3_4::Cmp::from_ptr(0x4004_a000usize as _) };
pub const CMP3: cmp_ll_v3_4::Cmp = unsafe { cmp_ll_v3_4::Cmp::from_ptr(0x4004_a000usize as _) };
pub const CMP_COMMON: cmp_common_ll_v1::CmpCommon =
    unsafe { cmp_common_ll_v1::CmpCommon::from_ptr(0x4004_a000usize as _) };
pub const I2C: i2c_ll_v4_3::I2c = unsafe { i2c_ll_v4_3::I2c::from_ptr(0x4004_e000usize as _) };
pub const CRC: crc_ll_v3::Crc = unsafe { crc_ll_v3::Crc::from_ptr(0x4005_0400usize as _) };
pub const INTC: intc_ll_v2_4::Intc = unsafe { intc_ll_v2_4::Intc::from_ptr(0x4005_1000usize as _) };
pub const DCU: dcu_v10::Dcu = unsafe { dcu_v10::Dcu::from_ptr(0x4005_2000usize as _) };
pub const DMA: dma_ll_v2_2::Dma = unsafe { dma_ll_v2_2::Dma::from_ptr(0x4005_3000usize as _) };
pub const GPIO: gpio_ll_v2_8::Gpio = unsafe { gpio_ll_v2_8::Gpio::from_ptr(0x4005_3800usize as _) };
pub const PWC: pwc_ll_v2_2::Pwc = unsafe { pwc_ll_v2_2::Pwc::from_ptr(0x4005_4000usize as _) };
pub const RMU: rmu_ll_v2_1::Rmu = unsafe { rmu_ll_v2_1::Rmu::from_ptr(0x4005_4100usize as _) };
pub const CMU: cmu_ll_v2::Cmu = unsafe { cmu_ll_v2::Cmu::from_ptr(0x4005_4400usize as _) };
pub const MAU: mau_v3::Mau = unsafe { mau_v3::Mau::from_ptr(0x4005_4800usize as _) };
pub const PERIC: peric_ll_v1_1::Peric =
    unsafe { peric_ll_v1_1::Peric::from_ptr(0x4005_4c00usize as _) };
pub const DBGC: dbgc_ll_v4::Dbgc = unsafe { dbgc_ll_v4::Dbgc::from_ptr(0xe004_2000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ll_v4.rs"]
pub mod adc_ll_v4;
#[path = "../../peripherals/aos_ll_v4.rs"]
pub mod aos_ll_v4;
#[path = "../../peripherals/cmp_common_ll_v1.rs"]
pub mod cmp_common_ll_v1;
#[path = "../../peripherals/cmp_ll_v3_4.rs"]
pub mod cmp_ll_v3_4;
#[path = "../../peripherals/cmu_ll_v2.rs"]
pub mod cmu_ll_v2;
#[path = "../../peripherals/crc_ll_v3.rs"]
pub mod crc_ll_v3;
#[path = "../../peripherals/dbgc_ll_v4.rs"]
pub mod dbgc_ll_v4;
#[path = "../../peripherals/dcu_v10.rs"]
pub mod dcu_v10;
#[path = "../../peripherals/dma_ll_v2_2.rs"]
pub mod dma_ll_v2_2;
#[path = "../../peripherals/efm_ll_v2_1.rs"]
pub mod efm_ll_v2_1;
#[path = "../../peripherals/emb_ll_v2.rs"]
pub mod emb_ll_v2;
#[path = "../../peripherals/fcm_ll_v1_2.rs"]
pub mod fcm_ll_v1_2;
#[path = "../../peripherals/gpio_ll_v2_8.rs"]
pub mod gpio_ll_v2_8;
#[path = "../../peripherals/hall_common.rs"]
pub mod hall_common;
#[path = "../../peripherals/i2c_ll_v4_3.rs"]
pub mod i2c_ll_v4_3;
#[path = "../../peripherals/icg_ll_v1_4.rs"]
pub mod icg_ll_v1_4;
#[path = "../../peripherals/intc_ll_v2_4.rs"]
pub mod intc_ll_v2_4;
#[path = "../../peripherals/mau_v3.rs"]
pub mod mau_v3;
#[path = "../../peripherals/peric_ll_v1_1.rs"]
pub mod peric_ll_v1_1;
#[path = "../../peripherals/pwc_ll_v2_2.rs"]
pub mod pwc_ll_v2_2;
#[path = "../../peripherals/rmu_ll_v2_1.rs"]
pub mod rmu_ll_v2_1;
#[path = "../../peripherals/spi_ll_v2_5.rs"]
pub mod spi_ll_v2_5;
#[path = "../../peripherals/swdt_ll_v1_1.rs"]
pub mod swdt_ll_v1_1;
#[path = "../../peripherals/tmr0_ll_v2_4.rs"]
pub mod tmr0_ll_v2_4;
#[path = "../../peripherals/tmr4_ll_v3_2.rs"]
pub mod tmr4_ll_v3_2;
#[path = "../../peripherals/tmra_ll_v1_1.rs"]
pub mod tmra_ll_v1_1;
#[path = "../../peripherals/tmra_ll_v3.rs"]
pub mod tmra_ll_v3;
#[path = "../../peripherals/usart_ll_v3_10.rs"]
pub mod usart_ll_v3_10;
#[path = "../../peripherals/usart_ll_v3_15.rs"]
pub mod usart_ll_v3_15;
#[path = "../../peripherals/wdt_ll_v1.rs"]
pub mod wdt_ll_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
