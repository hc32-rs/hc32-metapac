




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - PORTA"]
PORTA = 0 , # [doc = "1 - PORTB"]
PORTB = 1 , # [doc = "2 - PORTC_E"]
PORTC_E = 2 , # [doc = "3 - PORTD_F"]
PORTD_F = 3 , # [doc = "4 - DMAC"]
DMAC = 4 , # [doc = "5 - TIM3"]
TIM3 = 5 , # [doc = "6 - UART0_2"]
UART0_2 = 6 , # [doc = "7 - UART1_3"]
UART1_3 = 7 , # [doc = "10 - SPI0"]
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
WDT = 22 , # [doc = "24 - ADC_DAC"]
ADC_DAC = 24 , # [doc = "26 - VC0"]
VC0 = 26 , # [doc = "27 - VC1_2"]
VC1_2 = 27 , # [doc = "28 - LVD"]
LVD = 28 , # [doc = "29 - LCD"]
LCD = 29 , # [doc = "30 - FLASH_RAM"]
FLASH_RAM = 30 , # [doc = "31 - CLKTRIM"]
CLKTRIM = 31 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn PORTA () ; fn PORTB () ; fn PORTC_E () ; fn PORTD_F () ; fn DMAC () ; fn TIM3 () ; fn UART0_2 () ; fn UART1_3 () ; fn SPI0 () ; fn SPI1 () ; fn I2C0 () ; fn I2C1 () ; fn TIM0 () ; fn TIM1 () ; fn TIM2 () ; fn ADTIM4 () ; fn ADTIM5 () ; fn ADTIM6 () ; fn PCA () ; fn WDT () ; fn ADC_DAC () ; fn VC0 () ; fn VC1_2 () ; fn LVD () ; fn LCD () ; fn FLASH_RAM () ; fn CLKTRIM () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 32]
= [Vector { _handler : PORTA } , Vector { _handler : PORTB } , Vector { _handler : PORTC_E } , Vector { _handler : PORTD_F } , Vector { _handler : DMAC } , Vector { _handler : TIM3 } , Vector { _handler : UART0_2 } , Vector { _handler : UART1_3 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : I2C0 } , Vector { _handler : I2C1 } , Vector { _handler : TIM0 } , Vector { _handler : TIM1 } , Vector { _handler : TIM2 } , Vector { _reserved : 0 } , Vector { _handler : ADTIM4 } , Vector { _handler : ADTIM5 } , Vector { _handler : ADTIM6 } , Vector { _handler : PCA } , Vector { _handler : WDT } , Vector { _reserved : 0 } , Vector { _handler : ADC_DAC } , Vector { _reserved : 0 } , Vector { _handler : VC0 } , Vector { _handler : VC1_2 } , Vector { _handler : LVD } , Vector { _handler : LCD } , Vector { _handler : FLASH_RAM } , Vector { _handler : CLKTRIM } ,]
; } pub const UART0 : uart_v3 :: Uart = unsafe { uart_v3 :: Uart :: from_ptr (0x4000_0000usize as _) } ; pub const UART1 : uart_v3 :: Uart = unsafe { uart_v3 :: Uart :: from_ptr (0x4000_0100usize as _) } ; pub const I2C0 : i_v3 :: I = unsafe { i_v3 :: I :: from_ptr (0x4000_0400usize as _) } ; pub const SPI0 : spi_v2 :: Spi = unsafe { spi_v2 :: Spi :: from_ptr (0x4000_0800usize as _) } ; pub const TIM0 : * mut () = 0x4000_0c00usize as _ ; pub const TIM1 : * mut () = 0x4000_0d00usize as _ ; pub const TIM2 : * mut () = 0x4000_0e00usize as _ ; pub const WDT : wdt_v1 :: Wdt = unsafe { wdt_v1 :: Wdt :: from_ptr (0x4000_0f00usize as _) } ; pub const PCA : pca_v1 :: Pca = unsafe { pca_v1 :: Pca :: from_ptr (0x4000_1000usize as _) } ; pub const CLK_TRIM : clk_v1 :: Clk = unsafe { clk_v1 :: Clk :: from_ptr (0x4000_1800usize as _) } ; pub const SYSCTRL : sysctrl_v7 :: Sysctrl = unsafe { sysctrl_v7 :: Sysctrl :: from_ptr (0x4000_2000usize as _) } ; pub const RESET : reset_v4 :: Reset = unsafe { reset_v4 :: Reset :: from_ptr (0x4000_201cusize as _) } ; pub const DEBUG_ACTIVE : debug_v3 :: Debug = unsafe { debug_v3 :: Debug :: from_ptr (0x4000_2038usize as _) } ; pub const ADC : adc_v6 :: Adc = unsafe { adc_v6 :: Adc :: from_ptr (0x4000_2400usize as _) } ; pub const BGR : bgr_v1 :: Bgr = unsafe { bgr_v1 :: Bgr :: from_ptr (0x4000_2400usize as _) } ; pub const LVD : lvd_v2 :: Lvd = unsafe { lvd_v2 :: Lvd :: from_ptr (0x4000_2400usize as _) } ; pub const OPA : opa_v4 :: Opa = unsafe { opa_v4 :: Opa :: from_ptr (0x4000_2400usize as _) } ; pub const VC : vc_v4 :: Vc = unsafe { vc_v4 :: Vc :: from_ptr (0x4000_2400usize as _) } ; pub const DAC : dac_v2 :: Dac = unsafe { dac_v2 :: Dac :: from_ptr (0x4000_2500usize as _) } ; pub const ADTIM4 : adtim_v1 :: Adtim = unsafe { adtim_v1 :: Adtim :: from_ptr (0x4000_3000usize as _) } ; pub const ADTIM5 : adtim_v1 :: Adtim = unsafe { adtim_v1 :: Adtim :: from_ptr (0x4000_3400usize as _) } ; pub const ADTIM6 : adtim_v1 :: Adtim = unsafe { adtim_v1 :: Adtim :: from_ptr (0x4000_3800usize as _) } ; pub const I2C1 : i_v3 :: I = unsafe { i_v3 :: I :: from_ptr (0x4000_4400usize as _) } ; pub const SPI1 : spi_v2 :: Spi = unsafe { spi_v2 :: Spi :: from_ptr (0x4000_4800usize as _) } ; pub const TRNG : trng_v1 :: Trng = unsafe { trng_v1 :: Trng :: from_ptr (0x4000_4c00usize as _) } ; pub const TIM3 : * mut () = 0x4000_5800usize as _ ; pub const LCD : lcd_v2 :: Lcd = unsafe { lcd_v2 :: Lcd :: from_ptr (0x4000_5c00usize as _) } ; pub const UART2 : uart_v3 :: Uart = unsafe { uart_v3 :: Uart :: from_ptr (0x4000_6000usize as _) } ; pub const UART3 : uart_v3 :: Uart = unsafe { uart_v3 :: Uart :: from_ptr (0x4000_6400usize as _) } ; pub const FLASH : flash_v6 :: Flash = unsafe { flash_v6 :: Flash :: from_ptr (0x4002_0000usize as _) } ; pub const RAM : ram_v4 :: Ram = unsafe { ram_v4 :: Ram :: from_ptr (0x4002_0400usize as _) } ; pub const CRC : crc_v2 :: Crc = unsafe { crc_v2 :: Crc :: from_ptr (0x4002_0900usize as _) } ; pub const GPIO : gpio_v4 :: Gpio = unsafe { gpio_v4 :: Gpio :: from_ptr (0x4002_0c00usize as _) } ; pub const DMAC : dmac_v3 :: Dmac = unsafe { dmac_v3 :: Dmac :: from_ptr (0x4002_1000usize as _) } ; pub const AES : aes_v2 :: Aes = unsafe { aes_v2 :: Aes :: from_ptr (0x4002_1400usize as _) } ; # [cfg (feature = "rt")]
 # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/adc_v6.rs"] pub mod adc_v6;
