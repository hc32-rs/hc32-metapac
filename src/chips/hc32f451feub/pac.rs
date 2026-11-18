#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - INT000"]
    INT000 = 0,
    #[doc = "1 - INT001"]
    INT001 = 1,
    #[doc = "2 - INT002"]
    INT002 = 2,
    #[doc = "3 - INT003"]
    INT003 = 3,
    #[doc = "4 - INT004"]
    INT004 = 4,
    #[doc = "5 - INT005"]
    INT005 = 5,
    #[doc = "6 - INT006"]
    INT006 = 6,
    #[doc = "7 - INT007"]
    INT007 = 7,
    #[doc = "8 - INT008"]
    INT008 = 8,
    #[doc = "9 - INT009"]
    INT009 = 9,
    #[doc = "10 - INT010"]
    INT010 = 10,
    #[doc = "11 - INT011"]
    INT011 = 11,
    #[doc = "12 - INT012"]
    INT012 = 12,
    #[doc = "13 - INT013"]
    INT013 = 13,
    #[doc = "14 - INT014"]
    INT014 = 14,
    #[doc = "15 - INT015"]
    INT015 = 15,
    #[doc = "16 - INT016"]
    INT016 = 16,
    #[doc = "17 - INT017"]
    INT017 = 17,
    #[doc = "18 - INT018"]
    INT018 = 18,
    #[doc = "19 - INT019"]
    INT019 = 19,
    #[doc = "20 - INT020"]
    INT020 = 20,
    #[doc = "21 - INT021"]
    INT021 = 21,
    #[doc = "22 - INT022"]
    INT022 = 22,
    #[doc = "23 - INT023"]
    INT023 = 23,
    #[doc = "24 - INT024"]
    INT024 = 24,
    #[doc = "25 - INT025"]
    INT025 = 25,
    #[doc = "26 - INT026"]
    INT026 = 26,
    #[doc = "27 - INT027"]
    INT027 = 27,
    #[doc = "28 - INT028"]
    INT028 = 28,
    #[doc = "29 - INT029"]
    INT029 = 29,
    #[doc = "30 - INT030"]
    INT030 = 30,
    #[doc = "31 - INT031"]
    INT031 = 31,
    #[doc = "32 - INT032"]
    INT032 = 32,
    #[doc = "33 - INT033"]
    INT033 = 33,
    #[doc = "34 - INT034"]
    INT034 = 34,
    #[doc = "35 - INT035"]
    INT035 = 35,
    #[doc = "36 - INT036"]
    INT036 = 36,
    #[doc = "37 - INT037"]
    INT037 = 37,
    #[doc = "38 - INT038"]
    INT038 = 38,
    #[doc = "39 - INT039"]
    INT039 = 39,
    #[doc = "40 - INT040"]
    INT040 = 40,
    #[doc = "41 - INT041"]
    INT041 = 41,
    #[doc = "42 - INT042"]
    INT042 = 42,
    #[doc = "43 - INT043"]
    INT043 = 43,
    #[doc = "44 - INT044"]
    INT044 = 44,
    #[doc = "45 - INT045"]
    INT045 = 45,
    #[doc = "46 - INT046"]
    INT046 = 46,
    #[doc = "47 - INT047"]
    INT047 = 47,
    #[doc = "48 - INT048"]
    INT048 = 48,
    #[doc = "49 - INT049"]
    INT049 = 49,
    #[doc = "50 - INT050"]
    INT050 = 50,
    #[doc = "51 - INT051"]
    INT051 = 51,
    #[doc = "52 - INT052"]
    INT052 = 52,
    #[doc = "53 - INT053"]
    INT053 = 53,
    #[doc = "54 - INT054"]
    INT054 = 54,
    #[doc = "55 - INT055"]
    INT055 = 55,
    #[doc = "56 - INT056"]
    INT056 = 56,
    #[doc = "57 - INT057"]
    INT057 = 57,
    #[doc = "58 - INT058"]
    INT058 = 58,
    #[doc = "59 - INT059"]
    INT059 = 59,
    #[doc = "60 - INT060"]
    INT060 = 60,
    #[doc = "61 - INT061"]
    INT061 = 61,
    #[doc = "62 - INT062"]
    INT062 = 62,
    #[doc = "63 - INT063"]
    INT063 = 63,
    #[doc = "64 - INT064"]
    INT064 = 64,
    #[doc = "65 - INT065"]
    INT065 = 65,
    #[doc = "66 - INT066"]
    INT066 = 66,
    #[doc = "67 - INT067"]
    INT067 = 67,
    #[doc = "68 - INT068"]
    INT068 = 68,
    #[doc = "69 - INT069"]
    INT069 = 69,
    #[doc = "70 - INT070"]
    INT070 = 70,
    #[doc = "71 - INT071"]
    INT071 = 71,
    #[doc = "72 - INT072"]
    INT072 = 72,
    #[doc = "73 - INT073"]
    INT073 = 73,
    #[doc = "74 - INT074"]
    INT074 = 74,
    #[doc = "75 - INT075"]
    INT075 = 75,
    #[doc = "76 - INT076"]
    INT076 = 76,
    #[doc = "77 - INT077"]
    INT077 = 77,
    #[doc = "78 - INT078"]
    INT078 = 78,
    #[doc = "79 - INT079"]
    INT079 = 79,
    #[doc = "80 - INT080"]
    INT080 = 80,
    #[doc = "81 - INT081"]
    INT081 = 81,
    #[doc = "82 - INT082"]
    INT082 = 82,
    #[doc = "83 - INT083"]
    INT083 = 83,
    #[doc = "84 - INT084"]
    INT084 = 84,
    #[doc = "85 - INT085"]
    INT085 = 85,
    #[doc = "86 - INT086"]
    INT086 = 86,
    #[doc = "87 - INT087"]
    INT087 = 87,
    #[doc = "88 - INT088"]
    INT088 = 88,
    #[doc = "89 - INT089"]
    INT089 = 89,
    #[doc = "90 - INT090"]
    INT090 = 90,
    #[doc = "91 - INT091"]
    INT091 = 91,
    #[doc = "92 - INT092"]
    INT092 = 92,
    #[doc = "93 - INT093"]
    INT093 = 93,
    #[doc = "94 - INT094"]
    INT094 = 94,
    #[doc = "95 - INT095"]
    INT095 = 95,
    #[doc = "96 - INT096"]
    INT096 = 96,
    #[doc = "97 - INT097"]
    INT097 = 97,
    #[doc = "98 - INT098"]
    INT098 = 98,
    #[doc = "99 - INT099"]
    INT099 = 99,
    #[doc = "100 - INT100"]
    INT100 = 100,
    #[doc = "101 - INT101"]
    INT101 = 101,
    #[doc = "102 - INT102"]
    INT102 = 102,
    #[doc = "103 - INT103"]
    INT103 = 103,
    #[doc = "104 - INT104"]
    INT104 = 104,
    #[doc = "105 - INT105"]
    INT105 = 105,
    #[doc = "106 - INT106"]
    INT106 = 106,
    #[doc = "107 - INT107"]
    INT107 = 107,
    #[doc = "108 - INT108"]
    INT108 = 108,
    #[doc = "109 - INT109"]
    INT109 = 109,
    #[doc = "110 - INT110"]
    INT110 = 110,
    #[doc = "111 - INT111"]
    INT111 = 111,
    #[doc = "112 - INT112"]
    INT112 = 112,
    #[doc = "113 - INT113"]
    INT113 = 113,
    #[doc = "114 - INT114"]
    INT114 = 114,
    #[doc = "115 - INT115"]
    INT115 = 115,
    #[doc = "116 - INT116"]
    INT116 = 116,
    #[doc = "117 - INT117"]
    INT117 = 117,
    #[doc = "118 - INT118"]
    INT118 = 118,
    #[doc = "119 - INT119"]
    INT119 = 119,
    #[doc = "120 - INT120"]
    INT120 = 120,
    #[doc = "121 - INT121"]
    INT121 = 121,
    #[doc = "122 - INT122"]
    INT122 = 122,
    #[doc = "123 - INT123"]
    INT123 = 123,
    #[doc = "124 - INT124"]
    INT124 = 124,
    #[doc = "125 - INT125"]
    INT125 = 125,
    #[doc = "126 - INT126"]
    INT126 = 126,
    #[doc = "127 - INT127"]
    INT127 = 127,
    #[doc = "128 - INT128"]
    INT128 = 128,
    #[doc = "129 - INT129"]
    INT129 = 129,
    #[doc = "130 - INT130"]
    INT130 = 130,
    #[doc = "131 - INT131"]
    INT131 = 131,
    #[doc = "132 - INT132"]
    INT132 = 132,
    #[doc = "133 - INT133"]
    INT133 = 133,
    #[doc = "134 - INT134"]
    INT134 = 134,
    #[doc = "135 - INT135"]
    INT135 = 135,
    #[doc = "136 - INT136"]
    INT136 = 136,
    #[doc = "137 - INT137"]
    INT137 = 137,
    #[doc = "138 - INT138"]
    INT138 = 138,
    #[doc = "139 - INT139"]
    INT139 = 139,
    #[doc = "140 - INT140"]
    INT140 = 140,
    #[doc = "141 - INT141"]
    INT141 = 141,
    #[doc = "142 - INT142"]
    INT142 = 142,
    #[doc = "143 - INT143"]
    INT143 = 143,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn INT000();
        fn INT001();
        fn INT002();
        fn INT003();
        fn INT004();
        fn INT005();
        fn INT006();
        fn INT007();
        fn INT008();
        fn INT009();
        fn INT010();
        fn INT011();
        fn INT012();
        fn INT013();
        fn INT014();
        fn INT015();
        fn INT016();
        fn INT017();
        fn INT018();
        fn INT019();
        fn INT020();
        fn INT021();
        fn INT022();
        fn INT023();
        fn INT024();
        fn INT025();
        fn INT026();
        fn INT027();
        fn INT028();
        fn INT029();
        fn INT030();
        fn INT031();
        fn INT032();
        fn INT033();
        fn INT034();
        fn INT035();
        fn INT036();
        fn INT037();
        fn INT038();
        fn INT039();
        fn INT040();
        fn INT041();
        fn INT042();
        fn INT043();
        fn INT044();
        fn INT045();
        fn INT046();
        fn INT047();
        fn INT048();
        fn INT049();
        fn INT050();
        fn INT051();
        fn INT052();
        fn INT053();
        fn INT054();
        fn INT055();
        fn INT056();
        fn INT057();
        fn INT058();
        fn INT059();
        fn INT060();
        fn INT061();
        fn INT062();
        fn INT063();
        fn INT064();
        fn INT065();
        fn INT066();
        fn INT067();
        fn INT068();
        fn INT069();
        fn INT070();
        fn INT071();
        fn INT072();
        fn INT073();
        fn INT074();
        fn INT075();
        fn INT076();
        fn INT077();
        fn INT078();
        fn INT079();
        fn INT080();
        fn INT081();
        fn INT082();
        fn INT083();
        fn INT084();
        fn INT085();
        fn INT086();
        fn INT087();
        fn INT088();
        fn INT089();
        fn INT090();
        fn INT091();
        fn INT092();
        fn INT093();
        fn INT094();
        fn INT095();
        fn INT096();
        fn INT097();
        fn INT098();
        fn INT099();
        fn INT100();
        fn INT101();
        fn INT102();
        fn INT103();
        fn INT104();
        fn INT105();
        fn INT106();
        fn INT107();
        fn INT108();
        fn INT109();
        fn INT110();
        fn INT111();
        fn INT112();
        fn INT113();
        fn INT114();
        fn INT115();
        fn INT116();
        fn INT117();
        fn INT118();
        fn INT119();
        fn INT120();
        fn INT121();
        fn INT122();
        fn INT123();
        fn INT124();
        fn INT125();
        fn INT126();
        fn INT127();
        fn INT128();
        fn INT129();
        fn INT130();
        fn INT131();
        fn INT132();
        fn INT133();
        fn INT134();
        fn INT135();
        fn INT136();
        fn INT137();
        fn INT138();
        fn INT139();
        fn INT140();
        fn INT141();
        fn INT142();
        fn INT143();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 144] = [
        Vector { _handler: INT000 },
        Vector { _handler: INT001 },
        Vector { _handler: INT002 },
        Vector { _handler: INT003 },
        Vector { _handler: INT004 },
        Vector { _handler: INT005 },
        Vector { _handler: INT006 },
        Vector { _handler: INT007 },
        Vector { _handler: INT008 },
        Vector { _handler: INT009 },
        Vector { _handler: INT010 },
        Vector { _handler: INT011 },
        Vector { _handler: INT012 },
        Vector { _handler: INT013 },
        Vector { _handler: INT014 },
        Vector { _handler: INT015 },
        Vector { _handler: INT016 },
        Vector { _handler: INT017 },
        Vector { _handler: INT018 },
        Vector { _handler: INT019 },
        Vector { _handler: INT020 },
        Vector { _handler: INT021 },
        Vector { _handler: INT022 },
        Vector { _handler: INT023 },
        Vector { _handler: INT024 },
        Vector { _handler: INT025 },
        Vector { _handler: INT026 },
        Vector { _handler: INT027 },
        Vector { _handler: INT028 },
        Vector { _handler: INT029 },
        Vector { _handler: INT030 },
        Vector { _handler: INT031 },
        Vector { _handler: INT032 },
        Vector { _handler: INT033 },
        Vector { _handler: INT034 },
        Vector { _handler: INT035 },
        Vector { _handler: INT036 },
        Vector { _handler: INT037 },
        Vector { _handler: INT038 },
        Vector { _handler: INT039 },
        Vector { _handler: INT040 },
        Vector { _handler: INT041 },
        Vector { _handler: INT042 },
        Vector { _handler: INT043 },
        Vector { _handler: INT044 },
        Vector { _handler: INT045 },
        Vector { _handler: INT046 },
        Vector { _handler: INT047 },
        Vector { _handler: INT048 },
        Vector { _handler: INT049 },
        Vector { _handler: INT050 },
        Vector { _handler: INT051 },
        Vector { _handler: INT052 },
        Vector { _handler: INT053 },
        Vector { _handler: INT054 },
        Vector { _handler: INT055 },
        Vector { _handler: INT056 },
        Vector { _handler: INT057 },
        Vector { _handler: INT058 },
        Vector { _handler: INT059 },
        Vector { _handler: INT060 },
        Vector { _handler: INT061 },
        Vector { _handler: INT062 },
        Vector { _handler: INT063 },
        Vector { _handler: INT064 },
        Vector { _handler: INT065 },
        Vector { _handler: INT066 },
        Vector { _handler: INT067 },
        Vector { _handler: INT068 },
        Vector { _handler: INT069 },
        Vector { _handler: INT070 },
        Vector { _handler: INT071 },
        Vector { _handler: INT072 },
        Vector { _handler: INT073 },
        Vector { _handler: INT074 },
        Vector { _handler: INT075 },
        Vector { _handler: INT076 },
        Vector { _handler: INT077 },
        Vector { _handler: INT078 },
        Vector { _handler: INT079 },
        Vector { _handler: INT080 },
        Vector { _handler: INT081 },
        Vector { _handler: INT082 },
        Vector { _handler: INT083 },
        Vector { _handler: INT084 },
        Vector { _handler: INT085 },
        Vector { _handler: INT086 },
        Vector { _handler: INT087 },
        Vector { _handler: INT088 },
        Vector { _handler: INT089 },
        Vector { _handler: INT090 },
        Vector { _handler: INT091 },
        Vector { _handler: INT092 },
        Vector { _handler: INT093 },
        Vector { _handler: INT094 },
        Vector { _handler: INT095 },
        Vector { _handler: INT096 },
        Vector { _handler: INT097 },
        Vector { _handler: INT098 },
        Vector { _handler: INT099 },
        Vector { _handler: INT100 },
        Vector { _handler: INT101 },
        Vector { _handler: INT102 },
        Vector { _handler: INT103 },
        Vector { _handler: INT104 },
        Vector { _handler: INT105 },
        Vector { _handler: INT106 },
        Vector { _handler: INT107 },
        Vector { _handler: INT108 },
        Vector { _handler: INT109 },
        Vector { _handler: INT110 },
        Vector { _handler: INT111 },
        Vector { _handler: INT112 },
        Vector { _handler: INT113 },
        Vector { _handler: INT114 },
        Vector { _handler: INT115 },
        Vector { _handler: INT116 },
        Vector { _handler: INT117 },
        Vector { _handler: INT118 },
        Vector { _handler: INT119 },
        Vector { _handler: INT120 },
        Vector { _handler: INT121 },
        Vector { _handler: INT122 },
        Vector { _handler: INT123 },
        Vector { _handler: INT124 },
        Vector { _handler: INT125 },
        Vector { _handler: INT126 },
        Vector { _handler: INT127 },
        Vector { _handler: INT128 },
        Vector { _handler: INT129 },
        Vector { _handler: INT130 },
        Vector { _handler: INT131 },
        Vector { _handler: INT132 },
        Vector { _handler: INT133 },
        Vector { _handler: INT134 },
        Vector { _handler: INT135 },
        Vector { _handler: INT136 },
        Vector { _handler: INT137 },
        Vector { _handler: INT138 },
        Vector { _handler: INT139 },
        Vector { _handler: INT140 },
        Vector { _handler: INT141 },
        Vector { _handler: INT142 },
        Vector { _handler: INT143 },
    ];
}
pub const ICG: icg_ll_v1_10::Icg = unsafe { icg_ll_v1_10::Icg::from_ptr(0x0400usize as _) };
pub const AES: aes_ll_v1::Aes = unsafe { aes_ll_v1::Aes::from_ptr(0x4000_8000usize as _) };
pub const HASH: hash_v1::Hash = unsafe { hash_v1::Hash::from_ptr(0x4000_8400usize as _) };
pub const CRC: crc_ll_v1_2::Crc = unsafe { crc_ll_v1_2::Crc::from_ptr(0x4000_8c00usize as _) };
pub const EFM: efm_ll_v2::Efm = unsafe { efm_ll_v2::Efm::from_ptr(0x4001_0400usize as _) };
pub const AOS: aos_ll_v6::Aos = unsafe { aos_ll_v6::Aos::from_ptr(0x4001_0800usize as _) };
pub const TMRA1: tmra_ll_v4::Tmra = unsafe { tmra_ll_v4::Tmra::from_ptr(0x4001_5000usize as _) };
pub const TMRA2: tmra_ll_v4::Tmra = unsafe { tmra_ll_v4::Tmra::from_ptr(0x4001_5400usize as _) };
pub const TMRA3: tmra_ll_v4::Tmra = unsafe { tmra_ll_v4::Tmra::from_ptr(0x4001_5800usize as _) };
pub const TMRA4: tmra_ll_v4::Tmra = unsafe { tmra_ll_v4::Tmra::from_ptr(0x4001_5c00usize as _) };
pub const TMRA5: tmra_ll_v4::Tmra = unsafe { tmra_ll_v4::Tmra::from_ptr(0x4001_6000usize as _) };
pub const TMRA6: tmra_ll_v4::Tmra = unsafe { tmra_ll_v4::Tmra::from_ptr(0x4001_6400usize as _) };
pub const TMR41: tmr4_ll_v1_1::Tmr4 =
    unsafe { tmr4_ll_v1_1::Tmr4::from_ptr(0x4001_7000usize as _) };
