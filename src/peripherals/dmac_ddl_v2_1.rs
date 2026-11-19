#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc DMAC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac {
    ptr: *mut u8,
}
unsafe impl Send for Dmac {}
unsafe impl Sync for Dmac {}
impl Dmac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CONF."]
    #[inline(always)]
    pub const fn conf(self) -> crate::common::Reg<regs::Conf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CONFA0."]
    #[inline(always)]
    pub const fn confa0(self) -> crate::common::Reg<regs::Confa0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc CONFB0."]
    #[inline(always)]
    pub const fn confb0(self) -> crate::common::Reg<regs::Confb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc SRCADR0."]
    #[inline(always)]
    pub const fn srcadr0(self) -> crate::common::Reg<regs::Srcadr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc DSTADR0."]
    #[inline(always)]
    pub const fn dstadr0(self) -> crate::common::Reg<regs::Dstadr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc CONFA1."]
    #[inline(always)]
    pub const fn confa1(self) -> crate::common::Reg<regs::Confa1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc CONFB1."]
    #[inline(always)]
    pub const fn confb1(self) -> crate::common::Reg<regs::Confb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc SRCADR1."]
    #[inline(always)]
    pub const fn srcadr1(self) -> crate::common::Reg<regs::Srcadr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc DSTADR1."]
    #[inline(always)]
    pub const fn dstadr1(self) -> crate::common::Reg<regs::Dstadr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc CONFA2."]
    #[inline(always)]
    pub const fn confa2(self) -> crate::common::Reg<regs::Confa2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc CONFB2."]
    #[inline(always)]
    pub const fn confb2(self) -> crate::common::Reg<regs::Confb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc SRCADR2."]
    #[inline(always)]
    pub const fn srcadr2(self) -> crate::common::Reg<regs::Srcadr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc DSTADR2."]
    #[inline(always)]
    pub const fn dstadr2(self) -> crate::common::Reg<regs::Dstadr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc CONFA3."]
    #[inline(always)]
    pub const fn confa3(self) -> crate::common::Reg<regs::Confa3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc CONFB3."]
    #[inline(always)]
    pub const fn confb3(self) -> crate::common::Reg<regs::Confb3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc SRCADR3."]
    #[inline(always)]
    pub const fn srcadr3(self) -> crate::common::Reg<regs::Srcadr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc DSTADR3."]
    #[inline(always)]
    pub const fn dstadr3(self) -> crate::common::Reg<regs::Dstadr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc CONFA4."]
    #[inline(always)]
    pub const fn confa4(self) -> crate::common::Reg<regs::Confa4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc CONFB4."]
    #[inline(always)]
    pub const fn confb4(self) -> crate::common::Reg<regs::Confb4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc SRCADR4."]
    #[inline(always)]
    pub const fn srcadr4(self) -> crate::common::Reg<regs::Srcadr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc DSTADR4."]
    #[inline(always)]
    pub const fn dstadr4(self) -> crate::common::Reg<regs::Dstadr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc CONFA5."]
    #[inline(always)]
    pub const fn confa5(self) -> crate::common::Reg<regs::Confa5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc CONFB5."]
    #[inline(always)]
    pub const fn confb5(self) -> crate::common::Reg<regs::Confb5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "desc SRCADR5."]
    #[inline(always)]
    pub const fn srcadr5(self) -> crate::common::Reg<regs::Srcadr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "desc DSTADR5."]
    #[inline(always)]
    pub const fn dstadr5(self) -> crate::common::Reg<regs::Dstadr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CONF."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Conf(pub u32);
    impl Conf {
        #[doc = "desc HALT."]
        #[must_use]
        #[inline(always)]
        pub const fn halt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "desc HALT."]
        #[inline(always)]
        pub const fn set_halt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "desc PRIO."]
        #[must_use]
        #[inline(always)]
        pub const fn prio(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRIO."]
        #[inline(always)]
        pub const fn set_prio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Conf {
        #[inline(always)]
        fn default() -> Conf {
            Conf(0)
        }
    }
    impl core::fmt::Debug for Conf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Conf")
                .field("halt", &self.halt())
                .field("prio", &self.prio())
                .field("en", &self.en())
                .finish()
        }
    }
    #[doc = "desc CONFA0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confa0(pub u32);
    impl Confa0 {
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc BC."]
        #[must_use]
        #[inline(always)]
        pub const fn bc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc BC."]
        #[inline(always)]
        pub const fn set_bc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc TRI_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn tri_sel(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRI_SEL."]
        #[inline(always)]
        pub const fn set_tri_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ST."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ST."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc PAS."]
        #[must_use]
        #[inline(always)]
        pub const fn pas(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc PAS."]
        #[inline(always)]
        pub const fn set_pas(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc ENS."]
        #[must_use]
        #[inline(always)]
        pub const fn ens(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENS."]
        #[inline(always)]
        pub const fn set_ens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Confa0 {
        #[inline(always)]
        fn default() -> Confa0 {
            Confa0(0)
        }
    }
    impl core::fmt::Debug for Confa0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confa0")
                .field("tc", &self.tc())
                .field("bc", &self.bc())
                .field("tri_sel", &self.tri_sel())
                .field("st", &self.st())
                .field("pas", &self.pas())
                .field("ens", &self.ens())
                .finish()
        }
    }
    #[doc = "desc CONFA1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confa1(pub u32);
    impl Confa1 {
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc BC."]
        #[must_use]
        #[inline(always)]
        pub const fn bc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc BC."]
        #[inline(always)]
        pub const fn set_bc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc TRI_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn tri_sel(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRI_SEL."]
        #[inline(always)]
        pub const fn set_tri_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ST."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ST."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc PAS."]
        #[must_use]
        #[inline(always)]
        pub const fn pas(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc PAS."]
        #[inline(always)]
        pub const fn set_pas(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc ENS."]
        #[must_use]
        #[inline(always)]
        pub const fn ens(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENS."]
        #[inline(always)]
        pub const fn set_ens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Confa1 {
        #[inline(always)]
        fn default() -> Confa1 {
            Confa1(0)
        }
    }
    impl core::fmt::Debug for Confa1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confa1")
                .field("tc", &self.tc())
                .field("bc", &self.bc())
                .field("tri_sel", &self.tri_sel())
                .field("st", &self.st())
                .field("pas", &self.pas())
                .field("ens", &self.ens())
                .finish()
        }
    }
    #[doc = "desc CONFA2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confa2(pub u32);
    impl Confa2 {
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc BC."]
        #[must_use]
        #[inline(always)]
        pub const fn bc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc BC."]
        #[inline(always)]
        pub const fn set_bc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc TRI_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn tri_sel(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRI_SEL."]
        #[inline(always)]
        pub const fn set_tri_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ST."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ST."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc PAS."]
        #[must_use]
        #[inline(always)]
        pub const fn pas(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc PAS."]
        #[inline(always)]
        pub const fn set_pas(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc ENS."]
        #[must_use]
        #[inline(always)]
        pub const fn ens(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENS."]
        #[inline(always)]
        pub const fn set_ens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Confa2 {
        #[inline(always)]
        fn default() -> Confa2 {
            Confa2(0)
        }
    }
    impl core::fmt::Debug for Confa2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confa2")
                .field("tc", &self.tc())
                .field("bc", &self.bc())
                .field("tri_sel", &self.tri_sel())
                .field("st", &self.st())
                .field("pas", &self.pas())
                .field("ens", &self.ens())
                .finish()
        }
    }
    #[doc = "desc CONFA3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confa3(pub u32);
    impl Confa3 {
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc BC."]
        #[must_use]
        #[inline(always)]
        pub const fn bc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc BC."]
        #[inline(always)]
        pub const fn set_bc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc TRI_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn tri_sel(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRI_SEL."]
        #[inline(always)]
        pub const fn set_tri_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ST."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ST."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc PAS."]
        #[must_use]
        #[inline(always)]
        pub const fn pas(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc PAS."]
        #[inline(always)]
        pub const fn set_pas(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc ENS."]
        #[must_use]
        #[inline(always)]
        pub const fn ens(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENS."]
        #[inline(always)]
        pub const fn set_ens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Confa3 {
        #[inline(always)]
        fn default() -> Confa3 {
            Confa3(0)
        }
    }
    impl core::fmt::Debug for Confa3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confa3")
                .field("tc", &self.tc())
                .field("bc", &self.bc())
                .field("tri_sel", &self.tri_sel())
                .field("st", &self.st())
                .field("pas", &self.pas())
                .field("ens", &self.ens())
                .finish()
        }
    }
    #[doc = "desc CONFA4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confa4(pub u32);
    impl Confa4 {
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc BC."]
        #[must_use]
        #[inline(always)]
        pub const fn bc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc BC."]
        #[inline(always)]
        pub const fn set_bc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc TRI_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn tri_sel(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRI_SEL."]
        #[inline(always)]
        pub const fn set_tri_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ST."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ST."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc PAS."]
        #[must_use]
        #[inline(always)]
        pub const fn pas(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc PAS."]
        #[inline(always)]
        pub const fn set_pas(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc ENS."]
        #[must_use]
        #[inline(always)]
        pub const fn ens(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENS."]
        #[inline(always)]
        pub const fn set_ens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Confa4 {
        #[inline(always)]
        fn default() -> Confa4 {
            Confa4(0)
        }
    }
    impl core::fmt::Debug for Confa4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confa4")
                .field("tc", &self.tc())
                .field("bc", &self.bc())
                .field("tri_sel", &self.tri_sel())
                .field("st", &self.st())
                .field("pas", &self.pas())
                .field("ens", &self.ens())
                .finish()
        }
    }
    #[doc = "desc CONFA5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confa5(pub u32);
    impl Confa5 {
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc BC."]
        #[must_use]
        #[inline(always)]
        pub const fn bc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc BC."]
        #[inline(always)]
        pub const fn set_bc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc TRI_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn tri_sel(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRI_SEL."]
        #[inline(always)]
        pub const fn set_tri_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ST."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ST."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc PAS."]
        #[must_use]
        #[inline(always)]
        pub const fn pas(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc PAS."]
        #[inline(always)]
        pub const fn set_pas(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc ENS."]
        #[must_use]
        #[inline(always)]
        pub const fn ens(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENS."]
        #[inline(always)]
        pub const fn set_ens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Confa5 {
        #[inline(always)]
        fn default() -> Confa5 {
            Confa5(0)
        }
    }
    impl core::fmt::Debug for Confa5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confa5")
                .field("tc", &self.tc())
                .field("bc", &self.bc())
                .field("tri_sel", &self.tri_sel())
                .field("st", &self.st())
                .field("pas", &self.pas())
                .field("ens", &self.ens())
                .finish()
        }
    }
    #[doc = "desc CONFB0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confb0(pub u32);
    impl Confb0 {
        #[doc = "desc MSK."]
        #[must_use]
        #[inline(always)]
        pub const fn msk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSK."]
        #[inline(always)]
        pub const fn set_msk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc STAT."]
        #[must_use]
        #[inline(always)]
        pub const fn stat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc STAT."]
        #[inline(always)]
        pub const fn set_stat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc FIS_IE."]
        #[must_use]
        #[inline(always)]
        pub const fn fis_ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIS_IE."]
        #[inline(always)]
        pub const fn set_fis_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc ERR_IE."]
        #[must_use]
        #[inline(always)]
        pub const fn err_ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERR_IE."]
        #[inline(always)]
        pub const fn set_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc RD."]
        #[must_use]
        #[inline(always)]
        pub const fn rd(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc RD."]
        #[inline(always)]
        pub const fn set_rd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc RS."]
        #[must_use]
        #[inline(always)]
        pub const fn rs(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc RS."]
        #[inline(always)]
        pub const fn set_rs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc RC."]
        #[must_use]
        #[inline(always)]
        pub const fn rc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc RC."]
        #[inline(always)]
        pub const fn set_rc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FD."]
        #[must_use]
        #[inline(always)]
        pub const fn fd(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FD."]
        #[inline(always)]
        pub const fn set_fd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FS."]
        #[must_use]
        #[inline(always)]
        pub const fn fs(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FS."]
        #[inline(always)]
        pub const fn set_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc WIDTH."]
        #[must_use]
        #[inline(always)]
        pub const fn width(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc WIDTH."]
        #[inline(always)]
        pub const fn set_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Confb0 {
        #[inline(always)]
        fn default() -> Confb0 {
            Confb0(0)
        }
    }
    impl core::fmt::Debug for Confb0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confb0")
                .field("msk", &self.msk())
                .field("stat", &self.stat())
                .field("fis_ie", &self.fis_ie())
                .field("err_ie", &self.err_ie())
                .field("rd", &self.rd())
                .field("rs", &self.rs())
                .field("rc", &self.rc())
                .field("fd", &self.fd())
                .field("fs", &self.fs())
                .field("width", &self.width())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[doc = "desc CONFB1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confb1(pub u32);
    impl Confb1 {
        #[doc = "desc MSK."]
        #[must_use]
        #[inline(always)]
        pub const fn msk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSK."]
        #[inline(always)]
        pub const fn set_msk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc STAT."]
        #[must_use]
        #[inline(always)]
        pub const fn stat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc STAT."]
        #[inline(always)]
        pub const fn set_stat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc FIS_IE."]
        #[must_use]
        #[inline(always)]
        pub const fn fis_ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIS_IE."]
        #[inline(always)]
        pub const fn set_fis_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc ERR_IE."]
        #[must_use]
        #[inline(always)]
        pub const fn err_ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERR_IE."]
        #[inline(always)]
        pub const fn set_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc RD."]
        #[must_use]
        #[inline(always)]
        pub const fn rd(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc RD."]
        #[inline(always)]
        pub const fn set_rd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc RS."]
        #[must_use]
        #[inline(always)]
        pub const fn rs(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc RS."]
        #[inline(always)]
        pub const fn set_rs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc RC."]
        #[must_use]
        #[inline(always)]
        pub const fn rc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc RC."]
        #[inline(always)]
        pub const fn set_rc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FD."]
        #[must_use]
        #[inline(always)]
        pub const fn fd(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FD."]
        #[inline(always)]
        pub const fn set_fd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FS."]
        #[must_use]
        #[inline(always)]
        pub const fn fs(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FS."]
        #[inline(always)]
        pub const fn set_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc WIDTH."]
        #[must_use]
        #[inline(always)]
        pub const fn width(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc WIDTH."]
        #[inline(always)]
        pub const fn set_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Confb1 {
        #[inline(always)]
        fn default() -> Confb1 {
            Confb1(0)
        }
    }
    impl core::fmt::Debug for Confb1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confb1")
                .field("msk", &self.msk())
                .field("stat", &self.stat())
                .field("fis_ie", &self.fis_ie())
                .field("err_ie", &self.err_ie())
                .field("rd", &self.rd())
                .field("rs", &self.rs())
                .field("rc", &self.rc())
                .field("fd", &self.fd())
                .field("fs", &self.fs())
                .field("width", &self.width())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[doc = "desc CONFB2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confb2(pub u32);
    impl Confb2 {
        #[doc = "desc MSK."]
        #[must_use]
        #[inline(always)]
        pub const fn msk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSK."]
        #[inline(always)]
        pub const fn set_msk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc STAT."]
        #[must_use]
        #[inline(always)]
        pub const fn stat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc STAT."]
        #[inline(always)]
        pub const fn set_stat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc FIS_IE."]
        #[must_use]
        #[inline(always)]
        pub const fn fis_ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIS_IE."]
        #[inline(always)]
        pub const fn set_fis_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc ERR_IE."]
        #[must_use]
        #[inline(always)]
        pub const fn err_ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERR_IE."]
        #[inline(always)]
        pub const fn set_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc RD."]
        #[must_use]
        #[inline(always)]
        pub const fn rd(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc RD."]
        #[inline(always)]
        pub const fn set_rd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc RS."]
        #[must_use]
        #[inline(always)]
        pub const fn rs(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc RS."]
        #[inline(always)]
        pub const fn set_rs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc RC."]
        #[must_use]
        #[inline(always)]
        pub const fn rc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc RC."]
        #[inline(always)]
        pub const fn set_rc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FD."]
        #[must_use]
        #[inline(always)]
        pub const fn fd(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FD."]
        #[inline(always)]
        pub const fn set_fd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FS."]
        #[must_use]
        #[inline(always)]
        pub const fn fs(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FS."]
        #[inline(always)]
        pub const fn set_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc WIDTH."]
        #[must_use]
        #[inline(always)]
        pub const fn width(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc WIDTH."]
        #[inline(always)]
        pub const fn set_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Confb2 {
        #[inline(always)]
        fn default() -> Confb2 {
            Confb2(0)
        }
    }
    impl core::fmt::Debug for Confb2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confb2")
                .field("msk", &self.msk())
                .field("stat", &self.stat())
                .field("fis_ie", &self.fis_ie())
                .field("err_ie", &self.err_ie())
                .field("rd", &self.rd())
                .field("rs", &self.rs())
                .field("rc", &self.rc())
                .field("fd", &self.fd())
                .field("fs", &self.fs())
                .field("width", &self.width())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[doc = "desc CONFB3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confb3(pub u32);
    impl Confb3 {
        #[doc = "desc MSK."]
        #[must_use]
        #[inline(always)]
        pub const fn msk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSK."]
        #[inline(always)]
        pub const fn set_msk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc STAT."]
        #[must_use]
        #[inline(always)]
        pub const fn stat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc STAT."]
        #[inline(always)]
        pub const fn set_stat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc FIS_IE."]
        #[must_use]
        #[inline(always)]
        pub const fn fis_ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIS_IE."]
        #[inline(always)]
        pub const fn set_fis_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc ERR_IE."]
        #[must_use]
        #[inline(always)]
        pub const fn err_ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERR_IE."]
        #[inline(always)]
        pub const fn set_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc RD."]
        #[must_use]
        #[inline(always)]
        pub const fn rd(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc RD."]
        #[inline(always)]
        pub const fn set_rd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc RS."]
        #[must_use]
        #[inline(always)]
        pub const fn rs(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc RS."]
        #[inline(always)]
        pub const fn set_rs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc RC."]
        #[must_use]
        #[inline(always)]
        pub const fn rc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc RC."]
        #[inline(always)]
        pub const fn set_rc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FD."]
        #[must_use]
        #[inline(always)]
        pub const fn fd(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FD."]
        #[inline(always)]
        pub const fn set_fd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FS."]
        #[must_use]
        #[inline(always)]
        pub const fn fs(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FS."]
        #[inline(always)]
        pub const fn set_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc WIDTH."]
        #[must_use]
        #[inline(always)]
        pub const fn width(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc WIDTH."]
        #[inline(always)]
        pub const fn set_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Confb3 {
        #[inline(always)]
        fn default() -> Confb3 {
            Confb3(0)
        }
    }
    impl core::fmt::Debug for Confb3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confb3")
                .field("msk", &self.msk())
                .field("stat", &self.stat())
                .field("fis_ie", &self.fis_ie())
                .field("err_ie", &self.err_ie())
                .field("rd", &self.rd())
                .field("rs", &self.rs())
                .field("rc", &self.rc())
                .field("fd", &self.fd())
                .field("fs", &self.fs())
                .field("width", &self.width())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[doc = "desc CONFB4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confb4(pub u32);
    impl Confb4 {
        #[doc = "desc MSK."]
        #[must_use]
        #[inline(always)]
        pub const fn msk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSK."]
        #[inline(always)]
        pub const fn set_msk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc STAT."]
        #[must_use]
        #[inline(always)]
        pub const fn stat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc STAT."]
        #[inline(always)]
        pub const fn set_stat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc FIS_IE."]
        #[must_use]
        #[inline(always)]
        pub const fn fis_ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIS_IE."]
        #[inline(always)]
        pub const fn set_fis_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc ERR_IE."]
        #[must_use]
        #[inline(always)]
        pub const fn err_ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERR_IE."]
        #[inline(always)]
        pub const fn set_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc RD."]
        #[must_use]
        #[inline(always)]
        pub const fn rd(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc RD."]
        #[inline(always)]
        pub const fn set_rd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc RS."]
        #[must_use]
        #[inline(always)]
        pub const fn rs(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc RS."]
        #[inline(always)]
        pub const fn set_rs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc RC."]
        #[must_use]
        #[inline(always)]
        pub const fn rc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc RC."]
        #[inline(always)]
        pub const fn set_rc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FD."]
        #[must_use]
        #[inline(always)]
        pub const fn fd(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FD."]
        #[inline(always)]
        pub const fn set_fd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FS."]
        #[must_use]
        #[inline(always)]
        pub const fn fs(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FS."]
        #[inline(always)]
        pub const fn set_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc WIDTH."]
        #[must_use]
        #[inline(always)]
        pub const fn width(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc WIDTH."]
        #[inline(always)]
        pub const fn set_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Confb4 {
        #[inline(always)]
        fn default() -> Confb4 {
            Confb4(0)
        }
    }
    impl core::fmt::Debug for Confb4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confb4")
                .field("msk", &self.msk())
                .field("stat", &self.stat())
                .field("fis_ie", &self.fis_ie())
                .field("err_ie", &self.err_ie())
                .field("rd", &self.rd())
                .field("rs", &self.rs())
                .field("rc", &self.rc())
                .field("fd", &self.fd())
                .field("fs", &self.fs())
                .field("width", &self.width())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[doc = "desc CONFB5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Confb5(pub u32);
    impl Confb5 {
        #[doc = "desc MSK."]
        #[must_use]
        #[inline(always)]
        pub const fn msk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSK."]
        #[inline(always)]
        pub const fn set_msk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc STAT."]
        #[must_use]
        #[inline(always)]
        pub const fn stat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc STAT."]
        #[inline(always)]
        pub const fn set_stat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc FIS_IE."]
        #[must_use]
        #[inline(always)]
        pub const fn fis_ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIS_IE."]
        #[inline(always)]
        pub const fn set_fis_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc ERR_IE."]
        #[must_use]
        #[inline(always)]
        pub const fn err_ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERR_IE."]
        #[inline(always)]
        pub const fn set_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc RD."]
        #[must_use]
        #[inline(always)]
        pub const fn rd(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc RD."]
        #[inline(always)]
        pub const fn set_rd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc RS."]
        #[must_use]
        #[inline(always)]
        pub const fn rs(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc RS."]
        #[inline(always)]
        pub const fn set_rs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc RC."]
        #[must_use]
        #[inline(always)]
        pub const fn rc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc RC."]
        #[inline(always)]
        pub const fn set_rc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FD."]
        #[must_use]
        #[inline(always)]
        pub const fn fd(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FD."]
        #[inline(always)]
        pub const fn set_fd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FS."]
        #[must_use]
        #[inline(always)]
        pub const fn fs(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FS."]
        #[inline(always)]
        pub const fn set_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc WIDTH."]
        #[must_use]
        #[inline(always)]
        pub const fn width(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc WIDTH."]
        #[inline(always)]
        pub const fn set_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Confb5 {
        #[inline(always)]
        fn default() -> Confb5 {
            Confb5(0)
        }
    }
    impl core::fmt::Debug for Confb5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Confb5")
                .field("msk", &self.msk())
                .field("stat", &self.stat())
                .field("fis_ie", &self.fis_ie())
                .field("err_ie", &self.err_ie())
                .field("rd", &self.rd())
                .field("rs", &self.rs())
                .field("rc", &self.rc())
                .field("fd", &self.fd())
                .field("fs", &self.fs())
                .field("width", &self.width())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[doc = "desc DSTADR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dstadr0(pub u32);
    impl Dstadr0 {
        #[doc = "desc DSTADR."]
        #[must_use]
        #[inline(always)]
        pub const fn dstadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DSTADR."]
        #[inline(always)]
        pub const fn set_dstadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dstadr0 {
        #[inline(always)]
        fn default() -> Dstadr0 {
            Dstadr0(0)
        }
    }
    impl core::fmt::Debug for Dstadr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dstadr0")
                .field("dstadr", &self.dstadr())
                .finish()
        }
    }
    #[doc = "desc DSTADR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dstadr1(pub u32);
    impl Dstadr1 {
        #[doc = "desc DSTADR."]
        #[must_use]
        #[inline(always)]
        pub const fn dstadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DSTADR."]
        #[inline(always)]
        pub const fn set_dstadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dstadr1 {
        #[inline(always)]
        fn default() -> Dstadr1 {
            Dstadr1(0)
        }
    }
    impl core::fmt::Debug for Dstadr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dstadr1")
                .field("dstadr", &self.dstadr())
                .finish()
        }
    }
    #[doc = "desc DSTADR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dstadr2(pub u32);
    impl Dstadr2 {
        #[doc = "desc DSTADR."]
        #[must_use]
        #[inline(always)]
        pub const fn dstadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DSTADR."]
        #[inline(always)]
        pub const fn set_dstadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dstadr2 {
        #[inline(always)]
        fn default() -> Dstadr2 {
            Dstadr2(0)
        }
    }
    impl core::fmt::Debug for Dstadr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dstadr2")
                .field("dstadr", &self.dstadr())
                .finish()
        }
    }
    #[doc = "desc DSTADR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dstadr3(pub u32);
    impl Dstadr3 {
        #[doc = "desc DSTADR."]
        #[must_use]
        #[inline(always)]
        pub const fn dstadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DSTADR."]
        #[inline(always)]
        pub const fn set_dstadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dstadr3 {
        #[inline(always)]
        fn default() -> Dstadr3 {
            Dstadr3(0)
        }
    }
    impl core::fmt::Debug for Dstadr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dstadr3")
                .field("dstadr", &self.dstadr())
                .finish()
        }
    }
    #[doc = "desc DSTADR4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dstadr4(pub u32);
    impl Dstadr4 {
        #[doc = "desc DSTADR."]
        #[must_use]
        #[inline(always)]
        pub const fn dstadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DSTADR."]
        #[inline(always)]
        pub const fn set_dstadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dstadr4 {
        #[inline(always)]
        fn default() -> Dstadr4 {
            Dstadr4(0)
        }
    }
    impl core::fmt::Debug for Dstadr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dstadr4")
                .field("dstadr", &self.dstadr())
                .finish()
        }
    }
    #[doc = "desc DSTADR5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dstadr5(pub u32);
    impl Dstadr5 {
        #[doc = "desc DSTADR."]
        #[must_use]
        #[inline(always)]
        pub const fn dstadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DSTADR."]
        #[inline(always)]
        pub const fn set_dstadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dstadr5 {
        #[inline(always)]
        fn default() -> Dstadr5 {
            Dstadr5(0)
        }
    }
    impl core::fmt::Debug for Dstadr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dstadr5")
                .field("dstadr", &self.dstadr())
                .finish()
        }
    }
    #[doc = "desc SRCADR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcadr0(pub u32);
    impl Srcadr0 {
        #[doc = "desc SRCADR."]
        #[must_use]
        #[inline(always)]
        pub const fn srcadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SRCADR."]
        #[inline(always)]
        pub const fn set_srcadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Srcadr0 {
        #[inline(always)]
        fn default() -> Srcadr0 {
            Srcadr0(0)
        }
    }
    impl core::fmt::Debug for Srcadr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srcadr0")
                .field("srcadr", &self.srcadr())
                .finish()
        }
    }
    #[doc = "desc SRCADR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcadr1(pub u32);
    impl Srcadr1 {
        #[doc = "desc SRCADR."]
        #[must_use]
        #[inline(always)]
        pub const fn srcadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SRCADR."]
        #[inline(always)]
        pub const fn set_srcadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Srcadr1 {
        #[inline(always)]
        fn default() -> Srcadr1 {
            Srcadr1(0)
        }
    }
    impl core::fmt::Debug for Srcadr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srcadr1")
                .field("srcadr", &self.srcadr())
                .finish()
        }
    }
    #[doc = "desc SRCADR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcadr2(pub u32);
    impl Srcadr2 {
        #[doc = "desc SRCADR."]
        #[must_use]
        #[inline(always)]
        pub const fn srcadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SRCADR."]
        #[inline(always)]
        pub const fn set_srcadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Srcadr2 {
        #[inline(always)]
        fn default() -> Srcadr2 {
            Srcadr2(0)
        }
    }
    impl core::fmt::Debug for Srcadr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srcadr2")
                .field("srcadr", &self.srcadr())
                .finish()
        }
    }
    #[doc = "desc SRCADR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcadr3(pub u32);
    impl Srcadr3 {
        #[doc = "desc SRCADR."]
        #[must_use]
        #[inline(always)]
        pub const fn srcadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SRCADR."]
        #[inline(always)]
        pub const fn set_srcadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Srcadr3 {
        #[inline(always)]
        fn default() -> Srcadr3 {
            Srcadr3(0)
        }
    }
    impl core::fmt::Debug for Srcadr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srcadr3")
                .field("srcadr", &self.srcadr())
                .finish()
        }
    }
    #[doc = "desc SRCADR4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcadr4(pub u32);
    impl Srcadr4 {
        #[doc = "desc SRCADR."]
        #[must_use]
        #[inline(always)]
        pub const fn srcadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SRCADR."]
        #[inline(always)]
        pub const fn set_srcadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Srcadr4 {
        #[inline(always)]
        fn default() -> Srcadr4 {
            Srcadr4(0)
        }
    }
    impl core::fmt::Debug for Srcadr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srcadr4")
                .field("srcadr", &self.srcadr())
                .finish()
        }
    }
    #[doc = "desc SRCADR5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcadr5(pub u32);
    impl Srcadr5 {
        #[doc = "desc SRCADR."]
        #[must_use]
        #[inline(always)]
        pub const fn srcadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SRCADR."]
        #[inline(always)]
        pub const fn set_srcadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Srcadr5 {
        #[inline(always)]
        fn default() -> Srcadr5 {
            Srcadr5(0)
        }
    }
    impl core::fmt::Debug for Srcadr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srcadr5")
                .field("srcadr", &self.srcadr())
                .finish()
        }
    }
}
