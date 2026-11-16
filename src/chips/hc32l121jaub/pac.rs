




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - PORTA"]
PORTA = 0 , # [doc = "1 - PORTB"]
PORTB = 1 , # [doc = "2 - PORTC"]
PORTC = 2 , # [doc = "3 - PORTD"]
PORTD = 3 , # [doc = "4 - DMAC"]
DMAC = 4 , # [doc = "5 - TIM3"]
TIM3 = 5 , # [doc = "6 - USART0"]
USART0 = 6 , # [doc = "7 - USART1"]
USART1 = 7 , # [doc = "8 - LPUART0"]
LPUART0 = 8 , # [doc = "9 - LPUART1"]
LPUART1 = 9 , # [doc = "10 - SPI0"]
SPI0 = 10 , # [doc = "11 - SPI1"]
SPI1 = 11 , # [doc = "12 - I2C0"]
I2C0 = 12 , # [doc = "13 - I2C1"]
I2C1 = 13 , # [doc = "14 - TIM0"]
TIM0 = 14 , # [doc = "15 - TIM1"]
TIM1 = 15 , # [doc = "16 - TIM2"]
TIM2 = 16 , # [doc = "17 - LPTIM_0_1"]
LPTIM_0_1 = 17 , # [doc = "18 - HSI2C"]
HSI2C = 18 , # [doc = "19 - LPUART2"]
LPUART2 = 19 , # [doc = "21 - WWDT"]
WWDT = 21 , # [doc = "22 - IWDT"]
IWDT = 22 , # [doc = "23 - RTC"]
RTC = 23 , # [doc = "24 - ADC"]
ADC = 24 , # [doc = "26 - VC0"]
VC0 = 26 , # [doc = "27 - VC1"]
VC1 = 27 , # [doc = "28 - LVD"]
LVD = 28 , # [doc = "29 - LCD"]
LCD = 29 , # [doc = "30 - FLASH_RAM"]
FLASH_RAM = 30 , # [doc = "31 - CTRIM_CLKDET"]
CTRIM_CLKDET = 31 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn PORTA () ; fn PORTB () ; fn PORTC () ; fn PORTD () ; fn DMAC () ; fn TIM3 () ; fn USART0 () ; fn USART1 () ; fn LPUART0 () ; fn LPUART1 () ; fn SPI0 () ; fn SPI1 () ; fn I2C0 () ; fn I2C1 () ; fn TIM0 () ; fn TIM1 () ; fn TIM2 () ; fn LPTIM_0_1 () ; fn HSI2C () ; fn LPUART2 () ; fn WWDT () ; fn IWDT () ; fn RTC () ; fn ADC () ; fn VC0 () ; fn VC1 () ; fn LVD () ; fn LCD () ; fn FLASH_RAM () ; fn CTRIM_CLKDET () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 32]
= [Vector { _handler : PORTA } , Vector { _handler : PORTB } , Vector { _handler : PORTC } , Vector { _handler : PORTD } , Vector { _handler : DMAC } , Vector { _handler : TIM3 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : LPUART0 } , Vector { _handler : LPUART1 } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : I2C0 } , Vector { _handler : I2C1 } , Vector { _handler : TIM0 } , Vector { _handler : TIM1 } , Vector { _handler : TIM2 } , Vector { _handler : LPTIM_0_1 } , Vector { _handler : HSI2C } , Vector { _handler : LPUART2 } , Vector { _reserved : 0 } , Vector { _handler : WWDT } , Vector { _handler : IWDT } , Vector { _handler : RTC } , Vector { _handler : ADC } , Vector { _reserved : 0 } , Vector { _handler : VC0 } , Vector { _handler : VC1 } , Vector { _handler : LVD } , Vector { _handler : LCD } , Vector { _handler : FLASH_RAM } , Vector { _handler : CTRIM_CLKDET } ,]
; } pub const LPUART0 : lpuart_v7 :: Lpuart = unsafe { lpuart_v7 :: Lpuart :: from_ptr (0x4000_0000usize as _) } ; pub const I2C0 : i_v9 :: I = unsafe { i_v9 :: I :: from_ptr (0x4000_0400usize as _) } ; pub const SPI0 : spi_v1 :: Spi = unsafe { spi_v1 :: Spi :: from_ptr (0x4000_0800usize as _) } ; pub const LPTIMER0 : lptimer_v4 :: Lptimer = unsafe { lptimer_v4 :: Lptimer :: from_ptr (0x4000_0c00usize as _) } ; pub const IWDT : iwdt_v4 :: Iwdt = unsafe { iwdt_v4 :: Iwdt :: from_ptr (0x4000_1080usize as _) } ; pub const RTC : rtc_v3 :: Rtc = unsafe { rtc_v3 :: Rtc :: from_ptr (0x4000_1400usize as _) } ; pub const ADC : adc_v11 :: Adc = unsafe { adc_v11 :: Adc :: from_ptr (0x4000_2400usize as _) } ; pub const VC0 : vc_v8 :: Vc = unsafe { vc_v8 :: Vc :: from_ptr (0x4000_2800usize as _) } ; pub const VC1 : vc_v8 :: Vc = unsafe { vc_v8 :: Vc :: from_ptr (0x4000_2810usize as _) } ; pub const VCTRIM : vctrim_common :: Vctrim = unsafe { vctrim_common :: Vctrim :: from_ptr (0x4000_2820usize as _) } ; pub const LVD : lvd_v7 :: Lvd = unsafe { lvd_v7 :: Lvd :: from_ptr (0x4000_2c00usize as _) } ; pub const TIM0 : * mut () = 0x4000_3000usize as _ ; pub const TIM1 : * mut () = 0x4000_3400usize as _ ; pub const TIM2 : * mut () = 0x4000_3800usize as _ ; pub const LPTIMER1 : lptimer_v4 :: Lptimer = unsafe { lptimer_v4 :: Lptimer :: from_ptr (0x4000_3c00usize as _) } ; pub const LPUART1 : lpuart_v7 :: Lpuart = unsafe { lpuart_v7 :: Lpuart :: from_ptr (0x4000_8000usize as _) } ; pub const I2C1 : i_v9 :: I = unsafe { i_v9 :: I :: from_ptr (0x4000_8400usize as _) } ; pub const SPI1 : spi_v1 :: Spi = unsafe { spi_v1 :: Spi :: from_ptr (0x4000_8800usize as _) } ; pub const TRNG : trng_v2 :: Trng = unsafe { trng_v2 :: Trng :: from_ptr (0x4000_8c00usize as _) } ; pub const CTRIM : ctrim_v4 :: Ctrim = unsafe { ctrim_v4 :: Ctrim :: from_ptr (0x4000_9000usize as _) } ; pub const TIM3 : * mut () = 0x4000_9800usize as _ ; pub const LCD : lcd_v4 :: Lcd = unsafe { lcd_v4 :: Lcd :: from_ptr (0x4000_9c00usize as _) } ; pub const USART0 : usart_common :: Usart = unsafe { usart_common :: Usart :: from_ptr (0x4000_a000usize as _) } ; pub const USART1 : usart_common :: Usart = unsafe { usart_common :: Usart :: from_ptr (0x4000_a400usize as _) } ; pub const LPUART2 : lpuart_v7 :: Lpuart = unsafe { lpuart_v7 :: Lpuart :: from_ptr (0x4000_b400usize as _) } ; pub const WWDT : wwdt_common :: Wwdt = unsafe { wwdt_common :: Wwdt :: from_ptr (0x4000_b800usize as _) } ; pub const HSI2C : hsi_v2 :: Hsi = unsafe { hsi_v2 :: Hsi :: from_ptr (0x4000_bc00usize as _) } ; pub const FLASH : flash_v11 :: Flash = unsafe { flash_v11 :: Flash :: from_ptr (0x4002_0000usize as _) } ; pub const RAM : ram_v4 :: Ram = unsafe { ram_v4 :: Ram :: from_ptr (0x4002_0400usize as _) } ; pub const CRC : crc_v5 :: Crc = unsafe { crc_v5 :: Crc :: from_ptr (0x4002_0800usize as _) } ; pub const GPIO : gpio_v5 :: Gpio = unsafe { gpio_v5 :: Gpio :: from_ptr (0x4002_0c00usize as _) } ; pub const DMAC : dmac_v5 :: Dmac = unsafe { dmac_v5 :: Dmac :: from_ptr (0x4002_1000usize as _) } ; pub const AES : aes_v3 :: Aes = unsafe { aes_v3 :: Aes :: from_ptr (0x4002_1400usize as _) } ; pub const SYSCTRL : sysctrl_v11 :: Sysctrl = unsafe { sysctrl_v11 :: Sysctrl :: from_ptr (0x4002_1800usize as _) } ; pub const RESET : reset_v7 :: Reset = unsafe { reset_v7 :: Reset :: from_ptr (0x4002_181cusize as _) } ; pub const DEBUG_ACTIVE : debug_v4 :: Debug = unsafe { debug_v4 :: Debug :: from_ptr (0x4002_1838usize as _) } ; # [cfg (feature = "rt")]
 # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/adc_v11.rs"] pub mod adc_v11;
