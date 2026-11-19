#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - EXTINT00_EXTINT01"]
    EXTINT00_EXTINT01 = 0,
    #[doc = "1 - EXTINT02_EXTINT03"]
    EXTINT02_EXTINT03 = 1,
    #[doc = "2 - EXTINT04_EXTINT05"]
    EXTINT04_EXTINT05 = 2,
    #[doc = "3 - EXTINT06_EXTINT07"]
    EXTINT06_EXTINT07 = 3,
    #[doc = "4 - DMA"]
    DMA = 4,
    #[doc = "5 - EFM"]
    EFM = 5,
    #[doc = "6 - TMR0"]
    TMR0 = 6,
    #[doc = "7 - TMR2"]
    TMR2 = 7,
    #[doc = "8 - TMR4_CMP_OVF_UDF_RELOAD"]
    TMR4_CMP_OVF_UDF_RELOAD = 8,
    #[doc = "9 - TMR4_SCMP"]
    TMR4_SCMP = 9,
    #[doc = "10 - TMRA"]
    TMRA = 10,
    #[doc = "11 - USART1_WKUP_RI_TI"]
    USART1_WKUP_RI_TI = 11,
    #[doc = "12 - USART1_EI_TEDNI_TCI_RTO"]
    USART1_EI_TEDNI_TCI_RTO = 12,
    #[doc = "13 - USART2_RI_TI"]
    USART2_RI_TI = 13,
    #[doc = "14 - USART2_EI_TEDNI_TCI_RTO"]
    USART2_EI_TEDNI_TCI_RTO = 14,
    #[doc = "15 - USART3_RI_TI"]
    USART3_RI_TI = 15,
    #[doc = "16 - USART3_EI_TEDNI_TCI_RTO_BRKWKP"]
    USART3_EI_TEDNI_TCI_RTO_BRKWKP = 16,
    #[doc = "17 - USART4_RI_TI"]
    USART4_RI_TI = 17,
    #[doc = "18 - USART4_EI_TEDNI_TCI_RTO_BRKWKP"]
    USART4_EI_TEDNI_TCI_RTO_BRKWKP = 18,
    #[doc = "19 - SPI1_SPRI_SPTI_SPTEND"]
    SPI1_SPRI_SPTI_SPTEND = 19,
    #[doc = "20 - SPI1_SPII_SPEI"]
    SPI1_SPII_SPEI = 20,
    #[doc = "21 - SPI2_SPRI_SPTI_SPTEND"]
    SPI2_SPRI_SPTI_SPTEND = 21,
    #[doc = "22 - SPI2_SPII_SPEI"]
    SPI2_SPII_SPEI = 22,
    #[doc = "23 - I2C_RXI_TXI"]
    I2C_RXI_TXI = 23,
    #[doc = "24 - I2C_TEI_EEI"]
    I2C_TEI_EEI = 24,
    #[doc = "25 - MCAN"]
    MCAN = 25,
    #[doc = "26 - FCM_XTAL"]
    FCM_XTAL = 26,
    #[doc = "27 - EMB"]
    EMB = 27,
    #[doc = "29 - CMP"]
    CMP = 29,
    #[doc = "30 - ADC"]
    ADC = 30,
    #[doc = "31 - EVENTPORT_RTC_CTC_SWDT"]
    EVENTPORT_RTC_CTC_SWDT = 31,
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
        fn EXTINT00_EXTINT01();
        fn EXTINT02_EXTINT03();
        fn EXTINT04_EXTINT05();
        fn EXTINT06_EXTINT07();
        fn DMA();
        fn EFM();
        fn TMR0();
        fn TMR2();
        fn TMR4_CMP_OVF_UDF_RELOAD();
        fn TMR4_SCMP();
        fn TMRA();
        fn USART1_WKUP_RI_TI();
        fn USART1_EI_TEDNI_TCI_RTO();
        fn USART2_RI_TI();
        fn USART2_EI_TEDNI_TCI_RTO();
        fn USART3_RI_TI();
        fn USART3_EI_TEDNI_TCI_RTO_BRKWKP();
        fn USART4_RI_TI();
        fn USART4_EI_TEDNI_TCI_RTO_BRKWKP();
        fn SPI1_SPRI_SPTI_SPTEND();
        fn SPI1_SPII_SPEI();
        fn SPI2_SPRI_SPTI_SPTEND();
        fn SPI2_SPII_SPEI();
        fn I2C_RXI_TXI();
        fn I2C_TEI_EEI();
        fn MCAN();
        fn FCM_XTAL();
        fn EMB();
        fn CMP();
        fn ADC();
        fn EVENTPORT_RTC_CTC_SWDT();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 32] = [
        Vector {
            _handler: EXTINT00_EXTINT01,
        },
        Vector {
            _handler: EXTINT02_EXTINT03,
        },
        Vector {
            _handler: EXTINT04_EXTINT05,
        },
        Vector {
            _handler: EXTINT06_EXTINT07,
        },
        Vector { _handler: DMA },
        Vector { _handler: EFM },
        Vector { _handler: TMR0 },
        Vector { _handler: TMR2 },
        Vector {
            _handler: TMR4_CMP_OVF_UDF_RELOAD,
        },
        Vector {
            _handler: TMR4_SCMP,
        },
        Vector { _handler: TMRA },
        Vector {
            _handler: USART1_WKUP_RI_TI,
        },
        Vector {
            _handler: USART1_EI_TEDNI_TCI_RTO,
        },
        Vector {
            _handler: USART2_RI_TI,
        },
        Vector {
            _handler: USART2_EI_TEDNI_TCI_RTO,
        },
        Vector {
            _handler: USART3_RI_TI,
        },
        Vector {
            _handler: USART3_EI_TEDNI_TCI_RTO_BRKWKP,
        },
        Vector {
            _handler: USART4_RI_TI,
        },
        Vector {
            _handler: USART4_EI_TEDNI_TCI_RTO_BRKWKP,
        },
        Vector {
            _handler: SPI1_SPRI_SPTI_SPTEND,
        },
        Vector {
            _handler: SPI1_SPII_SPEI,
        },
        Vector {
            _handler: SPI2_SPRI_SPTI_SPTEND,
        },
        Vector {
            _handler: SPI2_SPII_SPEI,
        },
        Vector {
            _handler: I2C_RXI_TXI,
        },
        Vector {
            _handler: I2C_TEI_EEI,
        },
        Vector { _handler: MCAN },
        Vector { _handler: FCM_XTAL },
        Vector { _handler: EMB },
        Vector { _reserved: 0 },
        Vector { _handler: CMP },
        Vector { _handler: ADC },
        Vector {
            _handler: EVENTPORT_RTC_CTC_SWDT,
        },
    ];
}
pub const ICGA: icga_ll_v1::Icga = unsafe { icga_ll_v1::Icga::from_ptr(0x0100_0a00usize as _) };
pub const CTC: ctc_ll_v1_2::Ctc = unsafe { ctc_ll_v1_2::Ctc::from_ptr(0x4000_0000usize as _) };
pub const EFM: efm_ll_v4::Efm = unsafe { efm_ll_v4::Efm::from_ptr(0x4000_0400usize as _) };
pub const AOS: aos_ll_v5::Aos = unsafe { aos_ll_v5::Aos::from_ptr(0x4000_0c00usize as _) };
pub const USART1: usart_ll_v3_9::Usart =
    unsafe { usart_ll_v3_9::Usart::from_ptr(0x4000_1800usize as _) };
