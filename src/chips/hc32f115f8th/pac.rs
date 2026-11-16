




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - EXTINT00"]
EXTINT00 = 0 , # [doc = "1 - EXTINT01"]
EXTINT01 = 1 , # [doc = "2 - EXTINT02"]
EXTINT02 = 2 , # [doc = "3 - EXTINT03"]
EXTINT03 = 3 , # [doc = "4 - EXTINT04"]
EXTINT04 = 4 , # [doc = "5 - EXTINT05"]
EXTINT05 = 5 , # [doc = "6 - EXTINT06"]
EXTINT06 = 6 , # [doc = "7 - EXTINT07"]
EXTINT07 = 7 , # [doc = "8 - INT008"]
INT008 = 8 , # [doc = "9 - INT009"]
INT009 = 9 , # [doc = "10 - INT010"]
INT010 = 10 , # [doc = "11 - INT011"]
INT011 = 11 , # [doc = "12 - INT012"]
INT012 = 12 , # [doc = "13 - INT013"]
INT013 = 13 , # [doc = "14 - INT014"]
INT014 = 14 , # [doc = "15 - INT015"]
INT015 = 15 , # [doc = "16 - INT016"]
INT016 = 16 , # [doc = "17 - INT017"]
INT017 = 17 , # [doc = "18 - INT018"]
INT018 = 18 , # [doc = "19 - INT019"]
INT019 = 19 , # [doc = "20 - INT020"]
INT020 = 20 , # [doc = "21 - INT021"]
INT021 = 21 , # [doc = "22 - INT022"]
INT022 = 22 , # [doc = "23 - INT023"]
INT023 = 23 , # [doc = "24 - INT024"]
INT024 = 24 , # [doc = "25 - INT025"]
INT025 = 25 , # [doc = "26 - INT026"]
INT026 = 26 , # [doc = "27 - INT027"]
INT027 = 27 , # [doc = "28 - INT028"]
INT028 = 28 , # [doc = "29 - INT029"]
INT029 = 29 , # [doc = "30 - INT030"]
INT030 = 30 , # [doc = "31 - INT031"]
INT031 = 31 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn EXTINT00 () ; fn EXTINT01 () ; fn EXTINT02 () ; fn EXTINT03 () ; fn EXTINT04 () ; fn EXTINT05 () ; fn EXTINT06 () ; fn EXTINT07 () ; fn INT008 () ; fn INT009 () ; fn INT010 () ; fn INT011 () ; fn INT012 () ; fn INT013 () ; fn INT014 () ; fn INT015 () ; fn INT016 () ; fn INT017 () ; fn INT018 () ; fn INT019 () ; fn INT020 () ; fn INT021 () ; fn INT022 () ; fn INT023 () ; fn INT024 () ; fn INT025 () ; fn INT026 () ; fn INT027 () ; fn INT028 () ; fn INT029 () ; fn INT030 () ; fn INT031 () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 32]
= [Vector { _handler : EXTINT00 } , Vector { _handler : EXTINT01 } , Vector { _handler : EXTINT02 } , Vector { _handler : EXTINT03 } , Vector { _handler : EXTINT04 } , Vector { _handler : EXTINT05 } , Vector { _handler : EXTINT06 } , Vector { _handler : EXTINT07 } , Vector { _handler : INT008 } , Vector { _handler : INT009 } , Vector { _handler : INT010 } , Vector { _handler : INT011 } , Vector { _handler : INT012 } , Vector { _handler : INT013 } , Vector { _handler : INT014 } , Vector { _handler : INT015 } , Vector { _handler : INT016 } , Vector { _handler : INT017 } , Vector { _handler : INT018 } , Vector { _handler : INT019 } , Vector { _handler : INT020 } , Vector { _handler : INT021 } , Vector { _handler : INT022 } , Vector { _handler : INT023 } , Vector { _handler : INT024 } , Vector { _handler : INT025 } , Vector { _handler : INT026 } , Vector { _handler : INT027 } , Vector { _handler : INT028 } , Vector { _handler : INT029 } , Vector { _handler : INT030 } , Vector { _handler : INT031 } ,]
; } pub const ICG : icg_v5 :: Icg = unsafe { icg_v5 :: Icg :: from_ptr (0xc0usize as _) } ; pub const CTC : ctc_v3 :: Ctc = unsafe { ctc_v3 :: Ctc :: from_ptr (0x4000_0000usize as _) } ; pub const EFM : efm_v6 :: Efm = unsafe { efm_v6 :: Efm :: from_ptr (0x4000_0800usize as _) } ; pub const AOS : aos_v6 :: Aos = unsafe { aos_v6 :: Aos :: from_ptr (0x4000_0c00usize as _) } ; pub const USART1 : usart_v25 :: Usart = unsafe { usart_v25 :: Usart :: from_ptr (0x4000_1800usize as _) } ; pub const USART2 : usart_v25 :: Usart = unsafe { usart_v25 :: Usart :: from_ptr (0x4000_1800usize as _) } ; pub const USART3 : usart_v25 :: Usart = unsafe { usart_v25 :: Usart :: from_ptr (0x4000_1800usize as _) } ; pub const USART4 : usart_v25 :: Usart = unsafe { usart_v25 :: Usart :: from_ptr (0x4000_1800usize as _) } ; pub const SPI : spi_v10 :: Spi = unsafe { spi_v10 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const I2C : i_v25 :: I = unsafe { i_v25 :: I :: from_ptr (0x4000_4800usize as _) } ; pub const TMR0 : tmr_v27 :: Tmr = unsafe { tmr_v27 :: Tmr :: from_ptr (0x4000_5800usize as _) } ; pub const TMRB1 : tmrb_v1 :: Tmrb = unsafe { tmrb_v1 :: Tmrb :: from_ptr (0x4000_7800usize as _) } ; pub const TMRB2 : tmrb_v1 :: Tmrb = unsafe { tmrb_v1 :: Tmrb :: from_ptr (0x4000_7800usize as _) } ; pub const TMRB3 : tmrb_v1 :: Tmrb = unsafe { tmrb_v1 :: Tmrb :: from_ptr (0x4000_7800usize as _) } ; pub const TMRB4 : tmrb_v1 :: Tmrb = unsafe { tmrb_v1 :: Tmrb :: from_ptr (0x4000_7800usize as _) } ; pub const TMRB5 : tmrb_v1 :: Tmrb = unsafe { tmrb_v1 :: Tmrb :: from_ptr (0x4000_7800usize as _) } ; pub const TMRB6 : tmrb_v1 :: Tmrb = unsafe { tmrb_v1 :: Tmrb :: from_ptr (0x4000_7800usize as _) } ; pub const TMRB7 : tmrb_v1 :: Tmrb = unsafe { tmrb_v1 :: Tmrb :: from_ptr (0x4000_7800usize as _) } ; pub const TMRB8 : tmrb_v1 :: Tmrb = unsafe { tmrb_v1 :: Tmrb :: from_ptr (0x4000_7800usize as _) } ; pub const ADC : adc_v19 :: Adc = unsafe { adc_v19 :: Adc :: from_ptr (0x4000_b800usize as _) } ; pub const SWDT : swdt_v1 :: Swdt = unsafe { swdt_v1 :: Swdt :: from_ptr (0x4000_cc00usize as _) } ; pub const RTC : rtc_v10 :: Rtc = unsafe { rtc_v10 :: Rtc :: from_ptr (0x4000_d400usize as _) } ; pub const INTC : intc_v6 :: Intc = unsafe { intc_v6 :: Intc :: from_ptr (0x4001_1000usize as _) } ; pub const GPIO : gpio_v18 :: Gpio = unsafe { gpio_v18 :: Gpio :: from_ptr (0x4001_3800usize as _) } ; pub const PWC : pwc_v6 :: Pwc = unsafe { pwc_v6 :: Pwc :: from_ptr (0x4001_4000usize as _) } ; pub const RMU : rmu_v6 :: Rmu = unsafe { rmu_v6 :: Rmu :: from_ptr (0x4001_4100usize as _) } ; pub const CMU : cmu_v6 :: Cmu = unsafe { cmu_v6 :: Cmu :: from_ptr (0x4001_4400usize as _) } ; pub const DBGC : dbgc_v6 :: Dbgc = unsafe { dbgc_v6 :: Dbgc :: from_ptr (0x4001_5000usize as _) } ; pub const CRC : crc_v8 :: Crc = unsafe { crc_v8 :: Crc :: from_ptr (0x4001_5400usize as _) } ; # [cfg (feature = "rt")]
 # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/adc_v19.rs"] pub mod adc_v19;
#[path="../../peripherals/aos_v6.rs"] pub mod aos_v6;
#[path="../../peripherals/cmu_v6.rs"] pub mod cmu_v6;
#[path="../../peripherals/crc_v8.rs"] pub mod crc_v8;
#[path="../../peripherals/ctc_v3.rs"] pub mod ctc_v3;
#[path="../../peripherals/dbgc_v6.rs"] pub mod dbgc_v6;
#[path="../../peripherals/efm_v6.rs"] pub mod efm_v6;
#[path="../../peripherals/gpio_v18.rs"] pub mod gpio_v18;
#[path="../../peripherals/i_v25.rs"] pub mod i_v25;
#[path="../../peripherals/icg_v5.rs"] pub mod icg_v5;
#[path="../../peripherals/intc_v6.rs"] pub mod intc_v6;
#[path="../../peripherals/pwc_v6.rs"] pub mod pwc_v6;
#[path="../../peripherals/rmu_v6.rs"] pub mod rmu_v6;
#[path="../../peripherals/rtc_v10.rs"] pub mod rtc_v10;
#[path="../../peripherals/spi_v10.rs"] pub mod spi_v10;
#[path="../../peripherals/swdt_v1.rs"] pub mod swdt_v1;
#[path="../../peripherals/tmr_v27.rs"] pub mod tmr_v27;
#[path="../../peripherals/tmrb_v1.rs"] pub mod tmrb_v1;
#[path="../../peripherals/usart_v25.rs"] pub mod usart_v25;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout { regions: crate::memory_select::MEMORY };
