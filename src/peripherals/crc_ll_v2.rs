#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc CRC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc {
    ptr: *mut u8,
}
unsafe impl Send for Crc {}
unsafe impl Sync for Crc {}
impl Crc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc RESLT1."]
    #[inline(always)]
    pub const fn reslt1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc RESLT2."]
    #[inline(always)]
    pub const fn reslt2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc DAT0."]
    #[inline(always)]
    pub const fn dat0(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "desc DAT1."]
    #[inline(always)]
    pub const fn dat1(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "desc DAT2."]
    #[inline(always)]
    pub const fn dat2(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "desc DAT3."]
    #[inline(always)]
    pub const fn dat3(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "desc DAT4."]
    #[inline(always)]
    pub const fn dat4(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "desc DAT5."]
    #[inline(always)]
    pub const fn dat5(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "desc DAT6."]
    #[inline(always)]
    pub const fn dat6(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "desc DAT7."]
    #[inline(always)]
    pub const fn dat7(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "desc DAT8."]
    #[inline(always)]
    pub const fn dat8(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "desc DAT9."]
    #[inline(always)]
    pub const fn dat9(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "desc DAT10."]
    #[inline(always)]
    pub const fn dat10(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "desc DAT11."]
    #[inline(always)]
    pub const fn dat11(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "desc DAT12."]
    #[inline(always)]
    pub const fn dat12(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "desc DAT13."]
    #[inline(always)]
    pub const fn dat13(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "desc DAT14."]
    #[inline(always)]
    pub const fn dat14(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "desc DAT15."]
    #[inline(always)]
    pub const fn dat15(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "desc DAT16."]
    #[inline(always)]
    pub const fn dat16(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "desc DAT17."]
    #[inline(always)]
    pub const fn dat17(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "desc DAT18."]
    #[inline(always)]
    pub const fn dat18(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "desc DAT19."]
    #[inline(always)]
    pub const fn dat19(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "desc DAT20."]
    #[inline(always)]
    pub const fn dat20(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "desc DAT21."]
    #[inline(always)]
    pub const fn dat21(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "desc DAT22."]
    #[inline(always)]
    pub const fn dat22(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "desc DAT23."]
    #[inline(always)]
    pub const fn dat23(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "desc DAT24."]
    #[inline(always)]
    pub const fn dat24(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "desc DAT25."]
    #[inline(always)]
    pub const fn dat25(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "desc DAT26."]
    #[inline(always)]
    pub const fn dat26(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "desc DAT27."]
    #[inline(always)]
    pub const fn dat27(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "desc DAT28."]
    #[inline(always)]
    pub const fn dat28(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "desc DAT29."]
    #[inline(always)]
    pub const fn dat29(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "desc DAT30."]
    #[inline(always)]
    pub const fn dat30(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "desc DAT31."]
    #[inline(always)]
    pub const fn dat31(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc CR."]
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CR."]
        #[inline(always)]
        pub const fn set_cr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FLAG."]
        #[must_use]
        #[inline(always)]
        pub const fn flag(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLAG."]
        #[inline(always)]
        pub const fn set_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SELECT."]
        #[must_use]
        #[inline(always)]
        pub const fn select(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "desc SELECT."]
        #[inline(always)]
        pub const fn set_select(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("cr", &self.cr())
                .field("flag", &self.flag())
                .field("select", &self.select())
                .finish()
        }
    }
}
