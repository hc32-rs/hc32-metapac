#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - WWDT"]
    WWDT = 0,
    #[doc = "1 - IWDT"]
    IWDT = 1,
    #[doc = "2 - LVD"]
    LVD = 2,
    #[doc = "3 - INPUT0_3"]
    INPUT0_3 = 3,
    #[doc = "4 - INPUT4_7"]
    INPUT4_7 = 4,
    #[doc = "5 - INPUT8_11"]
    INPUT8_11 = 5,
    #[doc = "6 - INPUT12_15"]
    INPUT12_15 = 6,
    #[doc = "7 - DMAC"]
    DMAC = 7,
    #[doc = "8 - ATIM3"]
    ATIM3 = 8,
    #[doc = "9 - USART0"]
    USART0 = 9,
    #[doc = "10 - USART1"]
    USART1 = 10,
    #[doc = "11 - LPUART0"]
    LPUART0 = 11,
    #[doc = "12 - LPUART1"]
    LPUART1 = 12,
    #[doc = "13 - SPI0"]
    SPI0 = 13,
    #[doc = "14 - SPI1"]
    SPI1 = 14,
    #[doc = "15 - I2C0"]
    I2C0 = 15,
    #[doc = "16 - I2C1"]
    I2C1 = 16,
    #[doc = "17 - GTIM0_1"]
    GTIM0_1 = 17,
    #[doc = "18 - GTIM2_3"]
    GTIM2_3 = 18,
    #[doc = "19 - GTIM4_5"]
    GTIM4_5 = 19,
    #[doc = "20 - GTIM6_7_8"]
    GTIM6_7_8 = 20,
    #[doc = "21 - LPTIM0_1"]
    LPTIM0_1 = 21,
    #[doc = "22 - USART2"]
    USART2 = 22,
    #[doc = "23 - USART3"]
    USART3 = 23,
    #[doc = "24 - ADC"]
    ADC = 24,
    #[doc = "25 - USART4"]
    USART4 = 25,
    #[doc = "26 - USART5"]
    USART5 = 26,
    #[doc = "27 - WKUPIO"]
    WKUPIO = 27,
    #[doc = "28 - RTC"]
    RTC = 28,
    #[doc = "30 - EFM_DFM"]
    EFM_DFM = 30,
    #[doc = "31 - CTRIM"]
    CTRIM = 31,
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
        fn WWDT();
        fn IWDT();
        fn LVD();
        fn INPUT0_3();
        fn INPUT4_7();
        fn INPUT8_11();
        fn INPUT12_15();
        fn DMAC();
        fn ATIM3();
        fn USART0();
        fn USART1();
        fn LPUART0();
        fn LPUART1();
        fn SPI0();
        fn SPI1();
        fn I2C0();
        fn I2C1();
        fn GTIM0_1();
        fn GTIM2_3();
        fn GTIM4_5();
        fn GTIM6_7_8();
        fn LPTIM0_1();
        fn USART2();
        fn USART3();
        fn ADC();
        fn USART4();
        fn USART5();
        fn WKUPIO();
        fn RTC();
        fn EFM_DFM();
        fn CTRIM();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 32] = [
        Vector { _handler: WWDT },
        Vector { _handler: IWDT },
        Vector { _handler: LVD },
        Vector { _handler: INPUT0_3 },
        Vector { _handler: INPUT4_7 },
        Vector {
            _handler: INPUT8_11,
        },
        Vector {
            _handler: INPUT12_15,
        },
        Vector { _handler: DMAC },
        Vector { _handler: ATIM3 },
        Vector { _handler: USART0 },
        Vector { _handler: USART1 },
        Vector { _handler: LPUART0 },
        Vector { _handler: LPUART1 },
        Vector { _handler: SPI0 },
        Vector { _handler: SPI1 },
        Vector { _handler: I2C0 },
        Vector { _handler: I2C1 },
        Vector { _handler: GTIM0_1 },
        Vector { _handler: GTIM2_3 },
        Vector { _handler: GTIM4_5 },
        Vector {
            _handler: GTIM6_7_8,
        },
        Vector { _handler: LPTIM0_1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: ADC },
        Vector { _handler: USART4 },
        Vector { _handler: USART5 },
        Vector { _handler: WKUPIO },
        Vector { _handler: RTC },
        Vector { _reserved: 0 },
        Vector { _handler: EFM_DFM },
        Vector { _handler: CTRIM },
    ];
}
pub const CTRIM: ctrim_ll_v1::Ctrim =
    unsafe { ctrim_ll_v1::Ctrim::from_ptr(0x4000_0000usize as _) };
