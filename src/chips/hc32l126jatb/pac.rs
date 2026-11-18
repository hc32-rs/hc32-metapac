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
    #[doc = "5 - TIM3"]
    TIM3 = 5,
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
    #[doc = "14 - TIM0"]
    TIM0 = 14,
    #[doc = "15 - TIM1"]
    TIM1 = 15,
    #[doc = "16 - TIM2"]
    TIM2 = 16,
    #[doc = "17 - LPTIM_0_1"]
    LPTIM_0_1 = 17,
    #[doc = "18 - HSI2C"]
    HSI2C = 18,
    #[doc = "19 - LPUART2"]
    LPUART2 = 19,
    #[doc = "21 - WWDT"]
    WWDT = 21,
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
    #[doc = "29 - LCD"]
    LCD = 29,
    #[doc = "30 - FLASH_RAM"]
    FLASH_RAM = 30,
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
        fn PORTC();
        fn PORTD();
        fn DMAC();
        fn TIM3();
        fn USART0();
        fn USART1();
        fn LPUART0();
        fn LPUART1();
        fn SPI0();
        fn SPI1();
        fn I2C0();
        fn I2C1();
        fn TIM0();
        fn TIM1();
        fn TIM2();
        fn LPTIM_0_1();
        fn HSI2C();
        fn LPUART2();
        fn WWDT();
        fn IWDT();
        fn RTC();
        fn ADC();
        fn VC0();
        fn VC1();
        fn LVD();
        fn LCD();
        fn FLASH_RAM();
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
        Vector { _handler: PORTC },
        Vector { _handler: PORTD },
        Vector { _handler: DMAC },
        Vector { _handler: TIM3 },
        Vector { _handler: USART0 },
        Vector { _handler: USART1 },
        Vector { _handler: LPUART0 },
        Vector { _handler: LPUART1 },
        Vector { _handler: SPI0 },
        Vector { _handler: SPI1 },
        Vector { _handler: I2C0 },
        Vector { _handler: I2C1 },
        Vector { _handler: TIM0 },
        Vector { _handler: TIM1 },
        Vector { _handler: TIM2 },
        Vector {
            _handler: LPTIM_0_1,
        },
        Vector { _handler: HSI2C },
        Vector { _handler: LPUART2 },
        Vector { _reserved: 0 },
        Vector { _handler: WWDT },
        Vector { _handler: IWDT },
        Vector { _handler: RTC },
        Vector { _handler: ADC },
        Vector { _reserved: 0 },
        Vector { _handler: VC0 },
        Vector { _handler: VC1 },
        Vector { _handler: LVD },
        Vector { _handler: LCD },
        Vector {
            _handler: FLASH_RAM,
        },
        Vector {
            _handler: CTRIM_CLKDET,
        },
    ];
}
pub const LPUART0: lpuart_ddl_v4_1::Lpuart =
    unsafe { lpuart_ddl_v4_1::Lpuart::from_ptr(0x4000_0000usize as _) };
pub const I2C0: i2c_ddl_v2_3::I2c = unsafe { i2c_ddl_v2_3::I2c::from_ptr(0x4000_0400usize as _) };
pub const SPI0: spi_ddl_v2_2::Spi = unsafe { spi_ddl_v2_2::Spi::from_ptr(0x4000_0800usize as _) };
pub const LPTIMER0: lptim_ddl_v2::Lptim =
    unsafe { lptim_ddl_v2::Lptim::from_ptr(0x4000_0c00usize as _) };
pub const IWDT: iwdt_ddl_v1_3::Iwdt =
    unsafe { iwdt_ddl_v1_3::Iwdt::from_ptr(0x4000_1080usize as _) };
pub const RTC: rtc_ddl_v2_1::Rtc = unsafe { rtc_ddl_v2_1::Rtc::from_ptr(0x4000_1400usize as _) };
pub const ADC: adc_ddl_v6::Adc = unsafe { adc_ddl_v6::Adc::from_ptr(0x4000_2400usize as _) };
pub const VC0: vc_ddl_v1_2::Vc = unsafe { vc_ddl_v1_2::Vc::from_ptr(0x4000_2800usize as _) };
pub const VC1: vc_ddl_v1_2::Vc = unsafe { vc_ddl_v1_2::Vc::from_ptr(0x4000_2810usize as _) };
pub const VCTRIM: vctrim_ddl_v1::Vctrim =
    unsafe { vctrim_ddl_v1::Vctrim::from_ptr(0x4000_2820usize as _) };
pub const LVD: lvd_ddl_v1_6::Lvd = unsafe { lvd_ddl_v1_6::Lvd::from_ptr(0x4000_2c00usize as _) };
#[doc = "Register-layout view `TIM0_MODE0` of physical timer `TIM0` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM0`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM0_MODE0: tim_ddl_v2::Tim = unsafe { tim_ddl_v2::Tim::from_ptr(0x4000_3000usize as _) };
#[doc = "Register-layout view `TIM0_MODE1` of physical timer `TIM0` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM0`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM0_MODE1: tim_ddl_v3_4::Tim =
    unsafe { tim_ddl_v3_4::Tim::from_ptr(0x4000_3000usize as _) };