pub const EMB0: emb_ll_v1_1::Emb = unsafe { emb_ll_v1_1::Emb::from_ptr(0x4001_7c00usize as _) };
pub const EMB1: emb_ll_v1_1::Emb = unsafe { emb_ll_v1_1::Emb::from_ptr(0x4001_7c20usize as _) };
pub const EMB2: emb_ll_v1_1::Emb = unsafe { emb_ll_v1_1::Emb::from_ptr(0x4001_7c40usize as _) };
pub const EMB3: emb_ll_v1_1::Emb = unsafe { emb_ll_v1_1::Emb::from_ptr(0x4001_7c60usize as _) };
pub const TMR61: tmr_v10::Tmr = unsafe { tmr_v10::Tmr::from_ptr(0x4001_8000usize as _) };
pub const TMR6_COMMON: tmr_v11::Tmr = unsafe { tmr_v11::Tmr::from_ptr(0x4001_8300usize as _) };
pub const TMR62: tmr_v10::Tmr = unsafe { tmr_v10::Tmr::from_ptr(0x4001_8400usize as _) };
pub const TMR63: tmr_v10::Tmr = unsafe { tmr_v10::Tmr::from_ptr(0x4001_8800usize as _) };
pub const SPI1: spi_ll_v1_2::Spi = unsafe { spi_ll_v1_2::Spi::from_ptr(0x4001_c000usize as _) };
pub const SPI2: spi_ll_v1_2::Spi = unsafe { spi_ll_v1_2::Spi::from_ptr(0x4001_c400usize as _) };
pub const USART1: usart_ll_v2_1::Usart =
    unsafe { usart_ll_v2_1::Usart::from_ptr(0x4001_d000usize as _) };
