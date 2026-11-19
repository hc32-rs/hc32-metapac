#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc ADC2."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc STR."]
    #[inline(always)]
    pub const fn str(self) -> crate::common::Reg<regs::Str, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CR0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CR2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "desc TRGSR."]
    #[inline(always)]
    pub const fn trgsr(self) -> crate::common::Reg<regs::Trgsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "desc CHSELRA."]
    #[inline(always)]
    pub const fn chselra(self) -> crate::common::Reg<regs::Chselra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc CHSELRB."]
    #[inline(always)]
    pub const fn chselrb(self) -> crate::common::Reg<regs::Chselrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc AVCHSELR."]
    #[inline(always)]
    pub const fn avchselr(self) -> crate::common::Reg<regs::Avchselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc EXCHSELR."]
    #[inline(always)]
    pub const fn exchselr(self) -> crate::common::Reg<regs::Exchselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc SSTR0."]
    #[inline(always)]
    pub const fn sstr0(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc SSTR1."]
    #[inline(always)]
    pub const fn sstr1(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21usize) as _) }
    }
    #[doc = "desc SSTR2."]
    #[inline(always)]
    pub const fn sstr2(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "desc SSTR3."]
    #[inline(always)]
    pub const fn sstr3(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23usize) as _) }
    }
    #[doc = "desc SSTR4."]
    #[inline(always)]
    pub const fn sstr4(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc SSTR5."]
    #[inline(always)]
    pub const fn sstr5(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25usize) as _) }
    }
    #[doc = "desc SSTR6."]
    #[inline(always)]
    pub const fn sstr6(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "desc SSTR7."]
    #[inline(always)]
    pub const fn sstr7(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x27usize) as _) }
    }
    #[doc = "desc CHMUXR0."]
    #[inline(always)]
    pub const fn chmuxr0(self) -> crate::common::Reg<regs::Chmuxr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc CHMUXR1."]
    #[inline(always)]
    pub const fn chmuxr1(self) -> crate::common::Reg<regs::Chmuxr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3ausize) as _) }
    }
    #[doc = "desc ISR."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x45usize) as _) }
    }
    #[doc = "desc ISCLRR."]
    #[inline(always)]
    pub const fn isclrr(self) -> crate::common::Reg<regs::Isclrr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x46usize) as _) }
    }
    #[doc = "desc DR0."]
    #[inline(always)]
    pub const fn dr0(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc DR1."]
    #[inline(always)]
    pub const fn dr1(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x52usize) as _) }
    }
    #[doc = "desc DR2."]
    #[inline(always)]
    pub const fn dr2(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc DR3."]
    #[inline(always)]
    pub const fn dr3(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x56usize) as _) }
    }
    #[doc = "desc DR4."]
    #[inline(always)]
    pub const fn dr4(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc DR5."]
    #[inline(always)]
    pub const fn dr5(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5ausize) as _) }
    }
    #[doc = "desc DR6."]
    #[inline(always)]
    pub const fn dr6(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc DR7."]
    #[inline(always)]
    pub const fn dr7(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5eusize) as _) }
    }
    #[doc = "desc AWDCR."]
    #[inline(always)]
    pub const fn awdcr(self) -> crate::common::Reg<regs::Awdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "desc AWDSR."]
    #[inline(always)]
    pub const fn awdsr(self) -> crate::common::Reg<regs::Awdsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize) as _) }
    }
    #[doc = "desc AWDSCLRR."]
    #[inline(always)]
    pub const fn awdsclrr(self) -> crate::common::Reg<regs::Awdsclrr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa3usize) as _) }
    }
    #[doc = "desc AWD0DR0."]
    #[inline(always)]
    pub const fn awd0dr0(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "desc AWD0DR1."]
    #[inline(always)]
    pub const fn awd0dr1(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa6usize) as _) }
    }
    #[doc = "desc AWD0CHSR."]
    #[inline(always)]
    pub const fn awd0chsr(self) -> crate::common::Reg<regs::Awd0chsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "desc AWD1DR0."]
    #[inline(always)]
    pub const fn awd1dr0(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "desc AWD1DR1."]
    #[inline(always)]
    pub const fn awd1dr1(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xaeusize) as _) }
    }
    #[doc = "desc AWD1CHSR."]
    #[inline(always)]
    pub const fn awd1chsr(self) -> crate::common::Reg<regs::Awd1chsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc AVCHSELR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Avchselr(pub u32);
    impl Avchselr {
        #[doc = "desc AVCHSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn avchsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc AVCHSEL."]
        #[inline(always)]
        pub const fn set_avchsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Avchselr {
        #[inline(always)]
        fn default() -> Avchselr {
            Avchselr(0)
        }
    }
    impl core::fmt::Debug for Avchselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Avchselr")
                .field("avchsel", &self.avchsel())
                .finish()
        }
    }
    #[doc = "desc AWD0CHSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd0chsr(pub u8);
    impl Awd0chsr {
        #[doc = "desc AWDCH."]
        #[must_use]
        #[inline(always)]
        pub const fn awdch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc AWDCH."]
        #[inline(always)]
        pub const fn set_awdch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Awd0chsr {
        #[inline(always)]
        fn default() -> Awd0chsr {
            Awd0chsr(0)
        }
    }
    impl core::fmt::Debug for Awd0chsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awd0chsr")
                .field("awdch", &self.awdch())
                .finish()
        }
    }
    #[doc = "desc AWD1CHSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd1chsr(pub u8);
    impl Awd1chsr {
        #[doc = "desc AWDCH."]
        #[must_use]
        #[inline(always)]
        pub const fn awdch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc AWDCH."]
        #[inline(always)]
        pub const fn set_awdch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Awd1chsr {
        #[inline(always)]
        fn default() -> Awd1chsr {
            Awd1chsr(0)
        }
    }
    impl core::fmt::Debug for Awd1chsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awd1chsr")
                .field("awdch", &self.awdch())
                .finish()
        }
    }
    #[doc = "desc AWDCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awdcr(pub u16);
    impl Awdcr {
        #[doc = "desc AWD0EN."]
        #[must_use]
        #[inline(always)]
        pub const fn awd0en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWD0EN."]
        #[inline(always)]
        pub const fn set_awd0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc AWD0IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn awd0ien(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWD0IEN."]
        #[inline(always)]
        pub const fn set_awd0ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc AWD0MD."]
        #[must_use]
        #[inline(always)]
        pub const fn awd0md(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWD0MD."]
        #[inline(always)]
        pub const fn set_awd0md(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc AWD1EN."]
        #[must_use]
        #[inline(always)]
        pub const fn awd1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWD1EN."]
        #[inline(always)]
        pub const fn set_awd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc AWD1IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn awd1ien(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWD1IEN."]
        #[inline(always)]
        pub const fn set_awd1ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc AWD1MD."]
        #[must_use]
        #[inline(always)]
        pub const fn awd1md(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWD1MD."]
        #[inline(always)]
        pub const fn set_awd1md(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc AWDCM."]
        #[must_use]
        #[inline(always)]
        pub const fn awdcm(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc AWDCM."]
        #[inline(always)]
        pub const fn set_awdcm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
    }
    impl Default for Awdcr {
        #[inline(always)]
        fn default() -> Awdcr {
            Awdcr(0)
        }
    }
    impl core::fmt::Debug for Awdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awdcr")
                .field("awd0en", &self.awd0en())
                .field("awd0ien", &self.awd0ien())
                .field("awd0md", &self.awd0md())
                .field("awd1en", &self.awd1en())
                .field("awd1ien", &self.awd1ien())
                .field("awd1md", &self.awd1md())
                .field("awdcm", &self.awdcm())
                .finish()
        }
    }
    #[doc = "desc AWDSCLRR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awdsclrr(pub u8);
    impl Awdsclrr {
        #[doc = "desc CLRAWD0F."]
        #[must_use]
        #[inline(always)]
        pub const fn clrawd0f(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLRAWD0F."]
        #[inline(always)]
        pub const fn set_clrawd0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc CLRAWD1F."]
        #[must_use]
        #[inline(always)]
        pub const fn clrawd1f(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLRAWD1F."]
        #[inline(always)]
        pub const fn set_clrawd1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc CLRAWDCMF."]
        #[must_use]
        #[inline(always)]
        pub const fn clrawdcmf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLRAWDCMF."]
        #[inline(always)]
        pub const fn set_clrawdcmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for Awdsclrr {
        #[inline(always)]
        fn default() -> Awdsclrr {
            Awdsclrr(0)
        }
    }
    impl core::fmt::Debug for Awdsclrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awdsclrr")
                .field("clrawd0f", &self.clrawd0f())
                .field("clrawd1f", &self.clrawd1f())
                .field("clrawdcmf", &self.clrawdcmf())
                .finish()
        }
    }
    #[doc = "desc AWDSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awdsr(pub u8);
    impl Awdsr {
        #[doc = "desc AWD0F."]
        #[must_use]
        #[inline(always)]
        pub const fn awd0f(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWD0F."]
        #[inline(always)]
        pub const fn set_awd0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc AWD1F."]
        #[must_use]
        #[inline(always)]
        pub const fn awd1f(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWD1F."]
        #[inline(always)]
        pub const fn set_awd1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc AWDCMF."]
        #[must_use]
        #[inline(always)]
        pub const fn awdcmf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWDCMF."]
        #[inline(always)]
        pub const fn set_awdcmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for Awdsr {
        #[inline(always)]
        fn default() -> Awdsr {
            Awdsr(0)
        }
    }
    impl core::fmt::Debug for Awdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awdsr")
                .field("awd0f", &self.awd0f())
                .field("awd1f", &self.awd1f())
                .field("awdcmf", &self.awdcmf())
                .finish()
        }
    }
    #[doc = "desc CHMUXR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chmuxr0(pub u16);
    impl Chmuxr0 {
        #[doc = "desc CH00MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch00mux(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CH00MUX."]
        #[inline(always)]
        pub const fn set_ch00mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[doc = "desc CH01MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch01mux(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CH01MUX."]
        #[inline(always)]
        pub const fn set_ch01mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[doc = "desc CH02MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch02mux(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CH02MUX."]
        #[inline(always)]
        pub const fn set_ch02mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[doc = "desc CH03MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch03mux(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CH03MUX."]
        #[inline(always)]
        pub const fn set_ch03mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for Chmuxr0 {
        #[inline(always)]
        fn default() -> Chmuxr0 {
            Chmuxr0(0)
        }
    }
    impl core::fmt::Debug for Chmuxr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chmuxr0")
                .field("ch00mux", &self.ch00mux())
                .field("ch01mux", &self.ch01mux())
                .field("ch02mux", &self.ch02mux())
                .field("ch03mux", &self.ch03mux())
                .finish()
        }
    }
    #[doc = "desc CHMUXR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chmuxr1(pub u16);
    impl Chmuxr1 {
        #[doc = "desc CH04MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch04mux(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CH04MUX."]
        #[inline(always)]
        pub const fn set_ch04mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[doc = "desc CH05MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch05mux(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CH05MUX."]
        #[inline(always)]
        pub const fn set_ch05mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[doc = "desc CH06MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch06mux(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CH06MUX."]
        #[inline(always)]
        pub const fn set_ch06mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[doc = "desc CH07MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch07mux(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CH07MUX."]
        #[inline(always)]
        pub const fn set_ch07mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for Chmuxr1 {
        #[inline(always)]
        fn default() -> Chmuxr1 {
            Chmuxr1(0)
        }
    }
    impl core::fmt::Debug for Chmuxr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chmuxr1")
                .field("ch04mux", &self.ch04mux())
                .field("ch05mux", &self.ch05mux())
                .field("ch06mux", &self.ch06mux())
                .field("ch07mux", &self.ch07mux())
                .finish()
        }
    }
    #[doc = "desc CHSELRA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chselra(pub u32);
    impl Chselra {
        #[doc = "desc CHSELA."]
        #[must_use]
        #[inline(always)]
        pub const fn chsela(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CHSELA."]
        #[inline(always)]
        pub const fn set_chsela(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Chselra {
        #[inline(always)]
        fn default() -> Chselra {
            Chselra(0)
        }
    }
    impl core::fmt::Debug for Chselra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chselra")
                .field("chsela", &self.chsela())
                .finish()
        }
    }
    #[doc = "desc CHSELRB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chselrb(pub u32);
    impl Chselrb {
        #[doc = "desc CHSELB."]
        #[must_use]
        #[inline(always)]
        pub const fn chselb(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CHSELB."]
        #[inline(always)]
        pub const fn set_chselb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Chselrb {
        #[inline(always)]
        fn default() -> Chselrb {
            Chselrb(0)
        }
    }
    impl core::fmt::Debug for Chselrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chselrb")
                .field("chselb", &self.chselb())
                .finish()
        }
    }
    #[doc = "desc CR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u16);
    impl Cr0 {
        #[doc = "desc MS."]
        #[must_use]
        #[inline(always)]
        pub const fn ms(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc MS."]
        #[inline(always)]
        pub const fn set_ms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
        }
        #[doc = "desc ACCSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn accsel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc ACCSEL."]
        #[inline(always)]
        pub const fn set_accsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
        }
        #[doc = "desc CLREN."]
        #[must_use]
        #[inline(always)]
        pub const fn clren(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLREN."]
        #[inline(always)]
        pub const fn set_clren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc DFMT."]
        #[must_use]
        #[inline(always)]
        pub const fn dfmt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DFMT."]
        #[inline(always)]
        pub const fn set_dfmt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc AVCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn avcnt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc AVCNT."]
        #[inline(always)]
        pub const fn set_avcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
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
                .field("ms", &self.ms())
                .field("accsel", &self.accsel())
                .field("clren", &self.clren())
                .field("dfmt", &self.dfmt())
                .field("avcnt", &self.avcnt())
                .finish()
        }
    }
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u16);
    impl Cr1 {
        #[doc = "desc RSCHSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn rschsel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSCHSEL."]
        #[inline(always)]
        pub const fn set_rschsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
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
                .field("rschsel", &self.rschsel())
                .finish()
        }
    }
    #[doc = "desc CR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u16);
    impl Cr2 {
        #[doc = "desc OVSS."]
        #[must_use]
        #[inline(always)]
        pub const fn ovss(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc OVSS."]
        #[inline(always)]
        pub const fn set_ovss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[doc = "desc OVSMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn ovsmod(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVSMOD."]
        #[inline(always)]
        pub const fn set_ovsmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("ovss", &self.ovss())
                .field("ovsmod", &self.ovsmod())
                .finish()
        }
    }
    #[doc = "desc EXCHSELR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exchselr(pub u8);
    impl Exchselr {
        #[doc = "desc EXCHSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn exchsel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EXCHSEL."]
        #[inline(always)]
        pub const fn set_exchsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Exchselr {
        #[inline(always)]
        fn default() -> Exchselr {
            Exchselr(0)
        }
    }
    impl core::fmt::Debug for Exchselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exchselr")
                .field("exchsel", &self.exchsel())
                .finish()
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u8);
    impl Icr {
        #[doc = "desc EOCAIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn eocaien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOCAIEN."]
        #[inline(always)]
        pub const fn set_eocaien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc EOCBIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn eocbien(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOCBIEN."]
        #[inline(always)]
        pub const fn set_eocbien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
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
                .field("eocaien", &self.eocaien())
                .field("eocbien", &self.eocbien())
                .finish()
        }
    }
    #[doc = "desc ISCLRR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isclrr(pub u8);
    impl Isclrr {
        #[doc = "desc CLREOCAF."]
        #[must_use]
        #[inline(always)]
        pub const fn clreocaf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLREOCAF."]
        #[inline(always)]
        pub const fn set_clreocaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc CLREOCBF."]
        #[must_use]
        #[inline(always)]
        pub const fn clreocbf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLREOCBF."]
        #[inline(always)]
        pub const fn set_clreocbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc CLRSASTPDF."]
        #[must_use]
        #[inline(always)]
        pub const fn clrsastpdf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLRSASTPDF."]
        #[inline(always)]
        pub const fn set_clrsastpdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for Isclrr {
        #[inline(always)]
        fn default() -> Isclrr {
            Isclrr(0)
        }
    }
    impl core::fmt::Debug for Isclrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isclrr")
                .field("clreocaf", &self.clreocaf())
                .field("clreocbf", &self.clreocbf())
                .field("clrsastpdf", &self.clrsastpdf())
                .finish()
        }
    }
    #[doc = "desc ISR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u8);
    impl Isr {
        #[doc = "desc EOCAF."]
        #[must_use]
        #[inline(always)]
        pub const fn eocaf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOCAF."]
        #[inline(always)]
        pub const fn set_eocaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc EOCBF."]
        #[must_use]
        #[inline(always)]
        pub const fn eocbf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOCBF."]
        #[inline(always)]
        pub const fn set_eocbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc SASTPDF."]
        #[must_use]
        #[inline(always)]
        pub const fn sastpdf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SASTPDF."]
        #[inline(always)]
        pub const fn set_sastpdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("eocaf", &self.eocaf())
                .field("eocbf", &self.eocbf())
                .field("sastpdf", &self.sastpdf())
                .finish()
        }
    }
    #[doc = "desc STR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Str(pub u8);
    impl Str {
        #[doc = "desc STRT."]
        #[must_use]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc STRT."]
        #[inline(always)]
        pub const fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Str {
        #[inline(always)]
        fn default() -> Str {
            Str(0)
        }
    }
    impl core::fmt::Debug for Str {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Str").field("strt", &self.strt()).finish()
        }
    }
    #[doc = "desc TRGSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trgsr(pub u16);
    impl Trgsr {
        #[doc = "desc TRGSELA."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsela(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc TRGSELA."]
        #[inline(always)]
        pub const fn set_trgsela(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[doc = "desc TRGENA."]
        #[must_use]
        #[inline(always)]
        pub const fn trgena(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRGENA."]
        #[inline(always)]
        pub const fn set_trgena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc TRGSELB."]
        #[must_use]
        #[inline(always)]
        pub const fn trgselb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc TRGSELB."]
        #[inline(always)]
        pub const fn set_trgselb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "desc TRGENB."]
        #[must_use]
        #[inline(always)]
        pub const fn trgenb(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRGENB."]
        #[inline(always)]
        pub const fn set_trgenb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Trgsr {
        #[inline(always)]
        fn default() -> Trgsr {
            Trgsr(0)
        }
    }
    impl core::fmt::Debug for Trgsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trgsr")
                .field("trgsela", &self.trgsela())
                .field("trgena", &self.trgena())
                .field("trgselb", &self.trgselb())
                .field("trgenb", &self.trgenb())
                .finish()
        }
    }
}
