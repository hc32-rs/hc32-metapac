




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - PORTA"]
PORTA = 0 , # [doc = "1 - PORTB"]
PORTB = 1 , # [doc = "2 - PORTC"]
PORTC = 2 , # [doc = "3 - PORTD"]
PORTD = 3 , # [doc = "4 - DMAC"]
DMAC = 4 , # [doc = "5 - TIM3"]
TIM3 = 5 , # [doc = "6 - UART0"]
UART0 = 6 , # [doc = "7 - UART1"]
UART1 = 7 , # [doc = "10 - SPI0"]
SPI0 = 10 , # [doc = "11 - SPI1"]
SPI1 = 11 , # [doc = "12 - I2C0"]
I2C0 = 12 , # [doc = "13 - I2C1"]
I2C1 = 13 , # [doc = "14 - TIM0"]
TIM0 = 14 , # [doc = "15 - TIM1"]
TIM1 = 15 , # [doc = "16 - TIM2"]
TIM2 = 16 , # [doc = "18 - ADTIM4"]
ADTIM4 = 18 , # [doc = "19 - ADTIM5"]
ADTIM5 = 19 , # [doc = "20 - ADTIM6"]
ADTIM6 = 20 , # [doc = "21 - PCA"]
PCA = 21 , # [doc = "22 - WDT"]
WDT = 22 , # [doc = "24 - ADC"]
ADC = 24 , # [doc = "26 - VC0"]
VC0 = 26 , # [doc = "27 - VC1"]
VC1 = 27 , # [doc = "28 - LVD"]
LVD = 28 , # [doc = "30 - FLASH_RAM"]
FLASH_RAM = 30 , # [doc = "31 - CLKTRIM"]
CLKTRIM = 31 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn PORTA () ; fn PORTB () ; fn PORTC () ; fn PORTD () ; fn DMAC () ; fn TIM3 () ; fn UART0 () ; fn UART1 () ; fn SPI0 () ; fn SPI1 () ; fn I2C0 () ; fn I2C1 () ; fn TIM0 () ; fn TIM1 () ; fn TIM2 () ; fn ADTIM4 () ; fn ADTIM5 () ; fn ADTIM6 () ; fn PCA () ; fn WDT () ; fn ADC () ; fn VC0 () ; fn VC1 () ; fn LVD () ; fn FLASH_RAM () ; fn CLKTRIM () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 32]
= [Vector { _handler : PORTA } , Vector { _handler : PORTB } , Vector { _handler : PORTC } , Vector { _handler : PORTD } , Vector { _handler : DMAC } , Vector { _handler : TIM3 } , Vector { _handler : UART0 } , Vector { _handler : UART1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : I2C0 } , Vector { _handler : I2C1 } , Vector { _handler : TIM0 } , Vector { _handler : TIM1 } , Vector { _handler : TIM2 } , Vector { _reserved : 0 } , Vector { _handler : ADTIM4 } , Vector { _handler : ADTIM5 } , Vector { _handler : ADTIM6 } , Vector { _handler : PCA } , Vector { _handler : WDT } , Vector { _reserved : 0 } , Vector { _handler : ADC } , Vector { _reserved : 0 } , Vector { _handler : VC0 } , Vector { _handler : VC1 } , Vector { _handler : LVD } , Vector { _reserved : 0 } , Vector { _handler : FLASH_RAM } , Vector { _handler : CLKTRIM } ,]
; } pub const UART0 : uart_v1 :: Uart = unsafe { uart_v1 :: Uart :: from_ptr (0x4000_0000usize as _) } ; pub const UART1 : uart_v1 :: Uart = unsafe { uart_v1 :: Uart :: from_ptr (0x4000_0100usize as _) } ; pub const I2C0 : i_v3 :: I = unsafe { i_v3 :: I :: from_ptr (0x4000_0400usize as _) } ; pub const SPI0 : spi_v2 :: Spi = unsafe { spi_v2 :: Spi :: from_ptr (0x4000_0800usize as _) } ; pub const TIM0 : * mut () = 0x4000_0c00usize as _ ; pub const TIM1 : * mut () = 0x4000_0d00usize as _ ; pub const TIM2 : * mut () = 0x4000_0e00usize as _ ; pub const WDT : wdt_v1 :: Wdt = unsafe { wdt_v1 :: Wdt :: from_ptr (0x4000_0f00usize as _) } ; pub const PCA : pca_v1 :: Pca = unsafe { pca_v1 :: Pca :: from_ptr (0x4000_1000usize as _) } ; pub const CLK_TRIM : clk_v1 :: Clk = unsafe { clk_v1 :: Clk :: from_ptr (0x4000_1800usize as _) } ; pub const SYSCTRL : sysctrl_v5 :: Sysctrl = unsafe { sysctrl_v5 :: Sysctrl :: from_ptr (0x4000_2000usize as _) } ; pub const RESET : reset_v1 :: Reset = unsafe { reset_v1 :: Reset :: from_ptr (0x4000_201cusize as _) } ; pub const DEBUG_ACTIVE : debug_v2 :: Debug = unsafe { debug_v2 :: Debug :: from_ptr (0x4000_2038usize as _) } ; pub const ADC : adc_v2 :: Adc = unsafe { adc_v2 :: Adc :: from_ptr (0x4000_2400usize as _) } ; pub const BGR : bgr_v1 :: Bgr = unsafe { bgr_v1 :: Bgr :: from_ptr (0x4000_2400usize as _) } ; pub const LVD : lvd_v2 :: Lvd = unsafe { lvd_v2 :: Lvd :: from_ptr (0x4000_2400usize as _) } ; pub const OPA : opa_v2 :: Opa = unsafe { opa_v2 :: Opa :: from_ptr (0x4000_2400usize as _) } ; pub const VC : vc_v2 :: Vc = unsafe { vc_v2 :: Vc :: from_ptr (0x4000_2400usize as _) } ; pub const ADTIM4 : adtim_v1 :: Adtim = unsafe { adtim_v1 :: Adtim :: from_ptr (0x4000_3000usize as _) } ; pub const ADTIM5 : adtim_v1 :: Adtim = unsafe { adtim_v1 :: Adtim :: from_ptr (0x4000_3400usize as _) } ; pub const ADTIM6 : adtim_v1 :: Adtim = unsafe { adtim_v1 :: Adtim :: from_ptr (0x4000_3800usize as _) } ; pub const I2C1 : i_v3 :: I = unsafe { i_v3 :: I :: from_ptr (0x4000_4400usize as _) } ; pub const SPI1 : spi_v2 :: Spi = unsafe { spi_v2 :: Spi :: from_ptr (0x4000_4800usize as _) } ; pub const TRNG : trng_v1 :: Trng = unsafe { trng_v1 :: Trng :: from_ptr (0x4000_4c00usize as _) } ; pub const TIM3 : * mut () = 0x4000_5800usize as _ ; pub const FLASH : flash_v2 :: Flash = unsafe { flash_v2 :: Flash :: from_ptr (0x4002_0000usize as _) } ; pub const RAM : ram_v2 :: Ram = unsafe { ram_v2 :: Ram :: from_ptr (0x4002_0400usize as _) } ; pub const CRC : crc_v2 :: Crc = unsafe { crc_v2 :: Crc :: from_ptr (0x4002_0900usize as _) } ; pub const GPIO : gpio_v1 :: Gpio = unsafe { gpio_v1 :: Gpio :: from_ptr (0x4002_0c00usize as _) } ; pub const DMAC : dmac_v2 :: Dmac = unsafe { dmac_v2 :: Dmac :: from_ptr (0x4002_1000usize as _) } ; pub const AES : aes_v1 :: Aes = unsafe { aes_v1 :: Aes :: from_ptr (0x4002_1400usize as _) } ; pub const HDIV : hdiv_common :: Hdiv = unsafe { hdiv_common :: Hdiv :: from_ptr (0x4002_1800usize as _) } ; # [cfg (feature = "rt")]
 # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/adc_v2.rs"] pub mod adc_v2;
