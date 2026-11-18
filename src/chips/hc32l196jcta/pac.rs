#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - PORTA"]
    PORTA = 0,
    #[doc = "1 - PORTB"]
    PORTB = 1,
    #[doc = "2 - PORTC_E"]
    PORTC_E = 2,
    #[doc = "3 - PORTD_F"]
    PORTD_F = 3,
    #[doc = "4 - DMAC"]
    DMAC = 4,
    #[doc = "5 - TIM3"]
    TIM3 = 5,
    #[doc = "6 - UART0_2"]
    UART0_2 = 6,
    #[doc = "7 - UART1_3"]
    UART1_3 = 7,
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
    #[doc = "18 - ADTIM4"]
    ADTIM4 = 18,
    #[doc = "19 - ADTIM5"]
    ADTIM5 = 19,
    #[doc = "20 - ADTIM6"]
    ADTIM6 = 20,
    #[doc = "21 - PCA"]
    PCA = 21,
    #[doc = "22 - WDT"]
    WDT = 22,
    #[doc = "23 - RTC"]
    RTC = 23,
    #[doc = "24 - ADC_DAC"]
    ADC_DAC = 24,
    #[doc = "25 - PCNT"]
    PCNT = 25,
    #[doc = "26 - VC0"]
    VC0 = 26,
    #[doc = "27 - VC1_2"]
    VC1_2 = 27,
    #[doc = "28 - LVD"]
    LVD = 28,
    #[doc = "29 - LCD"]
    LCD = 29,
    #[doc = "30 - FLASH_RAM"]
    FLASH_RAM = 30,
    #[doc = "31 - CLKTRIM"]
    CLKTRIM = 31,
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
        fn PORTC_E();
        fn PORTD_F();
        fn DMAC();
        fn TIM3();
        fn UART0_2();
        fn UART1_3();
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
        fn ADTIM4();
        fn ADTIM5();
        fn ADTIM6();
        fn PCA();
        fn WDT();
        fn RTC();
        fn ADC_DAC();
        fn PCNT();
        fn VC0();
        fn VC1_2();
        fn LVD();
        fn LCD();
        fn FLASH_RAM();
        fn CLKTRIM();
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
        Vector { _handler: PORTC_E },
        Vector { _handler: PORTD_F },
        Vector { _handler: DMAC },
        Vector { _handler: TIM3 },
        Vector { _handler: UART0_2 },
        Vector { _handler: UART1_3 },
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
        Vector { _handler: ADTIM4 },
        Vector { _handler: ADTIM5 },
        Vector { _handler: ADTIM6 },
        Vector { _handler: PCA },
        Vector { _handler: WDT },
        Vector { _handler: RTC },
        Vector { _handler: ADC_DAC },
        Vector { _handler: PCNT },
        Vector { _handler: VC0 },
        Vector { _handler: VC1_2 },
        Vector { _handler: LVD },
        Vector { _handler: LCD },
        Vector {
            _handler: FLASH_RAM,
        },
        Vector { _handler: CLKTRIM },
    ];
}
pub const UART0: uart_ddl_v3::Uart = unsafe { uart_ddl_v3::Uart::from_ptr(0x4000_0000usize as _) };
pub const UART1: uart_ddl_v3::Uart = unsafe { uart_ddl_v3::Uart::from_ptr(0x4000_0100usize as _) };
pub const LPUART0: lpuart_ddl_v3_2::Lpuart =
    unsafe { lpuart_ddl_v3_2::Lpuart::from_ptr(0x4000_0200usize as _) };
pub const I2C0: i2c_ddl_v1_1::I2c = unsafe { i2c_ddl_v1_1::I2c::from_ptr(0x4000_0400usize as _) };
pub const SPI0: spi_ddl_v1_2::Spi = unsafe { spi_ddl_v1_2::Spi::from_ptr(0x4000_0800usize as _) };
#[doc = "Register-layout view `TIM0_MODE0` of physical timer `TIM0` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM0`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM0_MODE0: tim_ddl_v2::Tim = unsafe { tim_ddl_v2::Tim::from_ptr(0x4000_0c00usize as _) };
#[doc = "Register-layout view `TIM0_MODE1` of physical timer `TIM0` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM0`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM0_MODE1: tim_ddl_v3_1::Tim =
    unsafe { tim_ddl_v3_1::Tim::from_ptr(0x4000_0c00usize as _) };
