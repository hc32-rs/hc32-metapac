




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - PORTA"]
PORTA = 0 , # [doc = "1 - PORTB"]
PORTB = 1 , # [doc = "5 - ATIM3"]
ATIM3 = 5 , # [doc = "8 - LPUART0"]
LPUART0 = 8 , # [doc = "9 - LPUART1"]
LPUART1 = 9 , # [doc = "10 - SPI"]
SPI = 10 , # [doc = "14 - CTIM0"]
CTIM0 = 14 , # [doc = "15 - CTIM1"]
CTIM1 = 15 , # [doc = "18 - HSI2C"]
HSI2C = 18 , # [doc = "22 - IWDT"]
IWDT = 22 , # [doc = "23 - RTC"]
RTC = 23 , # [doc = "24 - ADC"]
ADC = 24 , # [doc = "26 - VC0"]
VC0 = 26 , # [doc = "27 - VC1"]
VC1 = 27 , # [doc = "28 - LVD"]
LVD = 28 , # [doc = "30 - FLASH"]
FLASH = 30 , # [doc = "31 - CTRIM_CLKDET"]
CTRIM_CLKDET = 31 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn PORTA () ; fn PORTB () ; fn ATIM3 () ; fn LPUART0 () ; fn LPUART1 () ; fn SPI () ; fn CTIM0 () ; fn CTIM1 () ; fn HSI2C () ; fn IWDT () ; fn RTC () ; fn ADC () ; fn VC0 () ; fn VC1 () ; fn LVD () ; fn FLASH () ; fn CTRIM_CLKDET () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 32]
= [Vector { _handler : PORTA } , Vector { _handler : PORTB } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : ATIM3 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : LPUART0 } , Vector { _handler : LPUART1 } , Vector { _handler : SPI } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : CTIM0 } , Vector { _handler : CTIM1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : HSI2C } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : IWDT } , Vector { _handler : RTC } , Vector { _handler : ADC } , Vector { _reserved : 0 } , Vector { _handler : VC0 } , Vector { _handler : VC1 } , Vector { _handler : LVD } , Vector { _reserved : 0 } , Vector { _handler : FLASH } , Vector { _handler : CTRIM_CLKDET } ,]
; } pub const LPUART0 : lpuart_v5 :: Lpuart = unsafe { lpuart_v5 :: Lpuart :: from_ptr (0x4000_0000usize as _) } ; pub const SPI : spi_v5 :: Spi = unsafe { spi_v5 :: Spi :: from_ptr (0x4000_0800usize as _) } ; pub const IWDT : iwdt_v3 :: Iwdt = unsafe { iwdt_v3 :: Iwdt :: from_ptr (0x4000_1000usize as _) } ; pub const RTC : rtc_v3 :: Rtc = unsafe { rtc_v3 :: Rtc :: from_ptr (0x4000_1400usize as _) } ; pub const BTIM0 : btim_v1 :: Btim = unsafe { btim_v1 :: Btim :: from_ptr (0x4000_1800usize as _) } ; pub const GTIM0 : gtim_v4 :: Gtim = unsafe { gtim_v4 :: Gtim :: from_ptr (0x4000_1800usize as _) } ; pub const BTIM1 : btim_v1 :: Btim = unsafe { btim_v1 :: Btim :: from_ptr (0x4000_1900usize as _) } ; pub const BTIM2 : btim_v1 :: Btim = unsafe { btim_v1 :: Btim :: from_ptr (0x4000_1a00usize as _) } ; pub const BTIM3 : btim_v1 :: Btim = unsafe { btim_v1 :: Btim :: from_ptr (0x4000_1c00usize as _) } ; pub const GTIM1 : gtim_v4 :: Gtim = unsafe { gtim_v4 :: Gtim :: from_ptr (0x4000_1c00usize as _) } ; pub const BTIM4 : btim_v1 :: Btim = unsafe { btim_v1 :: Btim :: from_ptr (0x4000_1d00usize as _) } ; pub const BTIM5 : btim_v1 :: Btim = unsafe { btim_v1 :: Btim :: from_ptr (0x4000_1e00usize as _) } ; pub const ADC : adc_v9 :: Adc = unsafe { adc_v9 :: Adc :: from_ptr (0x4000_2400usize as _) } ; pub const VC : vc_v6 :: Vc = unsafe { vc_v6 :: Vc :: from_ptr (0x4000_2800usize as _) } ; pub const LVD : lvd_v5 :: Lvd = unsafe { lvd_v5 :: Lvd :: from_ptr (0x4000_2c00usize as _) } ; pub const LPUART1 : lpuart_v5 :: Lpuart = unsafe { lpuart_v5 :: Lpuart :: from_ptr (0x4000_8000usize as _) } ; pub const TRNG : trng_v1 :: Trng = unsafe { trng_v1 :: Trng :: from_ptr (0x4000_8c00usize as _) } ; pub const CTRIM : ctrim_v4 :: Ctrim = unsafe { ctrim_v4 :: Ctrim :: from_ptr (0x4000_9000usize as _) } ; pub const ATIM3 : * mut () = 0x4000_9800usize as _ ; pub const HSI2C : hsi_v1 :: Hsi = unsafe { hsi_v1 :: Hsi :: from_ptr (0x4000_bc00usize as _) } ; pub const FLASH : flash_v9 :: Flash = unsafe { flash_v9 :: Flash :: from_ptr (0x4002_0000usize as _) } ; pub const GPIOA : gpioa_v3 :: Gpioa = unsafe { gpioa_v3 :: Gpioa :: from_ptr (0x4002_0c00usize as _) } ; pub const GPIOAUX : gpioaux_v4 :: Gpioaux = unsafe { gpioaux_v4 :: Gpioaux :: from_ptr (0x4002_0c00usize as _) } ; pub const GPIOB : gpiob_common :: Gpiob = unsafe { gpiob_common :: Gpiob :: from_ptr (0x4002_0c40usize as _) } ; pub const SYSCTRL : sysctrl_v9 :: Sysctrl = unsafe { sysctrl_v9 :: Sysctrl :: from_ptr (0x4002_1800usize as _) } ; # [cfg (feature = "rt")]
 # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/adc_v9.rs"] pub mod adc_v9;
#[path="../../peripherals/btim_v1.rs"] pub mod btim_v1;
#[path="../../peripherals/ctrim_v4.rs"] pub mod ctrim_v4;
#[path="../../peripherals/flash_v9.rs"] pub mod flash_v9;
#[path="../../peripherals/gpioa_v3.rs"] pub mod gpioa_v3;
#[path="../../peripherals/gpioaux_v4.rs"] pub mod gpioaux_v4;
#[path="../../peripherals/gpiob_common.rs"] pub mod gpiob_common;
#[path="../../peripherals/gtim_v4.rs"] pub mod gtim_v4;
#[path="../../peripherals/hsi_v1.rs"] pub mod hsi_v1;
#[path="../../peripherals/iwdt_v3.rs"] pub mod iwdt_v3;
#[path="../../peripherals/lpuart_v5.rs"] pub mod lpuart_v5;
#[path="../../peripherals/lvd_v5.rs"] pub mod lvd_v5;
#[path="../../peripherals/rtc_v3.rs"] pub mod rtc_v3;
#[path="../../peripherals/spi_v5.rs"] pub mod spi_v5;
#[path="../../peripherals/sysctrl_v9.rs"] pub mod sysctrl_v9;
#[path="../../peripherals/trng_v1.rs"] pub mod trng_v1;
#[path="../../peripherals/vc_v6.rs"] pub mod vc_v6;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout { regions: crate::memory_select::MEMORY };