#[doc = "Register-layout view `TIM0_MODE23` of physical timer `TIM0` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM0`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM0_MODE23: tim_ddl_v4_6::Tim =
    unsafe { tim_ddl_v4_6::Tim::from_ptr(0x4000_3000usize as _) };
pub const TIM0: *mut () = 0x4000_3000usize as _;
#[doc = "Register-layout view `TIM1_MODE0` of physical timer `TIM1` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM1`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM1_MODE0: tim_ddl_v2::Tim = unsafe { tim_ddl_v2::Tim::from_ptr(0x4000_3400usize as _) };
#[doc = "Register-layout view `TIM1_MODE1` of physical timer `TIM1` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM1`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM1_MODE1: tim_ddl_v3_4::Tim =
    unsafe { tim_ddl_v3_4::Tim::from_ptr(0x4000_3400usize as _) };
#[doc = "Register-layout view `TIM1_MODE23` of physical timer `TIM1` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM1`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM1_MODE23: tim_ddl_v4_6::Tim =
    unsafe { tim_ddl_v4_6::Tim::from_ptr(0x4000_3400usize as _) };
pub const TIM1: *mut () = 0x4000_3400usize as _;
#[doc = "Register-layout view `TIM2_MODE0` of physical timer `TIM2` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM2`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM2_MODE0: tim_ddl_v2::Tim = unsafe { tim_ddl_v2::Tim::from_ptr(0x4000_3800usize as _) };
#[doc = "Register-layout view `TIM2_MODE1` of physical timer `TIM2` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM2`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM2_MODE1: tim_ddl_v3_4::Tim =
    unsafe { tim_ddl_v3_4::Tim::from_ptr(0x4000_3800usize as _) };
#[doc = "Register-layout view `TIM2_MODE23` of physical timer `TIM2` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM2`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM2_MODE23: tim_ddl_v4_6::Tim =
    unsafe { tim_ddl_v4_6::Tim::from_ptr(0x4000_3800usize as _) };
pub const TIM2: *mut () = 0x4000_3800usize as _;
pub const LPTIMER1: lptim_ddl_v2::Lptim =
    unsafe { lptim_ddl_v2::Lptim::from_ptr(0x4000_3c00usize as _) };
pub const LPUART1: lpuart_ddl_v4_1::Lpuart =
    unsafe { lpuart_ddl_v4_1::Lpuart::from_ptr(0x4000_8000usize as _) };
pub const I2C1: i2c_ddl_v2_3::I2c = unsafe { i2c_ddl_v2_3::I2c::from_ptr(0x4000_8400usize as _) };
pub const SPI1: spi_ddl_v2_2::Spi = unsafe { spi_ddl_v2_2::Spi::from_ptr(0x4000_8800usize as _) };
pub const TRNG: trng_ddl_v1_1::Trng =
    unsafe { trng_ddl_v1_1::Trng::from_ptr(0x4000_8c00usize as _) };
pub const CTRIM: ctrim_ddl_v1_2::Ctrim =
    unsafe { ctrim_ddl_v1_2::Ctrim::from_ptr(0x4000_9000usize as _) };
#[doc = "Register-layout view `TIM3_MODE0` of physical timer `TIM3` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM3`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM3_MODE0: tim_ddl_v2::Tim = unsafe { tim_ddl_v2::Tim::from_ptr(0x4000_9800usize as _) };
#[doc = "Register-layout view `TIM3_MODE1` of physical timer `TIM3` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM3`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM3_MODE1: tim_ddl_v3_4::Tim =
    unsafe { tim_ddl_v3_4::Tim::from_ptr(0x4000_9800usize as _) };
#[doc = "Register-layout view `TIM3_MODE23` of physical timer `TIM3` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM3`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM3_MODE23: tim_ddl_v5_3::Tim =
    unsafe { tim_ddl_v5_3::Tim::from_ptr(0x4000_9800usize as _) };
pub const TIM3: *mut () = 0x4000_9800usize as _;
pub const LCD: lcd_ddl_v2::Lcd = unsafe { lcd_ddl_v2::Lcd::from_ptr(0x4000_9c00usize as _) };
pub const USART0: usart_ddl_v1::Usart =
    unsafe { usart_ddl_v1::Usart::from_ptr(0x4000_a000usize as _) };
pub const USART1: usart_ddl_v1::Usart =
    unsafe { usart_ddl_v1::Usart::from_ptr(0x4000_a400usize as _) };
pub const LPUART2: lpuart_ddl_v4_1::Lpuart =
    unsafe { lpuart_ddl_v4_1::Lpuart::from_ptr(0x4000_b400usize as _) };
pub const WWDT: wwdt_ddl_v1::Wwdt = unsafe { wwdt_ddl_v1::Wwdt::from_ptr(0x4000_b800usize as _) };
pub const HSI2C: hsi2c_ddl_v1_1::Hsi2c =
    unsafe { hsi2c_ddl_v1_1::Hsi2c::from_ptr(0x4000_bc00usize as _) };