#[path="../../peripherals/adtim_v1.rs"] pub mod adtim_v1;
#[path="../../peripherals/aes_v1.rs"] pub mod aes_v1;
#[path="../../peripherals/bgr_v1.rs"] pub mod bgr_v1;
#[path="../../peripherals/clk_v1.rs"] pub mod clk_v1;
#[path="../../peripherals/crc_v2.rs"] pub mod crc_v2;
#[path="../../peripherals/debug_v2.rs"] pub mod debug_v2;
#[path="../../peripherals/dmac_v2.rs"] pub mod dmac_v2;
#[path="../../peripherals/flash_v2.rs"] pub mod flash_v2;
#[path="../../peripherals/gpio_v1.rs"] pub mod gpio_v1;
#[path="../../peripherals/hdiv_common.rs"] pub mod hdiv_common;
#[path="../../peripherals/i_v3.rs"] pub mod i_v3;
#[path="../../peripherals/lvd_v2.rs"] pub mod lvd_v2;
#[path="../../peripherals/opa_v2.rs"] pub mod opa_v2;
#[path="../../peripherals/pca_v1.rs"] pub mod pca_v1;
#[path="../../peripherals/ram_v2.rs"] pub mod ram_v2;
#[path="../../peripherals/reset_v1.rs"] pub mod reset_v1;
#[path="../../peripherals/spi_v2.rs"] pub mod spi_v2;
#[path="../../peripherals/sysctrl_v5.rs"] pub mod sysctrl_v5;
#[path="../../peripherals/trng_v1.rs"] pub mod trng_v1;
#[path="../../peripherals/uart_v1.rs"] pub mod uart_v1;
#[path="../../peripherals/vc_v2.rs"] pub mod vc_v2;
#[path="../../peripherals/wdt_v1.rs"] pub mod wdt_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout { regions: crate::memory_select::MEMORY };
