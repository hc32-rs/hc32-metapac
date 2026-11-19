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
    #[doc = "4 - DMAC"]
    DMAC = 4,
    #[doc = "5 - ATIM3"]
    ATIM3 = 5,
    #[doc = "6 - USART0"]
    USART0 = 6,
    #[doc = "7 - USART1"]
    USART1 = 7,
    #[doc = "8 - LPUART0"]
    LPUART0 = 8,
    #[doc = "9 - LPUART1"]
    LPUART1 = 9,
    #[doc = "10 - SPI0"]
    SPI0 = 10,
    #[doc = "11 - SPI1"]
    SPI1 = 11,
    #[doc = "12 - I2C0"]
    I2C0 = 12,
    #[doc = "13 - I2C1"]
    I2C1 = 13,
    #[doc = "14 - ATIM0"]
    ATIM0 = 14,
    #[doc = "15 - ATIM1"]
    ATIM1 = 15,
    #[doc = "16 - ATIM2"]
    ATIM2 = 16,
    #[doc = "17 - LPTIM"]
    LPTIM = 17,
    #[doc = "18 - GTIM0_BTIM0_1_2"]
    GTIM0_BTIM0_1_2 = 18,
    #[doc = "19 - GTIM1_BTIM3_4_5"]
    GTIM1_BTIM3_4_5 = 19,
    #[doc = "20 - I2CSLV"]
    I2CSLV = 20,
    #[doc = "21 - WWDT"]
    WWDT = 21,
    #[doc = "22 - IWDT"]
    IWDT = 22,
    #[doc = "23 - RTC"]
    RTC = 23,
    #[doc = "24 - ADC"]
    ADC = 24,
    #[doc = "26 - VC0_VC1_LVD"]
    VC0_VC1_LVD = 26,
    #[doc = "28 - CAN"]
    CAN = 28,
    #[doc = "29 - ATIM_BK"]
    ATIM_BK = 29,
    #[doc = "30 - FLASH_RAM"]
    FLASH_RAM = 30,
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
        fn ATIM0();
        fn ATIM1();
        fn ATIM2();
        fn LPTIM();
        fn GTIM0_BTIM0_1_2();
        fn GTIM1_BTIM3_4_5();
        fn I2CSLV();
        fn WWDT();
        fn IWDT();
        fn RTC();
        fn ADC();
        fn VC0_VC1_LVD();
        fn CAN();
        fn ATIM_BK();
        fn FLASH_RAM();
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
        Vector { _handler: ATIM0 },
        Vector { _handler: ATIM1 },
        Vector { _handler: ATIM2 },
        Vector { _handler: LPTIM },
        Vector {
            _handler: GTIM0_BTIM0_1_2,
        },
        Vector {
            _handler: GTIM1_BTIM3_4_5,
        },
        Vector { _handler: I2CSLV },
        Vector { _handler: WWDT },
        Vector { _handler: IWDT },
        Vector { _handler: RTC },
        Vector { _handler: ADC },
        Vector { _reserved: 0 },
        Vector {
            _handler: VC0_VC1_LVD,
        },
        Vector { _reserved: 0 },
        Vector { _handler: CAN },
        Vector { _handler: ATIM_BK },
        Vector {
            _handler: FLASH_RAM,
        },
        Vector { _handler: CTRIM },
    ];
}
pub const LPUART0: lpuart_ddl_v3_1::Lpuart =
    unsafe { lpuart_ddl_v3_1::Lpuart::from_ptr(0x4000_0000usize as _) };
pub const I2C0: i2c_ddl_v2::I2c = unsafe { i2c_ddl_v2::I2c::from_ptr(0x4000_0400usize as _) };
pub const SPI0: spi_ddl_v2_2::Spi = unsafe { spi_ddl_v2_2::Spi::from_ptr(0x4000_0800usize as _) };
pub const LPTIM: lptim_ddl_v1_2::Lptim =
    unsafe { lptim_ddl_v1_2::Lptim::from_ptr(0x4000_0c00usize as _) };
