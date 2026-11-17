#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc GPIO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc ADS."]
    #[inline(always)]
    pub const fn ads(self) -> crate::common::Reg<regs::Ads, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc DIR."]
    #[inline(always)]
    pub const fn dir(self) -> crate::common::Reg<regs::Dir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc OPENDRAIN."]
    #[inline(always)]
    pub const fn opendrain(self) -> crate::common::Reg<regs::Opendrain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc PU."]
    #[inline(always)]
    pub const fn pu(self) -> crate::common::Reg<regs::Pu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc IN."]
    #[inline(always)]
    pub const fn in_(self) -> crate::common::Reg<regs::In, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc OUT."]
    #[inline(always)]
    pub const fn out(self) -> crate::common::Reg<regs::Out, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc BRR."]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc BSRR."]
    #[inline(always)]
    pub const fn bsrr(self) -> crate::common::Reg<regs::Bsrr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc AFRL."]
    #[inline(always)]
    pub const fn afrl(self) -> crate::common::Reg<regs::Afrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc HIGHIE."]
    #[inline(always)]
    pub const fn highie(self) -> crate::common::Reg<regs::Highie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc LOWIE."]
    #[inline(always)]
    pub const fn lowie(self) -> crate::common::Reg<regs::Lowie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc RISEIE."]
    #[inline(always)]
    pub const fn riseie(self) -> crate::common::Reg<regs::Riseie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc FALLIE."]
    #[inline(always)]
    pub const fn fallie(self) -> crate::common::Reg<regs::Fallie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc IFR."]
    #[inline(always)]
    pub const fn ifr(self) -> crate::common::Reg<regs::Ifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ADS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ads(pub u32);
    impl Ads {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
    impl Default for Ads {
        #[inline(always)]
        fn default() -> Ads {
            Ads(0)
        }
    }
    impl core::fmt::Debug for Ads {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ads")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc AFRL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afrl(pub u32);
    impl Afrl {
        #[doc = "desc AFSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn afsel0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFSEL0."]
        #[inline(always)]
        pub const fn set_afsel0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc AFSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn afsel1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFSEL1."]
        #[inline(always)]
        pub const fn set_afsel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc AFSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn afsel2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFSEL2."]
        #[inline(always)]
        pub const fn set_afsel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc AFSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn afsel3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFSEL3."]
        #[inline(always)]
        pub const fn set_afsel3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "desc AFSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn afsel4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFSEL4."]
        #[inline(always)]
        pub const fn set_afsel4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc AFSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn afsel5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFSEL5."]
        #[inline(always)]
        pub const fn set_afsel5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "desc AFSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn afsel6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFSEL6."]
        #[inline(always)]
        pub const fn set_afsel6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "desc AFSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn afsel7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFSEL7."]
        #[inline(always)]
        pub const fn set_afsel7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Afrl {
        #[inline(always)]
        fn default() -> Afrl {
            Afrl(0)
        }
    }
    impl core::fmt::Debug for Afrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afrl")
                .field("afsel0", &self.afsel0())
                .field("afsel1", &self.afsel1())
                .field("afsel2", &self.afsel2())
                .field("afsel3", &self.afsel3())
                .field("afsel4", &self.afsel4())
                .field("afsel5", &self.afsel5())
                .field("afsel6", &self.afsel6())
                .field("afsel7", &self.afsel7())
                .finish()
        }
    }
    #[doc = "desc BRR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brr(pub u32);
    impl Brr {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
    impl Default for Brr {
        #[inline(always)]
        fn default() -> Brr {
            Brr(0)
        }
    }
    impl core::fmt::Debug for Brr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brr")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc BSRR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bsrr(pub u32);
    impl Bsrr {
        #[doc = "desc BSPIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn bspin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSPIN0."]
        #[inline(always)]
        pub const fn set_bspin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc BSPIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn bspin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSPIN1."]
        #[inline(always)]
        pub const fn set_bspin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc BSPIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn bspin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSPIN2."]
        #[inline(always)]
        pub const fn set_bspin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc BSPIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn bspin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSPIN3."]
        #[inline(always)]
        pub const fn set_bspin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc BSPIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn bspin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSPIN4."]
        #[inline(always)]
        pub const fn set_bspin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc BSPIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn bspin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSPIN5."]
        #[inline(always)]
        pub const fn set_bspin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc BSPIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn bspin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSPIN6."]
        #[inline(always)]
        pub const fn set_bspin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BSPIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn bspin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSPIN7."]
        #[inline(always)]
        pub const fn set_bspin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BRPIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn brpin0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRPIN0."]
        #[inline(always)]
        pub const fn set_brpin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc BRPIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn brpin1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRPIN1."]
        #[inline(always)]
        pub const fn set_brpin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc BRPIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn brpin2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRPIN2."]
        #[inline(always)]
        pub const fn set_brpin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc BRPIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn brpin3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRPIN3."]
        #[inline(always)]
        pub const fn set_brpin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc BRPIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn brpin4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRPIN4."]
        #[inline(always)]
        pub const fn set_brpin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc BRPIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn brpin5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRPIN5."]
        #[inline(always)]
        pub const fn set_brpin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc BRPIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn brpin6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRPIN6."]
        #[inline(always)]
        pub const fn set_brpin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc BRPIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn brpin7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRPIN7."]
        #[inline(always)]
        pub const fn set_brpin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
    impl Default for Bsrr {
        #[inline(always)]
        fn default() -> Bsrr {
            Bsrr(0)
        }
    }
    impl core::fmt::Debug for Bsrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bsrr")
                .field("bspin0", &self.bspin0())
                .field("bspin1", &self.bspin1())
                .field("bspin2", &self.bspin2())
                .field("bspin3", &self.bspin3())
                .field("bspin4", &self.bspin4())
                .field("bspin5", &self.bspin5())
                .field("bspin6", &self.bspin6())
                .field("bspin7", &self.bspin7())
                .field("brpin0", &self.brpin0())
                .field("brpin1", &self.brpin1())
                .field("brpin2", &self.brpin2())
                .field("brpin3", &self.brpin3())
                .field("brpin4", &self.brpin4())
                .field("brpin5", &self.brpin5())
                .field("brpin6", &self.brpin6())
                .field("brpin7", &self.brpin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc DIR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dir(pub u32);
    impl Dir {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
    impl Default for Dir {
        #[inline(always)]
        fn default() -> Dir {
            Dir(0)
        }
    }
    impl core::fmt::Debug for Dir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dir")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc FALLIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fallie(pub u32);
    impl Fallie {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
    impl Default for Fallie {
        #[inline(always)]
        fn default() -> Fallie {
            Fallie(0)
        }
    }
    impl core::fmt::Debug for Fallie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fallie")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc HIGHIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Highie(pub u32);
    impl Highie {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
    impl Default for Highie {
        #[inline(always)]
        fn default() -> Highie {
            Highie(0)
        }
    }
    impl core::fmt::Debug for Highie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Highie")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc IFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifr(pub u32);
    impl Ifr {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
    impl Default for Ifr {
        #[inline(always)]
        fn default() -> Ifr {
            Ifr(0)
        }
    }
    impl core::fmt::Debug for Ifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ifr")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc IN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct In(pub u32);
    impl In {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
    impl Default for In {
        #[inline(always)]
        fn default() -> In {
            In(0)
        }
    }
    impl core::fmt::Debug for In {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("In")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc LOWIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lowie(pub u32);
    impl Lowie {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
    impl Default for Lowie {
        #[inline(always)]
        fn default() -> Lowie {
            Lowie(0)
        }
    }
    impl core::fmt::Debug for Lowie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lowie")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc OPENDRAIN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opendrain(pub u32);
    impl Opendrain {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
    impl Default for Opendrain {
        #[inline(always)]
        fn default() -> Opendrain {
            Opendrain(0)
        }
    }
    impl core::fmt::Debug for Opendrain {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Opendrain")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc OUT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Out(pub u32);
    impl Out {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
    impl Default for Out {
        #[inline(always)]
        fn default() -> Out {
            Out(0)
        }
    }
    impl core::fmt::Debug for Out {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Out")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PU."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pu(pub u32);
    impl Pu {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
    impl Default for Pu {
        #[inline(always)]
        fn default() -> Pu {
            Pu(0)
        }
    }
    impl core::fmt::Debug for Pu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pu")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc RISEIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Riseie(pub u32);
    impl Riseie {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
    impl Default for Riseie {
        #[inline(always)]
        fn default() -> Riseie {
            Riseie(0)
        }
    }
    impl core::fmt::Debug for Riseie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Riseie")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