pub const FLASH: flash_ddl_v1_1::Flash =
    unsafe { flash_ddl_v1_1::Flash::from_ptr(0x4002_0000usize as _) };
pub const RAM: ram_ddl_v3_1::Ram = unsafe { ram_ddl_v3_1::Ram::from_ptr(0x4002_0400usize as _) };
pub const CRC: crc_ddl_v2_3::Crc = unsafe { crc_ddl_v2_3::Crc::from_ptr(0x4002_0800usize as _) };
pub const GPIO: gpio_ddl_v3_4::Gpio =
    unsafe { gpio_ddl_v3_4::Gpio::from_ptr(0x4002_0c00usize as _) };
pub const DMAC: dmac_ddl_v1::Dmac = unsafe { dmac_ddl_v1::Dmac::from_ptr(0x4002_1000usize as _) };
pub const AES: aes_ddl_v2_1::Aes = unsafe { aes_ddl_v2_1::Aes::from_ptr(0x4002_1400usize as _) };
pub const SYSCTRL: sysctrl_ddl_v2_1::Sysctrl =
    unsafe { sysctrl_ddl_v2_1::Sysctrl::from_ptr(0x4002_1800usize as _) };
pub const RESET: reset_ddl_v1_3::Reset =
    unsafe { reset_ddl_v1_3::Reset::from_ptr(0x4002_181cusize as _) };
pub const DEBUG_ACTIVE: debug_active_ddl_v1_1::DebugActive =
    unsafe { debug_active_ddl_v1_1::DebugActive::from_ptr(0x4002_1838usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ddl_v6.rs"]
pub mod adc_ddl_v6;
#[path = "../../peripherals/aes_ddl_v2_1.rs"]
pub mod aes_ddl_v2_1;
#[path = "../../peripherals/crc_ddl_v2_3.rs"]
pub mod crc_ddl_v2_3;
#[path = "../../peripherals/ctrim_ddl_v1_2.rs"]
pub mod ctrim_ddl_v1_2;
#[path = "../../peripherals/debug_active_ddl_v1_1.rs"]
pub mod debug_active_ddl_v1_1;
#[path = "../../peripherals/dmac_ddl_v1.rs"]
pub mod dmac_ddl_v1;
#[path = "../../peripherals/flash_ddl_v1_1.rs"]
pub mod flash_ddl_v1_1;
#[path = "../../peripherals/gpio_ddl_v3_4.rs"]
pub mod gpio_ddl_v3_4;
#[path = "../../peripherals/hsi2c_ddl_v1_1.rs"]
pub mod hsi2c_ddl_v1_1;
#[path = "../../peripherals/i2c_ddl_v2_3.rs"]
pub mod i2c_ddl_v2_3;
#[path = "../../peripherals/iwdt_ddl_v1_3.rs"]
pub mod iwdt_ddl_v1_3;
#[path = "../../peripherals/lcd_ddl_v2.rs"]
pub mod lcd_ddl_v2;
#[path = "../../peripherals/lptim_ddl_v2.rs"]
pub mod lptim_ddl_v2;
#[path = "../../peripherals/lpuart_ddl_v4_1.rs"]
pub mod lpuart_ddl_v4_1;
#[path = "../../peripherals/lvd_ddl_v1_6.rs"]
pub mod lvd_ddl_v1_6;
#[path = "../../peripherals/ram_ddl_v3_1.rs"]
pub mod ram_ddl_v3_1;
#[path = "../../peripherals/reset_ddl_v1_3.rs"]
pub mod reset_ddl_v1_3;
#[path = "../../peripherals/rtc_ddl_v2_1.rs"]
pub mod rtc_ddl_v2_1;
#[path = "../../peripherals/spi_ddl_v2_2.rs"]
pub mod spi_ddl_v2_2;
#[path = "../../peripherals/sysctrl_ddl_v2_1.rs"]
pub mod sysctrl_ddl_v2_1;
#[path = "../../peripherals/tim_ddl_v2.rs"]
pub mod tim_ddl_v2;
#[path = "../../peripherals/tim_ddl_v3_4.rs"]
pub mod tim_ddl_v3_4;
#[path = "../../peripherals/tim_ddl_v4_6.rs"]
pub mod tim_ddl_v4_6;
#[path = "../../peripherals/tim_ddl_v5_3.rs"]
pub mod tim_ddl_v5_3;
#[path = "../../peripherals/trng_ddl_v1_1.rs"]
pub mod trng_ddl_v1_1;
#[path = "../../peripherals/usart_ddl_v1.rs"]
pub mod usart_ddl_v1;
#[path = "../../peripherals/vc_ddl_v1_2.rs"]
pub mod vc_ddl_v1_2;
#[path = "../../peripherals/vctrim_ddl_v1.rs"]
pub mod vctrim_ddl_v1;
#[path = "../../peripherals/wwdt_ddl_v1.rs"]
pub mod wwdt_ddl_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