pub const IWDT: iwdt_ddl_v1::Iwdt = unsafe { iwdt_ddl_v1::Iwdt::from_ptr(0x4000_1080usize as _) };
pub const RTC: rtc_ddl_v2::Rtc = unsafe { rtc_ddl_v2::Rtc::from_ptr(0x4000_1400usize as _) };
pub const BTIM0: btim_ddl_v1::Btim = unsafe { btim_ddl_v1::Btim::from_ptr(0x4000_1800usize as _) };
pub const GTIM0: gtim_ddl_v1_2::Gtim =
    unsafe { gtim_ddl_v1_2::Gtim::from_ptr(0x4000_1800usize as _) };
pub const BTIM1: btim_ddl_v1::Btim = unsafe { btim_ddl_v1::Btim::from_ptr(0x4000_1900usize as _) };
pub const BTIM2: btim_ddl_v1::Btim = unsafe { btim_ddl_v1::Btim::from_ptr(0x4000_1a00usize as _) };
pub const BTIM3: btim_ddl_v1_1::Btim =
    unsafe { btim_ddl_v1_1::Btim::from_ptr(0x4000_1c00usize as _) };
pub const GTIM1: gtim_ddl_v1_2::Gtim =
    unsafe { gtim_ddl_v1_2::Gtim::from_ptr(0x4000_1c00usize as _) };
pub const BTIM4: btim_ddl_v1_1::Btim =
    unsafe { btim_ddl_v1_1::Btim::from_ptr(0x4000_1d00usize as _) };
pub const BTIM5: btim_ddl_v1_1::Btim =
    unsafe { btim_ddl_v1_1::Btim::from_ptr(0x4000_1e00usize as _) };
pub const GPIOAUX: gpioaux_ddl_v1_1::Gpioaux =
    unsafe { gpioaux_ddl_v1_1::Gpioaux::from_ptr(0x4000_2000usize as _) };
pub const SYSCTRL: sysctrl_ddl_v3_6::Sysctrl =
    unsafe { sysctrl_ddl_v3_6::Sysctrl::from_ptr(0x4000_2000usize as _) };
pub const VC0: vc_ddl_v1_1::Vc = unsafe { vc_ddl_v1_1::Vc::from_ptr(0x4000_2400usize as _) };
pub const VC1: vc_ddl_v1_1::Vc = unsafe { vc_ddl_v1_1::Vc::from_ptr(0x4000_2410usize as _) };
pub const LVD: lvd_ddl_v1_2::Lvd = unsafe { lvd_ddl_v1_2::Lvd::from_ptr(0x4000_2480usize as _) };
pub const ADC: adc_ddl_v5::Adc = unsafe { adc_ddl_v5::Adc::from_ptr(0x4000_2500usize as _) };
pub const OPA: opa_ddl_v3::Opa = unsafe { opa_ddl_v3::Opa::from_ptr(0x4000_2600usize as _) };
#[doc = "Register-layout view `ATIM0MODE0` of physical timer `ATIM0` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM0`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM0MODE0: atim_ddl_v1::Atim =
    unsafe { atim_ddl_v1::Atim::from_ptr(0x4000_3000usize as _) };
#[doc = "Register-layout view `ATIM0MODE1` of physical timer `ATIM0` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM0`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM0MODE1: atim_ddl_v2_4::Atim =
    unsafe { atim_ddl_v2_4::Atim::from_ptr(0x4000_3000usize as _) };
#[doc = "Register-layout view `ATIM0MODE23` of physical timer `ATIM0` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM0`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM0MODE23: atim_ddl_v3::Atim =
    unsafe { atim_ddl_v3::Atim::from_ptr(0x4000_3000usize as _) };
pub const ATIM0: *mut () = 0x4000_3000usize as _;
#[doc = "Register-layout view `ATIM1MODE0` of physical timer `ATIM1` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM1`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM1MODE0: atim_ddl_v1::Atim =
    unsafe { atim_ddl_v1::Atim::from_ptr(0x4000_3400usize as _) };
#[doc = "Register-layout view `ATIM1MODE1` of physical timer `ATIM1` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM1`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM1MODE1: atim_ddl_v2_4::Atim =
    unsafe { atim_ddl_v2_4::Atim::from_ptr(0x4000_3400usize as _) };
#[doc = "Register-layout view `ATIM1MODE23` of physical timer `ATIM1` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM1`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM1MODE23: atim_ddl_v3::Atim =
    unsafe { atim_ddl_v3::Atim::from_ptr(0x4000_3400usize as _) };