pub const IWDT: iwdt_ll_v1::Iwdt = unsafe { iwdt_ll_v1::Iwdt::from_ptr(0x4000_0800usize as _) };
pub const WWDT: wwdt_ll_v1::Wwdt = unsafe { wwdt_ll_v1::Wwdt::from_ptr(0x4000_0c00usize as _) };
pub const RTC: rtc_ll_v2::Rtc = unsafe { rtc_ll_v2::Rtc::from_ptr(0x4000_1000usize as _) };
pub const DBGC: dbgc_ll_v1::Dbgc = unsafe { dbgc_ll_v1::Dbgc::from_ptr(0x4000_1c00usize as _) };
pub const USART0: usart_ll_v2_15::Usart =
    unsafe { usart_ll_v2_15::Usart::from_ptr(0x4000_2400usize as _) };
pub const USART1: usart_ll_v2_15::Usart =
    unsafe { usart_ll_v2_15::Usart::from_ptr(0x4000_2400usize as _) };
pub const USART2: usart_ll_v2_15::Usart =
    unsafe { usart_ll_v2_15::Usart::from_ptr(0x4000_2400usize as _) };
pub const USART3: usart_ll_v2_15::Usart =
    unsafe { usart_ll_v2_15::Usart::from_ptr(0x4000_2400usize as _) };
pub const USART4: usart_ll_v2_15::Usart =
    unsafe { usart_ll_v2_15::Usart::from_ptr(0x4000_2400usize as _) };
pub const USART5: usart_ll_v2_15::Usart =
    unsafe { usart_ll_v2_15::Usart::from_ptr(0x4000_2400usize as _) };
pub const LPUART0: lpuart_ll_v1::Lpuart =
    unsafe { lpuart_ll_v1::Lpuart::from_ptr(0x4000_3800usize as _) };
pub const LPUART1: lpuart_ll_v1::Lpuart =
    unsafe { lpuart_ll_v1::Lpuart::from_ptr(0x4000_3800usize as _) };
pub const SPI0: spi_ll_v1::Spi = unsafe { spi_ll_v1::Spi::from_ptr(0x4000_4000usize as _) };
pub const SPI1: spi_ll_v1::Spi = unsafe { spi_ll_v1::Spi::from_ptr(0x4000_4000usize as _) };
pub const I2C0: i2c_ll_v1::I2c = unsafe { i2c_ll_v1::I2c::from_ptr(0x4000_4800usize as _) };
pub const I2C1: i2c_ll_v1::I2c = unsafe { i2c_ll_v1::I2c::from_ptr(0x4000_4800usize as _) };
pub const LPTIM0: lptim_ll_v1::Lptim =
    unsafe { lptim_ll_v1::Lptim::from_ptr(0x4000_5000usize as _) };
pub const LPTIM1: lptim_ll_v1::Lptim =
    unsafe { lptim_ll_v1::Lptim::from_ptr(0x4000_5000usize as _) };
pub const GTIM0: gtim_ll_v1::Gtim = unsafe { gtim_ll_v1::Gtim::from_ptr(0x4000_5800usize as _) };
pub const GTIM1: gtim_ll_v1::Gtim = unsafe { gtim_ll_v1::Gtim::from_ptr(0x4000_5800usize as _) };
pub const GTIM2: gtim_ll_v1::Gtim = unsafe { gtim_ll_v1::Gtim::from_ptr(0x4000_5800usize as _) };
pub const GTIM3: gtim_ll_v1::Gtim = unsafe { gtim_ll_v1::Gtim::from_ptr(0x4000_5800usize as _) };
pub const GTIM4: gtim_ll_v1::Gtim = unsafe { gtim_ll_v1::Gtim::from_ptr(0x4000_5800usize as _) };
pub const GTIM5: gtim_ll_v1::Gtim = unsafe { gtim_ll_v1::Gtim::from_ptr(0x4000_5800usize as _) };
pub const GTIM6: gtim_ll_v1::Gtim = unsafe { gtim_ll_v1::Gtim::from_ptr(0x4000_5800usize as _) };
pub const GTIM7: gtim_ll_v1::Gtim = unsafe { gtim_ll_v1::Gtim::from_ptr(0x4000_5800usize as _) };
pub const GTIM8: gtim_ll_v1::Gtim = unsafe { gtim_ll_v1::Gtim::from_ptr(0x4000_5800usize as _) };
pub const LVD: lvd_ll_v1::Lvd = unsafe { lvd_ll_v1::Lvd::from_ptr(0x4000_7400usize as _) };
pub const ADC: adc_ll_v1::Adc = unsafe { adc_ll_v1::Adc::from_ptr(0x4000_7800usize as _) };
pub const ATIM3: atim_ll_v1::Atim = unsafe { atim_ll_v1::Atim::from_ptr(0x4000_e400usize as _) };
pub const GPIO: gpio_ll_v1::Gpio = unsafe { gpio_ll_v1::Gpio::from_ptr(0x4002_0000usize as _) };
pub const EXTI: exti_ll_v1::Exti = unsafe { exti_ll_v1::Exti::from_ptr(0x4002_4000usize as _) };
pub const DMAC: dmac_ll_v1::Dmac = unsafe { dmac_ll_v1::Dmac::from_ptr(0x4002_4800usize as _) };
pub const SRAM: sram_ll_v1::Sram = unsafe { sram_ll_v1::Sram::from_ptr(0x4002_4c00usize as _) };
pub const SYSCTRL: sysctrl_ll_v1::Sysctrl =
    unsafe { sysctrl_ll_v1::Sysctrl::from_ptr(0x4002_5000usize as _) };
