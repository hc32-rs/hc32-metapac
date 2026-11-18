#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - EXTINT_PORT_EIRQ0"]
    EXTINT_PORT_EIRQ0 = 0,
    #[doc = "1 - EXTINT_PORT_EIRQ1"]
    EXTINT_PORT_EIRQ1 = 1,
    #[doc = "2 - EXTINT_PORT_EIRQ2"]
    EXTINT_PORT_EIRQ2 = 2,
    #[doc = "3 - EXTINT_PORT_EIRQ3"]
    EXTINT_PORT_EIRQ3 = 3,
    #[doc = "4 - EXTINT_PORT_EIRQ4"]
    EXTINT_PORT_EIRQ4 = 4,
    #[doc = "5 - EXTINT_PORT_EIRQ5"]
    EXTINT_PORT_EIRQ5 = 5,
    #[doc = "6 - EXTINT_PORT_EIRQ6"]
    EXTINT_PORT_EIRQ6 = 6,
    #[doc = "7 - EXTINT_PORT_EIRQ7"]
    EXTINT_PORT_EIRQ7 = 7,
    #[doc = "8 - TMR0_1"]
    TMR0_1 = 8,
    #[doc = "9 - TMR0_2"]
    TMR0_2 = 9,
    #[doc = "10 - XTAL"]
    XTAL = 10,
    #[doc = "11 - SWDT"]
    SWDT = 11,
    #[doc = "12 - LVD1"]
    LVD1 = 12,
    #[doc = "13 - LVD2"]
    LVD2 = 13,
    #[doc = "14 - DMA_TC0_BTC0"]
    DMA_TC0_BTC0 = 14,
    #[doc = "15 - DMA_TC1_BTC1"]
    DMA_TC1_BTC1 = 15,
    #[doc = "16 - DMA_ERR"]
    DMA_ERR = 16,
    #[doc = "17 - EFM"]
    EFM = 17,
    #[doc = "18 - DFM"]
    DFM = 18,
    #[doc = "19 - FPU"]
    FPU = 19,
    #[doc = "20 - MAU"]
    MAU = 20,
    #[doc = "21 - TMR4_1_OVF"]
    TMR4_1_OVF = 21,
    #[doc = "22 - TMR4_1_UDF"]
    TMR4_1_UDF = 22,
    #[doc = "23 - TMR4_1_RELOAD"]
    TMR4_1_RELOAD = 23,
    #[doc = "24 - TMR4_1_GCMP"]
    TMR4_1_GCMP = 24,
    #[doc = "25 - TMR4_1_SCMP"]
    TMR4_1_SCMP = 25,
    #[doc = "26 - TMRA_1_OVF"]
    TMRA_1_OVF = 26,
    #[doc = "27 - TMRA_1_UDF"]
    TMRA_1_UDF = 27,
    #[doc = "28 - TMRA_1_CMP"]
    TMRA_1_CMP = 28,
    #[doc = "29 - TMRA_2_OVF"]
    TMRA_2_OVF = 29,
    #[doc = "30 - TMRA_2_UDF"]
    TMRA_2_UDF = 30,
    #[doc = "31 - TMRA_2_CMP"]
    TMRA_2_CMP = 31,
    #[doc = "32 - TMR4_2_OVF"]
    TMR4_2_OVF = 32,
    #[doc = "33 - TMR4_2_UDF"]
    TMR4_2_UDF = 33,
    #[doc = "34 - TMR4_2_RELOAD"]
    TMR4_2_RELOAD = 34,
    #[doc = "35 - TMR4_2_GCMP"]
    TMR4_2_GCMP = 35,
    #[doc = "36 - TMR4_2_SCMP"]
    TMR4_2_SCMP = 36,
    #[doc = "37 - TMRA_3_OVF"]
    TMRA_3_OVF = 37,
    #[doc = "38 - TMRA_3_UDF"]
    TMRA_3_UDF = 38,
    #[doc = "39 - TMRA_3_CMP"]
    TMRA_3_CMP = 39,
    #[doc = "40 - TMRA_4_OVF"]
    TMRA_4_OVF = 40,
    #[doc = "41 - TMRA_4_UDF"]
    TMRA_4_UDF = 41,
    #[doc = "42 - TMRA_4_CMP"]
    TMRA_4_CMP = 42,
    #[doc = "43 - CMP1"]
    CMP1 = 43,
    #[doc = "44 - CMP2"]
    CMP2 = 44,
    #[doc = "45 - EMB"]
    EMB = 45,
    #[doc = "46 - I2C_TX_RX"]
    I2C_TX_RX = 46,
    #[doc = "47 - I2C_TE_EE"]
    I2C_TE_EE = 47,
    #[doc = "48 - TMRA_5_OVF"]
    TMRA_5_OVF = 48,
    #[doc = "49 - TMRA_5_UDF"]
    TMRA_5_UDF = 49,
    #[doc = "50 - TMRA_5_CMP"]
    TMRA_5_CMP = 50,
    #[doc = "51 - USART1_TX_RX"]
    USART1_TX_RX = 51,
    #[doc = "52 - USART1_EI_TE_RTO_BW"]
    USART1_EI_TE_RTO_BW = 52,
    #[doc = "53 - USART1_TCI"]
    USART1_TCI = 53,
    #[doc = "54 - USART2_TX_RX"]
    USART2_TX_RX = 54,
    #[doc = "55 - USART2_EI_TE_RTO_BW"]
    USART2_EI_TE_RTO_BW = 55,
    #[doc = "56 - USART2_TCI"]
    USART2_TCI = 56,
    #[doc = "57 - USART3_TX_RX"]
    USART3_TX_RX = 57,
    #[doc = "58 - USART3_EI_TE_RTO_BW"]
    USART3_EI_TE_RTO_BW = 58,
    #[doc = "59 - USART3_TCI"]
    USART3_TCI = 59,
    #[doc = "60 - USART4_TX_RX"]
    USART4_TX_RX = 60,
    #[doc = "61 - USART4_EI_TE_RTO_BW"]
    USART4_EI_TE_RTO_BW = 61,
    #[doc = "62 - USART4_TCI"]
    USART4_TCI = 62,
    #[doc = "63 - SPI1_TX_RX"]
    SPI1_TX_RX = 63,
    #[doc = "64 - SPI1_IDLE_ERR"]
    SPI1_IDLE_ERR = 64,
    #[doc = "65 - SPI2_TX_RX"]
    SPI2_TX_RX = 65,
    #[doc = "66 - SPI2_IDLE_ERR"]
    SPI2_IDLE_ERR = 66,
    #[doc = "67 - FCM"]
    FCM = 67,
    #[doc = "68 - WDT"]
    WDT = 68,
    #[doc = "69 - USART1_WUPI"]
    USART1_WUPI = 69,
    #[doc = "70 - ADC1_ECOA"]
    ADC1_ECOA = 70,
    #[doc = "71 - ADC1_ECOB"]
    ADC1_ECOB = 71,
    #[doc = "72 - ADC1_ECOC"]
    ADC1_ECOC = 72,
    #[doc = "73 - ADC1_CMP"]
    ADC1_CMP = 73,
    #[doc = "74 - ADC2_ECOA"]
    ADC2_ECOA = 74,
    #[doc = "75 - ADC2_ECOB"]
    ADC2_ECOB = 75,
    #[doc = "76 - ADC2_ECOC"]
    ADC2_ECOC = 76,
    #[doc = "77 - ADC2_CMP"]
    ADC2_CMP = 77,
    #[doc = "78 - TRNG"]
    TRNG = 78,
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
        fn EXTINT_PORT_EIRQ0();
        fn EXTINT_PORT_EIRQ1();
        fn EXTINT_PORT_EIRQ2();
        fn EXTINT_PORT_EIRQ3();
        fn EXTINT_PORT_EIRQ4();
        fn EXTINT_PORT_EIRQ5();
        fn EXTINT_PORT_EIRQ6();
        fn EXTINT_PORT_EIRQ7();
        fn TMR0_1();
        fn TMR0_2();
        fn XTAL();
        fn SWDT();
        fn LVD1();
        fn LVD2();
        fn DMA_TC0_BTC0();
        fn DMA_TC1_BTC1();
        fn DMA_ERR();
        fn EFM();
        fn DFM();
        fn FPU();
        fn MAU();
        fn TMR4_1_OVF();
        fn TMR4_1_UDF();
        fn TMR4_1_RELOAD();
        fn TMR4_1_GCMP();
        fn TMR4_1_SCMP();
        fn TMRA_1_OVF();
        fn TMRA_1_UDF();
        fn TMRA_1_CMP();
        fn TMRA_2_OVF();
        fn TMRA_2_UDF();
        fn TMRA_2_CMP();
        fn TMR4_2_OVF();
        fn TMR4_2_UDF();
        fn TMR4_2_RELOAD();
        fn TMR4_2_GCMP();
        fn TMR4_2_SCMP();
        fn TMRA_3_OVF();
        fn TMRA_3_UDF();
        fn TMRA_3_CMP();
        fn TMRA_4_OVF();
        fn TMRA_4_UDF();
        fn TMRA_4_CMP();
        fn CMP1();
        fn CMP2();
        fn EMB();
        fn I2C_TX_RX();
        fn I2C_TE_EE();
        fn TMRA_5_OVF();
        fn TMRA_5_UDF();
        fn TMRA_5_CMP();
        fn USART1_TX_RX();
        fn USART1_EI_TE_RTO_BW();
        fn USART1_TCI();
        fn USART2_TX_RX();
        fn USART2_EI_TE_RTO_BW();
        fn USART2_TCI();
        fn USART3_TX_RX();
        fn USART3_EI_TE_RTO_BW();
        fn USART3_TCI();
        fn USART4_TX_RX();
        fn USART4_EI_TE_RTO_BW();
        fn USART4_TCI();
        fn SPI1_TX_RX();
        fn SPI1_IDLE_ERR();
        fn SPI2_TX_RX();
        fn SPI2_IDLE_ERR();
        fn FCM();
        fn WDT();
        fn USART1_WUPI();
        fn ADC1_ECOA();
        fn ADC1_ECOB();
        fn ADC1_ECOC();
        fn ADC1_CMP();
        fn ADC2_ECOA();
        fn ADC2_ECOB();
        fn ADC2_ECOC();
        fn ADC2_CMP();
        fn TRNG();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 79] = [
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
        Vector { _handler: TMR0_1 },
        Vector { _handler: TMR0_2 },
        Vector { _handler: XTAL },
        Vector { _handler: SWDT },
        Vector { _handler: LVD1 },
        Vector { _handler: LVD2 },
        Vector {
            _handler: DMA_TC0_BTC0,
        },
        Vector {
            _handler: DMA_TC1_BTC1,
        },
        Vector { _handler: DMA_ERR },
        Vector { _handler: EFM },
        Vector { _handler: DFM },
        Vector { _handler: FPU },
        Vector { _handler: MAU },
        Vector {
            _handler: TMR4_1_OVF,
        },
        Vector {
            _handler: TMR4_1_UDF,
        },
        Vector {
            _handler: TMR4_1_RELOAD,
        },
        Vector {
            _handler: TMR4_1_GCMP,
        },
        Vector {
            _handler: TMR4_1_SCMP,
        },
        Vector {
            _handler: TMRA_1_OVF,
        },
        Vector {
            _handler: TMRA_1_UDF,
        },
        Vector {
            _handler: TMRA_1_CMP,
        },
        Vector {
            _handler: TMRA_2_OVF,
        },
        Vector {
            _handler: TMRA_2_UDF,
        },
        Vector {
            _handler: TMRA_2_CMP,
        },
        Vector {
            _handler: TMR4_2_OVF,
        },
        Vector {
            _handler: TMR4_2_UDF,
        },
        Vector {
            _handler: TMR4_2_RELOAD,
        },
        Vector {
            _handler: TMR4_2_GCMP,
        },
        Vector {
            _handler: TMR4_2_SCMP,
        },
        Vector {
            _handler: TMRA_3_OVF,
        },
        Vector {
            _handler: TMRA_3_UDF,
        },
        Vector {
            _handler: TMRA_3_CMP,
        },
        Vector {
            _handler: TMRA_4_OVF,
        },
        Vector {
            _handler: TMRA_4_UDF,
        },
        Vector {
            _handler: TMRA_4_CMP,
        },
        Vector { _handler: CMP1 },
        Vector { _handler: CMP2 },
        Vector { _handler: EMB },
        Vector {
            _handler: I2C_TX_RX,
        },
        Vector {
            _handler: I2C_TE_EE,
        },
        Vector {
            _handler: TMRA_5_OVF,
        },
        Vector {
            _handler: TMRA_5_UDF,
        },
        Vector {
            _handler: TMRA_5_CMP,
        },
        Vector {
            _handler: USART1_TX_RX,
        },
        Vector {
            _handler: USART1_EI_TE_RTO_BW,
        },
        Vector {
            _handler: USART1_TCI,
        },
        Vector {
            _handler: USART2_TX_RX,
        },
        Vector {
            _handler: USART2_EI_TE_RTO_BW,
        },
        Vector {
            _handler: USART2_TCI,
        },
        Vector {
            _handler: USART3_TX_RX,
        },
        Vector {
            _handler: USART3_EI_TE_RTO_BW,
        },
        Vector {
            _handler: USART3_TCI,
        },
        Vector {
            _handler: USART4_TX_RX,
        },
        Vector {
            _handler: USART4_EI_TE_RTO_BW,
        },
        Vector {
            _handler: USART4_TCI,
        },
        Vector {
            _handler: SPI1_TX_RX,
        },
        Vector {
            _handler: SPI1_IDLE_ERR,
        },
        Vector {
            _handler: SPI2_TX_RX,
        },
        Vector {
            _handler: SPI2_IDLE_ERR,
        },
        Vector { _handler: FCM },
        Vector { _handler: WDT },
        Vector {
            _handler: USART1_WUPI,
        },
        Vector {
            _handler: ADC1_ECOA,
        },
        Vector {
            _handler: ADC1_ECOB,
        },
        Vector {
            _handler: ADC1_ECOC,
        },
        Vector { _handler: ADC1_CMP },
        Vector {
            _handler: ADC2_ECOA,
        },
        Vector {
            _handler: ADC2_ECOB,
        },
        Vector {
            _handler: ADC2_ECOC,
        },
        Vector { _handler: ADC2_CMP },
        Vector { _handler: TRNG },
    ];
}
pub const ICG: icg_ll_v1_2::Icg = unsafe { icg_ll_v1_2::Icg::from_ptr(0x0008_0000usize as _) };
pub const EFM2: efm2_ll_v1::Efm2 = unsafe { efm2_ll_v1::Efm2::from_ptr(0x0100_1c50usize as _) };
pub const EFM: efm_ll_v3::Efm = unsafe { efm_ll_v3::Efm::from_ptr(0x4001_0400usize as _) };
pub const AOS: aos_ll_v3::Aos = unsafe { aos_ll_v3::Aos::from_ptr(0x4001_0800usize as _) };
pub const EMB0: emb_ll_v3_1::Emb = unsafe { emb_ll_v3_1::Emb::from_ptr(0x4001_7c00usize as _) };
pub const EMB1: emb_ll_v3_1::Emb = unsafe { emb_ll_v3_1::Emb::from_ptr(0x4001_7c20usize as _) };
pub const EMB2: emb_ll_v3::Emb = unsafe { emb_ll_v3::Emb::from_ptr(0x4001_7c40usize as _) };
pub const EMB3: emb_ll_v3::Emb = unsafe { emb_ll_v3::Emb::from_ptr(0x4001_7c60usize as _) };
pub const SPI1: spi_ll_v1_6::Spi = unsafe { spi_ll_v1_6::Spi::from_ptr(0x4001_c000usize as _) };
pub const SPI2: spi_ll_v1_6::Spi = unsafe { spi_ll_v1_6::Spi::from_ptr(0x4001_c400usize as _) };
pub const USART1: usart_ll_v3_16::Usart =
    unsafe { usart_ll_v3_16::Usart::from_ptr(0x4001_cc00usize as _) };