pub const USART2: usart_ll_v3_9::Usart =
    unsafe { usart_ll_v3_9::Usart::from_ptr(0x4000_1c00usize as _) };
pub const USART3: usart_ll_v3_14::Usart =
    unsafe { usart_ll_v3_14::Usart::from_ptr(0x4000_2000usize as _) };
pub const USART4: usart_ll_v3_14::Usart =
    unsafe { usart_ll_v3_14::Usart::from_ptr(0x4000_2400usize as _) };
pub const SPI1: spi_ll_v1_1::Spi = unsafe { spi_ll_v1_1::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI2: spi_ll_v1_1::Spi = unsafe { spi_ll_v1_1::Spi::from_ptr(0x4000_3c00usize as _) };
pub const I2C: i2c_ll_v2::I2c = unsafe { i2c_ll_v2::I2c::from_ptr(0x4000_4800usize as _) };
pub const FCM: fcm_ll_v1_2::Fcm = unsafe { fcm_ll_v1_2::Fcm::from_ptr(0x4000_5400usize as _) };
pub const TMR0: tmr0_ll_v1_2::Tmr0 = unsafe { tmr0_ll_v1_2::Tmr0::from_ptr(0x4000_5800usize as _) };
pub const TMR21: tmr2_ll_v2_3::Tmr2 =
    unsafe { tmr2_ll_v2_3::Tmr2::from_ptr(0x4000_5c00usize as _) };
pub const TMR22: tmr2_ll_v2_3::Tmr2 =
    unsafe { tmr2_ll_v2_3::Tmr2::from_ptr(0x4000_6000usize as _) };
pub const TMR4: tmr4_ll_v1_4::Tmr4 = unsafe { tmr4_ll_v1_4::Tmr4::from_ptr(0x4000_6800usize as _) };
pub const EMB: emb_ll_v2_4::Emb = unsafe { emb_ll_v2_4::Emb::from_ptr(0x4000_6c00usize as _) };
pub const TMRA: tmra_ll_v4_1::Tmra = unsafe { tmra_ll_v4_1::Tmra::from_ptr(0x4000_7000usize as _) };
pub const ADC: adc_ll_v1_3::Adc = unsafe { adc_ll_v1_3::Adc::from_ptr(0x4000_b800usize as _) };
pub const CMP: cmp_ll_v4::Cmp = unsafe { cmp_ll_v4::Cmp::from_ptr(0x4000_c800usize as _) };
pub const SWDT: swdt_ll_v1_1::Swdt = unsafe { swdt_ll_v1_1::Swdt::from_ptr(0x4000_cc00usize as _) };
pub const RTC: rtc_ll_v1_3::Rtc = unsafe { rtc_ll_v1_3::Rtc::from_ptr(0x4000_d400usize as _) };
pub const SRAMC: sramc_ll_v2::Sramc =
    unsafe { sramc_ll_v2::Sramc::from_ptr(0x4001_0c00usize as _) };
pub const INTC: intc_ll_v1_1::Intc = unsafe { intc_ll_v1_1::Intc::from_ptr(0x4001_1000usize as _) };
pub const MCAN: mcan_ll_v1::Mcan = unsafe { mcan_ll_v1::Mcan::from_ptr(0x4001_1400usize as _) };
pub const MCANRAM: mcanram_ll_v1_2::Mcanram =
    unsafe { mcanram_ll_v1_2::Mcanram::from_ptr(0x4001_1800usize as _) };
pub const DMA: dma_ll_v1_2::Dma = unsafe { dma_ll_v1_2::Dma::from_ptr(0x4001_3000usize as _) };
pub const PERIC: peric_ll_v1::Peric =
    unsafe { peric_ll_v1::Peric::from_ptr(0x4001_3400usize as _) };
pub const GPIO: gpio_ll_v1_6::Gpio = unsafe { gpio_ll_v1_6::Gpio::from_ptr(0x4001_3800usize as _) };
pub const PWC: pwc_ll_v2::Pwc = unsafe { pwc_ll_v2::Pwc::from_ptr(0x4001_4000usize as _) };
pub const RMU: rmu_ll_v1_1::Rmu = unsafe { rmu_ll_v1_1::Rmu::from_ptr(0x4001_4100usize as _) };
pub const CMU: cmu_ll_v1_4::Cmu = unsafe { cmu_ll_v1_4::Cmu::from_ptr(0x4001_4400usize as _) };
pub const DBGC: dbgc_ll_v1::Dbgc = unsafe { dbgc_ll_v1::Dbgc::from_ptr(0x4001_5000usize as _) };
pub const CRC: crc_ll_v1::Crc = unsafe { crc_ll_v1::Crc::from_ptr(0x4001_5400usize as _) };
pub const ICGB: icgb_ll_v1::Icgb = unsafe { icgb_ll_v1::Icgb::from_ptr(0x4001_7800usize as _) };
pub const DBGC_T: dbgc_t_ll_v1_1::DbgcT =
    unsafe { dbgc_t_ll_v1_1::DbgcT::from_ptr(0xe004_2000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ll_v1_3.rs"]
pub mod adc_ll_v1_3;
#[path = "../../peripherals/aos_ll_v5.rs"]
pub mod aos_ll_v5;
#[path = "../../peripherals/cmp_ll_v4.rs"]
pub mod cmp_ll_v4;
#[path = "../../peripherals/cmu_ll_v1_4.rs"]
pub mod cmu_ll_v1_4;
#[path = "../../peripherals/crc_ll_v1.rs"]
pub mod crc_ll_v1;
#[path = "../../peripherals/ctc_ll_v1_2.rs"]
pub mod ctc_ll_v1_2;
#[path = "../../peripherals/dbgc_ll_v1.rs"]
pub mod dbgc_ll_v1;
#[path = "../../peripherals/dbgc_t_ll_v1_1.rs"]
pub mod dbgc_t_ll_v1_1;
#[path = "../../peripherals/dma_ll_v1_2.rs"]
pub mod dma_ll_v1_2;
#[path = "../../peripherals/efm_ll_v4.rs"]
pub mod efm_ll_v4;
#[path = "../../peripherals/emb_ll_v2_4.rs"]
pub mod emb_ll_v2_4;
#[path = "../../peripherals/fcm_ll_v1_2.rs"]
pub mod fcm_ll_v1_2;
#[path = "../../peripherals/gpio_ll_v1_6.rs"]
pub mod gpio_ll_v1_6;
#[path = "../../peripherals/i2c_ll_v2.rs"]
pub mod i2c_ll_v2;
#[path = "../../peripherals/icga_ll_v1.rs"]
pub mod icga_ll_v1;
#[path = "../../peripherals/icgb_ll_v1.rs"]
pub mod icgb_ll_v1;
#[path = "../../peripherals/intc_ll_v1_1.rs"]
pub mod intc_ll_v1_1;
#[path = "../../peripherals/mcan_ll_v1.rs"]
pub mod mcan_ll_v1;
#[path = "../../peripherals/mcanram_ll_v1_2.rs"]
pub mod mcanram_ll_v1_2;
#[path = "../../peripherals/peric_ll_v1.rs"]
pub mod peric_ll_v1;
#[path = "../../peripherals/pwc_ll_v2.rs"]
pub mod pwc_ll_v2;
#[path = "../../peripherals/rmu_ll_v1_1.rs"]
pub mod rmu_ll_v1_1;
#[path = "../../peripherals/rtc_ll_v1_3.rs"]
pub mod rtc_ll_v1_3;
#[path = "../../peripherals/spi_ll_v1_1.rs"]
pub mod spi_ll_v1_1;
#[path = "../../peripherals/sramc_ll_v2.rs"]
pub mod sramc_ll_v2;
#[path = "../../peripherals/swdt_ll_v1_1.rs"]
pub mod swdt_ll_v1_1;
#[path = "../../peripherals/tmr0_ll_v1_2.rs"]
pub mod tmr0_ll_v1_2;
#[path = "../../peripherals/tmr2_ll_v2_3.rs"]
pub mod tmr2_ll_v2_3;
#[path = "../../peripherals/tmr4_ll_v1_4.rs"]
pub mod tmr4_ll_v1_4;
#[path = "../../peripherals/tmra_ll_v4_1.rs"]
pub mod tmra_ll_v4_1;
#[path = "../../peripherals/usart_ll_v3_14.rs"]
pub mod usart_ll_v3_14;
#[path = "../../peripherals/usart_ll_v3_9.rs"]
pub mod usart_ll_v3_9;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
