




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - WWDT"]
WWDT = 0 , # [doc = "1 - IWDT"]
IWDT = 1 , # [doc = "2 - LVD"]
LVD = 2 , # [doc = "3 - INPUT0_3"]
INPUT0_3 = 3 , # [doc = "4 - INPUT4_7"]
INPUT4_7 = 4 , # [doc = "5 - INPUT8_11"]
INPUT8_11 = 5 , # [doc = "6 - INPUT12_15"]
INPUT12_15 = 6 , # [doc = "7 - DMAC"]
DMAC = 7 , # [doc = "8 - ATIM3"]
ATIM3 = 8 , # [doc = "9 - USART0"]
USART0 = 9 , # [doc = "10 - USART1"]
USART1 = 10 , # [doc = "11 - LPUART0"]
LPUART0 = 11 , # [doc = "12 - LPUART1"]
LPUART1 = 12 , # [doc = "13 - SPI0"]
SPI0 = 13 , # [doc = "14 - SPI1"]
SPI1 = 14 , # [doc = "15 - I2C0"]
I2C0 = 15 , # [doc = "16 - I2C1"]
I2C1 = 16 , # [doc = "17 - GTIM0_1"]
GTIM0_1 = 17 , # [doc = "18 - GTIM2_3"]
GTIM2_3 = 18 , # [doc = "19 - GTIM4_5"]
GTIM4_5 = 19 , # [doc = "20 - GTIM6_7_8"]
GTIM6_7_8 = 20 , # [doc = "21 - LPTIM0_1"]
LPTIM0_1 = 21 , # [doc = "22 - USART2"]
USART2 = 22 , # [doc = "23 - USART3"]
USART3 = 23 , # [doc = "24 - ADC"]
ADC = 24 , # [doc = "25 - USART4"]
USART4 = 25 , # [doc = "26 - USART5"]
USART5 = 26 , # [doc = "27 - WKUPIO"]
WKUPIO = 27 , # [doc = "28 - RTC"]
RTC = 28 , # [doc = "30 - EFM_DFM"]
EFM_DFM = 30 , # [doc = "31 - CTRIM"]
CTRIM = 31 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDT () ; fn IWDT () ; fn LVD () ; fn INPUT0_3 () ; fn INPUT4_7 () ; fn INPUT8_11 () ; fn INPUT12_15 () ; fn DMAC () ; fn ATIM3 () ; fn USART0 () ; fn USART1 () ; fn LPUART0 () ; fn LPUART1 () ; fn SPI0 () ; fn SPI1 () ; fn I2C0 () ; fn I2C1 () ; fn GTIM0_1 () ; fn GTIM2_3 () ; fn GTIM4_5 () ; fn GTIM6_7_8 () ; fn LPTIM0_1 () ; fn USART2 () ; fn USART3 () ; fn ADC () ; fn USART4 () ; fn USART5 () ; fn WKUPIO () ; fn RTC () ; fn EFM_DFM () ; fn CTRIM () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 32]
= [Vector { _handler : WWDT } , Vector { _handler : IWDT } , Vector { _handler : LVD } , Vector { _handler : INPUT0_3 } , Vector { _handler : INPUT4_7 } , Vector { _handler : INPUT8_11 } , Vector { _handler : INPUT12_15 } , Vector { _handler : DMAC } , Vector { _handler : ATIM3 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : LPUART0 } , Vector { _handler : LPUART1 } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : I2C0 } , Vector { _handler : I2C1 } , Vector { _handler : GTIM0_1 } , Vector { _handler : GTIM2_3 } , Vector { _handler : GTIM4_5 } , Vector { _handler : GTIM6_7_8 } , Vector { _handler : LPTIM0_1 } , Vector { _handler : USART2 } , Vector { _handler : USART3 } , Vector { _handler : ADC } , Vector { _handler : USART4 } , Vector { _handler : USART5 } , Vector { _handler : WKUPIO } , Vector { _handler : RTC } , Vector { _reserved : 0 } , Vector { _handler : EFM_DFM } , Vector { _handler : CTRIM } ,]
; } pub const CTRIM : ctrim_common :: Ctrim = unsafe { ctrim_common :: Ctrim :: from_ptr (0x4000_0000usize as _) } ; pub const IWDT : iwdt_common :: Iwdt = unsafe { iwdt_common :: Iwdt :: from_ptr (0x4000_0800usize as _) } ; pub const WWDT : wwdt_v1 :: Wwdt = unsafe { wwdt_v1 :: Wwdt :: from_ptr (0x4000_0c00usize as _) } ; pub const RTC : rtc_v11 :: Rtc = unsafe { rtc_v11 :: Rtc :: from_ptr (0x4000_1000usize as _) } ; pub const DBGC : dbgc_v9 :: Dbgc = unsafe { dbgc_v9 :: Dbgc :: from_ptr (0x4000_1c00usize as _) } ; pub const USART0 : usart_v26 :: Usart = unsafe { usart_v26 :: Usart :: from_ptr (0x4000_2400usize as _) } ; pub const USART1 : usart_v26 :: Usart = unsafe { usart_v26 :: Usart :: from_ptr (0x4000_2400usize as _) } ; pub const USART2 : usart_v26 :: Usart = unsafe { usart_v26 :: Usart :: from_ptr (0x4000_2400usize as _) } ; pub const USART3 : usart_v26 :: Usart = unsafe { usart_v26 :: Usart :: from_ptr (0x4000_2400usize as _) } ; pub const USART4 : usart_v26 :: Usart = unsafe { usart_v26 :: Usart :: from_ptr (0x4000_2400usize as _) } ; pub const USART5 : usart_v26 :: Usart = unsafe { usart_v26 :: Usart :: from_ptr (0x4000_2400usize as _) } ; pub const LPUART0 : lpuart_common :: Lpuart = unsafe { lpuart_common :: Lpuart :: from_ptr (0x4000_3800usize as _) } ; pub const LPUART1 : lpuart_common :: Lpuart = unsafe { lpuart_common :: Lpuart :: from_ptr (0x4000_3800usize as _) } ; pub const SPI0 : spi_v11 :: Spi = unsafe { spi_v11 :: Spi :: from_ptr (0x4000_4000usize as _) } ; pub const SPI1 : spi_v11 :: Spi = unsafe { spi_v11 :: Spi :: from_ptr (0x4000_4000usize as _) } ; pub const I2C0 : i_v13 :: I = unsafe { i_v13 :: I :: from_ptr (0x4000_4800usize as _) } ; pub const I2C1 : i_v13 :: I = unsafe { i_v13 :: I :: from_ptr (0x4000_4800usize as _) } ; pub const LPTIM0 : lptim_v1 :: Lptim = unsafe { lptim_v1 :: Lptim :: from_ptr (0x4000_5000usize as _) } ; pub const LPTIM1 : lptim_v1 :: Lptim = unsafe { lptim_v1 :: Lptim :: from_ptr (0x4000_5000usize as _) } ; pub const GTIM0 : gtim_common :: Gtim = unsafe { gtim_common :: Gtim :: from_ptr (0x4000_5800usize as _) } ; pub const GTIM1 : gtim_common :: Gtim = unsafe { gtim_common :: Gtim :: from_ptr (0x4000_5800usize as _) } ; pub const GTIM2 : gtim_common :: Gtim = unsafe { gtim_common :: Gtim :: from_ptr (0x4000_5800usize as _) } ; pub const GTIM3 : gtim_common :: Gtim = unsafe { gtim_common :: Gtim :: from_ptr (0x4000_5800usize as _) } ; pub const GTIM4 : gtim_common :: Gtim = unsafe { gtim_common :: Gtim :: from_ptr (0x4000_5800usize as _) } ; pub const GTIM5 : gtim_common :: Gtim = unsafe { gtim_common :: Gtim :: from_ptr (0x4000_5800usize as _) } ; pub const GTIM6 : gtim_common :: Gtim = unsafe { gtim_common :: Gtim :: from_ptr (0x4000_5800usize as _) } ; pub const GTIM7 : gtim_common :: Gtim = unsafe { gtim_common :: Gtim :: from_ptr (0x4000_5800usize as _) } ; pub const GTIM8 : gtim_common :: Gtim = unsafe { gtim_common :: Gtim :: from_ptr (0x4000_5800usize as _) } ; pub const LVD : lvd_common :: Lvd = unsafe { lvd_common :: Lvd :: from_ptr (0x4000_7400usize as _) } ; pub const ADC : adc_v21 :: Adc = unsafe { adc_v21 :: Adc :: from_ptr (0x4000_7800usize as _) } ; pub const ATIM3 : atim_common :: Atim = unsafe { atim_common :: Atim :: from_ptr (0x4000_e400usize as _) } ; pub const GPIO : gpio_v21 :: Gpio = unsafe { gpio_v21 :: Gpio :: from_ptr (0x4002_0000usize as _) } ; pub const EXTI : exti_common :: Exti = unsafe { exti_common :: Exti :: from_ptr (0x4002_4000usize as _) } ; pub const DMAC : dmac_common :: Dmac = unsafe { dmac_common :: Dmac :: from_ptr (0x4002_4800usize as _) } ; pub const SRAM : sram_common :: Sram = unsafe { sram_common :: Sram :: from_ptr (0x4002_4c00usize as _) } ; pub const SYSCTRL : sysctrl_common :: Sysctrl = unsafe { sysctrl_common :: Sysctrl :: from_ptr (0x4002_5000usize as _) } ; pub const CSU0 : csu_common :: Csu = unsafe { csu_common :: Csu :: from_ptr (0x4002_5100usize as _) } ; pub const CSU1 : csu_common :: Csu = unsafe { csu_common :: Csu :: from_ptr (0x4002_5100usize as _) } ; pub const CSU2 : csu_common :: Csu = unsafe { csu_common :: Csu :: from_ptr (0x4002_5100usize as _) } ; pub const EFM : efm_v8 :: Efm = unsafe { efm_v8 :: Efm :: from_ptr (0x4002_6400usize as _) } ; pub const DFM : dfm_v1 :: Dfm = unsafe { dfm_v1 :: Dfm :: from_ptr (0x4002_6800usize as _) } ; pub const CRC : crc_v10 :: Crc = unsafe { crc_v10 :: Crc :: from_ptr (0x4002_7800usize as _) } ; # [cfg (feature = "rt")]
 # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/adc_v21.rs"] pub mod adc_v21;