pub const USART2: usart_ll_v3_16::Usart =
    unsafe { usart_ll_v3_16::Usart::from_ptr(0x4001_d000usize as _) };
pub const USART3: usart_ll_v3_16::Usart =
    unsafe { usart_ll_v3_16::Usart::from_ptr(0x4002_0c00usize as _) };
pub const USART4: usart_ll_v3_16::Usart =
    unsafe { usart_ll_v3_16::Usart::from_ptr(0x4002_1000usize as _) };
pub const TMR01: tmr0_ll_v2::Tmr0 = unsafe { tmr0_ll_v2::Tmr0::from_ptr(0x4002_4000usize as _) };
pub const TMR02: tmr0_ll_v2::Tmr0 = unsafe { tmr0_ll_v2::Tmr0::from_ptr(0x4002_4400usize as _) };
pub const TMRA5: tmra_ll_v3_1::Tmra =
    unsafe { tmra_ll_v3_1::Tmra::from_ptr(0x4002_6000usize as _) };
pub const TMR41: tmr4_ll_v3::Tmr4 = unsafe { tmr4_ll_v3::Tmr4::from_ptr(0x4003_8000usize as _) };
pub const TMR42: tmr4_ll_v3_1::Tmr4 =
    unsafe { tmr4_ll_v3_1::Tmr4::from_ptr(0x4003_8400usize as _) };