#[doc = "Register-layout view `TIM0_MODE23` of physical timer `TIM0` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM0`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM0_MODE23: tim_ddl_v4::Tim =
    unsafe { tim_ddl_v4::Tim::from_ptr(0x4000_0c00usize as _) };
pub const TIM0: *mut () = 0x4000_0c00usize as _;
#[doc = "Register-layout view `TIM1_MODE0` of physical timer `TIM1` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM1`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM1_MODE0: tim_ddl_v2_2::Tim =
    unsafe { tim_ddl_v2_2::Tim::from_ptr(0x4000_0d00usize as _) };
#[doc = "Register-layout view `TIM1_MODE1` of physical timer `TIM1` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM1`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM1_MODE1: tim_ddl_v3_2::Tim =
    unsafe { tim_ddl_v3_2::Tim::from_ptr(0x4000_0d00usize as _) };
#[doc = "Register-layout view `TIM1_MODE23` of physical timer `TIM1` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM1`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM1_MODE23: tim_ddl_v4_1::Tim =
    unsafe { tim_ddl_v4_1::Tim::from_ptr(0x4000_0d00usize as _) };
pub const TIM1: *mut () = 0x4000_0d00usize as _;
#[doc = "Register-layout view `TIM2_MODE0` of physical timer `TIM2` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM2`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM2_MODE0: tim_ddl_v2_3::Tim =
    unsafe { tim_ddl_v2_3::Tim::from_ptr(0x4000_0e00usize as _) };
#[doc = "Register-layout view `TIM2_MODE1` of physical timer `TIM2` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM2`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM2_MODE1: tim_ddl_v3_3::Tim =
    unsafe { tim_ddl_v3_3::Tim::from_ptr(0x4000_0e00usize as _) };
#[doc = "Register-layout view `TIM2_MODE23` of physical timer `TIM2` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM2`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM2_MODE23: tim_ddl_v4_2::Tim =
    unsafe { tim_ddl_v4_2::Tim::from_ptr(0x4000_0e00usize as _) };
pub const TIM2: *mut () = 0x4000_0e00usize as _;
pub const LPTIMER0: lptim_ddl_v1_3::Lptim =
    unsafe { lptim_ddl_v1_3::Lptim::from_ptr(0x4000_0f00usize as _) };
pub const WDT: wdt_ddl_v1_1::Wdt = unsafe { wdt_ddl_v1_1::Wdt::from_ptr(0x4000_0f00usize as _) };
pub const LPTIMER1: lptim_ddl_v1_3::Lptim =
    unsafe { lptim_ddl_v1_3::Lptim::from_ptr(0x4000_0f40usize as _) };
pub const PCA: pca_ddl_v1_2::Pca = unsafe { pca_ddl_v1_2::Pca::from_ptr(0x4000_1000usize as _) };
pub const RTC: rtc_ddl_v1_2::Rtc = unsafe { rtc_ddl_v1_2::Rtc::from_ptr(0x4000_1400usize as _) };
pub const CLK_TRIM: clk_trim_ddl_v1_2::ClkTrim =
    unsafe { clk_trim_ddl_v1_2::ClkTrim::from_ptr(0x4000_1800usize as _) };
pub const SYSCTRL: sysctrl_ddl_v3_2::Sysctrl =
    unsafe { sysctrl_ddl_v3_2::Sysctrl::from_ptr(0x4000_2000usize as _) };
pub const RESET: reset_ddl_v1_4::Reset =
    unsafe { reset_ddl_v1_4::Reset::from_ptr(0x4000_201cusize as _) };
pub const DEBUG_ACTIVE: debug_active_ddl_v1_3::DebugActive =
    unsafe { debug_active_ddl_v1_3::DebugActive::from_ptr(0x4000_2038usize as _) };