#[path="../../peripherals/atim_common.rs"] pub mod atim_common;
#[path="../../peripherals/crc_v10.rs"] pub mod crc_v10;
#[path="../../peripherals/csu_common.rs"] pub mod csu_common;
#[path="../../peripherals/ctrim_common.rs"] pub mod ctrim_common;
#[path="../../peripherals/dbgc_v9.rs"] pub mod dbgc_v9;
#[path="../../peripherals/dfm_v1.rs"] pub mod dfm_v1;
#[path="../../peripherals/dmac_common.rs"] pub mod dmac_common;
#[path="../../peripherals/efm_v8.rs"] pub mod efm_v8;
#[path="../../peripherals/exti_common.rs"] pub mod exti_common;
#[path="../../peripherals/gpio_v21.rs"] pub mod gpio_v21;
#[path="../../peripherals/gtim_common.rs"] pub mod gtim_common;
#[path="../../peripherals/i_v13.rs"] pub mod i_v13;
#[path="../../peripherals/iwdt_common.rs"] pub mod iwdt_common;
#[path="../../peripherals/lptim_v1.rs"] pub mod lptim_v1;
#[path="../../peripherals/lpuart_common.rs"] pub mod lpuart_common;
#[path="../../peripherals/lvd_common.rs"] pub mod lvd_common;
#[path="../../peripherals/rtc_v11.rs"] pub mod rtc_v11;
#[path="../../peripherals/spi_v11.rs"] pub mod spi_v11;
#[path="../../peripherals/sram_common.rs"] pub mod sram_common;
#[path="../../peripherals/sysctrl_common.rs"] pub mod sysctrl_common;
#[path="../../peripherals/usart_v26.rs"] pub mod usart_v26;
#[path="../../peripherals/wwdt_v1.rs"] pub mod wwdt_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout { regions: crate::memory_select::MEMORY };
