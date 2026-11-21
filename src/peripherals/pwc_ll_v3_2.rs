#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc PWC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwc {
    ptr: *mut u8,
}
unsafe impl Send for Pwc {}
unsafe impl Sync for Pwc {}
impl Pwc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc FCG0."]
    #[inline(always)]
    pub const fn fcg0(self) -> crate::common::Reg<regs::Fcg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc FCG1."]
    #[inline(always)]
    pub const fn fcg1(self) -> crate::common::Reg<regs::Fcg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc FCG2."]
    #[inline(always)]
    pub const fn fcg2(self) -> crate::common::Reg<regs::Fcg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc FCG3."]
    #[inline(always)]
    pub const fn fcg3(self) -> crate::common::Reg<regs::Fcg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc FCG0PC."]
    #[inline(always)]
    pub const fn fcg0pc(self) -> crate::common::Reg<regs::Fcg0pc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc WKTCR."]
    #[inline(always)]
    pub const fn wktcr(self) -> crate::common::Reg<regs::Wktcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4400usize) as _) }
    }
    #[doc = "desc STPMCR."]
    #[inline(always)]
    pub const fn stpmcr(self) -> crate::common::Reg<regs::Stpmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc00cusize) as _) }
    }
    #[doc = "desc RAMPC0."]
    #[inline(always)]
    pub const fn rampc0(self) -> crate::common::Reg<regs::Rampc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc014usize) as _) }
    }
    #[doc = "desc RAMOPM."]
    #[inline(always)]
    pub const fn ramopm(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc018usize) as _) }
    }
    #[doc = "desc PVDICR."]
    #[inline(always)]
    pub const fn pvdicr(self) -> crate::common::Reg<regs::Pvdicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0e0usize) as _) }
    }
    #[doc = "desc PVDDSR."]
    #[inline(always)]
    pub const fn pvddsr(self) -> crate::common::Reg<regs::Pvddsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0e1usize) as _) }
    }
    #[doc = "desc FPRC."]
    #[inline(always)]
    pub const fn fprc(self) -> crate::common::Reg<regs::Fprc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc3feusize) as _) }
    }
    #[doc = "desc PWRC0."]
    #[inline(always)]
    pub const fn pwrc0(self) -> crate::common::Reg<regs::Pwrc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc400usize) as _) }
    }
    #[doc = "desc PWRC1."]
    #[inline(always)]
    pub const fn pwrc1(self) -> crate::common::Reg<regs::Pwrc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc401usize) as _) }
    }
    #[doc = "desc PWRC2."]
    #[inline(always)]
    pub const fn pwrc2(self) -> crate::common::Reg<regs::Pwrc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc402usize) as _) }
    }
    #[doc = "desc PWRC3."]
    #[inline(always)]
    pub const fn pwrc3(self) -> crate::common::Reg<regs::Pwrc3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc403usize) as _) }
    }
    #[doc = "desc PDWKE0."]
    #[inline(always)]
    pub const fn pdwke0(self) -> crate::common::Reg<regs::Pdwke0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc404usize) as _) }
    }
    #[doc = "desc PDWKE1."]
    #[inline(always)]
    pub const fn pdwke1(self) -> crate::common::Reg<regs::Pdwke1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc405usize) as _) }
    }
    #[doc = "desc PDWKE2."]
    #[inline(always)]
    pub const fn pdwke2(self) -> crate::common::Reg<regs::Pdwke2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc406usize) as _) }
    }
    #[doc = "desc PDWKES."]
    #[inline(always)]
    pub const fn pdwkes(self) -> crate::common::Reg<regs::Pdwkes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc407usize) as _) }
    }
    #[doc = "desc PDWKF0."]
    #[inline(always)]
    pub const fn pdwkf0(self) -> crate::common::Reg<regs::Pdwkf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc408usize) as _) }
    }
    #[doc = "desc PDWKF1."]
    #[inline(always)]
    pub const fn pdwkf1(self) -> crate::common::Reg<regs::Pdwkf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc409usize) as _) }
    }
    #[doc = "desc PWCMR."]
    #[inline(always)]
    pub const fn pwcmr(self) -> crate::common::Reg<regs::Pwcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc40ausize) as _) }
    }
    #[doc = "desc MDSWCR."]
    #[inline(always)]
    pub const fn mdswcr(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc40fusize) as _) }
    }
    #[doc = "desc PVDCR0."]
    #[inline(always)]
    pub const fn pvdcr0(self) -> crate::common::Reg<regs::Pvdcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc412usize) as _) }
    }
    #[doc = "desc PVDCR1."]
    #[inline(always)]
    pub const fn pvdcr1(self) -> crate::common::Reg<regs::Pvdcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc413usize) as _) }
    }
    #[doc = "desc PVDFCR."]
    #[inline(always)]
    pub const fn pvdfcr(self) -> crate::common::Reg<regs::Pvdfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc414usize) as _) }
    }
    #[doc = "desc PVDLCR."]
    #[inline(always)]
    pub const fn pvdlcr(self) -> crate::common::Reg<regs::Pvdlcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc415usize) as _) }
    }
    #[doc = "desc XTAL32CS."]
    #[inline(always)]
    pub const fn xtal32cs(self) -> crate::common::Reg<regs::Xtal32cs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc42busize) as _) }
    }
}
pub mod regs {
    #[doc = "desc FCG0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcg0(pub u32);
    impl Fcg0 {
        #[doc = "desc SRAMH."]
        #[must_use]
        #[inline(always)]
        pub const fn sramh(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAMH."]
        #[inline(always)]
        pub const fn set_sramh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SRAM12."]
        #[must_use]
        #[inline(always)]
        pub const fn sram12(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAM12."]
        #[inline(always)]
        pub const fn set_sram12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SRAM3."]
        #[must_use]
        #[inline(always)]
        pub const fn sram3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAM3."]
        #[inline(always)]
        pub const fn set_sram3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SRAMRET."]
        #[must_use]
        #[inline(always)]
        pub const fn sramret(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAMRET."]
        #[inline(always)]
        pub const fn set_sramret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc DMA1."]
        #[must_use]
        #[inline(always)]
        pub const fn dma1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMA1."]
        #[inline(always)]
        pub const fn set_dma1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc DMA2."]
        #[must_use]
        #[inline(always)]
        pub const fn dma2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMA2."]
        #[inline(always)]
        pub const fn set_dma2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc FCM."]
        #[must_use]
        #[inline(always)]
        pub const fn fcm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCM."]
        #[inline(always)]
        pub const fn set_fcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc AOS."]
        #[must_use]
        #[inline(always)]
        pub const fn aos(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc AOS."]
        #[inline(always)]
        pub const fn set_aos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc AES."]
        #[must_use]
        #[inline(always)]
        pub const fn aes(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc AES."]
        #[inline(always)]
        pub const fn set_aes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc HASH."]
        #[must_use]
        #[inline(always)]
        pub const fn hash(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc HASH."]
        #[inline(always)]
        pub const fn set_hash(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TRNG."]
        #[must_use]
        #[inline(always)]
        pub const fn trng(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRNG."]
        #[inline(always)]
        pub const fn set_trng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc CRC."]
        #[must_use]
        #[inline(always)]
        pub const fn crc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRC."]
        #[inline(always)]
        pub const fn set_crc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc DCU1."]
        #[must_use]
        #[inline(always)]
        pub const fn dcu1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCU1."]
        #[inline(always)]
        pub const fn set_dcu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc DCU2."]
        #[must_use]
        #[inline(always)]
        pub const fn dcu2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCU2."]
        #[inline(always)]
        pub const fn set_dcu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc DCU3."]
        #[must_use]
        #[inline(always)]
        pub const fn dcu3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCU3."]
        #[inline(always)]
        pub const fn set_dcu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc DCU4."]
        #[must_use]
        #[inline(always)]
        pub const fn dcu4(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCU4."]
        #[inline(always)]
        pub const fn set_dcu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Fcg0 {
        #[inline(always)]
        fn default() -> Fcg0 {
            Fcg0(0)
        }
    }
    impl core::fmt::Debug for Fcg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcg0")
                .field("sramh", &self.sramh())
                .field("sram12", &self.sram12())
                .field("sram3", &self.sram3())
                .field("sramret", &self.sramret())
                .field("dma1", &self.dma1())
                .field("dma2", &self.dma2())
                .field("fcm", &self.fcm())
                .field("aos", &self.aos())
                .field("aes", &self.aes())
                .field("hash", &self.hash())
                .field("trng", &self.trng())
                .field("crc", &self.crc())
                .field("dcu1", &self.dcu1())
                .field("dcu2", &self.dcu2())
                .field("dcu3", &self.dcu3())
                .field("dcu4", &self.dcu4())
                .field("key", &self.key())
                .finish()
        }
    }
    #[doc = "desc FCG0PC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcg0pc(pub u32);
    impl Fcg0pc {
        #[doc = "desc PRT0."]
        #[must_use]
        #[inline(always)]
        pub const fn prt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRT0."]
        #[inline(always)]
        pub const fn set_prt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FCG0PCWE."]
        #[must_use]
        #[inline(always)]
        pub const fn fcg0pcwe(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc FCG0PCWE."]
        #[inline(always)]
        pub const fn set_fcg0pcwe(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Fcg0pc {
        #[inline(always)]
        fn default() -> Fcg0pc {
            Fcg0pc(0)
        }
    }
    impl core::fmt::Debug for Fcg0pc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcg0pc")
                .field("prt0", &self.prt0())
                .field("fcg0pcwe", &self.fcg0pcwe())
                .finish()
        }
    }
    #[doc = "desc FCG1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcg1(pub u32);
    impl Fcg1 {
        #[doc = "desc QSPI."]
        #[must_use]
        #[inline(always)]
        pub const fn qspi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc QSPI."]
        #[inline(always)]
        pub const fn set_qspi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc I2C1."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C1."]
        #[inline(always)]
        pub const fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc I2C2."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C2."]
        #[inline(always)]
        pub const fn set_i2c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc I2C3."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C3."]
        #[inline(always)]
        pub const fn set_i2c3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc USBFS."]
        #[must_use]
        #[inline(always)]
        pub const fn usbfs(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBFS."]
        #[inline(always)]
        pub const fn set_usbfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SDIOC1."]
        #[must_use]
        #[inline(always)]
        pub const fn sdioc1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc SDIOC1."]
        #[inline(always)]
        pub const fn set_sdioc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc SDIOC2."]
        #[must_use]
        #[inline(always)]
        pub const fn sdioc2(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc SDIOC2."]
        #[inline(always)]
        pub const fn set_sdioc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc I2S1."]
        #[must_use]
        #[inline(always)]
        pub const fn i2s1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2S1."]
        #[inline(always)]
        pub const fn set_i2s1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc I2S2."]
        #[must_use]
        #[inline(always)]
        pub const fn i2s2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2S2."]
        #[inline(always)]
        pub const fn set_i2s2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc I2S3."]
        #[must_use]
        #[inline(always)]
        pub const fn i2s3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2S3."]
        #[inline(always)]
        pub const fn set_i2s3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc I2S4."]
        #[must_use]
        #[inline(always)]
        pub const fn i2s4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2S4."]
        #[inline(always)]
        pub const fn set_i2s4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc SPI1."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI1."]
        #[inline(always)]
        pub const fn set_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPI2."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI2."]
        #[inline(always)]
        pub const fn set_spi2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc SPI3."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI3."]
        #[inline(always)]
        pub const fn set_spi3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc SPI4."]
        #[must_use]
        #[inline(always)]
        pub const fn spi4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI4."]
        #[inline(always)]
        pub const fn set_spi4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc USART1."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART1."]
        #[inline(always)]
        pub const fn set_usart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc USART2."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART2."]
        #[inline(always)]
        pub const fn set_usart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc USART3."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART3."]
        #[inline(always)]
        pub const fn set_usart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc USART4."]
        #[must_use]
        #[inline(always)]
        pub const fn usart4(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART4."]
        #[inline(always)]
        pub const fn set_usart4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Fcg1 {
        #[inline(always)]
        fn default() -> Fcg1 {
            Fcg1(0)
        }
    }
    impl core::fmt::Debug for Fcg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcg1")
                .field("qspi", &self.qspi())
                .field("i2c1", &self.i2c1())
                .field("i2c2", &self.i2c2())
                .field("i2c3", &self.i2c3())
                .field("usbfs", &self.usbfs())
                .field("sdioc1", &self.sdioc1())
                .field("sdioc2", &self.sdioc2())
                .field("i2s1", &self.i2s1())
                .field("i2s2", &self.i2s2())
                .field("i2s3", &self.i2s3())
                .field("i2s4", &self.i2s4())
                .field("spi1", &self.spi1())
                .field("spi2", &self.spi2())
                .field("spi3", &self.spi3())
                .field("spi4", &self.spi4())
                .field("usart1", &self.usart1())
                .field("usart2", &self.usart2())
                .field("usart3", &self.usart3())
                .field("usart4", &self.usart4())
                .finish()
        }
    }
    #[doc = "desc FCG2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcg2(pub u32);
    impl Fcg2 {
        #[doc = "desc TIMER0_1."]
        #[must_use]
        #[inline(always)]
        pub const fn timer0_1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMER0_1."]
        #[inline(always)]
        pub const fn set_timer0_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TIMER0_2."]
        #[must_use]
        #[inline(always)]
        pub const fn timer0_2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMER0_2."]
        #[inline(always)]
        pub const fn set_timer0_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TIMERA_1."]
        #[must_use]
        #[inline(always)]
        pub const fn timera_1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMERA_1."]
        #[inline(always)]
        pub const fn set_timera_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TIMERA_2."]
        #[must_use]
        #[inline(always)]
        pub const fn timera_2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMERA_2."]
        #[inline(always)]
        pub const fn set_timera_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TIMERA_3."]
        #[must_use]
        #[inline(always)]
        pub const fn timera_3(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMERA_3."]
        #[inline(always)]
        pub const fn set_timera_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TIMERA_4."]
        #[must_use]
        #[inline(always)]
        pub const fn timera_4(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMERA_4."]
        #[inline(always)]
        pub const fn set_timera_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TIMERA_5."]
        #[must_use]
        #[inline(always)]
        pub const fn timera_5(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMERA_5."]
        #[inline(always)]
        pub const fn set_timera_5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TIMERA_6."]
        #[must_use]
        #[inline(always)]
        pub const fn timera_6(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMERA_6."]
        #[inline(always)]
        pub const fn set_timera_6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TIMER4_1."]
        #[must_use]
        #[inline(always)]
        pub const fn timer4_1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMER4_1."]
        #[inline(always)]
        pub const fn set_timer4_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc TIMER4_2."]
        #[must_use]
        #[inline(always)]
        pub const fn timer4_2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMER4_2."]
        #[inline(always)]
        pub const fn set_timer4_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc TIMER4_3."]
        #[must_use]
        #[inline(always)]
        pub const fn timer4_3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMER4_3."]
        #[inline(always)]
        pub const fn set_timer4_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc EMB."]
        #[must_use]
        #[inline(always)]
        pub const fn emb(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EMB."]
        #[inline(always)]
        pub const fn set_emb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc TIMER6_1."]
        #[must_use]
        #[inline(always)]
        pub const fn timer6_1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMER6_1."]
        #[inline(always)]
        pub const fn set_timer6_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc TIMER6_2."]
        #[must_use]
        #[inline(always)]
        pub const fn timer6_2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMER6_2."]
        #[inline(always)]
        pub const fn set_timer6_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc TIMER6_3."]
        #[must_use]
        #[inline(always)]
        pub const fn timer6_3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMER6_3."]
        #[inline(always)]
        pub const fn set_timer6_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Fcg2 {
        #[inline(always)]
        fn default() -> Fcg2 {
            Fcg2(0)
        }
    }
    impl core::fmt::Debug for Fcg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcg2")
                .field("timer0_1", &self.timer0_1())
                .field("timer0_2", &self.timer0_2())
                .field("timera_1", &self.timera_1())
                .field("timera_2", &self.timera_2())
                .field("timera_3", &self.timera_3())
                .field("timera_4", &self.timera_4())
                .field("timera_5", &self.timera_5())
                .field("timera_6", &self.timera_6())
                .field("timer4_1", &self.timer4_1())
                .field("timer4_2", &self.timer4_2())
                .field("timer4_3", &self.timer4_3())
                .field("emb", &self.emb())
                .field("timer6_1", &self.timer6_1())
                .field("timer6_2", &self.timer6_2())
                .field("timer6_3", &self.timer6_3())
                .finish()
        }
    }
    #[doc = "desc FCG3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcg3(pub u32);
    impl Fcg3 {
        #[doc = "desc ADC1."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC1."]
        #[inline(always)]
        pub const fn set_adc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ADC2."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC2."]
        #[inline(always)]
        pub const fn set_adc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CMP."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMP."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc OTS."]
        #[must_use]
        #[inline(always)]
        pub const fn ots(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTS."]
        #[inline(always)]
        pub const fn set_ots(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Fcg3 {
        #[inline(always)]
        fn default() -> Fcg3 {
            Fcg3(0)
        }
    }
    impl core::fmt::Debug for Fcg3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcg3")
                .field("adc1", &self.adc1())
                .field("adc2", &self.adc2())
                .field("cmp", &self.cmp())
                .field("ots", &self.ots())
                .finish()
        }
    }
    #[doc = "desc FPRC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fprc(pub u16);
    impl Fprc {
        #[doc = "desc FPRCB0."]
        #[must_use]
        #[inline(always)]
        pub const fn fprcb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FPRCB0."]
        #[inline(always)]
        pub const fn set_fprcb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc FPRCB1."]
        #[must_use]
        #[inline(always)]
        pub const fn fprcb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FPRCB1."]
        #[inline(always)]
        pub const fn set_fprcb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc FPRCB2."]
        #[must_use]
        #[inline(always)]
        pub const fn fprcb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FPRCB2."]
        #[inline(always)]
        pub const fn set_fprcb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc FPRCB3."]
        #[must_use]
        #[inline(always)]
        pub const fn fprcb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FPRCB3."]
        #[inline(always)]
        pub const fn set_fprcb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "desc FPRCWE."]
        #[must_use]
        #[inline(always)]
        pub const fn fprcwe(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc FPRCWE."]
        #[inline(always)]
        pub const fn set_fprcwe(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for Fprc {
        #[inline(always)]
        fn default() -> Fprc {
            Fprc(0)
        }
    }
    impl core::fmt::Debug for Fprc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fprc")
                .field("fprcb0", &self.fprcb0())
                .field("fprcb1", &self.fprcb1())
                .field("fprcb2", &self.fprcb2())
                .field("fprcb3", &self.fprcb3())
                .field("fprcwe", &self.fprcwe())
                .finish()
        }
    }
    #[doc = "desc PDWKE0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdwke0(pub u8);
    impl Pdwke0 {
        #[doc = "desc WKE00."]
        #[must_use]
        #[inline(always)]
        pub const fn wke00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE00."]
        #[inline(always)]
        pub const fn set_wke00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc WKE01."]
        #[must_use]
        #[inline(always)]
        pub const fn wke01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE01."]
        #[inline(always)]
        pub const fn set_wke01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc WKE02."]
        #[must_use]
        #[inline(always)]
        pub const fn wke02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE02."]
        #[inline(always)]
        pub const fn set_wke02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc WKE03."]
        #[must_use]
        #[inline(always)]
        pub const fn wke03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE03."]
        #[inline(always)]
        pub const fn set_wke03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc WKE10."]
        #[must_use]
        #[inline(always)]
        pub const fn wke10(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE10."]
        #[inline(always)]
        pub const fn set_wke10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc WKE11."]
        #[must_use]
        #[inline(always)]
        pub const fn wke11(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE11."]
        #[inline(always)]
        pub const fn set_wke11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc WKE12."]
        #[must_use]
        #[inline(always)]
        pub const fn wke12(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE12."]
        #[inline(always)]
        pub const fn set_wke12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc WKE13."]
        #[must_use]
        #[inline(always)]
        pub const fn wke13(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE13."]
        #[inline(always)]
        pub const fn set_wke13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Pdwke0 {
        #[inline(always)]
        fn default() -> Pdwke0 {
            Pdwke0(0)
        }
    }
    impl core::fmt::Debug for Pdwke0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdwke0")
                .field("wke00", &self.wke00())
                .field("wke01", &self.wke01())
                .field("wke02", &self.wke02())
                .field("wke03", &self.wke03())
                .field("wke10", &self.wke10())
                .field("wke11", &self.wke11())
                .field("wke12", &self.wke12())
                .field("wke13", &self.wke13())
                .finish()
        }
    }
    #[doc = "desc PDWKE1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdwke1(pub u8);
    impl Pdwke1 {
        #[doc = "desc WKE20."]
        #[must_use]
        #[inline(always)]
        pub const fn wke20(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE20."]
        #[inline(always)]
        pub const fn set_wke20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc WKE21."]
        #[must_use]
        #[inline(always)]
        pub const fn wke21(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE21."]
        #[inline(always)]
        pub const fn set_wke21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc WKE22."]
        #[must_use]
        #[inline(always)]
        pub const fn wke22(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE22."]
        #[inline(always)]
        pub const fn set_wke22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc WKE23."]
        #[must_use]
        #[inline(always)]
        pub const fn wke23(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE23."]
        #[inline(always)]
        pub const fn set_wke23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc WKE30."]
        #[must_use]
        #[inline(always)]
        pub const fn wke30(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE30."]
        #[inline(always)]
        pub const fn set_wke30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc WKE31."]
        #[must_use]
        #[inline(always)]
        pub const fn wke31(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE31."]
        #[inline(always)]
        pub const fn set_wke31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc WKE32."]
        #[must_use]
        #[inline(always)]
        pub const fn wke32(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE32."]
        #[inline(always)]
        pub const fn set_wke32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc WKE33."]
        #[must_use]
        #[inline(always)]
        pub const fn wke33(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKE33."]
        #[inline(always)]
        pub const fn set_wke33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Pdwke1 {
        #[inline(always)]
        fn default() -> Pdwke1 {
            Pdwke1(0)
        }
    }
    impl core::fmt::Debug for Pdwke1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdwke1")
                .field("wke20", &self.wke20())
                .field("wke21", &self.wke21())
                .field("wke22", &self.wke22())
                .field("wke23", &self.wke23())
                .field("wke30", &self.wke30())
                .field("wke31", &self.wke31())
                .field("wke32", &self.wke32())
                .field("wke33", &self.wke33())
                .finish()
        }
    }
    #[doc = "desc PDWKE2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdwke2(pub u8);
    impl Pdwke2 {
        #[doc = "desc VD1WKE."]
        #[must_use]
        #[inline(always)]
        pub const fn vd1wke(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VD1WKE."]
        #[inline(always)]
        pub const fn set_vd1wke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc VD2WKE."]
        #[must_use]
        #[inline(always)]
        pub const fn vd2wke(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VD2WKE."]
        #[inline(always)]
        pub const fn set_vd2wke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc NMIWKE."]
        #[must_use]
        #[inline(always)]
        pub const fn nmiwke(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIWKE."]
        #[inline(always)]
        pub const fn set_nmiwke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc RTCPRDWKE."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcprdwke(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTCPRDWKE."]
        #[inline(always)]
        pub const fn set_rtcprdwke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc RTCALMWKE."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcalmwke(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTCALMWKE."]
        #[inline(always)]
        pub const fn set_rtcalmwke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc WKTMWKE."]
        #[must_use]
        #[inline(always)]
        pub const fn wktmwke(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKTMWKE."]
        #[inline(always)]
        pub const fn set_wktmwke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Pdwke2 {
        #[inline(always)]
        fn default() -> Pdwke2 {
            Pdwke2(0)
        }
    }
    impl core::fmt::Debug for Pdwke2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdwke2")
                .field("vd1wke", &self.vd1wke())
                .field("vd2wke", &self.vd2wke())
                .field("nmiwke", &self.nmiwke())
                .field("rtcprdwke", &self.rtcprdwke())
                .field("rtcalmwke", &self.rtcalmwke())
                .field("wktmwke", &self.wktmwke())
                .finish()
        }
    }
    #[doc = "desc PDWKES."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdwkes(pub u8);
    impl Pdwkes {
        #[doc = "desc WK0EGS."]
        #[must_use]
        #[inline(always)]
        pub const fn wk0egs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc WK0EGS."]
        #[inline(always)]
        pub const fn set_wk0egs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc WK1EGS."]
        #[must_use]
        #[inline(always)]
        pub const fn wk1egs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc WK1EGS."]
        #[inline(always)]
        pub const fn set_wk1egs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc WK2EGS."]
        #[must_use]
        #[inline(always)]
        pub const fn wk2egs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc WK2EGS."]
        #[inline(always)]
        pub const fn set_wk2egs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc WK3EGS."]
        #[must_use]
        #[inline(always)]
        pub const fn wk3egs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc WK3EGS."]
        #[inline(always)]
        pub const fn set_wk3egs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc VD1EGS."]
        #[must_use]
        #[inline(always)]
        pub const fn vd1egs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VD1EGS."]
        #[inline(always)]
        pub const fn set_vd1egs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc VD2EGS."]
        #[must_use]
        #[inline(always)]
        pub const fn vd2egs(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VD2EGS."]
        #[inline(always)]
        pub const fn set_vd2egs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc NMIEGS."]
        #[must_use]
        #[inline(always)]
        pub const fn nmiegs(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIEGS."]
        #[inline(always)]
        pub const fn set_nmiegs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Pdwkes {
        #[inline(always)]
        fn default() -> Pdwkes {
            Pdwkes(0)
        }
    }
    impl core::fmt::Debug for Pdwkes {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdwkes")
                .field("wk0egs", &self.wk0egs())
                .field("wk1egs", &self.wk1egs())
                .field("wk2egs", &self.wk2egs())
                .field("wk3egs", &self.wk3egs())
                .field("vd1egs", &self.vd1egs())
                .field("vd2egs", &self.vd2egs())
                .field("nmiegs", &self.nmiegs())
                .finish()
        }
    }
    #[doc = "desc PDWKF0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdwkf0(pub u8);
    impl Pdwkf0 {
        #[doc = "desc PTWK0F."]
        #[must_use]
        #[inline(always)]
        pub const fn ptwk0f(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PTWK0F."]
        #[inline(always)]
        pub const fn set_ptwk0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc PTWK1F."]
        #[must_use]
        #[inline(always)]
        pub const fn ptwk1f(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PTWK1F."]
        #[inline(always)]
        pub const fn set_ptwk1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc PTWK2F."]
        #[must_use]
        #[inline(always)]
        pub const fn ptwk2f(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PTWK2F."]
        #[inline(always)]
        pub const fn set_ptwk2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc PTWK3F."]
        #[must_use]
        #[inline(always)]
        pub const fn ptwk3f(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PTWK3F."]
        #[inline(always)]
        pub const fn set_ptwk3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc VD1WKF."]
        #[must_use]
        #[inline(always)]
        pub const fn vd1wkf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VD1WKF."]
        #[inline(always)]
        pub const fn set_vd1wkf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc VD2WKF."]
        #[must_use]
        #[inline(always)]
        pub const fn vd2wkf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VD2WKF."]
        #[inline(always)]
        pub const fn set_vd2wkf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc NMIWKF."]
        #[must_use]
        #[inline(always)]
        pub const fn nmiwkf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIWKF."]
        #[inline(always)]
        pub const fn set_nmiwkf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Pdwkf0 {
        #[inline(always)]
        fn default() -> Pdwkf0 {
            Pdwkf0(0)
        }
    }
    impl core::fmt::Debug for Pdwkf0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdwkf0")
                .field("ptwk0f", &self.ptwk0f())
                .field("ptwk1f", &self.ptwk1f())
                .field("ptwk2f", &self.ptwk2f())
                .field("ptwk3f", &self.ptwk3f())
                .field("vd1wkf", &self.vd1wkf())
                .field("vd2wkf", &self.vd2wkf())
                .field("nmiwkf", &self.nmiwkf())
                .finish()
        }
    }
    #[doc = "desc PDWKF1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdwkf1(pub u8);
    impl Pdwkf1 {
        #[doc = "desc RTCPRDWKF."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcprdwkf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTCPRDWKF."]
        #[inline(always)]
        pub const fn set_rtcprdwkf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc RTCALMWKF."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcalmwkf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTCALMWKF."]
        #[inline(always)]
        pub const fn set_rtcalmwkf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc WKTMWKF."]
        #[must_use]
        #[inline(always)]
        pub const fn wktmwkf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKTMWKF."]
        #[inline(always)]
        pub const fn set_wktmwkf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Pdwkf1 {
        #[inline(always)]
        fn default() -> Pdwkf1 {
            Pdwkf1(0)
        }
    }
    impl core::fmt::Debug for Pdwkf1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdwkf1")
                .field("rtcprdwkf", &self.rtcprdwkf())
                .field("rtcalmwkf", &self.rtcalmwkf())
                .field("wktmwkf", &self.wktmwkf())
                .finish()
        }
    }
    #[doc = "desc PVDCR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pvdcr0(pub u8);
    impl Pvdcr0 {
        #[doc = "desc EXVCCINEN."]
        #[must_use]
        #[inline(always)]
        pub const fn exvccinen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EXVCCINEN."]
        #[inline(always)]
        pub const fn set_exvccinen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc PVD1EN."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1EN."]
        #[inline(always)]
        pub const fn set_pvd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc PVD2EN."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2EN."]
        #[inline(always)]
        pub const fn set_pvd2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Pvdcr0 {
        #[inline(always)]
        fn default() -> Pvdcr0 {
            Pvdcr0(0)
        }
    }
    impl core::fmt::Debug for Pvdcr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pvdcr0")
                .field("exvccinen", &self.exvccinen())
                .field("pvd1en", &self.pvd1en())
                .field("pvd2en", &self.pvd2en())
                .finish()
        }
    }
    #[doc = "desc PVDCR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pvdcr1(pub u8);
    impl Pvdcr1 {
        #[doc = "desc PVD1IRE."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1ire(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1IRE."]
        #[inline(always)]
        pub const fn set_pvd1ire(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc PVD1IRS."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1irs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1IRS."]
        #[inline(always)]
        pub const fn set_pvd1irs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc PVD1CMPOE."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1cmpoe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1CMPOE."]
        #[inline(always)]
        pub const fn set_pvd1cmpoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc PVD2IRE."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2ire(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2IRE."]
        #[inline(always)]
        pub const fn set_pvd2ire(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc PVD2IRS."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2irs(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2IRS."]
        #[inline(always)]
        pub const fn set_pvd2irs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc PVD2CMPOE."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2cmpoe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2CMPOE."]
        #[inline(always)]
        pub const fn set_pvd2cmpoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Pvdcr1 {
        #[inline(always)]
        fn default() -> Pvdcr1 {
            Pvdcr1(0)
        }
    }
    impl core::fmt::Debug for Pvdcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pvdcr1")
                .field("pvd1ire", &self.pvd1ire())
                .field("pvd1irs", &self.pvd1irs())
                .field("pvd1cmpoe", &self.pvd1cmpoe())
                .field("pvd2ire", &self.pvd2ire())
                .field("pvd2irs", &self.pvd2irs())
                .field("pvd2cmpoe", &self.pvd2cmpoe())
                .finish()
        }
    }
    #[doc = "desc PVDDSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pvddsr(pub u8);
    impl Pvddsr {
        #[doc = "desc PVD1MON."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1mon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1MON."]
        #[inline(always)]
        pub const fn set_pvd1mon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc PVD1DETFLG."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1detflg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1DETFLG."]
        #[inline(always)]
        pub const fn set_pvd1detflg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc PVD2MON."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2mon(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2MON."]
        #[inline(always)]
        pub const fn set_pvd2mon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc PVD2DETFLG."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2detflg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2DETFLG."]
        #[inline(always)]
        pub const fn set_pvd2detflg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Pvddsr {
        #[inline(always)]
        fn default() -> Pvddsr {
            Pvddsr(0)
        }
    }
    impl core::fmt::Debug for Pvddsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pvddsr")
                .field("pvd1mon", &self.pvd1mon())
                .field("pvd1detflg", &self.pvd1detflg())
                .field("pvd2mon", &self.pvd2mon())
                .field("pvd2detflg", &self.pvd2detflg())
                .finish()
        }
    }
    #[doc = "desc PVDFCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pvdfcr(pub u8);
    impl Pvdfcr {
        #[doc = "desc PVD1NFDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1nfdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1NFDIS."]
        #[inline(always)]
        pub const fn set_pvd1nfdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc PVD1NFCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1nfcks(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "desc PVD1NFCKS."]
        #[inline(always)]
        pub const fn set_pvd1nfcks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u8) & 0x03) << 1usize);
        }
        #[doc = "desc PVD2NFDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2nfdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2NFDIS."]
        #[inline(always)]
        pub const fn set_pvd2nfdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc PVD2NFCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2nfcks(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "desc PVD2NFCKS."]
        #[inline(always)]
        pub const fn set_pvd2nfcks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u8) & 0x03) << 5usize);
        }
    }
    impl Default for Pvdfcr {
        #[inline(always)]
        fn default() -> Pvdfcr {
            Pvdfcr(0)
        }
    }
    impl core::fmt::Debug for Pvdfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pvdfcr")
                .field("pvd1nfdis", &self.pvd1nfdis())
                .field("pvd1nfcks", &self.pvd1nfcks())
                .field("pvd2nfdis", &self.pvd2nfdis())
                .field("pvd2nfcks", &self.pvd2nfcks())
                .finish()
        }
    }
    #[doc = "desc PVDICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pvdicr(pub u8);
    impl Pvdicr {
        #[doc = "desc PVD1NMIS."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1nmis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1NMIS."]
        #[inline(always)]
        pub const fn set_pvd1nmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc PVD2NMIS."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2nmis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2NMIS."]
        #[inline(always)]
        pub const fn set_pvd2nmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for Pvdicr {
        #[inline(always)]
        fn default() -> Pvdicr {
            Pvdicr(0)
        }
    }
    impl core::fmt::Debug for Pvdicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pvdicr")
                .field("pvd1nmis", &self.pvd1nmis())
                .field("pvd2nmis", &self.pvd2nmis())
                .finish()
        }
    }
    #[doc = "desc PVDLCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pvdlcr(pub u8);
    impl Pvdlcr {
        #[doc = "desc PVD1LVL."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1lvl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc PVD1LVL."]
        #[inline(always)]
        pub const fn set_pvd1lvl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
        #[doc = "desc PVD2LVL."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2lvl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc PVD2LVL."]
        #[inline(always)]
        pub const fn set_pvd2lvl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
        }
    }
    impl Default for Pvdlcr {
        #[inline(always)]
        fn default() -> Pvdlcr {
            Pvdlcr(0)
        }
    }
    impl core::fmt::Debug for Pvdlcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pvdlcr")
                .field("pvd1lvl", &self.pvd1lvl())
                .field("pvd2lvl", &self.pvd2lvl())
                .finish()
        }
    }
    #[doc = "desc PWCMR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwcmr(pub u8);
    impl Pwcmr {
        #[doc = "desc ADBUFE."]
        #[must_use]
        #[inline(always)]
        pub const fn adbufe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADBUFE."]
        #[inline(always)]
        pub const fn set_adbufe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Pwcmr {
        #[inline(always)]
        fn default() -> Pwcmr {
            Pwcmr(0)
        }
    }
    impl core::fmt::Debug for Pwcmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwcmr")
                .field("adbufe", &self.adbufe())
                .finish()
        }
    }
    #[doc = "desc PWRC0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwrc0(pub u8);
    impl Pwrc0 {
        #[doc = "desc PDMDS."]
        #[must_use]
        #[inline(always)]
        pub const fn pdmds(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc PDMDS."]
        #[inline(always)]
        pub const fn set_pdmds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "desc VVDRSD."]
        #[must_use]
        #[inline(always)]
        pub const fn vvdrsd(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VVDRSD."]
        #[inline(always)]
        pub const fn set_vvdrsd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc RETRAMSD."]
        #[must_use]
        #[inline(always)]
        pub const fn retramsd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RETRAMSD."]
        #[inline(always)]
        pub const fn set_retramsd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc IORTN."]
        #[must_use]
        #[inline(always)]
        pub const fn iortn(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc IORTN."]
        #[inline(always)]
        pub const fn set_iortn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "desc PWDN."]
        #[must_use]
        #[inline(always)]
        pub const fn pwdn(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWDN."]
        #[inline(always)]
        pub const fn set_pwdn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Pwrc0 {
        #[inline(always)]
        fn default() -> Pwrc0 {
            Pwrc0(0)
        }
    }
    impl core::fmt::Debug for Pwrc0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwrc0")
                .field("pdmds", &self.pdmds())
                .field("vvdrsd", &self.vvdrsd())
                .field("retramsd", &self.retramsd())
                .field("iortn", &self.iortn())
                .field("pwdn", &self.pwdn())
                .finish()
        }
    }
    #[doc = "desc PWRC1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwrc1(pub u8);
    impl Pwrc1 {
        #[doc = "desc VPLLSD."]
        #[must_use]
        #[inline(always)]
        pub const fn vpllsd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VPLLSD."]
        #[inline(always)]
        pub const fn set_vpllsd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc VHRCSD."]
        #[must_use]
        #[inline(always)]
        pub const fn vhrcsd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VHRCSD."]
        #[inline(always)]
        pub const fn set_vhrcsd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc STPDAS."]
        #[must_use]
        #[inline(always)]
        pub const fn stpdas(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc STPDAS."]
        #[inline(always)]
        pub const fn set_stpdas(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Pwrc1 {
        #[inline(always)]
        fn default() -> Pwrc1 {
            Pwrc1(0)
        }
    }
    impl core::fmt::Debug for Pwrc1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwrc1")
                .field("vpllsd", &self.vpllsd())
                .field("vhrcsd", &self.vhrcsd())
                .field("stpdas", &self.stpdas())
                .finish()
        }
    }
    #[doc = "desc PWRC2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwrc2(pub u8);
    impl Pwrc2 {
        #[doc = "desc DDAS."]
        #[must_use]
        #[inline(always)]
        pub const fn ddas(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc DDAS."]
        #[inline(always)]
        pub const fn set_ddas(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc DVS."]
        #[must_use]
        #[inline(always)]
        pub const fn dvs(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc DVS."]
        #[inline(always)]
        pub const fn set_dvs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
    }
    impl Default for Pwrc2 {
        #[inline(always)]
        fn default() -> Pwrc2 {
            Pwrc2(0)
        }
    }
    impl core::fmt::Debug for Pwrc2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwrc2")
                .field("ddas", &self.ddas())
                .field("dvs", &self.dvs())
                .finish()
        }
    }
    #[doc = "desc PWRC3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwrc3(pub u8);
    impl Pwrc3 {
        #[doc = "desc PDTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pdts(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PDTS."]
        #[inline(always)]
        pub const fn set_pdts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for Pwrc3 {
        #[inline(always)]
        fn default() -> Pwrc3 {
            Pwrc3(0)
        }
    }
    impl core::fmt::Debug for Pwrc3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwrc3").field("pdts", &self.pdts()).finish()
        }
    }
    #[doc = "desc RAMPC0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rampc0(pub u32);
    impl Rampc0 {
        #[doc = "desc RAMPDC0."]
        #[must_use]
        #[inline(always)]
        pub const fn rampdc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAMPDC0."]
        #[inline(always)]
        pub const fn set_rampdc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RAMPDC1."]
        #[must_use]
        #[inline(always)]
        pub const fn rampdc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAMPDC1."]
        #[inline(always)]
        pub const fn set_rampdc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RAMPDC2."]
        #[must_use]
        #[inline(always)]
        pub const fn rampdc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAMPDC2."]
        #[inline(always)]
        pub const fn set_rampdc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RAMPDC3."]
        #[must_use]
        #[inline(always)]
        pub const fn rampdc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAMPDC3."]
        #[inline(always)]
        pub const fn set_rampdc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RAMPDC4."]
        #[must_use]
        #[inline(always)]
        pub const fn rampdc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAMPDC4."]
        #[inline(always)]
        pub const fn set_rampdc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RAMPDC5."]
        #[must_use]
        #[inline(always)]
        pub const fn rampdc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAMPDC5."]
        #[inline(always)]
        pub const fn set_rampdc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RAMPDC6."]
        #[must_use]
        #[inline(always)]
        pub const fn rampdc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAMPDC6."]
        #[inline(always)]
        pub const fn set_rampdc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RAMPDC7."]
        #[must_use]
        #[inline(always)]
        pub const fn rampdc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAMPDC7."]
        #[inline(always)]
        pub const fn set_rampdc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RAMPDC8."]
        #[must_use]
        #[inline(always)]
        pub const fn rampdc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAMPDC8."]
        #[inline(always)]
        pub const fn set_rampdc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Rampc0 {
        #[inline(always)]
        fn default() -> Rampc0 {
            Rampc0(0)
        }
    }
    impl core::fmt::Debug for Rampc0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rampc0")
                .field("rampdc0", &self.rampdc0())
                .field("rampdc1", &self.rampdc1())
                .field("rampdc2", &self.rampdc2())
                .field("rampdc3", &self.rampdc3())
                .field("rampdc4", &self.rampdc4())
                .field("rampdc5", &self.rampdc5())
                .field("rampdc6", &self.rampdc6())
                .field("rampdc7", &self.rampdc7())
                .field("rampdc8", &self.rampdc8())
                .finish()
        }
    }
    #[doc = "desc STPMCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stpmcr(pub u16);
    impl Stpmcr {
        #[doc = "desc FLNWT."]
        #[must_use]
        #[inline(always)]
        pub const fn flnwt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLNWT."]
        #[inline(always)]
        pub const fn set_flnwt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc CKSMRC."]
        #[must_use]
        #[inline(always)]
        pub const fn cksmrc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKSMRC."]
        #[inline(always)]
        pub const fn set_cksmrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc STOP."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc STOP."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Stpmcr {
        #[inline(always)]
        fn default() -> Stpmcr {
            Stpmcr(0)
        }
    }
    impl core::fmt::Debug for Stpmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stpmcr")
                .field("flnwt", &self.flnwt())
                .field("cksmrc", &self.cksmrc())
                .field("stop", &self.stop())
                .finish()
        }
    }
    #[doc = "desc WKTCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wktcr(pub u16);
    impl Wktcr {
        #[doc = "desc WKTMCMP."]
        #[must_use]
        #[inline(always)]
        pub const fn wktmcmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc WKTMCMP."]
        #[inline(always)]
        pub const fn set_wktmcmp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
        #[doc = "desc WKOVF."]
        #[must_use]
        #[inline(always)]
        pub const fn wkovf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKOVF."]
        #[inline(always)]
        pub const fn set_wkovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "desc WKCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn wkcks(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "desc WKCKS."]
        #[inline(always)]
        pub const fn set_wkcks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u16) & 0x03) << 13usize);
        }
        #[doc = "desc WKTCE."]
        #[must_use]
        #[inline(always)]
        pub const fn wktce(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKTCE."]
        #[inline(always)]
        pub const fn set_wktce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Wktcr {
        #[inline(always)]
        fn default() -> Wktcr {
            Wktcr(0)
        }
    }
    impl core::fmt::Debug for Wktcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wktcr")
                .field("wktmcmp", &self.wktmcmp())
                .field("wkovf", &self.wkovf())
                .field("wkcks", &self.wkcks())
                .field("wktce", &self.wktce())
                .finish()
        }
    }
    #[doc = "desc XTAL32CS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtal32cs(pub u8);
    impl Xtal32cs {
        #[doc = "desc CSDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn csdis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSDIS."]
        #[inline(always)]
        pub const fn set_csdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Xtal32cs {
        #[inline(always)]
        fn default() -> Xtal32cs {
            Xtal32cs(0)
        }
    }
    impl core::fmt::Debug for Xtal32cs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtal32cs")
                .field("csdis", &self.csdis())
                .finish()
        }
    }
}