pub const USART2: usart_ll_v2_1::Usart =
    unsafe { usart_ll_v2_1::Usart::from_ptr(0x4001_d400usize as _) };
pub const I2S1: i2s_ll_v1::I2s = unsafe { i2s_ll_v1::I2s::from_ptr(0x4001_e000usize as _) };
pub const I2S2: i2s_ll_v1::I2s = unsafe { i2s_ll_v1::I2s::from_ptr(0x4001_e400usize as _) };
pub const SPI3: spi_ll_v1_2::Spi = unsafe { spi_ll_v1_2::Spi::from_ptr(0x4002_0000usize as _) };
pub const SPI4: spi_ll_v1_2::Spi = unsafe { spi_ll_v1_2::Spi::from_ptr(0x4002_0400usize as _) };
pub const USART3: usart_ll_v2_1::Usart =
    unsafe { usart_ll_v2_1::Usart::from_ptr(0x4002_1000usize as _) };
pub const USART4: usart_ll_v2_1::Usart =
    unsafe { usart_ll_v2_1::Usart::from_ptr(0x4002_1400usize as _) };
pub const I2S3: i2s_ll_v1::I2s = unsafe { i2s_ll_v1::I2s::from_ptr(0x4002_2000usize as _) };
pub const I2S4: i2s_ll_v1::I2s = unsafe { i2s_ll_v1::I2s::from_ptr(0x4002_2400usize as _) };
pub const TMR01: tmr0_ll_v2_1::Tmr0 =
    unsafe { tmr0_ll_v2_1::Tmr0::from_ptr(0x4002_4000usize as _) };