pub const CMP1: cmp_ll_v4_1::Cmp = unsafe { cmp_ll_v4_1::Cmp::from_ptr(0x4003_8800usize as _) };
pub const CMP2: cmp_ll_v4_1::Cmp = unsafe { cmp_ll_v4_1::Cmp::from_ptr(0x4003_8900usize as _) };
pub const TMRA1: tmra_ll_v3_1::Tmra =
    unsafe { tmra_ll_v3_1::Tmra::from_ptr(0x4003_a000usize as _) };
pub const TMRA2: tmra_ll_v3_1::Tmra =
    unsafe { tmra_ll_v3_1::Tmra::from_ptr(0x4003_a400usize as _) };
pub const TMRA3: tmra_ll_v3_1::Tmra =
    unsafe { tmra_ll_v3_1::Tmra::from_ptr(0x4003_a800usize as _) };
pub const TMRA4: tmra_ll_v3_1::Tmra =
    unsafe { tmra_ll_v3_1::Tmra::from_ptr(0x4003_ac00usize as _) };
pub const I2C: i2c_ll_v3_4::I2c = unsafe { i2c_ll_v3_4::I2c::from_ptr(0x4003_b400usize as _) };
pub const ADC1: adc_ll_v4_1::Adc = unsafe { adc_ll_v4_1::Adc::from_ptr(0x4004_0000usize as _) };
pub const ADC2: adc_ll_v4::Adc = unsafe { adc_ll_v4::Adc::from_ptr(0x4004_0400usize as _) };
pub const TRNG: trng_ll_v1_1::Trng = unsafe { trng_ll_v1_1::Trng::from_ptr(0x4004_2000usize as _) };
pub const FCM: fcm_ll_v1_2::Fcm = unsafe { fcm_ll_v1_2::Fcm::from_ptr(0x4004_8400usize as _) };
pub const WDT: wdt_ll_v1::Wdt = unsafe { wdt_ll_v1::Wdt::from_ptr(0x4004_9000usize as _) };
pub const SWDT: swdt_ll_v1_1::Swdt = unsafe { swdt_ll_v1_1::Swdt::from_ptr(0x4004_9400usize as _) };
pub const DFM: dfm_v2::Dfm = unsafe { dfm_v2::Dfm::from_ptr(0x4005_0000usize as _) };
pub const CRC: crc_ll_v2::Crc = unsafe { crc_ll_v2::Crc::from_ptr(0x4005_0400usize as _) };
pub const SRAMC: sramc_ll_v3_2::Sramc =
    unsafe { sramc_ll_v3_2::Sramc::from_ptr(0x4005_0800usize as _) };
