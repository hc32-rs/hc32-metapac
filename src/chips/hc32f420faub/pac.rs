




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - CTRIM"]
CTRIM = 0 , # [doc = "1 - VC"]
VC = 1 , # [doc = "2 - LVD"]
LVD = 2 , # [doc = "3 - PORTA"]
PORTA = 3 , # [doc = "4 - PORTB"]
PORTB = 4 , # [doc = "5 - PORTC"]
PORTC = 5 , # [doc = "6 - PORTD"]
PORTD = 6 , # [doc = "7 - IWDT"]
IWDT = 7 , # [doc = "8 - WWDT"]
WWDT = 8 , # [doc = "9 - CLK_FAULT"]
CLK_FAULT = 9 , # [doc = "10 - ADC0_ADC1"]
ADC0_ADC1 = 10 , # [doc = "11 - FLASH"]
FLASH = 11 , # [doc = "12 - USART0_RX"]
USART0_RX = 12 , # [doc = "13 - USART0_TX"]
USART0_TX = 13 , # [doc = "14 - USART0_FLAG"]
USART0_FLAG = 14 , # [doc = "15 - USART1_RX"]
USART1_RX = 15 , # [doc = "16 - USART1_TX"]
USART1_TX = 16 , # [doc = "17 - USART1_FLAG"]
USART1_FLAG = 17 , # [doc = "18 - USART2_RX"]
USART2_RX = 18 , # [doc = "19 - USART2_TX"]
USART2_TX = 19 , # [doc = "20 - USART2_FLAG"]
USART2_FLAG = 20 , # [doc = "21 - USART3_RX"]
USART3_RX = 21 , # [doc = "22 - USART3_TX"]
USART3_TX = 22 , # [doc = "23 - USART3_FLAG"]
USART3_FLAG = 23 , # [doc = "24 - I2C0"]
I2C0 = 24 , # [doc = "25 - I2C1"]
I2C1 = 25 , # [doc = "26 - SPI0_TX"]
SPI0_TX = 26 , # [doc = "27 - SPI0_RX"]
SPI0_RX = 27 , # [doc = "28 - SPI0_SSR"]
SPI0_SSR = 28 , # [doc = "29 - SPI0_ERR"]
SPI0_ERR = 29 , # [doc = "30 - SPI1_TX"]
SPI1_TX = 30 , # [doc = "31 - SPI1_RX"]
SPI1_RX = 31 , # [doc = "32 - SPI1_SSR"]
SPI1_SSR = 32 , # [doc = "33 - SPI1_ERR"]
SPI1_ERR = 33 , # [doc = "34 - GTIM0_UEV_ETR_BTIMA"]
GTIM0_UEV_ETR_BTIMA = 34 , # [doc = "35 - GTIM0_CH01_BTIMB"]
GTIM0_CH01_BTIMB = 35 , # [doc = "36 - GTIM0_CH23_BTIMC"]
GTIM0_CH23_BTIMC = 36 , # [doc = "37 - GTIM1_UEV_ETR_BTIMA"]
GTIM1_UEV_ETR_BTIMA = 37 , # [doc = "38 - GTIM1_CH01_BTIMB"]
GTIM1_CH01_BTIMB = 38 , # [doc = "39 - GTIM1_CH23_BTIMC"]
GTIM1_CH23_BTIMC = 39 , # [doc = "40 - GTIM2_UEV_ETR_BTIMA"]
GTIM2_UEV_ETR_BTIMA = 40 , # [doc = "41 - GTIM2_CH01_BTIMB"]
GTIM2_CH01_BTIMB = 41 , # [doc = "42 - GTIM2_CH23_BTIMC"]
GTIM2_CH23_BTIMC = 42 , # [doc = "43 - ATIM0_UEV"]
ATIM0_UEV = 43 , # [doc = "44 - ATIM0_CHA"]
ATIM0_CHA = 44 , # [doc = "45 - ATIM0_CHB"]
ATIM0_CHB = 45 , # [doc = "46 - ATIM0_BKETR"]
ATIM0_BKETR = 46 , # [doc = "47 - ATIM3_UEV"]
ATIM3_UEV = 47 , # [doc = "48 - ATIM3_CHA"]
ATIM3_CHA = 48 , # [doc = "49 - ATIM3_CHB"]
ATIM3_CHB = 49 , # [doc = "50 - ATIM3_BK_ETR_OV_UD_CMP3"]
ATIM3_BK_ETR_OV_UD_CMP3 = 50 , # [doc = "51 - DMA_CH0"]
DMA_CH0 = 51 , # [doc = "52 - DMA_CH1"]
DMA_CH1 = 52 , # [doc = "53 - DMA_CH2"]
DMA_CH2 = 53 , # [doc = "54 - DMA_CH3"]
DMA_CH3 = 54 , # [doc = "55 - DMA_CH4"]
DMA_CH4 = 55 , # [doc = "56 - DMA_CH5"]
DMA_CH5 = 56 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn CTRIM () ; fn VC () ; fn LVD () ; fn PORTA () ; fn PORTB () ; fn PORTC () ; fn PORTD () ; fn IWDT () ; fn WWDT () ; fn CLK_FAULT () ; fn ADC0_ADC1 () ; fn FLASH () ; fn USART0_RX () ; fn USART0_TX () ; fn USART0_FLAG () ; fn USART1_RX () ; fn USART1_TX () ; fn USART1_FLAG () ; fn USART2_RX () ; fn USART2_TX () ; fn USART2_FLAG () ; fn USART3_RX () ; fn USART3_TX () ; fn USART3_FLAG () ; fn I2C0 () ; fn I2C1 () ; fn SPI0_TX () ; fn SPI0_RX () ; fn SPI0_SSR () ; fn SPI0_ERR () ; fn SPI1_TX () ; fn SPI1_RX () ; fn SPI1_SSR () ; fn SPI1_ERR () ; fn GTIM0_UEV_ETR_BTIMA () ; fn GTIM0_CH01_BTIMB () ; fn GTIM0_CH23_BTIMC () ; fn GTIM1_UEV_ETR_BTIMA () ; fn GTIM1_CH01_BTIMB () ; fn GTIM1_CH23_BTIMC () ; fn GTIM2_UEV_ETR_BTIMA () ; fn GTIM2_CH01_BTIMB () ; fn GTIM2_CH23_BTIMC () ; fn ATIM0_UEV () ; fn ATIM0_CHA () ; fn ATIM0_CHB () ; fn ATIM0_BKETR () ; fn ATIM3_UEV () ; fn ATIM3_CHA () ; fn ATIM3_CHB () ; fn ATIM3_BK_ETR_OV_UD_CMP3 () ; fn DMA_CH0 () ; fn DMA_CH1 () ; fn DMA_CH2 () ; fn DMA_CH3 () ; fn DMA_CH4 () ; fn DMA_CH5 () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 57]
= [Vector { _handler : CTRIM } , Vector { _handler : VC } , Vector { _handler : LVD } , Vector { _handler : PORTA } , Vector { _handler : PORTB } , Vector { _handler : PORTC } , Vector { _handler : PORTD } , Vector { _handler : IWDT } , Vector { _handler : WWDT } , Vector { _handler : CLK_FAULT } , Vector { _handler : ADC0_ADC1 } , Vector { _handler : FLASH } , Vector { _handler : USART0_RX } , Vector { _handler : USART0_TX } , Vector { _handler : USART0_FLAG } , Vector { _handler : USART1_RX } , Vector { _handler : USART1_TX } , Vector { _handler : USART1_FLAG } , Vector { _handler : USART2_RX } , Vector { _handler : USART2_TX } , Vector { _handler : USART2_FLAG } , Vector { _handler : USART3_RX } , Vector { _handler : USART3_TX } , Vector { _handler : USART3_FLAG } , Vector { _handler : I2C0 } , Vector { _handler : I2C1 } , Vector { _handler : SPI0_TX } , Vector { _handler : SPI0_RX } , Vector { _handler : SPI0_SSR } , Vector { _handler : SPI0_ERR } , Vector { _handler : SPI1_TX } , Vector { _handler : SPI1_RX } , Vector { _handler : SPI1_SSR } , Vector { _handler : SPI1_ERR } , Vector { _handler : GTIM0_UEV_ETR_BTIMA } , Vector { _handler : GTIM0_CH01_BTIMB } , Vector { _handler : GTIM0_CH23_BTIMC } , Vector { _handler : GTIM1_UEV_ETR_BTIMA } , Vector { _handler : GTIM1_CH01_BTIMB } , Vector { _handler : GTIM1_CH23_BTIMC } , Vector { _handler : GTIM2_UEV_ETR_BTIMA } , Vector { _handler : GTIM2_CH01_BTIMB } , Vector { _handler : GTIM2_CH23_BTIMC } , Vector { _handler : ATIM0_UEV } , Vector { _handler : ATIM0_CHA } , Vector { _handler : ATIM0_CHB } , Vector { _handler : ATIM0_BKETR } , Vector { _handler : ATIM3_UEV } , Vector { _handler : ATIM3_CHA } , Vector { _handler : ATIM3_CHB } , Vector { _handler : ATIM3_BK_ETR_OV_UD_CMP3 } , Vector { _handler : DMA_CH0 } , Vector { _handler : DMA_CH1 } , Vector { _handler : DMA_CH2 } , Vector { _handler : DMA_CH3 } , Vector { _handler : DMA_CH4 } , Vector { _handler : DMA_CH5 } ,]
; } pub const USART0 : usart_common :: Usart = unsafe { usart_common :: Usart :: from_ptr (0x4000_0000usize as _) } ; pub const USART1 : usart_common :: Usart = unsafe { usart_common :: Usart :: from_ptr (0x4000_0400usize as _) } ; pub const USART2 : usart_common :: Usart = unsafe { usart_common :: Usart :: from_ptr (0x4000_0800usize as _) } ; pub const USART3 : usart_common :: Usart = unsafe { usart_common :: Usart :: from_ptr (0x4000_0c00usize as _) } ; pub const I2C0 : i_v7 :: I = unsafe { i_v7 :: I :: from_ptr (0x4000_2800usize as _) } ; pub const I2C1 : i_v7 :: I = unsafe { i_v7 :: I :: from_ptr (0x4000_2c00usize as _) } ; pub const SPI0 : spi_v1 :: Spi = unsafe { spi_v1 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI1 : spi_v1 :: Spi = unsafe { spi_v1 :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const BTIM0 : btim_v3 :: Btim = unsafe { btim_v3 :: Btim :: from_ptr (0x4000_5000usize as _) } ; pub const GTIM0 : gtim_v3 :: Gtim = unsafe { gtim_v3 :: Gtim :: from_ptr (0x4000_5000usize as _) } ; pub const BTIM1 : btim_v3 :: Btim = unsafe { btim_v3 :: Btim :: from_ptr (0x4000_5100usize as _) } ; pub const BTIM2 : btim_v3 :: Btim = unsafe { btim_v3 :: Btim :: from_ptr (0x4000_5200usize as _) } ; pub const BTIM3 : btim_v2 :: Btim = unsafe { btim_v2 :: Btim :: from_ptr (0x4000_5400usize as _) } ; pub const GTIM1 : gtim_v3 :: Gtim = unsafe { gtim_v3 :: Gtim :: from_ptr (0x4000_5400usize as _) } ; pub const BTIM4 : btim_v2 :: Btim = unsafe { btim_v2 :: Btim :: from_ptr (0x4000_5500usize as _) } ; pub const BTIM5 : btim_v2 :: Btim = unsafe { btim_v2 :: Btim :: from_ptr (0x4000_5600usize as _) } ; pub const BTIM6 : btim_v4 :: Btim = unsafe { btim_v4 :: Btim :: from_ptr (0x4000_5800usize as _) } ; pub const GTIM2 : gtim_v3 :: Gtim = unsafe { gtim_v3 :: Gtim :: from_ptr (0x4000_5800usize as _) } ; pub const BTIM7 : btim_v4 :: Btim = unsafe { btim_v4 :: Btim :: from_ptr (0x4000_5900usize as _) } ; pub const BTIM8 : btim_v4 :: Btim = unsafe { btim_v4 :: Btim :: from_ptr (0x4000_5a00usize as _) } ; pub const ATIM0 : * mut () = 0x4000_6000usize as _ ; pub const ATIM3 : * mut () = 0x4000_6c00usize as _ ; pub const CTRIM : ctrim_v3 :: Ctrim = unsafe { ctrim_v3 :: Ctrim :: from_ptr (0x4000_8000usize as _) } ; pub const WWDT : wwdt_common :: Wwdt = unsafe { wwdt_common :: Wwdt :: from_ptr (0x4000_8800usize as _) } ; pub const IWDT : iwdt_v1 :: Iwdt = unsafe { iwdt_v1 :: Iwdt :: from_ptr (0x4000_8c80usize as _) } ; pub const ADC0 : adc_v7 :: Adc = unsafe { adc_v7 :: Adc :: from_ptr (0x4000_a000usize as _) } ; pub const ADC1 : adc_v8 :: Adc = unsafe { adc_v8 :: Adc :: from_ptr (0x4000_a400usize as _) } ; pub const VC0 : vc_v5 :: Vc = unsafe { vc_v5 :: Vc :: from_ptr (0x4000_a800usize as _) } ; pub const VC1 : vc_v5 :: Vc = unsafe { vc_v5 :: Vc :: from_ptr (0x4000_a810usize as _) } ; pub const LVD : lvd_v3 :: Lvd = unsafe { lvd_v3 :: Lvd :: from_ptr (0x4000_aa00usize as _) } ; pub const GPIOAUX : gpioaux_v3 :: Gpioaux = unsafe { gpioaux_v3 :: Gpioaux :: from_ptr (0x4000_b800usize as _) } ; pub const SYSCTRL : sysctrl_v8 :: Sysctrl = unsafe { sysctrl_v8 :: Sysctrl :: from_ptr (0x4000_b800usize as _) } ; pub const DMAC : dmac_v4 :: Dmac = unsafe { dmac_v4 :: Dmac :: from_ptr (0x4002_0000usize as _) } ; pub const CRC : crc_v4 :: Crc = unsafe { crc_v4 :: Crc :: from_ptr (0x4002_0800usize as _) } ; pub const FLASH : flash_v8 :: Flash = unsafe { flash_v8 :: Flash :: from_ptr (0x4002_1400usize as _) } ; pub const GPIOA : gpioa_v1 :: Gpioa = unsafe { gpioa_v1 :: Gpioa :: from_ptr (0x4002_4000usize as _) } ; pub const GPIOB : gpioa_v1 :: Gpioa = unsafe { gpioa_v1 :: Gpioa :: from_ptr (0x4002_4400usize as _) } ; pub const GPIOC : gpioa_v1 :: Gpioa = unsafe { gpioa_v1 :: Gpioa :: from_ptr (0x4002_4800usize as _) } ; pub const GPIOD : gpioa_v1 :: Gpioa = unsafe { gpioa_v1 :: Gpioa :: from_ptr (0x4002_4c00usize as _) } ; # [cfg (feature = "rt")]
 # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/adc_v7.rs"] pub mod adc_v7;
#[path="../../peripherals/adc_v8.rs"] pub mod adc_v8;
#[path="../../peripherals/btim_v2.rs"] pub mod btim_v2;
#[path="../../peripherals/btim_v3.rs"] pub mod btim_v3;
#[path="../../peripherals/btim_v4.rs"] pub mod btim_v4;
#[path="../../peripherals/crc_v4.rs"] pub mod crc_v4;
#[path="../../peripherals/ctrim_v3.rs"] pub mod ctrim_v3;
#[path="../../peripherals/dmac_v4.rs"] pub mod dmac_v4;
#[path="../../peripherals/flash_v8.rs"] pub mod flash_v8;
#[path="../../peripherals/gpioa_v1.rs"] pub mod gpioa_v1;
#[path="../../peripherals/gpioaux_v3.rs"] pub mod gpioaux_v3;
#[path="../../peripherals/gtim_v3.rs"] pub mod gtim_v3;
#[path="../../peripherals/i_v7.rs"] pub mod i_v7;
#[path="../../peripherals/iwdt_v1.rs"] pub mod iwdt_v1;
#[path="../../peripherals/lvd_v3.rs"] pub mod lvd_v3;
#[path="../../peripherals/spi_v1.rs"] pub mod spi_v1;
#[path="../../peripherals/sysctrl_v8.rs"] pub mod sysctrl_v8;
#[path="../../peripherals/usart_common.rs"] pub mod usart_common;
#[path="../../peripherals/vc_v5.rs"] pub mod vc_v5;
#[path="../../peripherals/wwdt_common.rs"] pub mod wwdt_common;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout { regions: crate::memory_select::MEMORY };
