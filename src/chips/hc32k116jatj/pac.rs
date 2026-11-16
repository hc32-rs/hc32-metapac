




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - EXTINT00_EXTINT01"]
EXTINT00_EXTINT01 = 0 , # [doc = "1 - EXTINT02_EXTINT03"]
EXTINT02_EXTINT03 = 1 , # [doc = "2 - EXTINT04_EXTINT05"]
EXTINT04_EXTINT05 = 2 , # [doc = "3 - EXTINT06_EXTINT07"]
EXTINT06_EXTINT07 = 3 , # [doc = "4 - DMA"]
DMA = 4 , # [doc = "5 - EFM"]
EFM = 5 , # [doc = "6 - TMR0"]
TMR0 = 6 , # [doc = "7 - TMR2"]
TMR2 = 7 , # [doc = "8 - TMR4_CMP_OVF_UDF_RELOAD"]
TMR4_CMP_OVF_UDF_RELOAD = 8 , # [doc = "9 - TMR4_SCMP"]
TMR4_SCMP = 9 , # [doc = "10 - TMRA"]
TMRA = 10 , # [doc = "11 - USART1_WKUP_RI_TI"]
USART1_WKUP_RI_TI = 11 , # [doc = "12 - USART1_EI_TEDNI_TCI_RTO"]
USART1_EI_TEDNI_TCI_RTO = 12 , # [doc = "13 - USART2_RI_TI"]
USART2_RI_TI = 13 , # [doc = "14 - USART2_EI_TEDNI_TCI_RTO"]
USART2_EI_TEDNI_TCI_RTO = 14 , # [doc = "15 - USART3_RI_TI"]
USART3_RI_TI = 15 , # [doc = "16 - USART3_EI_TEDNI_TCI_RTO_BRKWKP"]
USART3_EI_TEDNI_TCI_RTO_BRKWKP = 16 , # [doc = "17 - USART4_RI_TI"]
USART4_RI_TI = 17 , # [doc = "18 - USART4_EI_TEDNI_TCI_RTO_BRKWKP"]
USART4_EI_TEDNI_TCI_RTO_BRKWKP = 18 , # [doc = "19 - SPI1_SPRI_SPTI_SPTEND"]
SPI1_SPRI_SPTI_SPTEND = 19 , # [doc = "20 - SPI1_SPII_SPEI"]
SPI1_SPII_SPEI = 20 , # [doc = "21 - SPI2_SPRI_SPTI_SPTEND"]
SPI2_SPRI_SPTI_SPTEND = 21 , # [doc = "22 - SPI2_SPII_SPEI"]
SPI2_SPII_SPEI = 22 , # [doc = "23 - I2C_RXI_TXI"]
I2C_RXI_TXI = 23 , # [doc = "24 - I2C_TEI_EEI"]
I2C_TEI_EEI = 24 , # [doc = "25 - MCAN"]
MCAN = 25 , # [doc = "26 - FCM_XTAL"]
FCM_XTAL = 26 , # [doc = "27 - EMB"]
EMB = 27 , # [doc = "29 - CMP"]
CMP = 29 , # [doc = "30 - ADC"]
ADC = 30 , # [doc = "31 - EVENTPORT_RTC_CTC_SWDT"]
EVENTPORT_RTC_CTC_SWDT = 31 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn EXTINT00_EXTINT01 () ; fn EXTINT02_EXTINT03 () ; fn EXTINT04_EXTINT05 () ; fn EXTINT06_EXTINT07 () ; fn DMA () ; fn EFM () ; fn TMR0 () ; fn TMR2 () ; fn TMR4_CMP_OVF_UDF_RELOAD () ; fn TMR4_SCMP () ; fn TMRA () ; fn USART1_WKUP_RI_TI () ; fn USART1_EI_TEDNI_TCI_RTO () ; fn USART2_RI_TI () ; fn USART2_EI_TEDNI_TCI_RTO () ; fn USART3_RI_TI () ; fn USART3_EI_TEDNI_TCI_RTO_BRKWKP () ; fn USART4_RI_TI () ; fn USART4_EI_TEDNI_TCI_RTO_BRKWKP () ; fn SPI1_SPRI_SPTI_SPTEND () ; fn SPI1_SPII_SPEI () ; fn SPI2_SPRI_SPTI_SPTEND () ; fn SPI2_SPII_SPEI () ; fn I2C_RXI_TXI () ; fn I2C_TEI_EEI () ; fn MCAN () ; fn FCM_XTAL () ; fn EMB () ; fn CMP () ; fn ADC () ; fn EVENTPORT_RTC_CTC_SWDT () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 32]
= [Vector { _handler : EXTINT00_EXTINT01 } , Vector { _handler : EXTINT02_EXTINT03 } , Vector { _handler : EXTINT04_EXTINT05 } , Vector { _handler : EXTINT06_EXTINT07 } , Vector { _handler : DMA } , Vector { _handler : EFM } , Vector { _handler : TMR0 } , Vector { _handler : TMR2 } , Vector { _handler : TMR4_CMP_OVF_UDF_RELOAD } , Vector { _handler : TMR4_SCMP } , Vector { _handler : TMRA } , Vector { _handler : USART1_WKUP_RI_TI } , Vector { _handler : USART1_EI_TEDNI_TCI_RTO } , Vector { _handler : USART2_RI_TI } , Vector { _handler : USART2_EI_TEDNI_TCI_RTO } , Vector { _handler : USART3_RI_TI } , Vector { _handler : USART3_EI_TEDNI_TCI_RTO_BRKWKP } , Vector { _handler : USART4_RI_TI } , Vector { _handler : USART4_EI_TEDNI_TCI_RTO_BRKWKP } , Vector { _handler : SPI1_SPRI_SPTI_SPTEND } , Vector { _handler : SPI1_SPII_SPEI } , Vector { _handler : SPI2_SPRI_SPTI_SPTEND } , Vector { _handler : SPI2_SPII_SPEI } , Vector { _handler : I2C_RXI_TXI } , Vector { _handler : I2C_TEI_EEI } , Vector { _handler : MCAN } , Vector { _handler : FCM_XTAL } , Vector { _handler : EMB } , Vector { _reserved : 0 } , Vector { _handler : CMP } , Vector { _handler : ADC } , Vector { _handler : EVENTPORT_RTC_CTC_SWDT } ,]
; } pub const ICGA : icga_common :: Icga = unsafe { icga_common :: Icga :: from_ptr (0x0100_0a00usize as _) } ; pub const CTC : ctc_v4 :: Ctc = unsafe { ctc_v4 :: Ctc :: from_ptr (0x4000_0000usize as _) } ; pub const EFM : efm_v11 :: Efm = unsafe { efm_v11 :: Efm :: from_ptr (0x4000_0400usize as _) } ; pub const AOS : aos_v11 :: Aos = unsafe { aos_v11 :: Aos :: from_ptr (0x4000_0c00usize as _) } ; pub const USART1 : usart_v31 :: Usart = unsafe { usart_v31 :: Usart :: from_ptr (0x4000_1800usize as _) } ; pub const USART2 : usart_v31 :: Usart = unsafe { usart_v31 :: Usart :: from_ptr (0x4000_1800usize as _) } ; pub const USART3 : usart_v32 :: Usart = unsafe { usart_v32 :: Usart :: from_ptr (0x4000_2000usize as _) } ; pub const USART4 : usart_v32 :: Usart = unsafe { usart_v32 :: Usart :: from_ptr (0x4000_2000usize as _) } ; pub const SPI1 : spi_v13 :: Spi = unsafe { spi_v13 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : spi_v13 :: Spi = unsafe { spi_v13 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const I2C : i_v16 :: I = unsafe { i_v16 :: I :: from_ptr (0x4000_4800usize as _) } ; pub const FCM : fcm_v4 :: Fcm = unsafe { fcm_v4 :: Fcm :: from_ptr (0x4000_5400usize as _) } ; pub const TMR0 : tmr_v35 :: Tmr = unsafe { tmr_v35 :: Tmr :: from_ptr (0x4000_5800usize as _) } ; pub const TMR21 : tmr_v36 :: Tmr = unsafe { tmr_v36 :: Tmr :: from_ptr (0x4000_5c00usize as _) } ; pub const TMR22 : tmr_v36 :: Tmr = unsafe { tmr_v36 :: Tmr :: from_ptr (0x4000_5c00usize as _) } ; pub const TMR4 : tmr_v14 :: Tmr = unsafe { tmr_v14 :: Tmr :: from_ptr (0x4000_6800usize as _) } ; pub const EMB : emb_v15 :: Emb = unsafe { emb_v15 :: Emb :: from_ptr (0x4000_6c00usize as _) } ; pub const TMRA : tmra_v15 :: Tmra = unsafe { tmra_v15 :: Tmra :: from_ptr (0x4000_7000usize as _) } ; pub const ADC : adc_v28 :: Adc = unsafe { adc_v28 :: Adc :: from_ptr (0x4000_b800usize as _) } ; pub const CMP : cmp_v16 :: Cmp = unsafe { cmp_v16 :: Cmp :: from_ptr (0x4000_c800usize as _) } ; pub const SWDT : swdt_v1 :: Swdt = unsafe { swdt_v1 :: Swdt :: from_ptr (0x4000_cc00usize as _) } ; pub const RTC : rtc_v12 :: Rtc = unsafe { rtc_v12 :: Rtc :: from_ptr (0x4000_d400usize as _) } ; pub const SRAMC : sramc_v7 :: Sramc = unsafe { sramc_v7 :: Sramc :: from_ptr (0x4001_0c00usize as _) } ; pub const INTC : intc_v11 :: Intc = unsafe { intc_v11 :: Intc :: from_ptr (0x4001_1000usize as _) } ; pub const MCAN : mcan_v2 :: Mcan = unsafe { mcan_v2 :: Mcan :: from_ptr (0x4001_1400usize as _) } ; pub const MCANRAM : mcanram_v3 :: Mcanram = unsafe { mcanram_v3 :: Mcanram :: from_ptr (0x4001_1800usize as _) } ; pub const DMA : dma_v9 :: Dma = unsafe { dma_v9 :: Dma :: from_ptr (0x4001_3000usize as _) } ; pub const PERIC : peric_v9 :: Peric = unsafe { peric_v9 :: Peric :: from_ptr (0x4001_3400usize as _) } ; pub const GPIO : gpio_v13 :: Gpio = unsafe { gpio_v13 :: Gpio :: from_ptr (0x4001_3800usize as _) } ; pub const PWC : pwc_v12 :: Pwc = unsafe { pwc_v12 :: Pwc :: from_ptr (0x4001_4000usize as _) } ; pub const RMU : rmu_v9 :: Rmu = unsafe { rmu_v9 :: Rmu :: from_ptr (0x4001_4100usize as _) } ; pub const CMU : cmu_v11 :: Cmu = unsafe { cmu_v11 :: Cmu :: from_ptr (0x4001_4400usize as _) } ; pub const DBGC : dbgc_v12 :: Dbgc = unsafe { dbgc_v12 :: Dbgc :: from_ptr (0x4001_5000usize as _) } ; pub const CRC : crc_v6 :: Crc = unsafe { crc_v6 :: Crc :: from_ptr (0x4001_5400usize as _) } ; pub const ICGB : icgb_common :: Icgb = unsafe { icgb_common :: Icgb :: from_ptr (0x4001_7800usize as _) } ; pub const DBGC_T : dbgc_v13 :: Dbgc = unsafe { dbgc_v13 :: Dbgc :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
 # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/adc_v28.rs"] pub mod adc_v28;
#[path="../../peripherals/aos_v11.rs"] pub mod aos_v11;
#[path="../../peripherals/cmp_v16.rs"] pub mod cmp_v16;
#[path="../../peripherals/cmu_v11.rs"] pub mod cmu_v11;
#[path="../../peripherals/crc_v6.rs"] pub mod crc_v6;
#[path="../../peripherals/ctc_v4.rs"] pub mod ctc_v4;
#[path="../../peripherals/dbgc_v12.rs"] pub mod dbgc_v12;
#[path="../../peripherals/dbgc_v13.rs"] pub mod dbgc_v13;
#[path="../../peripherals/dma_v9.rs"] pub mod dma_v9;
#[path="../../peripherals/efm_v11.rs"] pub mod efm_v11;
#[path="../../peripherals/emb_v15.rs"] pub mod emb_v15;
#[path="../../peripherals/fcm_v4.rs"] pub mod fcm_v4;
#[path="../../peripherals/gpio_v13.rs"] pub mod gpio_v13;
#[path="../../peripherals/i_v16.rs"] pub mod i_v16;
#[path="../../peripherals/icga_common.rs"] pub mod icga_common;
#[path="../../peripherals/icgb_common.rs"] pub mod icgb_common;
#[path="../../peripherals/intc_v11.rs"] pub mod intc_v11;
#[path="../../peripherals/mcan_v2.rs"] pub mod mcan_v2;
#[path="../../peripherals/mcanram_v3.rs"] pub mod mcanram_v3;
#[path="../../peripherals/peric_v9.rs"] pub mod peric_v9;
#[path="../../peripherals/pwc_v12.rs"] pub mod pwc_v12;
#[path="../../peripherals/rmu_v9.rs"] pub mod rmu_v9;
#[path="../../peripherals/rtc_v12.rs"] pub mod rtc_v12;
#[path="../../peripherals/spi_v13.rs"] pub mod spi_v13;
#[path="../../peripherals/sramc_v7.rs"] pub mod sramc_v7;
#[path="../../peripherals/swdt_v1.rs"] pub mod swdt_v1;
#[path="../../peripherals/tmr_v14.rs"] pub mod tmr_v14;
#[path="../../peripherals/tmr_v35.rs"] pub mod tmr_v35;
#[path="../../peripherals/tmr_v36.rs"] pub mod tmr_v36;
#[path="../../peripherals/tmra_v15.rs"] pub mod tmra_v15;
#[path="../../peripherals/usart_v31.rs"] pub mod usart_v31;
#[path="../../peripherals/usart_v32.rs"] pub mod usart_v32;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout { regions: crate::memory_select::MEMORY };
