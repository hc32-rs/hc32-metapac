#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - PORTA"]
    PORTA = 0,
    #[doc = "1 - PORTB"]
    PORTB = 1,
    #[doc = "2 - PORTC"]
    PORTC = 2,
    #[doc = "3 - PORTD"]
    PORTD = 3,
    #[doc = "5 - ATIM3"]
    ATIM3 = 5,
    #[doc = "8 - LPUART0"]
    LPUART0 = 8,
    #[doc = "9 - LPUART1"]
    LPUART1 = 9,
    #[doc = "10 - SPI0"]
    SPI0 = 10,
    #[doc = "12 - I2C"]
    I2C = 12,
    #[doc = "15 - BTIM3_4_5"]
    BTIM3_4_5 = 15,
    #[doc = "22 - IWDT_WWDT"]
    IWDT_WWDT = 22,
    #[doc = "24 - ADC"]
    ADC = 24,
    #[doc = "28 - LVD"]
    LVD = 28,
    #[doc = "30 - FLASH"]
    FLASH = 30,
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
        fn PORTA();
        fn PORTB();
        fn PORTC();
        fn PORTD();
        fn ATIM3();
        fn LPUART0();
        fn LPUART1();
        fn SPI0();
        fn I2C();
        fn BTIM3_4_5();
        fn IWDT_WWDT();
        fn ADC();
        fn LVD();
        fn FLASH();
        fn CTRIM();
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
        Vector { _handler: PORTC },
        Vector { _handler: PORTD },
        Vector { _reserved: 0 },
        Vector { _handler: ATIM3 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: LPUART0 },
        Vector { _handler: LPUART1 },
        Vector { _handler: SPI0 },
        Vector { _reserved: 0 },
        Vector { _handler: I2C },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: BTIM3_4_5,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: IWDT_WWDT,
        },
        Vector { _reserved: 0 },
        Vector { _handler: ADC },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: LVD },
        Vector { _reserved: 0 },
        Vector { _handler: FLASH },
        Vector { _handler: CTRIM },
    ];
}
pub const LPUART0: lpuart_ddl_v3::Lpuart =
    unsafe { lpuart_ddl_v3::Lpuart::from_ptr(0x4000_0000usize as _) };
pub const I2C: i2c_ddl_v2_1::I2c = unsafe { i2c_ddl_v2_1::I2c::from_ptr(0x4000_0400usize as _) };
pub const SPI: spi_ddl_v2::Spi = unsafe { spi_ddl_v2::Spi::from_ptr(0x4000_0800usize as _) };
pub const IWDT: iwdt_ddl_v1_1::Iwdt =
    unsafe { iwdt_ddl_v1_1::Iwdt::from_ptr(0x4000_0f80usize as _) };
pub const SYSCTRL: sysctrl_ddl_v1::Sysctrl =
    unsafe { sysctrl_ddl_v1::Sysctrl::from_ptr(0x4000_2000usize as _) };
pub const ADC: adc_ddl_v1::Adc = unsafe { adc_ddl_v1::Adc::from_ptr(0x4000_2400usize as _) };
pub const LVD: lvd_ddl_v1_4::Lvd = unsafe { lvd_ddl_v1_4::Lvd::from_ptr(0x4000_2680usize as _) };
pub const LPUART1: lpuart_ddl_v3::Lpuart =
    unsafe { lpuart_ddl_v3::Lpuart::from_ptr(0x4000_4000usize as _) };
pub const CTRIM: ctrim_ddl_v1::Ctrim =
    unsafe { ctrim_ddl_v1::Ctrim::from_ptr(0x4000_5000usize as _) };
#[doc = "Register-layout view `ATIM3MODE0` of physical timer `ATIM3` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM3`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM3MODE0: atim_ddl_v1_4::Atim =
    unsafe { atim_ddl_v1_4::Atim::from_ptr(0x4000_5800usize as _) };
#[doc = "Register-layout view `ATIM3MODE1` of physical timer `ATIM3` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM3`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM3MODE1: atim_ddl_v2::Atim =
    unsafe { atim_ddl_v2::Atim::from_ptr(0x4000_5800usize as _) };
