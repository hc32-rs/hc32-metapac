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
}
