




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - PORT0"]
PORT0 = 0 , # [doc = "1 - PORT1"]
PORT1 = 1 , # [doc = "2 - PORT2"]
PORT2 = 2 , # [doc = "3 - PORT3"]
PORT3 = 3 , # [doc = "6 - UART0"]
UART0 = 6 , # [doc = "7 - UART1"]
UART1 = 7 , # [doc = "8 - LPUART"]
LPUART = 8 , # [doc = "10 - SPI"]
SPI = 10 , # [doc = "12 - I2C"]
I2C = 12 , # [doc = "14 - TIM0"]
TIM0 = 14 , # [doc = "15 - TIM1"]
TIM1 = 15 , # [doc = "16 - TIM2"]
TIM2 = 16 , # [doc = "17 - LPTIM"]
LPTIM = 17 , # [doc = "18 - TIM4"]
TIM4 = 18 , # [doc = "19 - TIM5"]
TIM5 = 19 , # [doc = "20 - TIM6"]
TIM6 = 20 , # [doc = "21 - PCA"]
PCA = 21 , # [doc = "22 - WDT"]
WDT = 22 , # [doc = "23 - RTC"]
RTC = 23 , # [doc = "24 - ADC"]
ADC = 24 , # [doc = "26 - VC0"]
VC0 = 26 , # [doc = "27 - VC1"]
VC1 = 27 , # [doc = "28 - LVD"]
LVD = 28 , # [doc = "30 - FLASH_RAM"]
FLASH_RAM = 30 , # [doc = "31 - CLK_TRIM"]
CLK_TRIM = 31 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn PORT0 () ; fn PORT1 () ; fn PORT2 () ; fn PORT3 () ; fn UART0 () ; fn UART1 () ; fn LPUART () ; fn SPI () ; fn I2C () ; fn TIM0 () ; fn TIM1 () ; fn TIM2 () ; fn LPTIM () ; fn TIM4 () ; fn TIM5 () ; fn TIM6 () ; fn PCA () ; fn WDT () ; fn RTC () ; fn ADC () ; fn VC0 () ; fn VC1 () ; fn LVD () ; fn FLASH_RAM () ; fn CLK_TRIM () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 32]
= [Vector { _handler : PORT0 } , Vector { _handler : PORT1 } , Vector { _handler : PORT2 } , Vector { _handler : PORT3 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : UART0 } , Vector { _handler : UART1 } , Vector { _handler : LPUART } , Vector { _reserved : 0 } , Vector { _handler : SPI } , Vector { _reserved : 0 } , Vector { _handler : I2C } , Vector { _reserved : 0 } , Vector { _handler : TIM0 } , Vector { _handler : TIM1 } , Vector { _handler : TIM2 } , Vector { _handler : LPTIM } , Vector { _handler : TIM4 } , Vector { _handler : TIM5 } , Vector { _handler : TIM6 } , Vector { _handler : PCA } , Vector { _handler : WDT } , Vector { _handler : RTC } , Vector { _handler : ADC } , Vector { _reserved : 0 } , Vector { _handler : VC0 } , Vector { _handler : VC1 } , Vector { _handler : LVD } , Vector { _reserved : 0 } , Vector { _handler : FLASH_RAM } , Vector { _handler : CLK_TRIM } ,]
; } pub const UART0 : uart_v2 :: Uart = unsafe { uart_v2 :: Uart :: from_ptr (0x4000_0000usize as _) } ; pub const UART1 : uart_v2 :: Uart = unsafe { uart_v2 :: Uart :: from_ptr (0x4000_0100usize as _) } ; pub const LPUART : lpuart_v6 :: Lpuart = unsafe { lpuart_v6 :: Lpuart :: from_ptr (0x4000_0200usize as _) } ; pub const I2C : i_v8 :: I = unsafe { i_v8 :: I :: from_ptr (0x4000_0400usize as _) } ; pub const SPI : spi_v6 :: Spi = unsafe { spi_v6 :: Spi :: from_ptr (0x4000_0800usize as _) } ; pub const BT0 : bt_common :: Bt = unsafe { bt_common :: Bt :: from_ptr (0x4000_0c00usize as _) } ; pub const LPTIMER : lptimer_v3 :: Lptimer = unsafe { lptimer_v3 :: Lptimer :: from_ptr (0x4000_0c00usize as _) } ; pub const WDT : wdt_v2 :: Wdt = unsafe { wdt_v2 :: Wdt :: from_ptr (0x4000_0c00usize as _) } ; pub const BT1 : bt_common :: Bt = unsafe { bt_common :: Bt :: from_ptr (0x4000_0c20usize as _) } ; pub const BT2 : bt_common :: Bt = unsafe { bt_common :: Bt :: from_ptr (0x4000_0c40usize as _) } ; pub const PCA : pca_v3 :: Pca = unsafe { pca_v3 :: Pca :: from_ptr (0x4000_1000usize as _) } ; pub const RTC : rtc_v5 :: Rtc = unsafe { rtc_v5 :: Rtc :: from_ptr (0x4000_1400usize as _) } ; pub const CLK_TRIM : clk_v3 :: Clk = unsafe { clk_v3 :: Clk :: from_ptr (0x4000_1800usize as _) } ; pub const CLOCK : clock_common :: Clock = unsafe { clock_common :: Clock :: from_ptr (0x4000_2000usize as _) } ; pub const RESET : reset_v6 :: Reset = unsafe { reset_v6 :: Reset :: from_ptr (0x4000_201cusize as _) } ; pub const ADC : adc_v4 :: Adc = unsafe { adc_v4 :: Adc :: from_ptr (0x4000_2400usize as _) } ; pub const BGR : bgr_v2 :: Bgr = unsafe { bgr_v2 :: Bgr :: from_ptr (0x4000_2400usize as _) } ; pub const LVD : lvd_v6 :: Lvd = unsafe { lvd_v6 :: Lvd :: from_ptr (0x4000_2400usize as _) } ; pub const VC : vc_v7 :: Vc = unsafe { vc_v7 :: Vc :: from_ptr (0x4000_2400usize as _) } ; pub const ADT0 : adt_common :: Adt = unsafe { adt_common :: Adt :: from_ptr (0x4000_3000usize as _) } ; pub const ADT1 : adt_common :: Adt = unsafe { adt_common :: Adt :: from_ptr (0x4000_3400usize as _) } ; pub const ADT2 : adt_common :: Adt = unsafe { adt_common :: Adt :: from_ptr (0x4000_3800usize as _) } ; pub const FLASH : flash_v10 :: Flash = unsafe { flash_v10 :: Flash :: from_ptr (0x4002_0000usize as _) } ; pub const RAM : ram_v3 :: Ram = unsafe { ram_v3 :: Ram :: from_ptr (0x4002_0400usize as _) } ; pub const CRC : crc_v3 :: Crc = unsafe { crc_v3 :: Crc :: from_ptr (0x4002_0900usize as _) } ; pub const GPIO : gpio_v2 :: Gpio = unsafe { gpio_v2 :: Gpio :: from_ptr (0x4002_0c00usize as _) } ; # [cfg (feature = "rt")]
 # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/adc_v4.rs"] pub mod adc_v4;