pub const CSU0: csu_ll_v1::Csu = unsafe { csu_ll_v1::Csu::from_ptr(0x4002_5100usize as _) };
pub const CSU1: csu_ll_v1::Csu = unsafe { csu_ll_v1::Csu::from_ptr(0x4002_5100usize as _) };
pub const CSU2: csu_ll_v1::Csu = unsafe { csu_ll_v1::Csu::from_ptr(0x4002_5100usize as _) };
pub const EFM: efm_ll_v3::Efm = unsafe { efm_ll_v3::Efm::from_ptr(0x4002_6400usize as _) };
pub const DFM: dfm_ll_v1::Dfm = unsafe { dfm_ll_v1::Dfm::from_ptr(0x4002_6800usize as _) };
pub const CRC: crc_ll_v1::Crc = unsafe { crc_ll_v1::Crc::from_ptr(0x4002_7800usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ll_v1.rs"]
pub mod adc_ll_v1;
#[path = "../../peripherals/atim_ll_v1.rs"]
pub mod atim_ll_v1;
#[path = "../../peripherals/crc_ll_v1.rs"]
pub mod crc_ll_v1;
#[path = "../../peripherals/csu_ll_v1.rs"]
pub mod csu_ll_v1;
#[path = "../../peripherals/ctrim_ll_v1.rs"]
pub mod ctrim_ll_v1;
#[path = "../../peripherals/dbgc_ll_v1.rs"]
pub mod dbgc_ll_v1;
#[path = "../../peripherals/dfm_ll_v1.rs"]
pub mod dfm_ll_v1;
#[path = "../../peripherals/dmac_ll_v1.rs"]
pub mod dmac_ll_v1;
#[path = "../../peripherals/efm_ll_v3.rs"]
pub mod efm_ll_v3;
#[path = "../../peripherals/exti_ll_v1.rs"]
pub mod exti_ll_v1;
#[path = "../../peripherals/gpio_ll_v1.rs"]
pub mod gpio_ll_v1;
#[path = "../../peripherals/gtim_ll_v1.rs"]
pub mod gtim_ll_v1;
#[path = "../../peripherals/i2c_ll_v1.rs"]
pub mod i2c_ll_v1;
#[path = "../../peripherals/iwdt_ll_v1.rs"]
pub mod iwdt_ll_v1;
#[path = "../../peripherals/lptim_ll_v1.rs"]
pub mod lptim_ll_v1;
#[path = "../../peripherals/lpuart_ll_v1.rs"]
pub mod lpuart_ll_v1;
#[path = "../../peripherals/lvd_ll_v1.rs"]
pub mod lvd_ll_v1;
#[path = "../../peripherals/rtc_ll_v2.rs"]
pub mod rtc_ll_v2;
#[path = "../../peripherals/spi_ll_v1.rs"]
pub mod spi_ll_v1;
#[path = "../../peripherals/sram_ll_v1.rs"]
pub mod sram_ll_v1;
#[path = "../../peripherals/sysctrl_ll_v1.rs"]
pub mod sysctrl_ll_v1;
#[path = "../../peripherals/usart_ll_v2_15.rs"]
pub mod usart_ll_v2_15;
#[path = "../../peripherals/wwdt_ll_v1.rs"]
pub mod wwdt_ll_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