pub const TMR02: tmr0_ll_v2_1::Tmr0 =
    unsafe { tmr0_ll_v2_1::Tmr0::from_ptr(0x4002_4400usize as _) };
pub const TMR42: tmr4_ll_v1_1::Tmr4 =
    unsafe { tmr4_ll_v1_1::Tmr4::from_ptr(0x4002_4800usize as _) };
pub const TMR43: tmr4_ll_v1_1::Tmr4 =
    unsafe { tmr4_ll_v1_1::Tmr4::from_ptr(0x4002_4c00usize as _) };
pub const ADC1: adc_ll_v3_1::Adc = unsafe { adc_ll_v3_1::Adc::from_ptr(0x4004_0000usize as _) };
pub const ADC2: adc_ll_v2::Adc = unsafe { adc_ll_v2::Adc::from_ptr(0x4004_0400usize as _) };
pub const TRNG: trng_ll_v1::Trng = unsafe { trng_ll_v1::Trng::from_ptr(0x4004_1000usize as _) };
pub const PWC: pwc_ll_v3_2::Pwc = unsafe { pwc_ll_v3_2::Pwc::from_ptr(0x4004_8000usize as _) };
pub const FCM: fcm_ll_v1_1::Fcm = unsafe { fcm_ll_v1_1::Fcm::from_ptr(0x4004_8400usize as _) };
pub const WDT: wdt_ll_v1::Wdt = unsafe { wdt_ll_v1::Wdt::from_ptr(0x4004_9000usize as _) };
pub const SWDT: swdt_ll_v1::Swdt = unsafe { swdt_ll_v1::Swdt::from_ptr(0x4004_9400usize as _) };
pub const CMP1: cmp_ll_v1::Cmp = unsafe { cmp_ll_v1::Cmp::from_ptr(0x4004_a000usize as _) };
pub const CMP_COMMON: cmp_common_ll_v1_1::CmpCommon =
    unsafe { cmp_common_ll_v1_1::CmpCommon::from_ptr(0x4004_a000usize as _) };