pub const ADC: adc_ddl_v7_3::Adc = unsafe { adc_ddl_v7_3::Adc::from_ptr(0x4000_2400usize as _) };
pub const BGR: bgr_ddl_v1_1::Bgr = unsafe { bgr_ddl_v1_1::Bgr::from_ptr(0x4000_2400usize as _) };
pub const LVD: lvd_ddl_v1_3::Lvd = unsafe { lvd_ddl_v1_3::Lvd::from_ptr(0x4000_2400usize as _) };
pub const OPA: opa_ddl_v2::Opa = unsafe { opa_ddl_v2::Opa::from_ptr(0x4000_2400usize as _) };
pub const VC: vc_ddl_v3::Vc = unsafe { vc_ddl_v3::Vc::from_ptr(0x4000_2400usize as _) };
pub const DAC: dac_ddl_v1::Dac = unsafe { dac_ddl_v1::Dac::from_ptr(0x4000_2500usize as _) };
pub const ADTIM4: adt_ddl_v2_1::Adt = unsafe { adt_ddl_v2_1::Adt::from_ptr(0x4000_3000usize as _) };
pub const ADTIM5: adt_ddl_v2_1::Adt = unsafe { adt_ddl_v2_1::Adt::from_ptr(0x4000_3400usize as _) };
pub const ADTIM6: adt_ddl_v2_1::Adt = unsafe { adt_ddl_v2_1::Adt::from_ptr(0x4000_3800usize as _) };
pub const LPUART1: lpuart_ddl_v3_2::Lpuart =
    unsafe { lpuart_ddl_v3_2::Lpuart::from_ptr(0x4000_4000usize as _) };
pub const I2C1: i2c_ddl_v1_1::I2c = unsafe { i2c_ddl_v1_1::I2c::from_ptr(0x4000_4400usize as _) };
pub const SPI1: spi_ddl_v1_2::Spi = unsafe { spi_ddl_v1_2::Spi::from_ptr(0x4000_4800usize as _) };
pub const TRNG: trng_ddl_v1::Trng = unsafe { trng_ddl_v1::Trng::from_ptr(0x4000_4c00usize as _) };
pub const PCNT: pcnt_ddl_v1_1::Pcnt =
    unsafe { pcnt_ddl_v1_1::Pcnt::from_ptr(0x4000_5400usize as _) };
#[doc = "Register-layout view `TIM3_MODE0` of physical timer `TIM3` (mode MODE0, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM3`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM3_MODE0: tim_ddl_v2_1::Tim =
    unsafe { tim_ddl_v2_1::Tim::from_ptr(0x4000_5800usize as _) };
#[doc = "Register-layout view `TIM3_MODE1` of physical timer `TIM3` (mode MODE1, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM3`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM3_MODE1: tim_ddl_v3::Tim = unsafe { tim_ddl_v3::Tim::from_ptr(0x4000_5800usize as _) };
#[doc = "Register-layout view `TIM3_MODE23` of physical timer `TIM3` (mode MODE23, same base address).
All MODE views alias the same physical timer: the working mode is selected at runtime via the control register MD bit-field, and the address, clock gate and interrupts are shared with `TIM3`.
Use the view that matches the mode you configured; do not access two views of the same timer concurrently."]
pub const TIM3_MODE23: tim_ddl_v5::Tim =
    unsafe { tim_ddl_v5::Tim::from_ptr(0x4000_5800usize as _) };
pub const TIM3: *mut () = 0x4000_5800usize as _;
pub const LCD: lcd_ddl_v1_1::Lcd = unsafe { lcd_ddl_v1_1::Lcd::from_ptr(0x4000_5c00usize as _) };
pub const UART2: uart_ddl_v3::Uart = unsafe { uart_ddl_v3::Uart::from_ptr(0x4000_6000usize as _) };
pub const UART3: uart_ddl_v3::Uart = unsafe { uart_ddl_v3::Uart::from_ptr(0x4000_6400usize as _) };
pub const FLASH: flash_ddl_v4::Flash =
    unsafe { flash_ddl_v4::Flash::from_ptr(0x4002_0000usize as _) };
pub const RAM: ram_ddl_v4::Ram = unsafe { ram_ddl_v4::Ram::from_ptr(0x4002_0400usize as _) };
pub const CRC: crc_ddl_v2_1::Crc = unsafe { crc_ddl_v2_1::Crc::from_ptr(0x4002_0900usize as _) };
pub const GPIO: gpio_ddl_v3_7::Gpio =
    unsafe { gpio_ddl_v3_7::Gpio::from_ptr(0x4002_0c00usize as _) };
pub const DMAC: dmac_ddl_v1_3::Dmac =
    unsafe { dmac_ddl_v1_3::Dmac::from_ptr(0x4002_1000usize as _) };
