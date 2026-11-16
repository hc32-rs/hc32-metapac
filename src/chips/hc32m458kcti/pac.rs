




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - EXTINT_PORT_EIRQ0"]
EXTINT_PORT_EIRQ0 = 0 , # [doc = "1 - EXTINT_PORT_EIRQ1"]
EXTINT_PORT_EIRQ1 = 1 , # [doc = "2 - EXTINT_PORT_EIRQ2"]
EXTINT_PORT_EIRQ2 = 2 , # [doc = "3 - EXTINT_PORT_EIRQ3"]
EXTINT_PORT_EIRQ3 = 3 , # [doc = "4 - EXTINT_PORT_EIRQ4"]
EXTINT_PORT_EIRQ4 = 4 , # [doc = "5 - EXTINT_PORT_EIRQ5"]
EXTINT_PORT_EIRQ5 = 5 , # [doc = "6 - EXTINT_PORT_EIRQ6"]
EXTINT_PORT_EIRQ6 = 6 , # [doc = "7 - EXTINT_PORT_EIRQ7"]
EXTINT_PORT_EIRQ7 = 7 , # [doc = "8 - TMR0_1"]
TMR0_1 = 8 , # [doc = "9 - TMR0_2"]
TMR0_2 = 9 , # [doc = "10 - XTAL"]
XTAL = 10 , # [doc = "11 - SWDT"]
SWDT = 11 , # [doc = "12 - LVD1"]
LVD1 = 12 , # [doc = "13 - LVD2"]
LVD2 = 13 , # [doc = "14 - DMA_TC0_BTC0"]
DMA_TC0_BTC0 = 14 , # [doc = "15 - DMA_TC1_BTC1"]
DMA_TC1_BTC1 = 15 , # [doc = "16 - DMA_ERR"]
DMA_ERR = 16 , # [doc = "17 - EFM"]
EFM = 17 , # [doc = "18 - DFM"]
DFM = 18 , # [doc = "19 - FPU"]
FPU = 19 , # [doc = "20 - MAU"]
MAU = 20 , # [doc = "21 - TMR4_1_OVF"]
TMR4_1_OVF = 21 , # [doc = "22 - TMR4_1_UDF"]
TMR4_1_UDF = 22 , # [doc = "23 - TMR4_1_RELOAD"]
TMR4_1_RELOAD = 23 , # [doc = "24 - TMR4_1_GCMP"]
TMR4_1_GCMP = 24 , # [doc = "25 - TMR4_1_SCMP"]
TMR4_1_SCMP = 25 , # [doc = "26 - TMRA_1_OVF"]
TMRA_1_OVF = 26 , # [doc = "27 - TMRA_1_UDF"]
TMRA_1_UDF = 27 , # [doc = "28 - TMRA_1_CMP"]
TMRA_1_CMP = 28 , # [doc = "29 - TMRA_2_OVF"]
TMRA_2_OVF = 29 , # [doc = "30 - TMRA_2_UDF"]
TMRA_2_UDF = 30 , # [doc = "31 - TMRA_2_CMP"]
TMRA_2_CMP = 31 , # [doc = "32 - TMR4_2_OVF"]
TMR4_2_OVF = 32 , # [doc = "33 - TMR4_2_UDF"]
TMR4_2_UDF = 33 , # [doc = "34 - TMR4_2_RELOAD"]
TMR4_2_RELOAD = 34 , # [doc = "35 - TMR4_2_GCMP"]
TMR4_2_GCMP = 35 , # [doc = "36 - TMR4_2_SCMP"]
TMR4_2_SCMP = 36 , # [doc = "37 - TMRA_3_OVF"]
TMRA_3_OVF = 37 , # [doc = "38 - TMRA_3_UDF"]
TMRA_3_UDF = 38 , # [doc = "39 - TMRA_3_CMP"]
TMRA_3_CMP = 39 , # [doc = "40 - TMRA_4_OVF"]
TMRA_4_OVF = 40 , # [doc = "41 - TMRA_4_UDF"]
TMRA_4_UDF = 41 , # [doc = "42 - TMRA_4_CMP"]
TMRA_4_CMP = 42 , # [doc = "43 - CMP1"]
CMP1 = 43 , # [doc = "44 - CMP2"]
CMP2 = 44 , # [doc = "45 - EMB"]
EMB = 45 , # [doc = "46 - I2C_TX_RX"]
I2C_TX_RX = 46 , # [doc = "47 - I2C_TE_EE"]
I2C_TE_EE = 47 , # [doc = "48 - TMRA_5_OVF"]
TMRA_5_OVF = 48 , # [doc = "49 - TMRA_5_UDF"]
TMRA_5_UDF = 49 , # [doc = "50 - TMRA_5_CMP"]
TMRA_5_CMP = 50 , # [doc = "51 - USART1_TX_RX"]
USART1_TX_RX = 51 , # [doc = "52 - USART1_EI_TE_RTO_BW"]
USART1_EI_TE_RTO_BW = 52 , # [doc = "53 - USART1_TCI"]
USART1_TCI = 53 , # [doc = "54 - USART2_TX_RX"]
USART2_TX_RX = 54 , # [doc = "55 - USART2_EI_TE_RTO_BW"]
USART2_EI_TE_RTO_BW = 55 , # [doc = "56 - USART2_TCI"]
USART2_TCI = 56 , # [doc = "57 - USART3_TX_RX"]
USART3_TX_RX = 57 , # [doc = "58 - USART3_EI_TE_RTO_BW"]
USART3_EI_TE_RTO_BW = 58 , # [doc = "59 - USART3_TCI"]
USART3_TCI = 59 , # [doc = "60 - USART4_TX_RX"]
USART4_TX_RX = 60 , # [doc = "61 - USART4_EI_TE_RTO_BW"]
USART4_EI_TE_RTO_BW = 61 , # [doc = "62 - USART4_TCI"]
USART4_TCI = 62 , # [doc = "63 - SPI1_TX_RX"]
SPI1_TX_RX = 63 , # [doc = "64 - SPI1_IDLE_ERR"]
SPI1_IDLE_ERR = 64 , # [doc = "65 - SPI2_TX_RX"]
SPI2_TX_RX = 65 , # [doc = "66 - SPI2_IDLE_ERR"]
SPI2_IDLE_ERR = 66 , # [doc = "67 - FCM"]
FCM = 67 , # [doc = "68 - WDT"]
WDT = 68 , # [doc = "69 - USART1_WUPI"]
USART1_WUPI = 69 , # [doc = "70 - ADC1_ECOA"]
ADC1_ECOA = 70 , # [doc = "71 - ADC1_ECOB"]
ADC1_ECOB = 71 , # [doc = "72 - ADC1_ECOC"]
ADC1_ECOC = 72 , # [doc = "73 - ADC1_CMP"]
ADC1_CMP = 73 , # [doc = "74 - ADC2_ECOA"]
ADC2_ECOA = 74 , # [doc = "75 - ADC2_ECOB"]
ADC2_ECOB = 75 , # [doc = "76 - ADC2_ECOC"]
ADC2_ECOC = 76 , # [doc = "77 - ADC2_CMP"]
ADC2_CMP = 77 , # [doc = "78 - TRNG"]
TRNG = 78 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn EXTINT_PORT_EIRQ0 () ; fn EXTINT_PORT_EIRQ1 () ; fn EXTINT_PORT_EIRQ2 () ; fn EXTINT_PORT_EIRQ3 () ; fn EXTINT_PORT_EIRQ4 () ; fn EXTINT_PORT_EIRQ5 () ; fn EXTINT_PORT_EIRQ6 () ; fn EXTINT_PORT_EIRQ7 () ; fn TMR0_1 () ; fn TMR0_2 () ; fn XTAL () ; fn SWDT () ; fn LVD1 () ; fn LVD2 () ; fn DMA_TC0_BTC0 () ; fn DMA_TC1_BTC1 () ; fn DMA_ERR () ; fn EFM () ; fn DFM () ; fn FPU () ; fn MAU () ; fn TMR4_1_OVF () ; fn TMR4_1_UDF () ; fn TMR4_1_RELOAD () ; fn TMR4_1_GCMP () ; fn TMR4_1_SCMP () ; fn TMRA_1_OVF () ; fn TMRA_1_UDF () ; fn TMRA_1_CMP () ; fn TMRA_2_OVF () ; fn TMRA_2_UDF () ; fn TMRA_2_CMP () ; fn TMR4_2_OVF () ; fn TMR4_2_UDF () ; fn TMR4_2_RELOAD () ; fn TMR4_2_GCMP () ; fn TMR4_2_SCMP () ; fn TMRA_3_OVF () ; fn TMRA_3_UDF () ; fn TMRA_3_CMP () ; fn TMRA_4_OVF () ; fn TMRA_4_UDF () ; fn TMRA_4_CMP () ; fn CMP1 () ; fn CMP2 () ; fn EMB () ; fn I2C_TX_RX () ; fn I2C_TE_EE () ; fn TMRA_5_OVF () ; fn TMRA_5_UDF () ; fn TMRA_5_CMP () ; fn USART1_TX_RX () ; fn USART1_EI_TE_RTO_BW () ; fn USART1_TCI () ; fn USART2_TX_RX () ; fn USART2_EI_TE_RTO_BW () ; fn USART2_TCI () ; fn USART3_TX_RX () ; fn USART3_EI_TE_RTO_BW () ; fn USART3_TCI () ; fn USART4_TX_RX () ; fn USART4_EI_TE_RTO_BW () ; fn USART4_TCI () ; fn SPI1_TX_RX () ; fn SPI1_IDLE_ERR () ; fn SPI2_TX_RX () ; fn SPI2_IDLE_ERR () ; fn FCM () ; fn WDT () ; fn USART1_WUPI () ; fn ADC1_ECOA () ; fn ADC1_ECOB () ; fn ADC1_ECOC () ; fn ADC1_CMP () ; fn ADC2_ECOA () ; fn ADC2_ECOB () ; fn ADC2_ECOC () ; fn ADC2_CMP () ; fn TRNG () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 79]
= [Vector { _handler : EXTINT_PORT_EIRQ0 } , Vector { _handler : EXTINT_PORT_EIRQ1 } , Vector { _handler : EXTINT_PORT_EIRQ2 } , Vector { _handler : EXTINT_PORT_EIRQ3 } , Vector { _handler : EXTINT_PORT_EIRQ4 } , Vector { _handler : EXTINT_PORT_EIRQ5 } , Vector { _handler : EXTINT_PORT_EIRQ6 } , Vector { _handler : EXTINT_PORT_EIRQ7 } , Vector { _handler : TMR0_1 } , Vector { _handler : TMR0_2 } , Vector { _handler : XTAL } , Vector { _handler : SWDT } , Vector { _handler : LVD1 } , Vector { _handler : LVD2 } , Vector { _handler : DMA_TC0_BTC0 } , Vector { _handler : DMA_TC1_BTC1 } , Vector { _handler : DMA_ERR } , Vector { _handler : EFM } , Vector { _handler : DFM } , Vector { _handler : FPU } , Vector { _handler : MAU } , Vector { _handler : TMR4_1_OVF } , Vector { _handler : TMR4_1_UDF } , Vector { _handler : TMR4_1_RELOAD } , Vector { _handler : TMR4_1_GCMP } , Vector { _handler : TMR4_1_SCMP } , Vector { _handler : TMRA_1_OVF } , Vector { _handler : TMRA_1_UDF } , Vector { _handler : TMRA_1_CMP } , Vector { _handler : TMRA_2_OVF } , Vector { _handler : TMRA_2_UDF } , Vector { _handler : TMRA_2_CMP } , Vector { _handler : TMR4_2_OVF } , Vector { _handler : TMR4_2_UDF } , Vector { _handler : TMR4_2_RELOAD } , Vector { _handler : TMR4_2_GCMP } , Vector { _handler : TMR4_2_SCMP } , Vector { _handler : TMRA_3_OVF } , Vector { _handler : TMRA_3_UDF } , Vector { _handler : TMRA_3_CMP } , Vector { _handler : TMRA_4_OVF } , Vector { _handler : TMRA_4_UDF } , Vector { _handler : TMRA_4_CMP } , Vector { _handler : CMP1 } , Vector { _handler : CMP2 } , Vector { _handler : EMB } , Vector { _handler : I2C_TX_RX } , Vector { _handler : I2C_TE_EE } , Vector { _handler : TMRA_5_OVF } , Vector { _handler : TMRA_5_UDF } , Vector { _handler : TMRA_5_CMP } , Vector { _handler : USART1_TX_RX } , Vector { _handler : USART1_EI_TE_RTO_BW } , Vector { _handler : USART1_TCI } , Vector { _handler : USART2_TX_RX } , Vector { _handler : USART2_EI_TE_RTO_BW } , Vector { _handler : USART2_TCI } , Vector { _handler : USART3_TX_RX } , Vector { _handler : USART3_EI_TE_RTO_BW } , Vector { _handler : USART3_TCI } , Vector { _handler : USART4_TX_RX } , Vector { _handler : USART4_EI_TE_RTO_BW } , Vector { _handler : USART4_TCI } , Vector { _handler : SPI1_TX_RX } , Vector { _handler : SPI1_IDLE_ERR } , Vector { _handler : SPI2_TX_RX } , Vector { _handler : SPI2_IDLE_ERR } , Vector { _handler : FCM } , Vector { _handler : WDT } , Vector { _handler : USART1_WUPI } , Vector { _handler : ADC1_ECOA } , Vector { _handler : ADC1_ECOB } , Vector { _handler : ADC1_ECOC } , Vector { _handler : ADC1_CMP } , Vector { _handler : ADC2_ECOA } , Vector { _handler : ADC2_ECOB } , Vector { _handler : ADC2_ECOC } , Vector { _handler : ADC2_CMP } , Vector { _handler : TRNG } ,]
; } pub const ICG : icg_v11 :: Icg = unsafe { icg_v11 :: Icg :: from_ptr (0x0008_0000usize as _) } ; pub const EFM2 : efm_v15 :: Efm = unsafe { efm_v15 :: Efm :: from_ptr (0x0100_1c50usize as _) } ; pub const EFM : efm_v14 :: Efm = unsafe { efm_v14 :: Efm :: from_ptr (0x4001_0400usize as _) } ; pub const AOS : aos_v13 :: Aos = unsafe { aos_v13 :: Aos :: from_ptr (0x4001_0800usize as _) } ; pub const EMB0 : emb_v17 :: Emb = unsafe { emb_v17 :: Emb :: from_ptr (0x4001_7c00usize as _) } ; pub const EMB1 : emb_v17 :: Emb = unsafe { emb_v17 :: Emb :: from_ptr (0x4001_7c00usize as _) } ; pub const EMB2 : emb_v18 :: Emb = unsafe { emb_v18 :: Emb :: from_ptr (0x4001_7c40usize as _) } ; pub const EMB3 : emb_v18 :: Emb = unsafe { emb_v18 :: Emb :: from_ptr (0x4001_7c40usize as _) } ; pub const SPI1 : spi_v14 :: Spi = unsafe { spi_v14 :: Spi :: from_ptr (0x4001_c000usize as _) } ; pub const SPI2 : spi_v14 :: Spi = unsafe { spi_v14 :: Spi :: from_ptr (0x4001_c000usize as _) } ; pub const USART1 : usart_v35 :: Usart = unsafe { usart_v35 :: Usart :: from_ptr (0x4001_cc00usize as _) } ; pub const USART2 : usart_v35 :: Usart = unsafe { usart_v35 :: Usart :: from_ptr (0x4001_cc00usize as _) } ; pub const USART3 : usart_v35 :: Usart = unsafe { usart_v35 :: Usart :: from_ptr (0x4001_cc00usize as _) } ; pub const USART4 : usart_v35 :: Usart = unsafe { usart_v35 :: Usart :: from_ptr (0x4001_cc00usize as _) } ; pub const TMR01 : tmr_v39 :: Tmr = unsafe { tmr_v39 :: Tmr :: from_ptr (0x4002_4000usize as _) } ; pub const TMR02 : tmr_v39 :: Tmr = unsafe { tmr_v39 :: Tmr :: from_ptr (0x4002_4000usize as _) } ; pub const TMR41 : tmr_v40 :: Tmr = unsafe { tmr_v40 :: Tmr :: from_ptr (0x4003_8000usize as _) } ; pub const TMR42 : tmr_v41 :: Tmr = unsafe { tmr_v41 :: Tmr :: from_ptr (0x4003_8400usize as _) } ; pub const CMP1 : cmp_v19 :: Cmp = unsafe { cmp_v19 :: Cmp :: from_ptr (0x4003_8800usize as _) } ; pub const CMP2 : cmp_v19 :: Cmp = unsafe { cmp_v19 :: Cmp :: from_ptr (0x4003_8800usize as _) } ; pub const TMRA1 : tmra_v17 :: Tmra = unsafe { tmra_v17 :: Tmra :: from_ptr (0x4003_a000usize as _) } ; pub const TMRA2 : tmra_v17 :: Tmra = unsafe { tmra_v17 :: Tmra :: from_ptr (0x4003_a000usize as _) } ; pub const TMRA3 : tmra_v17 :: Tmra = unsafe { tmra_v17 :: Tmra :: from_ptr (0x4003_a000usize as _) } ; pub const TMRA4 : tmra_v17 :: Tmra = unsafe { tmra_v17 :: Tmra :: from_ptr (0x4003_a000usize as _) } ; pub const TMRA5 : tmra_v17 :: Tmra = unsafe { tmra_v17 :: Tmra :: from_ptr (0x4003_a000usize as _) } ; pub const I2C : i_v17 :: I = unsafe { i_v17 :: I :: from_ptr (0x4003_b400usize as _) } ; pub const ADC1 : adc_v30 :: Adc = unsafe { adc_v30 :: Adc :: from_ptr (0x4004_0000usize as _) } ; pub const ADC2 : adc_v31 :: Adc = unsafe { adc_v31 :: Adc :: from_ptr (0x4004_0400usize as _) } ; pub const TRNG : trng_v4 :: Trng = unsafe { trng_v4 :: Trng :: from_ptr (0x4004_2000usize as _) } ; pub const FCM : fcm_v4 :: Fcm = unsafe { fcm_v4 :: Fcm :: from_ptr (0x4004_8400usize as _) } ; pub const WDT : wdt_common :: Wdt = unsafe { wdt_common :: Wdt :: from_ptr (0x4004_9000usize as _) } ; pub const SWDT : swdt_v1 :: Swdt = unsafe { swdt_v1 :: Swdt :: from_ptr (0x4004_9400usize as _) } ; pub const DFM : dfm_v2 :: Dfm = unsafe { dfm_v2 :: Dfm :: from_ptr (0x4005_0000usize as _) } ; pub const CRC : crc_v9 :: Crc = unsafe { crc_v9 :: Crc :: from_ptr (0x4005_0400usize as _) } ; pub const SRAMC : sramc_v9 :: Sramc = unsafe { sramc_v9 :: Sramc :: from_ptr (0x4005_0800usize as _) } ; pub const INTC : intc_v13 :: Intc = unsafe { intc_v13 :: Intc :: from_ptr (0x4005_1000usize as _) } ; pub const DMA : dma_v12 :: Dma = unsafe { dma_v12 :: Dma :: from_ptr (0x4005_3000usize as _) } ; pub const GPIO : gpio_v15 :: Gpio = unsafe { gpio_v15 :: Gpio :: from_ptr (0x4005_3800usize as _) } ; pub const CMU : cmu_v14 :: Cmu = unsafe { cmu_v14 :: Cmu :: from_ptr (0x4005_4000usize as _) } ; pub const PWC : pwc_v15 :: Pwc = unsafe { pwc_v15 :: Pwc :: from_ptr (0x4005_4400usize as _) } ; pub const RMU : rmu_v11 :: Rmu = unsafe { rmu_v11 :: Rmu :: from_ptr (0x4005_4500usize as _) } ; pub const MAU : mau_v4 :: Mau = unsafe { mau_v4 :: Mau :: from_ptr (0x4005_4800usize as _) } ; pub const PERIC : peric_v11 :: Peric = unsafe { peric_v11 :: Peric :: from_ptr (0x4005_5400usize as _) } ; pub const DBGC : dbgc_v15 :: Dbgc = unsafe { dbgc_v15 :: Dbgc :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
 # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/adc_v30.rs"] pub mod adc_v30;
#[path="../../peripherals/adc_v31.rs"] pub mod adc_v31;
#[path="../../peripherals/aos_v13.rs"] pub mod aos_v13;
#[path="../../peripherals/cmp_v19.rs"] pub mod cmp_v19;
#[path="../../peripherals/cmu_v14.rs"] pub mod cmu_v14;
#[path="../../peripherals/crc_v9.rs"] pub mod crc_v9;
#[path="../../peripherals/dbgc_v15.rs"] pub mod dbgc_v15;
#[path="../../peripherals/dfm_v2.rs"] pub mod dfm_v2;
#[path="../../peripherals/dma_v12.rs"] pub mod dma_v12;
#[path="../../peripherals/efm_v14.rs"] pub mod efm_v14;
#[path="../../peripherals/efm_v15.rs"] pub mod efm_v15;
#[path="../../peripherals/emb_v17.rs"] pub mod emb_v17;
#[path="../../peripherals/emb_v18.rs"] pub mod emb_v18;
#[path="../../peripherals/fcm_v4.rs"] pub mod fcm_v4;
#[path="../../peripherals/gpio_v15.rs"] pub mod gpio_v15;
#[path="../../peripherals/i_v17.rs"] pub mod i_v17;
#[path="../../peripherals/icg_v11.rs"] pub mod icg_v11;
#[path="../../peripherals/intc_v13.rs"] pub mod intc_v13;
#[path="../../peripherals/mau_v4.rs"] pub mod mau_v4;
#[path="../../peripherals/peric_v11.rs"] pub mod peric_v11;
#[path="../../peripherals/pwc_v15.rs"] pub mod pwc_v15;
#[path="../../peripherals/rmu_v11.rs"] pub mod rmu_v11;
#[path="../../peripherals/spi_v14.rs"] pub mod spi_v14;
#[path="../../peripherals/sramc_v9.rs"] pub mod sramc_v9;
#[path="../../peripherals/swdt_v1.rs"] pub mod swdt_v1;
#[path="../../peripherals/tmr_v39.rs"] pub mod tmr_v39;
#[path="../../peripherals/tmr_v40.rs"] pub mod tmr_v40;
#[path="../../peripherals/tmr_v41.rs"] pub mod tmr_v41;
#[path="../../peripherals/tmra_v17.rs"] pub mod tmra_v17;
#[path="../../peripherals/trng_v4.rs"] pub mod trng_v4;
#[path="../../peripherals/usart_v35.rs"] pub mod usart_v35;
#[path="../../peripherals/wdt_common.rs"] pub mod wdt_common;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout { regions: crate::memory_select::MEMORY };
