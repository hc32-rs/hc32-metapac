#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc FCM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcm {
    ptr: *mut u8,
}
unsafe impl Send for Fcm {}
unsafe impl Sync for Fcm {}
impl Fcm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc LVR."]
    #[inline(always)]
    pub const fn lvr(self) -> crate::common::Reg<regs::Lvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc UVR."]
    #[inline(always)]
    pub const fn uvr(self) -> crate::common::Reg<regs::Uvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CNTR."]
    #[inline(always)]
    pub const fn cntr(self) -> crate::common::Reg<regs::Cntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc STR."]
    #[inline(always)]
    pub const fn str(self) -> crate::common::Reg<regs::Str, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc MCCR."]
    #[inline(always)]
    pub const fn mccr(self) -> crate::common::Reg<regs::Mccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc RCCR."]
    #[inline(always)]
    pub const fn rccr(self) -> crate::common::Reg<regs::Rccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc RIER."]
    #[inline(always)]
    pub const fn rier(self) -> crate::common::Reg<regs::Rier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc SR."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc CLR."]
    #[inline(always)]
    pub const fn clr(self) -> crate::common::Reg<regs::Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clr(pub u32);
    impl Clr {
        #[doc = "desc ERRFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn errfclr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERRFCLR."]
        #[inline(always)]
        pub const fn set_errfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc MENDFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn mendfclr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc MENDFCLR."]
        #[inline(always)]
        pub const fn set_mendfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc OVFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfclr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVFCLR."]
        #[inline(always)]
        pub const fn set_ovfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Clr {
        #[inline(always)]
        fn default() -> Clr {
            Clr(0)
        }
    }
    impl core::fmt::Debug for Clr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clr")
                .field("errfclr", &self.errfclr())
                .field("mendfclr", &self.mendfclr())
                .field("ovfclr", &self.ovfclr())
                .finish()
        }
    }
    #[doc = "desc CNTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntr(pub u32);
    impl Cntr {
        #[doc = "desc CNTR."]
        #[must_use]
        #[inline(always)]
        pub const fn cntr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNTR."]
        #[inline(always)]
        pub const fn set_cntr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cntr {
        #[inline(always)]
        fn default() -> Cntr {
            Cntr(0)
        }
    }
    impl core::fmt::Debug for Cntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cntr").field("cntr", &self.cntr()).finish()
        }
    }
    #[doc = "desc LVR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lvr(pub u32);
    impl Lvr {
        #[doc = "desc LVR."]
        #[must_use]
        #[inline(always)]
        pub const fn lvr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc LVR."]
        #[inline(always)]
        pub const fn set_lvr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Lvr {
        #[inline(always)]
        fn default() -> Lvr {
            Lvr(0)
        }
    }
    impl core::fmt::Debug for Lvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lvr").field("lvr", &self.lvr()).finish()
        }
    }
    #[doc = "desc MCCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mccr(pub u32);
    impl Mccr {
        #[doc = "desc MDIVS."]
        #[must_use]
        #[inline(always)]
        pub const fn mdivs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc MDIVS."]
        #[inline(always)]
        pub const fn set_mdivs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc MCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn mcks(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MCKS."]
        #[inline(always)]
        pub const fn set_mcks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Mccr {
        #[inline(always)]
        fn default() -> Mccr {
            Mccr(0)
        }
    }
    impl core::fmt::Debug for Mccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mccr")
                .field("mdivs", &self.mdivs())
                .field("mcks", &self.mcks())
                .finish()
        }
    }
    #[doc = "desc RCCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccr(pub u32);
    impl Rccr {
        #[doc = "desc RDIVS."]
        #[must_use]
        #[inline(always)]
        pub const fn rdivs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc RDIVS."]
        #[inline(always)]
        pub const fn set_rdivs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc RCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn rcks(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "desc RCKS."]
        #[inline(always)]
        pub const fn set_rcks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "desc INEXS."]
        #[must_use]
        #[inline(always)]
        pub const fn inexs(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEXS."]
        #[inline(always)]
        pub const fn set_inexs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc DNFS."]
        #[must_use]
        #[inline(always)]
        pub const fn dnfs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc DNFS."]
        #[inline(always)]
        pub const fn set_dnfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc EDGES."]
        #[must_use]
        #[inline(always)]
        pub const fn edges(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc EDGES."]
        #[inline(always)]
        pub const fn set_edges(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc EXREFE."]
        #[must_use]
        #[inline(always)]
        pub const fn exrefe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EXREFE."]
        #[inline(always)]
        pub const fn set_exrefe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Rccr {
        #[inline(always)]
        fn default() -> Rccr {
            Rccr(0)
        }
    }
    impl core::fmt::Debug for Rccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccr")
                .field("rdivs", &self.rdivs())
                .field("rcks", &self.rcks())
                .field("inexs", &self.inexs())
                .field("dnfs", &self.dnfs())
                .field("edges", &self.edges())
                .field("exrefe", &self.exrefe())
                .finish()
        }
    }
    #[doc = "desc RIER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rier(pub u32);
    impl Rier {
        #[doc = "desc ERRIE."]
        #[must_use]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERRIE."]
        #[inline(always)]
        pub const fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc MENDIE."]
        #[must_use]
        #[inline(always)]
        pub const fn mendie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc MENDIE."]
        #[inline(always)]
        pub const fn set_mendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc OVFIE."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVFIE."]
        #[inline(always)]
        pub const fn set_ovfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc ERRINTRS."]
        #[must_use]
        #[inline(always)]
        pub const fn errintrs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERRINTRS."]
        #[inline(always)]
        pub const fn set_errintrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ERRRE."]
        #[must_use]
        #[inline(always)]
        pub const fn errre(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERRRE."]
        #[inline(always)]
        pub const fn set_errre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Rier {
        #[inline(always)]
        fn default() -> Rier {
            Rier(0)
        }
    }
    impl core::fmt::Debug for Rier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rier")
                .field("errie", &self.errie())
                .field("mendie", &self.mendie())
                .field("ovfie", &self.ovfie())
                .field("errintrs", &self.errintrs())
                .field("errre", &self.errre())
                .finish()
        }
    }
    #[doc = "desc SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "desc ERRF."]
        #[must_use]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERRF."]
        #[inline(always)]
        pub const fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc MENDF."]
        #[must_use]
        #[inline(always)]
        pub const fn mendf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc MENDF."]
        #[inline(always)]
        pub const fn set_mendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc OVF."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF."]
        #[inline(always)]
        pub const fn set_ovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("errf", &self.errf())
                .field("mendf", &self.mendf())
                .field("ovf", &self.ovf())
                .finish()
        }
    }
    #[doc = "desc STR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Str(pub u32);
    impl Str {
        #[doc = "desc START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("Str").field("start", &self.start()).finish()
        }
    }
    #[doc = "desc UVR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uvr(pub u32);
    impl Uvr {
        #[doc = "desc UVR."]
        #[must_use]
        #[inline(always)]
        pub const fn uvr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc UVR."]
        #[inline(always)]
        pub const fn set_uvr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Uvr {
        #[inline(always)]
        fn default() -> Uvr {
            Uvr(0)
        }
    }
    impl core::fmt::Debug for Uvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uvr").field("uvr", &self.uvr()).finish()
        }
    }
}