pub const CMP2: cmp_ll_v1::Cmp = unsafe { cmp_ll_v1::Cmp::from_ptr(0x4004_a010usize as _) };
pub const CMP3: cmp_ll_v1::Cmp = unsafe { cmp_ll_v1::Cmp::from_ptr(0x4004_a020usize as _) };
pub const OTS: ots_v1::Ots = unsafe { ots_v1::Ots::from_ptr(0x4004_a400usize as _) };
pub const RTC: rtc_ll_v1_2::Rtc = unsafe { rtc_ll_v1_2::Rtc::from_ptr(0x4004_c000usize as _) };
pub const I2C1: i2c_ll_v2_1::I2c = unsafe { i2c_ll_v2_1::I2c::from_ptr(0x4004_e000usize as _) };
pub const I2C2: i2c_ll_v2_1::I2c = unsafe { i2c_ll_v2_1::I2c::from_ptr(0x4004_e400usize as _) };
pub const I2C3: i2c_ll_v2_1::I2c = unsafe { i2c_ll_v2_1::I2c::from_ptr(0x4004_e800usize as _) };
pub const MPU: mpu_v2::Mpu = unsafe { mpu_v2::Mpu::from_ptr(0x4005_0000usize as _) };
pub const SRAMC: sramc_ll_v1::Sramc =
    unsafe { sramc_ll_v1::Sramc::from_ptr(0x4005_0800usize as _) };
