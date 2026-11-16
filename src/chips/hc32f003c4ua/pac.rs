




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - PORT0"]
PORT0 = 0 , # [doc = "1 - PORT1"]
PORT1 = 1 , # [doc = "2 - PORT2"]
PORT2 = 2 , # [doc = "3 - PORT3"]
PORT3 = 3 , # [doc = "6 - UART0"]
UART0 = 6 , # [doc = "7 - UART1"]
UART1 = 7 , # [doc = "10 - SPI"]
SPI = 10 , # [doc = "12 - I2C"]
I2C = 12 , # [doc = "14 - TIM0"]
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
mod _vectors { unsafe extern "C" { fn PORT0 () ; fn PORT1 () ; fn PORT2 () ; fn PORT3 () ; fn UART0 () ; fn UART1 () ; fn SPI () ; fn I2C () ; fn TIM0 () ; fn TIM1 () ; fn TIM2 () ; fn ADTIM4 () ; fn ADTIM5 () ; fn ADTIM6 () ; fn PCA () ; fn WDT () ; fn ADC () ; fn VC0 () ; fn VC1 () ; fn LVD () ; fn FLASH_RAM () ; fn CLKTRIM () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 32]
= [Vector { _handler : PORT0 } , Vector { _handler : PORT1 } , Vector { _handler : PORT2 } , Vector { _handler : PORT3 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : UART0 } , Vector { _handler : UART1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : SPI } , Vector { _reserved : 0 } , Vector { _handler : I2C } , Vector { _reserved : 0 } , Vector { _handler : TIM0 } , Vector { _handler : TIM1 } , Vector { _handler : TIM2 } , Vector { _reserved : 0 } , Vector { _handler : ADTIM4 } , Vector { _handler : ADTIM5 } , Vector { _handler : ADTIM6 } , Vector { _handler : PCA } , Vector { _handler : WDT } , Vector { _reserved : 0 } , Vector { _handler : ADC } , Vector { _reserved : 0 } , Vector { _handler : VC0 } , Vector { _handler : VC1 } , Vector { _handler : LVD } , Vector { _reserved : 0 } , Vector { _handler : FLASH_RAM } , Vector { _handler : CLKTRIM } ,]
; } pub const UART0 : uart_v2 :: Uart = unsafe { uart_v2 :: Uart :: from_ptr (0x4000_0000usize as _) } ; pub const UART1 : uart_v2 :: Uart = unsafe { uart_v2 :: Uart :: from_ptr (0x4000_0100usize as _) } ; pub const I2C : i_v5 :: I = unsafe { i_v5 :: I :: from_ptr (0x4000_0400usize as _) } ; pub const SPI : spi_v4 :: Spi = unsafe { spi_v4 :: Spi :: from_ptr (0x4000_0800usize as _) } ; pub const TIM0 : tim_v13 :: Tim = unsafe { tim_v13 :: Tim :: from_ptr (0x4000_0c00usize as _) } ; pub const WDT : wdt_v1 :: Wdt = unsafe { wdt_v1 :: Wdt :: from_ptr (0x4000_0c00usize as _) } ; pub const TIM1 : tim_v13 :: Tim = unsafe { tim_v13 :: Tim :: from_ptr (0x4000_0c20usize as _) } ; pub const TIM2 : tim_v13 :: Tim = unsafe { tim_v13 :: Tim :: from_ptr (0x4000_0c40usize as _) } ; pub const PCA : pca_v2 :: Pca = unsafe { pca_v2 :: Pca :: from_ptr (0x4000_1000usize as _) } ; pub const CLK_TRIM : clk_v2 :: Clk = unsafe { clk_v2 :: Clk :: from_ptr (0x4000_1800usize as _) } ; pub const SYSCTRL : sysctrl_v4 :: Sysctrl = unsafe { sysctrl_v4 :: Sysctrl :: from_ptr (0x4000_2000usize as _) } ; pub const RESET : reset_v2 :: Reset = unsafe { reset_v2 :: Reset :: from_ptr (0x4000_201cusize as _) } ; pub const DEBUG_ACTIVE : debug_v2 :: Debug = unsafe { debug_v2 :: Debug :: from_ptr (0x4000_2038usize as _) } ; pub const ADC : adc_v4 :: Adc = unsafe { adc_v4 :: Adc :: from_ptr (0x4000_2400usize as _) } ; pub const BGR : bgr_v1 :: Bgr = unsafe { bgr_v1 :: Bgr :: from_ptr (0x4000_2400usize as _) } ; pub const LVD : lvd_v4 :: Lvd = unsafe { lvd_v4 :: Lvd :: from_ptr (0x4000_2400usize as _) } ; pub const VC : vc_v3 :: Vc = unsafe { vc_v3 :: Vc :: from_ptr (0x4000_2400usize as _) } ; pub const ADTIM4 : adtim_v2 :: Adtim = unsafe { adtim_v2 :: Adtim :: from_ptr (0x4000_3000usize as _) } ; pub const ADTIM5 : adtim_v2 :: Adtim = unsafe { adtim_v2 :: Adtim :: from_ptr (0x4000_3400usize as _) } ; pub const ADTIM6 : adtim_v2 :: Adtim = unsafe { adtim_v2 :: Adtim :: from_ptr (0x4000_3800usize as _) } ; pub const FLASH : flash_v4 :: Flash = unsafe { flash_v4 :: Flash :: from_ptr (0x4002_0000usize as _) } ; pub const RAM : ram_v3 :: Ram = unsafe { ram_v3 :: Ram :: from_ptr (0x4002_0400usize as _) } ; pub const CRC : crc_v3 :: Crc = unsafe { crc_v3 :: Crc :: from_ptr (0x4002_0900usize as _) } ; pub const GPIO : gpio_v2 :: Gpio = unsafe { gpio_v2 :: Gpio :: from_ptr (0x4002_0c00usize as _) } ; # [cfg (feature = "rt")]
 # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/adc_v4.rs"] pub mod adc_v4;
#[path="../../peripherals/adtim_v2.rs"] pub mod adtim_v2;
#[path="../../peripherals/bgr_v1.rs"] pub mod bgr_v1;
#[path="../../peripherals/clk_v2.rs"] pub mod clk_v2;
#[path="../../peripherals/crc_v3.rs"] pub mod crc_v3;
#[path="../../peripherals/debug_v2.rs"] pub mod debug_v2;
#[path="../../peripherals/flash_v4.rs"] pub mod flash_v4;
#[path="../../peripherals/gpio_v2.rs"] pub mod gpio_v2;
#[path="../../peripherals/i_v5.rs"] pub mod i_v5;
#[path="../../peripherals/lvd_v4.rs"] pub mod lvd_v4;
#[path="../../peripherals/pca_v2.rs"] pub mod pca_v2;
#[path="../../peripherals/ram_v3.rs"] pub mod ram_v3;
#[path="../../peripherals/reset_v2.rs"] pub mod reset_v2;
#[path="../../peripherals/spi_v4.rs"] pub mod spi_v4;
#[path="../../peripherals/sysctrl_v4.rs"] pub mod sysctrl_v4;
#[path="../../peripherals/tim_v13.rs"] pub mod tim_v13;
#[path="../../peripherals/uart_v2.rs"] pub mod uart_v2;
#[path="../../peripherals/vc_v3.rs"] pub mod vc_v3;
#[path="../../peripherals/wdt_v1.rs"] pub mod wdt_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout { regions: crate::memory_select::MEMORY };
