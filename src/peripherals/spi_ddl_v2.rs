#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc SPI."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi {
    ptr: *mut u8,
}
unsafe impl Send for Spi {}
unsafe impl Sync for Spi {}
impl Spi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CR0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc HDOE."]
    #[inline(always)]
    pub const fn hdoe(self) -> crate::common::Reg<regs::Hdoe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc SSI."]
    #[inline(always)]
    pub const fn ssi(self) -> crate::common::Reg<regs::Ssi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc SR."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc DR."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc BRR."]
        #[must_use]
        #[inline(always)]
        pub const fn brr(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "desc BRR."]
        #[inline(always)]
        pub const fn set_brr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "desc CPHA."]
        #[must_use]
        #[inline(always)]
        pub const fn cpha(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CPHA."]
        #[inline(always)]
        pub const fn set_cpha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CPOL."]
        #[must_use]
        #[inline(always)]
        pub const fn cpol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CPOL."]
        #[inline(always)]
        pub const fn set_cpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc MSTR."]
        #[must_use]
        #[inline(always)]
        pub const fn mstr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSTR."]
        #[inline(always)]
        pub const fn set_mstr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc LSBF."]
        #[must_use]
        #[inline(always)]
        pub const fn lsbf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSBF."]
        #[inline(always)]
        pub const fn set_lsbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc WIDTH."]
        #[must_use]
        #[inline(always)]
        pub const fn width(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc WIDTH."]
        #[inline(always)]
        pub const fn set_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc SSM."]
        #[must_use]
        #[inline(always)]
        pub const fn ssm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSM."]
        #[inline(always)]
        pub const fn set_ssm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc CM."]
        #[must_use]
        #[inline(always)]
        pub const fn cm(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "desc CM."]
        #[inline(always)]
        pub const fn set_cm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr0 {
        #[inline(always)]
        fn default() -> Cr0 {
            Cr0(0)
        }
    }
    impl core::fmt::Debug for Cr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr0")
                .field("en", &self.en())
                .field("brr", &self.brr())
                .field("cpha", &self.cpha())
                .field("cpol", &self.cpol())
                .field("mstr", &self.mstr())
                .field("lsbf", &self.lsbf())
                .field("width", &self.width())
                .field("ssm", &self.ssm())
                .field("cm", &self.cm())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc TXEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn txeie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXEIE."]
        #[inline(always)]
        pub const fn set_txeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RXNEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxneie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXNEIE."]
        #[inline(always)]
        pub const fn set_rxneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SSFIE."]
        #[must_use]
        #[inline(always)]
        pub const fn ssfie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSFIE."]
        #[inline(always)]
        pub const fn set_ssfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SSRIE."]
        #[must_use]
        #[inline(always)]
        pub const fn ssrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSRIE."]
        #[inline(always)]
        pub const fn set_ssrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc UDFIE."]
        #[must_use]
        #[inline(always)]
        pub const fn udfie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDFIE."]
        #[inline(always)]
        pub const fn set_udfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc OVFIE."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVFIE."]
        #[inline(always)]
        pub const fn set_ovfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SSERRIE."]
        #[must_use]
        #[inline(always)]
        pub const fn sserrie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSERRIE."]
        #[inline(always)]
        pub const fn set_sserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc MODFIE."]
        #[must_use]
        #[inline(always)]
        pub const fn modfie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MODFIE."]
        #[inline(always)]
        pub const fn set_modfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("txeie", &self.txeie())
                .field("rxneie", &self.rxneie())
                .field("ssfie", &self.ssfie())
                .field("ssrie", &self.ssrie())
                .field("udfie", &self.udfie())
                .field("ovfie", &self.ovfie())
                .field("sserrie", &self.sserrie())
                .field("modfie", &self.modfie())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc DR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "desc DR."]
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc DR."]
        #[inline(always)]
        pub const fn set_dr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr")
                .field("dr", &self.dr())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc HDOE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdoe(pub u32);
    impl Hdoe {
        #[doc = "desc HDOE."]
        #[must_use]
        #[inline(always)]
        pub const fn hdoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HDOE."]
        #[inline(always)]
        pub const fn set_hdoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hdoe {
        #[inline(always)]
        fn default() -> Hdoe {
            Hdoe(0)
        }
    }
    impl core::fmt::Debug for Hdoe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdoe")
                .field("hdoe", &self.hdoe())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc TXE."]
        #[must_use]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXE."]
        #[inline(always)]
        pub const fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RXNE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXNE."]
        #[inline(always)]
        pub const fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SSF."]
        #[must_use]
        #[inline(always)]
        pub const fn ssf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSF."]
        #[inline(always)]
        pub const fn set_ssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SSR."]
        #[must_use]
        #[inline(always)]
        pub const fn ssr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSR."]
        #[inline(always)]
        pub const fn set_ssr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc UDF."]
        #[must_use]
        #[inline(always)]
        pub const fn udf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDF."]
        #[inline(always)]
        pub const fn set_udf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc OVF."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF."]
        #[inline(always)]
        pub const fn set_ovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SSERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sserr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSERR."]
        #[inline(always)]
        pub const fn set_sserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc MODF."]
        #[must_use]
        #[inline(always)]
        pub const fn modf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MODF."]
        #[inline(always)]
        pub const fn set_modf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("txe", &self.txe())
                .field("rxne", &self.rxne())
                .field("ssf", &self.ssf())
                .field("ssr", &self.ssr())
                .field("udf", &self.udf())
                .field("ovf", &self.ovf())
                .field("sserr", &self.sserr())
                .field("modf", &self.modf())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "desc TXE."]
        #[must_use]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXE."]
        #[inline(always)]
        pub const fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RXNE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXNE."]
        #[inline(always)]
        pub const fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SSF."]
        #[must_use]
        #[inline(always)]
        pub const fn ssf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSF."]
        #[inline(always)]
        pub const fn set_ssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SSR."]
        #[must_use]
        #[inline(always)]
        pub const fn ssr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSR."]
        #[inline(always)]
        pub const fn set_ssr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc UDF."]
        #[must_use]
        #[inline(always)]
        pub const fn udf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDF."]
        #[inline(always)]
        pub const fn set_udf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc OVF."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF."]
        #[inline(always)]
        pub const fn set_ovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SSERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sserr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSERR."]
        #[inline(always)]
        pub const fn set_sserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc MODF."]
        #[must_use]
        #[inline(always)]
        pub const fn modf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MODF."]
        #[inline(always)]
        pub const fn set_modf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSY."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SSLVL."]
        #[must_use]
        #[inline(always)]
        pub const fn sslvl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSLVL."]
        #[inline(always)]
        pub const fn set_sslvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("txe", &self.txe())
                .field("rxne", &self.rxne())
                .field("ssf", &self.ssf())
                .field("ssr", &self.ssr())
                .field("udf", &self.udf())
                .field("ovf", &self.ovf())
                .field("sserr", &self.sserr())
                .field("modf", &self.modf())
                .field("busy", &self.busy())
                .field("sslvl", &self.sslvl())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SSI."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ssi(pub u32);
    impl Ssi {
        #[doc = "desc SSI."]
        #[must_use]
        #[inline(always)]
        pub const fn ssi(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSI."]
        #[inline(always)]
        pub const fn set_ssi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ssi {
        #[inline(always)]
        fn default() -> Ssi {
            Ssi(0)
        }
    }
    impl core::fmt::Debug for Ssi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ssi")
                .field("ssi", &self.ssi())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
