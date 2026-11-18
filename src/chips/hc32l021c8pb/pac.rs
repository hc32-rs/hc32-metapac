#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - PORTA"]
    PORTA = 0,
    #[doc = "1 - PORTB"]
    PORTB = 1,
    #[doc = "5 - ATIM3"]
    ATIM3 = 5,
    #[doc = "8 - LPUART0"]
    LPUART0 = 8,
    #[doc = "9 - LPUART1"]
    LPUART1 = 9,
    #[doc = "10 - SPI"]
    SPI = 10,
    #[doc = "14 - CTIM0"]
    CTIM0 = 14,
    #[doc = "15 - CTIM1"]
    CTIM1 = 15,
    #[doc = "18 - HSI2C"]
    HSI2C = 18,
    #[doc = "22 - IWDT"]
    IWDT = 22,
    #[doc = "23 - RTC"]
    RTC = 23,
    #[doc = "24 - ADC"]
    ADC = 24,
    #[doc = "26 - VC0"]
    VC0 = 26,
    #[doc = "27 - VC1"]
    VC1 = 27,
    #[doc = "28 - LVD"]
    LVD = 28,
    #[doc = "30 - FLASH"]
    FLASH = 30,
    #[doc = "31 - CTRIM_CLKDET"]
    CTRIM_CLKDET = 31,
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
        fn PORTA();
        fn PORTB();
        fn ATIM3();
        fn LPUART0();
        fn LPUART1();
        fn SPI();
        fn CTIM0();
        fn CTIM1();
        fn HSI2C();
        fn IWDT();
        fn RTC();
        fn ADC();
        fn VC0();
        fn VC1();
        fn LVD();
        fn FLASH();
        fn CTRIM_CLKDET();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 32] = [
        Vector { _handler: PORTA },
        Vector { _handler: PORTB },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: ATIM3 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: LPUART0 },
        Vector { _handler: LPUART1 },
        Vector { _handler: SPI },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: CTIM0 },
        Vector { _handler: CTIM1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: HSI2C },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: IWDT },
        Vector { _handler: RTC },
        Vector { _handler: ADC },
        Vector { _reserved: 0 },
        Vector { _handler: VC0 },
        Vector { _handler: VC1 },
        Vector { _handler: LVD },
        Vector { _reserved: 0 },
        Vector { _handler: FLASH },
        Vector {
            _handler: CTRIM_CLKDET,
        },
    ];
}
pub const LPUART0: lpuart_ddl_v4::Lpuart =
    unsafe { lpuart_ddl_v4::Lpuart::from_ptr(0x4000_0000usize as _) };
pub const SPI: spi_ddl_v2_1::Spi = unsafe { spi_ddl_v2_1::Spi::from_ptr(0x4000_0800usize as _) };
pub const IWDT: iwdt_ddl_v1_2::Iwdt =
    unsafe { iwdt_ddl_v1_2::Iwdt::from_ptr(0x4000_1000usize as _) };
pub const RTC: rtc_ddl_v2_1::Rtc = unsafe { rtc_ddl_v2_1::Rtc::from_ptr(0x4000_1400usize as _) };
pub const BTIM0: btim_ddl_v1::Btim = unsafe { btim_ddl_v1::Btim::from_ptr(0x4000_1800usize as _) };
pub const GTIM0: gtim_ddl_v1_1::Gtim =
    unsafe { gtim_ddl_v1_1::Gtim::from_ptr(0x4000_1800usize as _) };
pub const BTIM1: btim_ddl_v1::Btim = unsafe { btim_ddl_v1::Btim::from_ptr(0x4000_1900usize as _) };
pub const BTIM2: btim_ddl_v1::Btim = unsafe { btim_ddl_v1::Btim::from_ptr(0x4000_1a00usize as _) };
pub const BTIM3: btim_ddl_v1::Btim = unsafe { btim_ddl_v1::Btim::from_ptr(0x4000_1c00usize as _) };
pub const GTIM1: gtim_ddl_v1_1::Gtim =
    unsafe { gtim_ddl_v1_1::Gtim::from_ptr(0x4000_1c00usize as _) };