pub const KEYSCAN: keyscan_common::Keyscan =
    unsafe { keyscan_common::Keyscan::from_ptr(0x4005_0c00usize as _) };
pub const INTC: intc_ll_v3::Intc = unsafe { intc_ll_v3::Intc::from_ptr(0x4005_1000usize as _) };
pub const DCU1: dcu_v2::Dcu = unsafe { dcu_v2::Dcu::from_ptr(0x4005_2000usize as _) };
pub const DCU2: dcu_v2::Dcu = unsafe { dcu_v2::Dcu::from_ptr(0x4005_2400usize as _) };
pub const DCU3: dcu_v2::Dcu = unsafe { dcu_v2::Dcu::from_ptr(0x4005_2800usize as _) };
pub const DCU4: dcu_v2::Dcu = unsafe { dcu_v2::Dcu::from_ptr(0x4005_2c00usize as _) };
pub const DMA1: dma_ll_v2::Dma = unsafe { dma_ll_v2::Dma::from_ptr(0x4005_3000usize as _) };
pub const DMA2: dma_ll_v2::Dma = unsafe { dma_ll_v2::Dma::from_ptr(0x4005_3400usize as _) };
pub const GPIO: gpio_ll_v1_9::Gpio = unsafe { gpio_ll_v1_9::Gpio::from_ptr(0x4005_3800usize as _) };
pub const CMU: cmu_ll_v4::Cmu = unsafe { cmu_ll_v4::Cmu::from_ptr(0x4005_4000usize as _) };
pub const RMU: rmu_ll_v1_2::Rmu = unsafe { rmu_ll_v1_2::Rmu::from_ptr(0x4005_40c0usize as _) };
pub const PERIC: peric_ll_v2::Peric =
    unsafe { peric_ll_v2::Peric::from_ptr(0x4005_5400usize as _) };