#[path="../../peripherals/aes_v3.rs"] pub mod aes_v3;
#[path="../../peripherals/crc_v5.rs"] pub mod crc_v5;
#[path="../../peripherals/ctrim_v4.rs"] pub mod ctrim_v4;
#[path="../../peripherals/debug_v4.rs"] pub mod debug_v4;
#[path="../../peripherals/dmac_v5.rs"] pub mod dmac_v5;
#[path="../../peripherals/flash_v11.rs"] pub mod flash_v11;
#[path="../../peripherals/gpio_v5.rs"] pub mod gpio_v5;
#[path="../../peripherals/hsi_v2.rs"] pub mod hsi_v2;
#[path="../../peripherals/i_v9.rs"] pub mod i_v9;
#[path="../../peripherals/iwdt_v4.rs"] pub mod iwdt_v4;
#[path="../../peripherals/lcd_v4.rs"] pub mod lcd_v4;
#[path="../../peripherals/lptimer_v4.rs"] pub mod lptimer_v4;
#[path="../../peripherals/lpuart_v7.rs"] pub mod lpuart_v7;
#[path="../../peripherals/lvd_v7.rs"] pub mod lvd_v7;
#[path="../../peripherals/ram_v4.rs"] pub mod ram_v4;
#[path="../../peripherals/reset_v7.rs"] pub mod reset_v7;
#[path="../../peripherals/rtc_v3.rs"] pub mod rtc_v3;
#[path="../../peripherals/spi_v1.rs"] pub mod spi_v1;
#[path="../../peripherals/sysctrl_v11.rs"] pub mod sysctrl_v11;
#[path="../../peripherals/trng_v2.rs"] pub mod trng_v2;
#[path="../../peripherals/usart_common.rs"] pub mod usart_common;
#[path="../../peripherals/vc_v8.rs"] pub mod vc_v8;
#[path="../../peripherals/vctrim_common.rs"] pub mod vctrim_common;
#[path="../../peripherals/wwdt_common.rs"] pub mod wwdt_common;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout { regions: crate::memory_select::MEMORY };