pub const AES: aes_ddl_v2::Aes = unsafe { aes_ddl_v2::Aes::from_ptr(0x4002_1400usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ddl_v7_3.rs"]
pub mod adc_ddl_v7_3;
#[path = "../../peripherals/adt_ddl_v2_1.rs"]
pub mod adt_ddl_v2_1;
#[path = "../../peripherals/aes_ddl_v2.rs"]
pub mod aes_ddl_v2;
#[path = "../../peripherals/bgr_ddl_v1_1.rs"]
pub mod bgr_ddl_v1_1;
#[path = "../../peripherals/clk_trim_ddl_v1_2.rs"]
pub mod clk_trim_ddl_v1_2;
#[path = "../../peripherals/crc_ddl_v2_1.rs"]
pub mod crc_ddl_v2_1;
#[path = "../../peripherals/dac_ddl_v1.rs"]
pub mod dac_ddl_v1;
#[path = "../../peripherals/debug_active_ddl_v1_3.rs"]
pub mod debug_active_ddl_v1_3;
#[path = "../../peripherals/dmac_ddl_v1_3.rs"]
pub mod dmac_ddl_v1_3;
#[path = "../../peripherals/flash_ddl_v4.rs"]
pub mod flash_ddl_v4;
#[path = "../../peripherals/gpio_ddl_v3_7.rs"]
pub mod gpio_ddl_v3_7;
#[path = "../../peripherals/i2c_ddl_v1_1.rs"]
pub mod i2c_ddl_v1_1;
#[path = "../../peripherals/lcd_ddl_v1_1.rs"]
pub mod lcd_ddl_v1_1;
#[path = "../../peripherals/lptim_ddl_v1_3.rs"]
pub mod lptim_ddl_v1_3;
#[path = "../../peripherals/lpuart_ddl_v3_2.rs"]
pub mod lpuart_ddl_v3_2;
#[path = "../../peripherals/lvd_ddl_v1_3.rs"]
pub mod lvd_ddl_v1_3;
#[path = "../../peripherals/opa_ddl_v2.rs"]
pub mod opa_ddl_v2;
#[path = "../../peripherals/pca_ddl_v1_2.rs"]
pub mod pca_ddl_v1_2;
#[path = "../../peripherals/pcnt_ddl_v1_1.rs"]
pub mod pcnt_ddl_v1_1;
#[path = "../../peripherals/ram_ddl_v4.rs"]
pub mod ram_ddl_v4;
#[path = "../../peripherals/reset_ddl_v1_4.rs"]
pub mod reset_ddl_v1_4;
#[path = "../../peripherals/rtc_ddl_v1_2.rs"]
pub mod rtc_ddl_v1_2;
#[path = "../../peripherals/spi_ddl_v1_2.rs"]
pub mod spi_ddl_v1_2;
#[path = "../../peripherals/sysctrl_ddl_v3_2.rs"]
pub mod sysctrl_ddl_v3_2;
#[path = "../../peripherals/tim_ddl_v2.rs"]
pub mod tim_ddl_v2;
#[path = "../../peripherals/tim_ddl_v2_1.rs"]
pub mod tim_ddl_v2_1;
#[path = "../../peripherals/tim_ddl_v2_2.rs"]
pub mod tim_ddl_v2_2;
#[path = "../../peripherals/tim_ddl_v2_3.rs"]
pub mod tim_ddl_v2_3;
#[path = "../../peripherals/tim_ddl_v3.rs"]
pub mod tim_ddl_v3;
#[path = "../../peripherals/tim_ddl_v3_1.rs"]
pub mod tim_ddl_v3_1;
#[path = "../../peripherals/tim_ddl_v3_2.rs"]
pub mod tim_ddl_v3_2;
#[path = "../../peripherals/tim_ddl_v3_3.rs"]
pub mod tim_ddl_v3_3;
#[path = "../../peripherals/tim_ddl_v4.rs"]
pub mod tim_ddl_v4;
#[path = "../../peripherals/tim_ddl_v4_1.rs"]
pub mod tim_ddl_v4_1;
#[path = "../../peripherals/tim_ddl_v4_2.rs"]
pub mod tim_ddl_v4_2;
#[path = "../../peripherals/tim_ddl_v5.rs"]
pub mod tim_ddl_v5;
#[path = "../../peripherals/trng_ddl_v1.rs"]
pub mod trng_ddl_v1;
#[path = "../../peripherals/uart_ddl_v3.rs"]
pub mod uart_ddl_v3;
#[path = "../../peripherals/vc_ddl_v3.rs"]
pub mod vc_ddl_v3;
#[path = "../../peripherals/wdt_ddl_v1_1.rs"]
pub mod wdt_ddl_v1_1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