pub const TMR4_ECER: tmr4_ecer_ll_v1::Tmr4Ecer =
    unsafe { tmr4_ecer_ll_v1::Tmr4Ecer::from_ptr(0x4005_5408usize as _) };
pub const SDIOC1: sdioc_common::Sdioc =
    unsafe { sdioc_common::Sdioc::from_ptr(0x4006_fc00usize as _) };
pub const SDIOC2: sdioc_common::Sdioc =
    unsafe { sdioc_common::Sdioc::from_ptr(0x4007_0000usize as _) };
pub const USBFS: usbfs_v1::Usbfs = unsafe { usbfs_v1::Usbfs::from_ptr(0x400c_0000usize as _) };
pub const QSPI: qspi_v2::Qspi = unsafe { qspi_v2::Qspi::from_ptr(0x9c00_0000usize as _) };
pub const DBGC: dbgc_ll_v2::Dbgc = unsafe { dbgc_ll_v2::Dbgc::from_ptr(0xe004_2000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ll_v2.rs"]
pub mod adc_ll_v2;
#[path = "../../peripherals/adc_ll_v3_1.rs"]
pub mod adc_ll_v3_1;
#[path = "../../peripherals/aes_ll_v1.rs"]
pub mod aes_ll_v1;
#[path = "../../peripherals/aos_ll_v6.rs"]
pub mod aos_ll_v6;
#[path = "../../peripherals/cmp_common_ll_v1_1.rs"]
pub mod cmp_common_ll_v1_1;
#[path = "../../peripherals/cmp_ll_v1.rs"]
pub mod cmp_ll_v1;
#[path = "../../peripherals/cmu_ll_v4.rs"]
pub mod cmu_ll_v4;
#[path = "../../peripherals/crc_ll_v1_2.rs"]
pub mod crc_ll_v1_2;
#[path = "../../peripherals/dbgc_ll_v2.rs"]
pub mod dbgc_ll_v2;
#[path = "../../peripherals/dcu_v2.rs"]
pub mod dcu_v2;
#[path = "../../peripherals/dma_ll_v2.rs"]
pub mod dma_ll_v2;
#[path = "../../peripherals/efm_ll_v2.rs"]
pub mod efm_ll_v2;
#[path = "../../peripherals/emb_ll_v1_1.rs"]
pub mod emb_ll_v1_1;
#[path = "../../peripherals/fcm_ll_v1_1.rs"]
pub mod fcm_ll_v1_1;
#[path = "../../peripherals/gpio_ll_v1_9.rs"]
pub mod gpio_ll_v1_9;
#[path = "../../peripherals/hash_v1.rs"]
pub mod hash_v1;
#[path = "../../peripherals/i2c_ll_v2_1.rs"]
pub mod i2c_ll_v2_1;
#[path = "../../peripherals/i2s_ll_v1.rs"]
pub mod i2s_ll_v1;
#[path = "../../peripherals/icg_ll_v1_10.rs"]
pub mod icg_ll_v1_10;
#[path = "../../peripherals/intc_ll_v3.rs"]
pub mod intc_ll_v3;
#[path = "../../peripherals/keyscan_common.rs"]
pub mod keyscan_common;
#[path = "../../peripherals/mpu_v2.rs"]
pub mod mpu_v2;
#[path = "../../peripherals/ots_v1.rs"]
pub mod ots_v1;
#[path = "../../peripherals/peric_ll_v2.rs"]
pub mod peric_ll_v2;
#[path = "../../peripherals/pwc_ll_v3_2.rs"]
pub mod pwc_ll_v3_2;
#[path = "../../peripherals/qspi_v2.rs"]
pub mod qspi_v2;
#[path = "../../peripherals/rmu_ll_v1_2.rs"]
pub mod rmu_ll_v1_2;
#[path = "../../peripherals/rtc_ll_v1_2.rs"]
pub mod rtc_ll_v1_2;
#[path = "../../peripherals/sdioc_common.rs"]
pub mod sdioc_common;
#[path = "../../peripherals/spi_ll_v1_2.rs"]
pub mod spi_ll_v1_2;
#[path = "../../peripherals/sramc_ll_v1.rs"]
pub mod sramc_ll_v1;
#[path = "../../peripherals/swdt_ll_v1.rs"]
pub mod swdt_ll_v1;
#[path = "../../peripherals/tmr0_ll_v2_1.rs"]
pub mod tmr0_ll_v2_1;
#[path = "../../peripherals/tmr4_ecer_ll_v1.rs"]
pub mod tmr4_ecer_ll_v1;
#[path = "../../peripherals/tmr4_ll_v1_1.rs"]
pub mod tmr4_ll_v1_1;
#[path = "../../peripherals/tmr_v10.rs"]
pub mod tmr_v10;
#[path = "../../peripherals/tmr_v11.rs"]
pub mod tmr_v11;
#[path = "../../peripherals/tmra_ll_v4.rs"]
pub mod tmra_ll_v4;
#[path = "../../peripherals/trng_ll_v1.rs"]
pub mod trng_ll_v1;
#[path = "../../peripherals/usart_ll_v2_1.rs"]
pub mod usart_ll_v2_1;
#[path = "../../peripherals/usbfs_v1.rs"]
pub mod usbfs_v1;
#[path = "../../peripherals/wdt_ll_v1.rs"]
pub mod wdt_ll_v1;
pub const CORE_INDEX: usize = 0;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