pub const ATIM1: *mut () = 0x4000_3400usize as _;
#[doc = "Register-layout view `ATIM2MODE0` of physical timer `ATIM2` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM2`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM2MODE0: atim_ddl_v1::Atim =
    unsafe { atim_ddl_v1::Atim::from_ptr(0x4000_3800usize as _) };
#[doc = "Register-layout view `ATIM2MODE1` of physical timer `ATIM2` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM2`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM2MODE1: atim_ddl_v2_4::Atim =
    unsafe { atim_ddl_v2_4::Atim::from_ptr(0x4000_3800usize as _) };
#[doc = "Register-layout view `ATIM2MODE23` of physical timer `ATIM2` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM2`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM2MODE23: atim_ddl_v3::Atim =
    unsafe { atim_ddl_v3::Atim::from_ptr(0x4000_3800usize as _) };
pub const ATIM2: *mut () = 0x4000_3800usize as _;
pub const LPUART1: lpuart_ddl_v3_1::Lpuart =
    unsafe { lpuart_ddl_v3_1::Lpuart::from_ptr(0x4000_4000usize as _) };
pub const I2C1: i2c_ddl_v2::I2c = unsafe { i2c_ddl_v2::I2c::from_ptr(0x4000_4400usize as _) };
pub const SPI1: spi_ddl_v2_2::Spi = unsafe { spi_ddl_v2_2::Spi::from_ptr(0x4000_4800usize as _) };
pub const I2CSLV: i2cslv_ddl_v1::I2cslv =
    unsafe { i2cslv_ddl_v1::I2cslv::from_ptr(0x4000_4c00usize as _) };
pub const CTRIM: ctrim_ddl_v1_3::Ctrim =
    unsafe { ctrim_ddl_v1_3::Ctrim::from_ptr(0x4000_5000usize as _) };
#[doc = "Register-layout view `ATIM3MODE0` of physical timer `ATIM3` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM3`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM3MODE0: atim_ddl_v1_3::Atim =
    unsafe { atim_ddl_v1_3::Atim::from_ptr(0x4000_5800usize as _) };
#[doc = "Register-layout view `ATIM3MODE1` of physical timer `ATIM3` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM3`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM3MODE1: atim_ddl_v2_5::Atim =
    unsafe { atim_ddl_v2_5::Atim::from_ptr(0x4000_5800usize as _) };
#[doc = "Register-layout view `ATIM3MODE23` of physical timer `ATIM3` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `ATIM3`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const ATIM3MODE23: atim_ddl_v4_3::Atim =
    unsafe { atim_ddl_v4_3::Atim::from_ptr(0x4000_5800usize as _) };
pub const ATIM3: *mut () = 0x4000_5800usize as _;
pub const USART0: usart_ddl_v1::Usart =
    unsafe { usart_ddl_v1::Usart::from_ptr(0x4000_6000usize as _) };
pub const USART1: usart_ddl_v1::Usart =
    unsafe { usart_ddl_v1::Usart::from_ptr(0x4000_6400usize as _) };
pub const WWDT: wwdt_ddl_v1::Wwdt = unsafe { wwdt_ddl_v1::Wwdt::from_ptr(0x4000_6800usize as _) };
pub const FLASH: flash_ddl_v1_3::Flash =
    unsafe { flash_ddl_v1_3::Flash::from_ptr(0x4002_0000usize as _) };
pub const RAM: ram_ddl_v3::Ram = unsafe { ram_ddl_v3::Ram::from_ptr(0x4002_0400usize as _) };
pub const CRC: crc_ddl_v2::Crc = unsafe { crc_ddl_v2::Crc::from_ptr(0x4002_0800usize as _) };
pub const GPIOA: gpio_ddl_v3_1::Gpio =
    unsafe { gpio_ddl_v3_1::Gpio::from_ptr(0x4002_0c00usize as _) };
pub const DMAC: dmac_ddl_v2::Dmac = unsafe { dmac_ddl_v2::Dmac::from_ptr(0x4002_1000usize as _) };
pub const EAU: eau_ddl_v1::Eau = unsafe { eau_ddl_v1::Eau::from_ptr(0x4002_1800usize as _) };
pub const GPIOB: gpio_ddl_v3_1::Gpio =
    unsafe { gpio_ddl_v3_1::Gpio::from_ptr(0x4002_1c00usize as _) };
