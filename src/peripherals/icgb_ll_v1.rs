#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc ICGB."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icgb {
    ptr: *mut u8,
}
unsafe impl Send for Icgb {}
unsafe impl Sync for Icgb {}
impl Icgb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc ICGB0LLA."]
    #[inline(always)]
    pub const fn icgb0lla(self) -> crate::common::Reg<regs::Icgb0lla, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc ICGB0LLB."]
    #[inline(always)]
    pub const fn icgb0llb(self) -> crate::common::Reg<regs::Icgb0llb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "desc ICGB0LLC."]
    #[inline(always)]
    pub const fn icgb0llc(self) -> crate::common::Reg<regs::Icgb0llc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "desc ICGB0LHA."]
    #[inline(always)]
    pub const fn icgb0lha(self) -> crate::common::Reg<regs::Icgb0lha, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc ICGB0LHB."]
    #[inline(always)]
    pub const fn icgb0lhb(self) -> crate::common::Reg<regs::Icgb0lhb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "desc ICGB0LHC."]
    #[inline(always)]
    pub const fn icgb0lhc(self) -> crate::common::Reg<regs::Icgb0lhc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "desc ICGB0HLA."]
    #[inline(always)]
    pub const fn icgb0hla(self) -> crate::common::Reg<regs::Icgb0hla, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc ICGB0HLB."]
    #[inline(always)]
    pub const fn icgb0hlb(self) -> crate::common::Reg<regs::Icgb0hlb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "desc ICGB0HLC."]
    #[inline(always)]
    pub const fn icgb0hlc(self) -> crate::common::Reg<regs::Icgb0hlc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "desc ICGB0HHA."]
    #[inline(always)]
    pub const fn icgb0hha(self) -> crate::common::Reg<regs::Icgb0hha, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc ICGB0HHB."]
    #[inline(always)]
    pub const fn icgb0hhb(self) -> crate::common::Reg<regs::Icgb0hhb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "desc ICGB0HHC."]
    #[inline(always)]
    pub const fn icgb0hhc(self) -> crate::common::Reg<regs::Icgb0hhc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "desc ICGB1LLA."]
    #[inline(always)]
    pub const fn icgb1lla(self) -> crate::common::Reg<regs::Icgb1lla, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc ICGB1LLB."]
    #[inline(always)]
    pub const fn icgb1llb(self) -> crate::common::Reg<regs::Icgb1llb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11usize) as _) }
    }
    #[doc = "desc ICGB1LLC."]
    #[inline(always)]
    pub const fn icgb1llc(self) -> crate::common::Reg<regs::Icgb1llc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "desc ICGB1LHA."]
    #[inline(always)]
    pub const fn icgb1lha(self) -> crate::common::Reg<regs::Icgb1lha, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc ICGB1LHB."]
    #[inline(always)]
    pub const fn icgb1lhb(self) -> crate::common::Reg<regs::Icgb1lhb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15usize) as _) }
    }
    #[doc = "desc ICGB1LHC."]
    #[inline(always)]
    pub const fn icgb1lhc(self) -> crate::common::Reg<regs::Icgb1lhc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "desc ICGB1HLA."]
    #[inline(always)]
    pub const fn icgb1hla(self) -> crate::common::Reg<regs::Icgb1hla, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc ICGB1HLB."]
    #[inline(always)]
    pub const fn icgb1hlb(self) -> crate::common::Reg<regs::Icgb1hlb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x19usize) as _) }
    }
    #[doc = "desc ICGB1HLC."]
    #[inline(always)]
    pub const fn icgb1hlc(self) -> crate::common::Reg<regs::Icgb1hlc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "desc ICGB1HHA."]
    #[inline(always)]
    pub const fn icgb1hha(self) -> crate::common::Reg<regs::Icgb1hha, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc ICGB1HHB."]
    #[inline(always)]
    pub const fn icgb1hhb(self) -> crate::common::Reg<regs::Icgb1hhb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "desc ICGB1HHC."]
    #[inline(always)]
    pub const fn icgb1hhc(self) -> crate::common::Reg<regs::Icgb1hhc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ICGB0HHA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb0hha(pub u8);
    impl Icgb0hha {
        #[doc = "desc FNWPRT24."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt24(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT24."]
        #[inline(always)]
        pub const fn set_fnwprt24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT25."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt25(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT25."]
        #[inline(always)]
        pub const fn set_fnwprt25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT26."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt26(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT26."]
        #[inline(always)]
        pub const fn set_fnwprt26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT27."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt27(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT27."]
        #[inline(always)]
        pub const fn set_fnwprt27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT28."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt28(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT28."]
        #[inline(always)]
        pub const fn set_fnwprt28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT29."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt29(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT29."]
        #[inline(always)]
        pub const fn set_fnwprt29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT30."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt30(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT30."]
        #[inline(always)]
        pub const fn set_fnwprt30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT31."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt31(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT31."]
        #[inline(always)]
        pub const fn set_fnwprt31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb0hha {
        #[inline(always)]
        fn default() -> Icgb0hha {
            Icgb0hha(0)
        }
    }
    impl core::fmt::Debug for Icgb0hha {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb0hha")
                .field("fnwprt24", &self.fnwprt24())
                .field("fnwprt25", &self.fnwprt25())
                .field("fnwprt26", &self.fnwprt26())
                .field("fnwprt27", &self.fnwprt27())
                .field("fnwprt28", &self.fnwprt28())
                .field("fnwprt29", &self.fnwprt29())
                .field("fnwprt30", &self.fnwprt30())
                .field("fnwprt31", &self.fnwprt31())
                .finish()
        }
    }
    #[doc = "desc ICGB0HHB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb0hhb(pub u8);
    impl Icgb0hhb {
        #[doc = "desc FNWPRT24."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt24(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT24."]
        #[inline(always)]
        pub const fn set_fnwprt24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT25."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt25(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT25."]
        #[inline(always)]
        pub const fn set_fnwprt25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT26."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt26(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT26."]
        #[inline(always)]
        pub const fn set_fnwprt26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT27."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt27(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT27."]
        #[inline(always)]
        pub const fn set_fnwprt27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT28."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt28(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT28."]
        #[inline(always)]
        pub const fn set_fnwprt28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT29."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt29(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT29."]
        #[inline(always)]
        pub const fn set_fnwprt29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT30."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt30(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT30."]
        #[inline(always)]
        pub const fn set_fnwprt30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT31."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt31(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT31."]
        #[inline(always)]
        pub const fn set_fnwprt31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb0hhb {
        #[inline(always)]
        fn default() -> Icgb0hhb {
            Icgb0hhb(0)
        }
    }
    impl core::fmt::Debug for Icgb0hhb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb0hhb")
                .field("fnwprt24", &self.fnwprt24())
                .field("fnwprt25", &self.fnwprt25())
                .field("fnwprt26", &self.fnwprt26())
                .field("fnwprt27", &self.fnwprt27())
                .field("fnwprt28", &self.fnwprt28())
                .field("fnwprt29", &self.fnwprt29())
                .field("fnwprt30", &self.fnwprt30())
                .field("fnwprt31", &self.fnwprt31())
                .finish()
        }
    }
    #[doc = "desc ICGB0HHC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb0hhc(pub u8);
    impl Icgb0hhc {
        #[doc = "desc FNWPRT24."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt24(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT24."]
        #[inline(always)]
        pub const fn set_fnwprt24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT25."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt25(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT25."]
        #[inline(always)]
        pub const fn set_fnwprt25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT26."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt26(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT26."]
        #[inline(always)]
        pub const fn set_fnwprt26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT27."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt27(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT27."]
        #[inline(always)]
        pub const fn set_fnwprt27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT28."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt28(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT28."]
        #[inline(always)]
        pub const fn set_fnwprt28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT29."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt29(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT29."]
        #[inline(always)]
        pub const fn set_fnwprt29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT30."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt30(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT30."]
        #[inline(always)]
        pub const fn set_fnwprt30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT31."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt31(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT31."]
        #[inline(always)]
        pub const fn set_fnwprt31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb0hhc {
        #[inline(always)]
        fn default() -> Icgb0hhc {
            Icgb0hhc(0)
        }
    }
    impl core::fmt::Debug for Icgb0hhc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb0hhc")
                .field("fnwprt24", &self.fnwprt24())
                .field("fnwprt25", &self.fnwprt25())
                .field("fnwprt26", &self.fnwprt26())
                .field("fnwprt27", &self.fnwprt27())
                .field("fnwprt28", &self.fnwprt28())
                .field("fnwprt29", &self.fnwprt29())
                .field("fnwprt30", &self.fnwprt30())
                .field("fnwprt31", &self.fnwprt31())
                .finish()
        }
    }
    #[doc = "desc ICGB0HLA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb0hla(pub u8);
    impl Icgb0hla {
        #[doc = "desc FNWPRT16."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt16(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT16."]
        #[inline(always)]
        pub const fn set_fnwprt16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT17."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt17(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT17."]
        #[inline(always)]
        pub const fn set_fnwprt17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT18."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt18(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT18."]
        #[inline(always)]
        pub const fn set_fnwprt18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT19."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt19(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT19."]
        #[inline(always)]
        pub const fn set_fnwprt19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT20."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt20(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT20."]
        #[inline(always)]
        pub const fn set_fnwprt20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT21."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt21(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT21."]
        #[inline(always)]
        pub const fn set_fnwprt21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT22."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt22(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT22."]
        #[inline(always)]
        pub const fn set_fnwprt22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT23."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt23(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT23."]
        #[inline(always)]
        pub const fn set_fnwprt23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb0hla {
        #[inline(always)]
        fn default() -> Icgb0hla {
            Icgb0hla(0)
        }
    }
    impl core::fmt::Debug for Icgb0hla {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb0hla")
                .field("fnwprt16", &self.fnwprt16())
                .field("fnwprt17", &self.fnwprt17())
                .field("fnwprt18", &self.fnwprt18())
                .field("fnwprt19", &self.fnwprt19())
                .field("fnwprt20", &self.fnwprt20())
                .field("fnwprt21", &self.fnwprt21())
                .field("fnwprt22", &self.fnwprt22())
                .field("fnwprt23", &self.fnwprt23())
                .finish()
        }
    }
    #[doc = "desc ICGB0HLB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb0hlb(pub u8);
    impl Icgb0hlb {
        #[doc = "desc FNWPRT16."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt16(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT16."]
        #[inline(always)]
        pub const fn set_fnwprt16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT17."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt17(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT17."]
        #[inline(always)]
        pub const fn set_fnwprt17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT18."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt18(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT18."]
        #[inline(always)]
        pub const fn set_fnwprt18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT19."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt19(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT19."]
        #[inline(always)]
        pub const fn set_fnwprt19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT20."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt20(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT20."]
        #[inline(always)]
        pub const fn set_fnwprt20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT21."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt21(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT21."]
        #[inline(always)]
        pub const fn set_fnwprt21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT22."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt22(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT22."]
        #[inline(always)]
        pub const fn set_fnwprt22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT23."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt23(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT23."]
        #[inline(always)]
        pub const fn set_fnwprt23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb0hlb {
        #[inline(always)]
        fn default() -> Icgb0hlb {
            Icgb0hlb(0)
        }
    }
    impl core::fmt::Debug for Icgb0hlb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb0hlb")
                .field("fnwprt16", &self.fnwprt16())
                .field("fnwprt17", &self.fnwprt17())
                .field("fnwprt18", &self.fnwprt18())
                .field("fnwprt19", &self.fnwprt19())
                .field("fnwprt20", &self.fnwprt20())
                .field("fnwprt21", &self.fnwprt21())
                .field("fnwprt22", &self.fnwprt22())
                .field("fnwprt23", &self.fnwprt23())
                .finish()
        }
    }
    #[doc = "desc ICGB0HLC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb0hlc(pub u8);
    impl Icgb0hlc {
        #[doc = "desc FNWPRT16."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt16(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT16."]
        #[inline(always)]
        pub const fn set_fnwprt16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT17."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt17(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT17."]
        #[inline(always)]
        pub const fn set_fnwprt17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT18."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt18(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT18."]
        #[inline(always)]
        pub const fn set_fnwprt18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT19."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt19(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT19."]
        #[inline(always)]
        pub const fn set_fnwprt19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT20."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt20(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT20."]
        #[inline(always)]
        pub const fn set_fnwprt20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT21."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt21(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT21."]
        #[inline(always)]
        pub const fn set_fnwprt21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT22."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt22(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT22."]
        #[inline(always)]
        pub const fn set_fnwprt22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT23."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt23(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT23."]
        #[inline(always)]
        pub const fn set_fnwprt23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb0hlc {
        #[inline(always)]
        fn default() -> Icgb0hlc {
            Icgb0hlc(0)
        }
    }
    impl core::fmt::Debug for Icgb0hlc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb0hlc")
                .field("fnwprt16", &self.fnwprt16())
                .field("fnwprt17", &self.fnwprt17())
                .field("fnwprt18", &self.fnwprt18())
                .field("fnwprt19", &self.fnwprt19())
                .field("fnwprt20", &self.fnwprt20())
                .field("fnwprt21", &self.fnwprt21())
                .field("fnwprt22", &self.fnwprt22())
                .field("fnwprt23", &self.fnwprt23())
                .finish()
        }
    }
    #[doc = "desc ICGB0LHA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb0lha(pub u8);
    impl Icgb0lha {
        #[doc = "desc FNWPRT8."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt8(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT8."]
        #[inline(always)]
        pub const fn set_fnwprt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT9."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt9(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT9."]
        #[inline(always)]
        pub const fn set_fnwprt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT10."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt10(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT10."]
        #[inline(always)]
        pub const fn set_fnwprt10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT11."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt11(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT11."]
        #[inline(always)]
        pub const fn set_fnwprt11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT12."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt12(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT12."]
        #[inline(always)]
        pub const fn set_fnwprt12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT13."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt13(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT13."]
        #[inline(always)]
        pub const fn set_fnwprt13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT14."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt14(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT14."]
        #[inline(always)]
        pub const fn set_fnwprt14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT15."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt15(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT15."]
        #[inline(always)]
        pub const fn set_fnwprt15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb0lha {
        #[inline(always)]
        fn default() -> Icgb0lha {
            Icgb0lha(0)
        }
    }
    impl core::fmt::Debug for Icgb0lha {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb0lha")
                .field("fnwprt8", &self.fnwprt8())
                .field("fnwprt9", &self.fnwprt9())
                .field("fnwprt10", &self.fnwprt10())
                .field("fnwprt11", &self.fnwprt11())
                .field("fnwprt12", &self.fnwprt12())
                .field("fnwprt13", &self.fnwprt13())
                .field("fnwprt14", &self.fnwprt14())
                .field("fnwprt15", &self.fnwprt15())
                .finish()
        }
    }
    #[doc = "desc ICGB0LHB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb0lhb(pub u8);
    impl Icgb0lhb {
        #[doc = "desc FNWPRT8."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt8(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT8."]
        #[inline(always)]
        pub const fn set_fnwprt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT9."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt9(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT9."]
        #[inline(always)]
        pub const fn set_fnwprt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT10."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt10(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT10."]
        #[inline(always)]
        pub const fn set_fnwprt10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT11."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt11(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT11."]
        #[inline(always)]
        pub const fn set_fnwprt11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT12."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt12(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT12."]
        #[inline(always)]
        pub const fn set_fnwprt12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT13."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt13(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT13."]
        #[inline(always)]
        pub const fn set_fnwprt13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT14."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt14(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT14."]
        #[inline(always)]
        pub const fn set_fnwprt14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT15."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt15(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT15."]
        #[inline(always)]
        pub const fn set_fnwprt15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb0lhb {
        #[inline(always)]
        fn default() -> Icgb0lhb {
            Icgb0lhb(0)
        }
    }
    impl core::fmt::Debug for Icgb0lhb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb0lhb")
                .field("fnwprt8", &self.fnwprt8())
                .field("fnwprt9", &self.fnwprt9())
                .field("fnwprt10", &self.fnwprt10())
                .field("fnwprt11", &self.fnwprt11())
                .field("fnwprt12", &self.fnwprt12())
                .field("fnwprt13", &self.fnwprt13())
                .field("fnwprt14", &self.fnwprt14())
                .field("fnwprt15", &self.fnwprt15())
                .finish()
        }
    }
    #[doc = "desc ICGB0LHC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb0lhc(pub u8);
    impl Icgb0lhc {
        #[doc = "desc FNWPRT8."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt8(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT8."]
        #[inline(always)]
        pub const fn set_fnwprt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT9."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt9(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT9."]
        #[inline(always)]
        pub const fn set_fnwprt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT10."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt10(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT10."]
        #[inline(always)]
        pub const fn set_fnwprt10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT11."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt11(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT11."]
        #[inline(always)]
        pub const fn set_fnwprt11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT12."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt12(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT12."]
        #[inline(always)]
        pub const fn set_fnwprt12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT13."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt13(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT13."]
        #[inline(always)]
        pub const fn set_fnwprt13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT14."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt14(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT14."]
        #[inline(always)]
        pub const fn set_fnwprt14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT15."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt15(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT15."]
        #[inline(always)]
        pub const fn set_fnwprt15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb0lhc {
        #[inline(always)]
        fn default() -> Icgb0lhc {
            Icgb0lhc(0)
        }
    }
    impl core::fmt::Debug for Icgb0lhc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb0lhc")
                .field("fnwprt8", &self.fnwprt8())
                .field("fnwprt9", &self.fnwprt9())
                .field("fnwprt10", &self.fnwprt10())
                .field("fnwprt11", &self.fnwprt11())
                .field("fnwprt12", &self.fnwprt12())
                .field("fnwprt13", &self.fnwprt13())
                .field("fnwprt14", &self.fnwprt14())
                .field("fnwprt15", &self.fnwprt15())
                .finish()
        }
    }
    #[doc = "desc ICGB0LLA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb0lla(pub u8);
    impl Icgb0lla {
        #[doc = "desc FNWPRT0."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT0."]
        #[inline(always)]
        pub const fn set_fnwprt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT1."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT1."]
        #[inline(always)]
        pub const fn set_fnwprt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT2."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT2."]
        #[inline(always)]
        pub const fn set_fnwprt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT3."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT3."]
        #[inline(always)]
        pub const fn set_fnwprt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT4."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT4."]
        #[inline(always)]
        pub const fn set_fnwprt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT5."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT5."]
        #[inline(always)]
        pub const fn set_fnwprt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT6."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT6."]
        #[inline(always)]
        pub const fn set_fnwprt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT7."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT7."]
        #[inline(always)]
        pub const fn set_fnwprt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb0lla {
        #[inline(always)]
        fn default() -> Icgb0lla {
            Icgb0lla(0)
        }
    }
    impl core::fmt::Debug for Icgb0lla {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb0lla")
                .field("fnwprt0", &self.fnwprt0())
                .field("fnwprt1", &self.fnwprt1())
                .field("fnwprt2", &self.fnwprt2())
                .field("fnwprt3", &self.fnwprt3())
                .field("fnwprt4", &self.fnwprt4())
                .field("fnwprt5", &self.fnwprt5())
                .field("fnwprt6", &self.fnwprt6())
                .field("fnwprt7", &self.fnwprt7())
                .finish()
        }
    }
    #[doc = "desc ICGB0LLB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb0llb(pub u8);
    impl Icgb0llb {
        #[doc = "desc FNWPRT0."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT0."]
        #[inline(always)]
        pub const fn set_fnwprt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT1."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT1."]
        #[inline(always)]
        pub const fn set_fnwprt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT2."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT2."]
        #[inline(always)]
        pub const fn set_fnwprt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT3."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT3."]
        #[inline(always)]
        pub const fn set_fnwprt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT4."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT4."]
        #[inline(always)]
        pub const fn set_fnwprt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT5."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT5."]
        #[inline(always)]
        pub const fn set_fnwprt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT6."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT6."]
        #[inline(always)]
        pub const fn set_fnwprt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT7."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT7."]
        #[inline(always)]
        pub const fn set_fnwprt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb0llb {
        #[inline(always)]
        fn default() -> Icgb0llb {
            Icgb0llb(0)
        }
    }
    impl core::fmt::Debug for Icgb0llb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb0llb")
                .field("fnwprt0", &self.fnwprt0())
                .field("fnwprt1", &self.fnwprt1())
                .field("fnwprt2", &self.fnwprt2())
                .field("fnwprt3", &self.fnwprt3())
                .field("fnwprt4", &self.fnwprt4())
                .field("fnwprt5", &self.fnwprt5())
                .field("fnwprt6", &self.fnwprt6())
                .field("fnwprt7", &self.fnwprt7())
                .finish()
        }
    }
    #[doc = "desc ICGB0LLC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb0llc(pub u8);
    impl Icgb0llc {
        #[doc = "desc FNWPRT0."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT0."]
        #[inline(always)]
        pub const fn set_fnwprt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT1."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT1."]
        #[inline(always)]
        pub const fn set_fnwprt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT2."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT2."]
        #[inline(always)]
        pub const fn set_fnwprt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT3."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT3."]
        #[inline(always)]
        pub const fn set_fnwprt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT4."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT4."]
        #[inline(always)]
        pub const fn set_fnwprt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT5."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT5."]
        #[inline(always)]
        pub const fn set_fnwprt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT6."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT6."]
        #[inline(always)]
        pub const fn set_fnwprt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT7."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT7."]
        #[inline(always)]
        pub const fn set_fnwprt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb0llc {
        #[inline(always)]
        fn default() -> Icgb0llc {
            Icgb0llc(0)
        }
    }
    impl core::fmt::Debug for Icgb0llc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb0llc")
                .field("fnwprt0", &self.fnwprt0())
                .field("fnwprt1", &self.fnwprt1())
                .field("fnwprt2", &self.fnwprt2())
                .field("fnwprt3", &self.fnwprt3())
                .field("fnwprt4", &self.fnwprt4())
                .field("fnwprt5", &self.fnwprt5())
                .field("fnwprt6", &self.fnwprt6())
                .field("fnwprt7", &self.fnwprt7())
                .finish()
        }
    }
    #[doc = "desc ICGB1HHA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb1hha(pub u8);
    impl Icgb1hha {
        #[doc = "desc ENWPRT24."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt24(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT24."]
        #[inline(always)]
        pub const fn set_enwprt24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc ENWPRT25."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt25(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT25."]
        #[inline(always)]
        pub const fn set_enwprt25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ENWPRT26."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt26(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT26."]
        #[inline(always)]
        pub const fn set_enwprt26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ENWPRT27."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt27(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT27."]
        #[inline(always)]
        pub const fn set_enwprt27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ENWPRT28."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt28(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT28."]
        #[inline(always)]
        pub const fn set_enwprt28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ENWPRT29."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt29(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT29."]
        #[inline(always)]
        pub const fn set_enwprt29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ENWPRT30."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt30(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT30."]
        #[inline(always)]
        pub const fn set_enwprt30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ENWPRT31."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt31(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT31."]
        #[inline(always)]
        pub const fn set_enwprt31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb1hha {
        #[inline(always)]
        fn default() -> Icgb1hha {
            Icgb1hha(0)
        }
    }
    impl core::fmt::Debug for Icgb1hha {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb1hha")
                .field("enwprt24", &self.enwprt24())
                .field("enwprt25", &self.enwprt25())
                .field("enwprt26", &self.enwprt26())
                .field("enwprt27", &self.enwprt27())
                .field("enwprt28", &self.enwprt28())
                .field("enwprt29", &self.enwprt29())
                .field("enwprt30", &self.enwprt30())
                .field("enwprt31", &self.enwprt31())
                .finish()
        }
    }
    #[doc = "desc ICGB1HHB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb1hhb(pub u8);
    impl Icgb1hhb {
        #[doc = "desc ENWPRT24."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt24(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT24."]
        #[inline(always)]
        pub const fn set_enwprt24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc ENWPRT25."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt25(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT25."]
        #[inline(always)]
        pub const fn set_enwprt25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ENWPRT26."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt26(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT26."]
        #[inline(always)]
        pub const fn set_enwprt26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ENWPRT27."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt27(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT27."]
        #[inline(always)]
        pub const fn set_enwprt27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ENWPRT28."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt28(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT28."]
        #[inline(always)]
        pub const fn set_enwprt28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ENWPRT29."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt29(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT29."]
        #[inline(always)]
        pub const fn set_enwprt29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ENWPRT30."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt30(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT30."]
        #[inline(always)]
        pub const fn set_enwprt30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ENWPRT31."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt31(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT31."]
        #[inline(always)]
        pub const fn set_enwprt31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb1hhb {
        #[inline(always)]
        fn default() -> Icgb1hhb {
            Icgb1hhb(0)
        }
    }
    impl core::fmt::Debug for Icgb1hhb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb1hhb")
                .field("enwprt24", &self.enwprt24())
                .field("enwprt25", &self.enwprt25())
                .field("enwprt26", &self.enwprt26())
                .field("enwprt27", &self.enwprt27())
                .field("enwprt28", &self.enwprt28())
                .field("enwprt29", &self.enwprt29())
                .field("enwprt30", &self.enwprt30())
                .field("enwprt31", &self.enwprt31())
                .finish()
        }
    }
    #[doc = "desc ICGB1HHC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb1hhc(pub u8);
    impl Icgb1hhc {
        #[doc = "desc ENWPRT24."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt24(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT24."]
        #[inline(always)]
        pub const fn set_enwprt24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc ENWPRT25."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt25(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT25."]
        #[inline(always)]
        pub const fn set_enwprt25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ENWPRT26."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt26(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT26."]
        #[inline(always)]
        pub const fn set_enwprt26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ENWPRT27."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt27(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT27."]
        #[inline(always)]
        pub const fn set_enwprt27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ENWPRT28."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt28(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT28."]
        #[inline(always)]
        pub const fn set_enwprt28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ENWPRT29."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt29(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT29."]
        #[inline(always)]
        pub const fn set_enwprt29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ENWPRT30."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt30(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT30."]
        #[inline(always)]
        pub const fn set_enwprt30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ENWPRT31."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt31(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT31."]
        #[inline(always)]
        pub const fn set_enwprt31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb1hhc {
        #[inline(always)]
        fn default() -> Icgb1hhc {
            Icgb1hhc(0)
        }
    }
    impl core::fmt::Debug for Icgb1hhc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb1hhc")
                .field("enwprt24", &self.enwprt24())
                .field("enwprt25", &self.enwprt25())
                .field("enwprt26", &self.enwprt26())
                .field("enwprt27", &self.enwprt27())
                .field("enwprt28", &self.enwprt28())
                .field("enwprt29", &self.enwprt29())
                .field("enwprt30", &self.enwprt30())
                .field("enwprt31", &self.enwprt31())
                .finish()
        }
    }
    #[doc = "desc ICGB1HLA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb1hla(pub u8);
    impl Icgb1hla {
        #[doc = "desc ENWPRT16."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt16(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT16."]
        #[inline(always)]
        pub const fn set_enwprt16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc ENWPRT17."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt17(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT17."]
        #[inline(always)]
        pub const fn set_enwprt17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ENWPRT18."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt18(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT18."]
        #[inline(always)]
        pub const fn set_enwprt18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ENWPRT19."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt19(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT19."]
        #[inline(always)]
        pub const fn set_enwprt19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ENWPRT20."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt20(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT20."]
        #[inline(always)]
        pub const fn set_enwprt20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ENWPRT21."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt21(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT21."]
        #[inline(always)]
        pub const fn set_enwprt21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ENWPRT22."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt22(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT22."]
        #[inline(always)]
        pub const fn set_enwprt22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ENWPRT23."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt23(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT23."]
        #[inline(always)]
        pub const fn set_enwprt23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb1hla {
        #[inline(always)]
        fn default() -> Icgb1hla {
            Icgb1hla(0)
        }
    }
    impl core::fmt::Debug for Icgb1hla {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb1hla")
                .field("enwprt16", &self.enwprt16())
                .field("enwprt17", &self.enwprt17())
                .field("enwprt18", &self.enwprt18())
                .field("enwprt19", &self.enwprt19())
                .field("enwprt20", &self.enwprt20())
                .field("enwprt21", &self.enwprt21())
                .field("enwprt22", &self.enwprt22())
                .field("enwprt23", &self.enwprt23())
                .finish()
        }
    }
    #[doc = "desc ICGB1HLB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb1hlb(pub u8);
    impl Icgb1hlb {
        #[doc = "desc ENWPRT16."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt16(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT16."]
        #[inline(always)]
        pub const fn set_enwprt16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc ENWPRT17."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt17(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT17."]
        #[inline(always)]
        pub const fn set_enwprt17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ENWPRT18."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt18(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT18."]
        #[inline(always)]
        pub const fn set_enwprt18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ENWPRT19."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt19(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT19."]
        #[inline(always)]
        pub const fn set_enwprt19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ENWPRT20."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt20(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT20."]
        #[inline(always)]
        pub const fn set_enwprt20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ENWPRT21."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt21(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT21."]
        #[inline(always)]
        pub const fn set_enwprt21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ENWPRT22."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt22(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT22."]
        #[inline(always)]
        pub const fn set_enwprt22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ENWPRT23."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt23(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT23."]
        #[inline(always)]
        pub const fn set_enwprt23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb1hlb {
        #[inline(always)]
        fn default() -> Icgb1hlb {
            Icgb1hlb(0)
        }
    }
    impl core::fmt::Debug for Icgb1hlb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb1hlb")
                .field("enwprt16", &self.enwprt16())
                .field("enwprt17", &self.enwprt17())
                .field("enwprt18", &self.enwprt18())
                .field("enwprt19", &self.enwprt19())
                .field("enwprt20", &self.enwprt20())
                .field("enwprt21", &self.enwprt21())
                .field("enwprt22", &self.enwprt22())
                .field("enwprt23", &self.enwprt23())
                .finish()
        }
    }
    #[doc = "desc ICGB1HLC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb1hlc(pub u8);
    impl Icgb1hlc {
        #[doc = "desc ENWPRT16."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt16(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT16."]
        #[inline(always)]
        pub const fn set_enwprt16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc ENWPRT17."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt17(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT17."]
        #[inline(always)]
        pub const fn set_enwprt17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ENWPRT18."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt18(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT18."]
        #[inline(always)]
        pub const fn set_enwprt18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ENWPRT19."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt19(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT19."]
        #[inline(always)]
        pub const fn set_enwprt19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ENWPRT20."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt20(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT20."]
        #[inline(always)]
        pub const fn set_enwprt20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ENWPRT21."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt21(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT21."]
        #[inline(always)]
        pub const fn set_enwprt21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ENWPRT22."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt22(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT22."]
        #[inline(always)]
        pub const fn set_enwprt22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ENWPRT23."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt23(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT23."]
        #[inline(always)]
        pub const fn set_enwprt23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb1hlc {
        #[inline(always)]
        fn default() -> Icgb1hlc {
            Icgb1hlc(0)
        }
    }
    impl core::fmt::Debug for Icgb1hlc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb1hlc")
                .field("enwprt16", &self.enwprt16())
                .field("enwprt17", &self.enwprt17())
                .field("enwprt18", &self.enwprt18())
                .field("enwprt19", &self.enwprt19())
                .field("enwprt20", &self.enwprt20())
                .field("enwprt21", &self.enwprt21())
                .field("enwprt22", &self.enwprt22())
                .field("enwprt23", &self.enwprt23())
                .finish()
        }
    }
    #[doc = "desc ICGB1LHA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb1lha(pub u8);
    impl Icgb1lha {
        #[doc = "desc ENWPRT8."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt8(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT8."]
        #[inline(always)]
        pub const fn set_enwprt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc ENWPRT9."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt9(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT9."]
        #[inline(always)]
        pub const fn set_enwprt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ENWPRT10."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt10(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT10."]
        #[inline(always)]
        pub const fn set_enwprt10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ENWPRT11."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt11(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT11."]
        #[inline(always)]
        pub const fn set_enwprt11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ENWPRT12."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt12(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT12."]
        #[inline(always)]
        pub const fn set_enwprt12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ENWPRT13."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt13(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT13."]
        #[inline(always)]
        pub const fn set_enwprt13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ENWPRT14."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt14(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT14."]
        #[inline(always)]
        pub const fn set_enwprt14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ENWPRT15."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt15(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT15."]
        #[inline(always)]
        pub const fn set_enwprt15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb1lha {
        #[inline(always)]
        fn default() -> Icgb1lha {
            Icgb1lha(0)
        }
    }
    impl core::fmt::Debug for Icgb1lha {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb1lha")
                .field("enwprt8", &self.enwprt8())
                .field("enwprt9", &self.enwprt9())
                .field("enwprt10", &self.enwprt10())
                .field("enwprt11", &self.enwprt11())
                .field("enwprt12", &self.enwprt12())
                .field("enwprt13", &self.enwprt13())
                .field("enwprt14", &self.enwprt14())
                .field("enwprt15", &self.enwprt15())
                .finish()
        }
    }
    #[doc = "desc ICGB1LHB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb1lhb(pub u8);
    impl Icgb1lhb {
        #[doc = "desc ENWPRT8."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt8(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT8."]
        #[inline(always)]
        pub const fn set_enwprt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc ENWPRT9."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt9(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT9."]
        #[inline(always)]
        pub const fn set_enwprt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ENWPRT10."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt10(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT10."]
        #[inline(always)]
        pub const fn set_enwprt10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ENWPRT11."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt11(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT11."]
        #[inline(always)]
        pub const fn set_enwprt11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ENWPRT12."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt12(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT12."]
        #[inline(always)]
        pub const fn set_enwprt12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ENWPRT13."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt13(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT13."]
        #[inline(always)]
        pub const fn set_enwprt13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ENWPRT14."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt14(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT14."]
        #[inline(always)]
        pub const fn set_enwprt14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ENWPRT15."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt15(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT15."]
        #[inline(always)]
        pub const fn set_enwprt15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb1lhb {
        #[inline(always)]
        fn default() -> Icgb1lhb {
            Icgb1lhb(0)
        }
    }
    impl core::fmt::Debug for Icgb1lhb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb1lhb")
                .field("enwprt8", &self.enwprt8())
                .field("enwprt9", &self.enwprt9())
                .field("enwprt10", &self.enwprt10())
                .field("enwprt11", &self.enwprt11())
                .field("enwprt12", &self.enwprt12())
                .field("enwprt13", &self.enwprt13())
                .field("enwprt14", &self.enwprt14())
                .field("enwprt15", &self.enwprt15())
                .finish()
        }
    }
    #[doc = "desc ICGB1LHC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb1lhc(pub u8);
    impl Icgb1lhc {
        #[doc = "desc ENWPRT8."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt8(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT8."]
        #[inline(always)]
        pub const fn set_enwprt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc ENWPRT9."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt9(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT9."]
        #[inline(always)]
        pub const fn set_enwprt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ENWPRT10."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt10(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT10."]
        #[inline(always)]
        pub const fn set_enwprt10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ENWPRT11."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt11(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT11."]
        #[inline(always)]
        pub const fn set_enwprt11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ENWPRT12."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt12(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT12."]
        #[inline(always)]
        pub const fn set_enwprt12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ENWPRT13."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt13(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT13."]
        #[inline(always)]
        pub const fn set_enwprt13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ENWPRT14."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt14(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT14."]
        #[inline(always)]
        pub const fn set_enwprt14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ENWPRT15."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt15(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT15."]
        #[inline(always)]
        pub const fn set_enwprt15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb1lhc {
        #[inline(always)]
        fn default() -> Icgb1lhc {
            Icgb1lhc(0)
        }
    }
    impl core::fmt::Debug for Icgb1lhc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb1lhc")
                .field("enwprt8", &self.enwprt8())
                .field("enwprt9", &self.enwprt9())
                .field("enwprt10", &self.enwprt10())
                .field("enwprt11", &self.enwprt11())
                .field("enwprt12", &self.enwprt12())
                .field("enwprt13", &self.enwprt13())
                .field("enwprt14", &self.enwprt14())
                .field("enwprt15", &self.enwprt15())
                .finish()
        }
    }
    #[doc = "desc ICGB1LLA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb1lla(pub u8);
    impl Icgb1lla {
        #[doc = "desc ENWPRT0."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT0."]
        #[inline(always)]
        pub const fn set_enwprt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc ENWPRT1."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT1."]
        #[inline(always)]
        pub const fn set_enwprt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ENWPRT2."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT2."]
        #[inline(always)]
        pub const fn set_enwprt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ENWPRT3."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT3."]
        #[inline(always)]
        pub const fn set_enwprt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ENWPRT4."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT4."]
        #[inline(always)]
        pub const fn set_enwprt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ENWPRT5."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT5."]
        #[inline(always)]
        pub const fn set_enwprt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ENWPRT6."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT6."]
        #[inline(always)]
        pub const fn set_enwprt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ENWPRT7."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT7."]
        #[inline(always)]
        pub const fn set_enwprt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb1lla {
        #[inline(always)]
        fn default() -> Icgb1lla {
            Icgb1lla(0)
        }
    }
    impl core::fmt::Debug for Icgb1lla {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb1lla")
                .field("enwprt0", &self.enwprt0())
                .field("enwprt1", &self.enwprt1())
                .field("enwprt2", &self.enwprt2())
                .field("enwprt3", &self.enwprt3())
                .field("enwprt4", &self.enwprt4())
                .field("enwprt5", &self.enwprt5())
                .field("enwprt6", &self.enwprt6())
                .field("enwprt7", &self.enwprt7())
                .finish()
        }
    }
    #[doc = "desc ICGB1LLB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb1llb(pub u8);
    impl Icgb1llb {
        #[doc = "desc ENWPRT0."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT0."]
        #[inline(always)]
        pub const fn set_enwprt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc ENWPRT1."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT1."]
        #[inline(always)]
        pub const fn set_enwprt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ENWPRT2."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT2."]
        #[inline(always)]
        pub const fn set_enwprt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ENWPRT3."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT3."]
        #[inline(always)]
        pub const fn set_enwprt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ENWPRT4."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT4."]
        #[inline(always)]
        pub const fn set_enwprt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ENWPRT5."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT5."]
        #[inline(always)]
        pub const fn set_enwprt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ENWPRT6."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT6."]
        #[inline(always)]
        pub const fn set_enwprt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ENWPRT7."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT7."]
        #[inline(always)]
        pub const fn set_enwprt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb1llb {
        #[inline(always)]
        fn default() -> Icgb1llb {
            Icgb1llb(0)
        }
    }
    impl core::fmt::Debug for Icgb1llb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb1llb")
                .field("enwprt0", &self.enwprt0())
                .field("enwprt1", &self.enwprt1())
                .field("enwprt2", &self.enwprt2())
                .field("enwprt3", &self.enwprt3())
                .field("enwprt4", &self.enwprt4())
                .field("enwprt5", &self.enwprt5())
                .field("enwprt6", &self.enwprt6())
                .field("enwprt7", &self.enwprt7())
                .finish()
        }
    }
    #[doc = "desc ICGB1LLC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icgb1llc(pub u8);
    impl Icgb1llc {
        #[doc = "desc ENWPRT0."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT0."]
        #[inline(always)]
        pub const fn set_enwprt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc ENWPRT1."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT1."]
        #[inline(always)]
        pub const fn set_enwprt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ENWPRT2."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT2."]
        #[inline(always)]
        pub const fn set_enwprt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ENWPRT3."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT3."]
        #[inline(always)]
        pub const fn set_enwprt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ENWPRT4."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT4."]
        #[inline(always)]
        pub const fn set_enwprt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ENWPRT5."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT5."]
        #[inline(always)]
        pub const fn set_enwprt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ENWPRT6."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT6."]
        #[inline(always)]
        pub const fn set_enwprt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ENWPRT7."]
        #[must_use]
        #[inline(always)]
        pub const fn enwprt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENWPRT7."]
        #[inline(always)]
        pub const fn set_enwprt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Icgb1llc {
        #[inline(always)]
        fn default() -> Icgb1llc {
            Icgb1llc(0)
        }
    }
    impl core::fmt::Debug for Icgb1llc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icgb1llc")
                .field("enwprt0", &self.enwprt0())
                .field("enwprt1", &self.enwprt1())
                .field("enwprt2", &self.enwprt2())
                .field("enwprt3", &self.enwprt3())
                .field("enwprt4", &self.enwprt4())
                .field("enwprt5", &self.enwprt5())
                .field("enwprt6", &self.enwprt6())
                .field("enwprt7", &self.enwprt7())
                .finish()
        }
    }
}