pub const BTIM4: btim_ddl_v1::Btim = unsafe { btim_ddl_v1::Btim::from_ptr(0x4000_1d00usize as _) };
pub const BTIM5: btim_ddl_v1::Btim = unsafe { btim_ddl_v1::Btim::from_ptr(0x4000_1e00usize as _) };
pub const ADC: adc_ddl_v2::Adc = unsafe { adc_ddl_v2::Adc::from_ptr(0x4000_2400usize as _) };
pub const VC: vc_ddl_v2_3::Vc = unsafe { vc_ddl_v2_3::Vc::from_ptr(0x4000_2800usize as _) };
pub const LVD: lvd_ddl_v1_5::Lvd = unsafe { lvd_ddl_v1_5::Lvd::from_ptr(0x4000_2c00usize as _) };
pub const LPUART1: lpuart_ddl_v4::Lpuart =
    unsafe { lpuart_ddl_v4::Lpuart::from_ptr(0x4000_8000usize as _) };
pub const TRNG: trng_ddl_v1::Trng = unsafe { trng_ddl_v1::Trng::from_ptr(0x4000_8c00usize as _) };
pub const CTRIM: ctrim_ddl_v1_2::Ctrim =
    unsafe { ctrim_ddl_v1_2::Ctrim::from_ptr(0x4000_9000usize as _) };
pub const ATIM3: *mut () = 0x4000_9800usize as _;
pub const HSI2C: hsi2c_ddl_v1::Hsi2c =
    unsafe { hsi2c_ddl_v1::Hsi2c::from_ptr(0x4000_bc00usize as _) };
pub const FLASH: flash_ddl_v1::Flash =
    unsafe { flash_ddl_v1::Flash::from_ptr(0x4002_0000usize as _) };
pub const GPIOA: gpio_ddl_v3_2::Gpio =
    unsafe { gpio_ddl_v3_2::Gpio::from_ptr(0x4002_0c00usize as _) };
pub const GPIOAUX: gpioaux_ddl_v1_3::Gpioaux =
    unsafe { gpioaux_ddl_v1_3::Gpioaux::from_ptr(0x4002_0c00usize as _) };
pub const GPIOB: gpio_ddl_v1::Gpio = unsafe { gpio_ddl_v1::Gpio::from_ptr(0x4002_0c40usize as _) };
pub const SYSCTRL: sysctrl_ddl_v2_3::Sysctrl =
    unsafe { sysctrl_ddl_v2_3::Sysctrl::from_ptr(0x4002_1800usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ddl_v2.rs"]
pub mod adc_ddl_v2;
#[path = "../../peripherals/btim_ddl_v1.rs"]
pub mod btim_ddl_v1;
#[path = "../../peripherals/ctrim_ddl_v1_2.rs"]
pub mod ctrim_ddl_v1_2;
#[path = "../../peripherals/flash_ddl_v1.rs"]
pub mod flash_ddl_v1;
#[path = "../../peripherals/gpio_ddl_v1.rs"]
pub mod gpio_ddl_v1;
#[path = "../../peripherals/gpio_ddl_v3_2.rs"]
pub mod gpio_ddl_v3_2;
#[path = "../../peripherals/gpioaux_ddl_v1_3.rs"]
pub mod gpioaux_ddl_v1_3;
#[path = "../../peripherals/gtim_ddl_v1_1.rs"]
pub mod gtim_ddl_v1_1;
#[path = "../../peripherals/hsi2c_ddl_v1.rs"]
pub mod hsi2c_ddl_v1;
#[path = "../../peripherals/iwdt_ddl_v1_2.rs"]
pub mod iwdt_ddl_v1_2;
#[path = "../../peripherals/lpuart_ddl_v4.rs"]
pub mod lpuart_ddl_v4;
#[path = "../../peripherals/lvd_ddl_v1_5.rs"]
pub mod lvd_ddl_v1_5;
#[path = "../../peripherals/rtc_ddl_v2_1.rs"]
pub mod rtc_ddl_v2_1;
#[path = "../../peripherals/spi_ddl_v2_1.rs"]
pub mod spi_ddl_v2_1;
#[path = "../../peripherals/sysctrl_ddl_v2_3.rs"]
pub mod sysctrl_ddl_v2_3;
#[path = "../../peripherals/trng_ddl_v1.rs"]
pub mod trng_ddl_v1;
#[path = "../../peripherals/vc_ddl_v2_3.rs"]
pub mod vc_ddl_v2_3;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