pub const GPIOC: gpio_ddl_v3_1::Gpio =
    unsafe { gpio_ddl_v3_1::Gpio::from_ptr(0x4002_2000usize as _) };
pub const GPIOD: gpio_ddl_v3_1::Gpio =
    unsafe { gpio_ddl_v3_1::Gpio::from_ptr(0x4002_2400usize as _) };
pub const CAN: can_ddl_v1::Can = unsafe { can_ddl_v1::Can::from_ptr(0x4003_0000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ddl_v5.rs"]
pub mod adc_ddl_v5;
#[path = "../../peripherals/atim_ddl_v1.rs"]
pub mod atim_ddl_v1;
#[path = "../../peripherals/atim_ddl_v1_3.rs"]
pub mod atim_ddl_v1_3;
#[path = "../../peripherals/atim_ddl_v2_4.rs"]
pub mod atim_ddl_v2_4;
#[path = "../../peripherals/atim_ddl_v2_5.rs"]
pub mod atim_ddl_v2_5;
#[path = "../../peripherals/atim_ddl_v3.rs"]
pub mod atim_ddl_v3;
#[path = "../../peripherals/atim_ddl_v4_3.rs"]
pub mod atim_ddl_v4_3;
#[path = "../../peripherals/btim_ddl_v1.rs"]
pub mod btim_ddl_v1;
#[path = "../../peripherals/btim_ddl_v1_1.rs"]
pub mod btim_ddl_v1_1;
#[path = "../../peripherals/can_ddl_v1.rs"]
pub mod can_ddl_v1;
#[path = "../../peripherals/crc_ddl_v2.rs"]
pub mod crc_ddl_v2;
#[path = "../../peripherals/ctrim_ddl_v1_3.rs"]
pub mod ctrim_ddl_v1_3;
#[path = "../../peripherals/dmac_ddl_v2.rs"]
pub mod dmac_ddl_v2;
#[path = "../../peripherals/eau_ddl_v1.rs"]
pub mod eau_ddl_v1;
#[path = "../../peripherals/flash_ddl_v1_3.rs"]
pub mod flash_ddl_v1_3;
#[path = "../../peripherals/gpio_ddl_v3_1.rs"]
pub mod gpio_ddl_v3_1;
#[path = "../../peripherals/gpioaux_ddl_v1_1.rs"]
pub mod gpioaux_ddl_v1_1;
#[path = "../../peripherals/gtim_ddl_v1_2.rs"]
pub mod gtim_ddl_v1_2;
#[path = "../../peripherals/i2c_ddl_v2.rs"]
pub mod i2c_ddl_v2;
#[path = "../../peripherals/i2cslv_ddl_v1.rs"]
pub mod i2cslv_ddl_v1;
#[path = "../../peripherals/iwdt_ddl_v1.rs"]
pub mod iwdt_ddl_v1;
#[path = "../../peripherals/lptim_ddl_v1_2.rs"]
pub mod lptim_ddl_v1_2;
#[path = "../../peripherals/lpuart_ddl_v3_1.rs"]
pub mod lpuart_ddl_v3_1;
#[path = "../../peripherals/lvd_ddl_v1_2.rs"]
pub mod lvd_ddl_v1_2;
#[path = "../../peripherals/opa_ddl_v3.rs"]
pub mod opa_ddl_v3;
#[path = "../../peripherals/ram_ddl_v3.rs"]
pub mod ram_ddl_v3;
#[path = "../../peripherals/rtc_ddl_v2.rs"]
pub mod rtc_ddl_v2;
#[path = "../../peripherals/spi_ddl_v2_2.rs"]
pub mod spi_ddl_v2_2;
#[path = "../../peripherals/sysctrl_ddl_v3_6.rs"]
pub mod sysctrl_ddl_v3_6;
#[path = "../../peripherals/usart_ddl_v1.rs"]
pub mod usart_ddl_v1;
#[path = "../../peripherals/vc_ddl_v1_1.rs"]
pub mod vc_ddl_v1_1;
#[path = "../../peripherals/wwdt_ddl_v1.rs"]
pub mod wwdt_ddl_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