pub const INTC: intc_ll_v1::Intc = unsafe { intc_ll_v1::Intc::from_ptr(0x4005_1000usize as _) };
pub const DMA: dma_ll_v2_1::Dma = unsafe { dma_ll_v2_1::Dma::from_ptr(0x4005_3000usize as _) };
pub const GPIO: gpio_ll_v1_4::Gpio = unsafe { gpio_ll_v1_4::Gpio::from_ptr(0x4005_3800usize as _) };
pub const CMU: cmu_ll_v2_1::Cmu = unsafe { cmu_ll_v2_1::Cmu::from_ptr(0x4005_4000usize as _) };
pub const PWC: pwc_ll_v2_1::Pwc = unsafe { pwc_ll_v2_1::Pwc::from_ptr(0x4005_4400usize as _) };
pub const RMU: rmu_ll_v3::Rmu = unsafe { rmu_ll_v3::Rmu::from_ptr(0x4005_4500usize as _) };
pub const MAU: mau_v4::Mau = unsafe { mau_v4::Mau::from_ptr(0x4005_4800usize as _) };
pub const PERIC: peric_ll_v1_4::Peric =
    unsafe { peric_ll_v1_4::Peric::from_ptr(0x4005_5400usize as _) };
pub const DBGC: dbgc_ll_v3_1::Dbgc = unsafe { dbgc_ll_v3_1::Dbgc::from_ptr(0xe004_2000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ll_v4.rs"]
pub mod adc_ll_v4;
#[path = "../../peripherals/adc_ll_v4_1.rs"]
pub mod adc_ll_v4_1;
#[path = "../../peripherals/aos_ll_v3.rs"]
pub mod aos_ll_v3;
#[path = "../../peripherals/cmp_ll_v4_1.rs"]
pub mod cmp_ll_v4_1;
#[path = "../../peripherals/cmu_ll_v2_1.rs"]
pub mod cmu_ll_v2_1;
#[path = "../../peripherals/crc_ll_v2.rs"]
pub mod crc_ll_v2;
#[path = "../../peripherals/dbgc_ll_v3_1.rs"]
pub mod dbgc_ll_v3_1;
#[path = "../../peripherals/dfm_v2.rs"]
pub mod dfm_v2;
#[path = "../../peripherals/dma_ll_v2_1.rs"]
pub mod dma_ll_v2_1;
#[path = "../../peripherals/efm2_ll_v1.rs"]
pub mod efm2_ll_v1;
#[path = "../../peripherals/efm_ll_v3.rs"]
pub mod efm_ll_v3;
#[path = "../../peripherals/emb_ll_v3.rs"]
pub mod emb_ll_v3;
#[path = "../../peripherals/emb_ll_v3_1.rs"]
pub mod emb_ll_v3_1;
#[path = "../../peripherals/fcm_ll_v1_2.rs"]
pub mod fcm_ll_v1_2;
#[path = "../../peripherals/gpio_ll_v1_4.rs"]
pub mod gpio_ll_v1_4;
#[path = "../../peripherals/i2c_ll_v3_4.rs"]
pub mod i2c_ll_v3_4;
#[path = "../../peripherals/icg_ll_v1_2.rs"]
pub mod icg_ll_v1_2;
#[path = "../../peripherals/intc_ll_v1.rs"]
pub mod intc_ll_v1;
#[path = "../../peripherals/mau_v4.rs"]
pub mod mau_v4;
#[path = "../../peripherals/peric_ll_v1_4.rs"]
pub mod peric_ll_v1_4;
#[path = "../../peripherals/pwc_ll_v2_1.rs"]
pub mod pwc_ll_v2_1;
#[path = "../../peripherals/rmu_ll_v3.rs"]
pub mod rmu_ll_v3;
#[path = "../../peripherals/spi_ll_v1_6.rs"]
pub mod spi_ll_v1_6;
#[path = "../../peripherals/sramc_ll_v3_2.rs"]
pub mod sramc_ll_v3_2;
#[path = "../../peripherals/swdt_ll_v1_1.rs"]
pub mod swdt_ll_v1_1;
#[path = "../../peripherals/tmr0_ll_v2.rs"]
pub mod tmr0_ll_v2;
#[path = "../../peripherals/tmr4_ll_v3.rs"]
pub mod tmr4_ll_v3;
#[path = "../../peripherals/tmr4_ll_v3_1.rs"]
pub mod tmr4_ll_v3_1;
#[path = "../../peripherals/tmra_ll_v3_1.rs"]
pub mod tmra_ll_v3_1;
#[path = "../../peripherals/trng_ll_v1_1.rs"]
pub mod trng_ll_v1_1;
#[path = "../../peripherals/usart_ll_v3_16.rs"]
pub mod usart_ll_v3_16;
#[path = "../../peripherals/wdt_ll_v1.rs"]
pub mod wdt_ll_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