#[path="../../peripherals/adt_common.rs"] pub mod adt_common;
#[path="../../peripherals/bgr_v2.rs"] pub mod bgr_v2;
#[path="../../peripherals/bt_common.rs"] pub mod bt_common;
#[path="../../peripherals/clk_v3.rs"] pub mod clk_v3;
#[path="../../peripherals/clock_common.rs"] pub mod clock_common;
#[path="../../peripherals/crc_v3.rs"] pub mod crc_v3;
#[path="../../peripherals/flash_v10.rs"] pub mod flash_v10;
#[path="../../peripherals/gpio_v2.rs"] pub mod gpio_v2;
#[path="../../peripherals/i_v8.rs"] pub mod i_v8;
#[path="../../peripherals/lptimer_v3.rs"] pub mod lptimer_v3;
#[path="../../peripherals/lpuart_v6.rs"] pub mod lpuart_v6;
#[path="../../peripherals/lvd_v6.rs"] pub mod lvd_v6;
#[path="../../peripherals/pca_v3.rs"] pub mod pca_v3;
#[path="../../peripherals/ram_v3.rs"] pub mod ram_v3;
#[path="../../peripherals/reset_v6.rs"] pub mod reset_v6;
#[path="../../peripherals/rtc_v5.rs"] pub mod rtc_v5;
#[path="../../peripherals/spi_v6.rs"] pub mod spi_v6;
#[path="../../peripherals/uart_v2.rs"] pub mod uart_v2;
#[path="../../peripherals/vc_v7.rs"] pub mod vc_v7;
#[path="../../peripherals/wdt_v2.rs"] pub mod wdt_v2;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout { regions: crate::memory_select::MEMORY };