#[path="../../peripherals/adtim_v1.rs"] pub mod adtim_v1;
#[path="../../peripherals/aes_v2.rs"] pub mod aes_v2;
#[path="../../peripherals/bgr_v1.rs"] pub mod bgr_v1;
#[path="../../peripherals/clk_v1.rs"] pub mod clk_v1;
#[path="../../peripherals/crc_v2.rs"] pub mod crc_v2;
#[path="../../peripherals/dac_v2.rs"] pub mod dac_v2;
#[path="../../peripherals/debug_v3.rs"] pub mod debug_v3;
#[path="../../peripherals/dmac_v3.rs"] pub mod dmac_v3;
#[path="../../peripherals/flash_v6.rs"] pub mod flash_v6;
#[path="../../peripherals/gpio_v4.rs"] pub mod gpio_v4;
#[path="../../peripherals/i_v3.rs"] pub mod i_v3;
#[path="../../peripherals/lcd_v2.rs"] pub mod lcd_v2;
#[path="../../peripherals/lvd_v2.rs"] pub mod lvd_v2;
#[path="../../peripherals/opa_v4.rs"] pub mod opa_v4;
#[path="../../peripherals/pca_v1.rs"] pub mod pca_v1;
#[path="../../peripherals/ram_v4.rs"] pub mod ram_v4;
#[path="../../peripherals/reset_v4.rs"] pub mod reset_v4;
#[path="../../peripherals/spi_v2.rs"] pub mod spi_v2;
#[path="../../peripherals/sysctrl_v7.rs"] pub mod sysctrl_v7;
#[path="../../peripherals/trng_v1.rs"] pub mod trng_v1;
#[path="../../peripherals/uart_v3.rs"] pub mod uart_v3;
#[path="../../peripherals/vc_v4.rs"] pub mod vc_v4;
#[path="../../peripherals/wdt_v1.rs"] pub mod wdt_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout { regions: crate::memory_select::MEMORY };