#[doc = "Register-layout view `ATIM3MODE23` of physical timer `ATIM3` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM3`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM3MODE23: atim_ddl_v4_2::Atim =
    unsafe { atim_ddl_v4_2::Atim::from_ptr(0x4000_5800usize as _) };
pub const ATIM3: *mut () = 0x4000_5800usize as _;
pub const WWDT: wwdt_ddl_v1::Wwdt = unsafe { wwdt_ddl_v1::Wwdt::from_ptr(0x4000_6800usize as _) };
pub const BTIM3: btim_ddl_v1_1::Btim =
    unsafe { btim_ddl_v1_1::Btim::from_ptr(0x4000_7400usize as _) };
pub const GTIM: gtim_ddl_v1::Gtim = unsafe { gtim_ddl_v1::Gtim::from_ptr(0x4000_7400usize as _) };
pub const BTIM4: btim_ddl_v1_1::Btim =
    unsafe { btim_ddl_v1_1::Btim::from_ptr(0x4000_7500usize as _) };
pub const BTIM5: btim_ddl_v1_1::Btim =
    unsafe { btim_ddl_v1_1::Btim::from_ptr(0x4000_7600usize as _) };
pub const FLASH: flash_ddl_v1_2::Flash =
    unsafe { flash_ddl_v1_2::Flash::from_ptr(0x4002_0000usize as _) };
pub const GPIOA: gpio_ddl_v3::Gpio = unsafe { gpio_ddl_v3::Gpio::from_ptr(0x4002_0c00usize as _) };
pub const GPIOAUX: gpioaux_ddl_v1::Gpioaux =
    unsafe { gpioaux_ddl_v1::Gpioaux::from_ptr(0x4002_0ca0usize as _) };
pub const GPIOB: gpio_ddl_v3::Gpio = unsafe { gpio_ddl_v3::Gpio::from_ptr(0x4002_0d00usize as _) };
pub const GPIOC: gpio_ddl_v3::Gpio = unsafe { gpio_ddl_v3::Gpio::from_ptr(0x4002_0e00usize as _) };
pub const GPIOD: gpio_ddl_v3::Gpio = unsafe { gpio_ddl_v3::Gpio::from_ptr(0x4002_0f00usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ddl_v1.rs"]
pub mod adc_ddl_v1;
#[path = "../../peripherals/atim_ddl_v1_4.rs"]
pub mod atim_ddl_v1_4;
#[path = "../../peripherals/atim_ddl_v2.rs"]
pub mod atim_ddl_v2;
#[path = "../../peripherals/atim_ddl_v4_2.rs"]
pub mod atim_ddl_v4_2;
#[path = "../../peripherals/btim_ddl_v1_1.rs"]
pub mod btim_ddl_v1_1;
#[path = "../../peripherals/ctrim_ddl_v1.rs"]
pub mod ctrim_ddl_v1;
#[path = "../../peripherals/flash_ddl_v1_2.rs"]
pub mod flash_ddl_v1_2;
#[path = "../../peripherals/gpio_ddl_v3.rs"]
pub mod gpio_ddl_v3;
#[path = "../../peripherals/gpioaux_ddl_v1.rs"]
pub mod gpioaux_ddl_v1;
#[path = "../../peripherals/gtim_ddl_v1.rs"]
pub mod gtim_ddl_v1;
#[path = "../../peripherals/i2c_ddl_v2_1.rs"]
pub mod i2c_ddl_v2_1;
#[path = "../../peripherals/iwdt_ddl_v1_1.rs"]
pub mod iwdt_ddl_v1_1;
#[path = "../../peripherals/lpuart_ddl_v3.rs"]
pub mod lpuart_ddl_v3;
#[path = "../../peripherals/lvd_ddl_v1_4.rs"]
pub mod lvd_ddl_v1_4;
#[path = "../../peripherals/spi_ddl_v2.rs"]
pub mod spi_ddl_v2;
#[path = "../../peripherals/sysctrl_ddl_v1.rs"]
pub mod sysctrl_ddl_v1;
#[path = "../../peripherals/wwdt_ddl_v1.rs"]
pub mod wwdt_ddl_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
