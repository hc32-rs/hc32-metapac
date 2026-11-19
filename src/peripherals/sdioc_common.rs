#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc SDIOC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdioc {
    ptr: *mut u8,
}
unsafe impl Send for Sdioc {}
unsafe impl Sync for Sdioc {}
impl Sdioc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc BLKSIZE."]
    #[inline(always)]
    pub const fn blksize(self) -> crate::common::Reg<regs::Blksize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc BLKCNT."]
    #[inline(always)]
    pub const fn blkcnt(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "desc ARG0."]
    #[inline(always)]
    pub const fn arg0(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc ARG1."]
    #[inline(always)]
    pub const fn arg1(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "desc TRANSMODE."]
    #[inline(always)]
    pub const fn transmode(self) -> crate::common::Reg<regs::Transmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc CMD."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "desc RESP0."]
    #[inline(always)]
    pub const fn resp0(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc RESP1."]
    #[inline(always)]
    pub const fn resp1(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "desc RESP2."]
    #[inline(always)]
    pub const fn resp2(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc RESP3."]
    #[inline(always)]
    pub const fn resp3(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "desc RESP4."]
    #[inline(always)]
    pub const fn resp4(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc RESP5."]
    #[inline(always)]
    pub const fn resp5(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "desc RESP6."]
    #[inline(always)]
    pub const fn resp6(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc RESP7."]
    #[inline(always)]
    pub const fn resp7(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "desc BUF0."]
    #[inline(always)]
    pub const fn buf0(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc BUF1."]
    #[inline(always)]
    pub const fn buf1(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "desc PSTAT."]
    #[inline(always)]
    pub const fn pstat(self) -> crate::common::Reg<regs::Pstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc HOSTCON."]
    #[inline(always)]
    pub const fn hostcon(self) -> crate::common::Reg<regs::Hostcon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc PWRCON."]
    #[inline(always)]
    pub const fn pwrcon(self) -> crate::common::Reg<regs::Pwrcon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x29usize) as _) }
    }
    #[doc = "desc BLKGPCON."]
    #[inline(always)]
    pub const fn blkgpcon(self) -> crate::common::Reg<regs::Blkgpcon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "desc CLKCON."]
    #[inline(always)]
    pub const fn clkcon(self) -> crate::common::Reg<regs::Clkcon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc TOUTCON."]
    #[inline(always)]
    pub const fn toutcon(self) -> crate::common::Reg<regs::Toutcon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "desc SFTRST."]
    #[inline(always)]
    pub const fn sftrst(self) -> crate::common::Reg<regs::Sftrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2fusize) as _) }
    }
    #[doc = "desc NORINTST."]
    #[inline(always)]
    pub const fn norintst(self) -> crate::common::Reg<regs::Norintst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc ERRINTST."]
    #[inline(always)]
    pub const fn errintst(self) -> crate::common::Reg<regs::Errintst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "desc NORINTSTEN."]
    #[inline(always)]
    pub const fn norintsten(self) -> crate::common::Reg<regs::Norintsten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc ERRINTSTEN."]
    #[inline(always)]
    pub const fn errintsten(self) -> crate::common::Reg<regs::Errintsten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "desc NORINTSGEN."]
    #[inline(always)]
    pub const fn norintsgen(self) -> crate::common::Reg<regs::Norintsgen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc ERRINTSGEN."]
    #[inline(always)]
    pub const fn errintsgen(self) -> crate::common::Reg<regs::Errintsgen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3ausize) as _) }
    }
    #[doc = "desc ATCERRST."]
    #[inline(always)]
    pub const fn atcerrst(self) -> crate::common::Reg<regs::Atcerrst, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc FEA."]
    #[inline(always)]
    pub const fn fea(self) -> crate::common::Reg<regs::Fea, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc FEE."]
    #[inline(always)]
    pub const fn fee(self) -> crate::common::Reg<regs::Fee, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x52usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ATCERRST."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atcerrst(pub u16);
    impl Atcerrst {
        #[doc = "desc NE."]
        #[must_use]
        #[inline(always)]
        pub const fn ne(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc NE."]
        #[inline(always)]
        pub const fn set_ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc TOE."]
        #[must_use]
        #[inline(always)]
        pub const fn toe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOE."]
        #[inline(always)]
        pub const fn set_toe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc CE."]
        #[must_use]
        #[inline(always)]
        pub const fn ce(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CE."]
        #[inline(always)]
        pub const fn set_ce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc EBE."]
        #[must_use]
        #[inline(always)]
        pub const fn ebe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc EBE."]
        #[inline(always)]
        pub const fn set_ebe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc CMDE."]
        #[must_use]
        #[inline(always)]
        pub const fn cmde(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMDE."]
        #[inline(always)]
        pub const fn set_cmde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
    }
    impl Default for Atcerrst {
        #[inline(always)]
        fn default() -> Atcerrst {
            Atcerrst(0)
        }
    }
    impl core::fmt::Debug for Atcerrst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Atcerrst")
                .field("ne", &self.ne())
                .field("toe", &self.toe())
                .field("ce", &self.ce())
                .field("ebe", &self.ebe())
                .field("ie", &self.ie())
                .field("cmde", &self.cmde())
                .finish()
        }
    }
    #[doc = "desc BLKGPCON."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Blkgpcon(pub u8);
    impl Blkgpcon {
        #[doc = "desc SABGR."]
        #[must_use]
        #[inline(always)]
        pub const fn sabgr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SABGR."]
        #[inline(always)]
        pub const fn set_sabgr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc CR."]
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CR."]
        #[inline(always)]
        pub const fn set_cr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc RWC."]
        #[must_use]
        #[inline(always)]
        pub const fn rwc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RWC."]
        #[inline(always)]
        pub const fn set_rwc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc IABG."]
        #[must_use]
        #[inline(always)]
        pub const fn iabg(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc IABG."]
        #[inline(always)]
        pub const fn set_iabg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for Blkgpcon {
        #[inline(always)]
        fn default() -> Blkgpcon {
            Blkgpcon(0)
        }
    }
    impl core::fmt::Debug for Blkgpcon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Blkgpcon")
                .field("sabgr", &self.sabgr())
                .field("cr", &self.cr())
                .field("rwc", &self.rwc())
                .field("iabg", &self.iabg())
                .finish()
        }
    }
    #[doc = "desc BLKSIZE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Blksize(pub u16);
    impl Blksize {
        #[doc = "desc TBS."]
        #[must_use]
        #[inline(always)]
        pub const fn tbs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc TBS."]
        #[inline(always)]
        pub const fn set_tbs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for Blksize {
        #[inline(always)]
        fn default() -> Blksize {
            Blksize(0)
        }
    }
    impl core::fmt::Debug for Blksize {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Blksize").field("tbs", &self.tbs()).finish()
        }
    }
    #[doc = "desc CLKCON."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clkcon(pub u16);
    impl Clkcon {
        #[doc = "desc ICE."]
        #[must_use]
        #[inline(always)]
        pub const fn ice(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ICE."]
        #[inline(always)]
        pub const fn set_ice(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc CE."]
        #[must_use]
        #[inline(always)]
        pub const fn ce(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CE."]
        #[inline(always)]
        pub const fn set_ce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc FS."]
        #[must_use]
        #[inline(always)]
        pub const fn fs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc FS."]
        #[inline(always)]
        pub const fn set_fs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for Clkcon {
        #[inline(always)]
        fn default() -> Clkcon {
            Clkcon(0)
        }
    }
    impl core::fmt::Debug for Clkcon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clkcon")
                .field("ice", &self.ice())
                .field("ce", &self.ce())
                .field("fs", &self.fs())
                .finish()
        }
    }
    #[doc = "desc CMD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u16);
    impl Cmd {
        #[doc = "desc RESTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn restyp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc RESTYP."]
        #[inline(always)]
        pub const fn set_restyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[doc = "desc CCE."]
        #[must_use]
        #[inline(always)]
        pub const fn cce(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCE."]
        #[inline(always)]
        pub const fn set_cce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "desc ICE."]
        #[must_use]
        #[inline(always)]
        pub const fn ice(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ICE."]
        #[inline(always)]
        pub const fn set_ice(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc DAT."]
        #[must_use]
        #[inline(always)]
        pub const fn dat(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DAT."]
        #[inline(always)]
        pub const fn set_dat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc TYP."]
        #[must_use]
        #[inline(always)]
        pub const fn typ(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc TYP."]
        #[inline(always)]
        pub const fn set_typ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[doc = "desc IDX."]
        #[must_use]
        #[inline(always)]
        pub const fn idx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "desc IDX."]
        #[inline(always)]
        pub const fn set_idx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u16) & 0x3f) << 8usize);
        }
    }
    impl Default for Cmd {
        #[inline(always)]
        fn default() -> Cmd {
            Cmd(0)
        }
    }
    impl core::fmt::Debug for Cmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmd")
                .field("restyp", &self.restyp())
                .field("cce", &self.cce())
                .field("ice", &self.ice())
                .field("dat", &self.dat())
                .field("typ", &self.typ())
                .field("idx", &self.idx())
                .finish()
        }
    }
    #[doc = "desc ERRINTSGEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Errintsgen(pub u16);
    impl Errintsgen {
        #[doc = "desc CTOESEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ctoesen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTOESEN."]
        #[inline(always)]
        pub const fn set_ctoesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc CCESEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ccesen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCESEN."]
        #[inline(always)]
        pub const fn set_ccesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc CEBESEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cebesen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CEBESEN."]
        #[inline(always)]
        pub const fn set_cebesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc CIESEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ciesen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIESEN."]
        #[inline(always)]
        pub const fn set_ciesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "desc DTOESEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dtoesen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTOESEN."]
        #[inline(always)]
        pub const fn set_dtoesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc DCESEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dcesen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCESEN."]
        #[inline(always)]
        pub const fn set_dcesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc DEBESEN."]
        #[must_use]
        #[inline(always)]
        pub const fn debesen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc DEBESEN."]
        #[inline(always)]
        pub const fn set_debesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc ACESEN."]
        #[must_use]
        #[inline(always)]
        pub const fn acesen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACESEN."]
        #[inline(always)]
        pub const fn set_acesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Errintsgen {
        #[inline(always)]
        fn default() -> Errintsgen {
            Errintsgen(0)
        }
    }
    impl core::fmt::Debug for Errintsgen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Errintsgen")
                .field("ctoesen", &self.ctoesen())
                .field("ccesen", &self.ccesen())
                .field("cebesen", &self.cebesen())
                .field("ciesen", &self.ciesen())
                .field("dtoesen", &self.dtoesen())
                .field("dcesen", &self.dcesen())
                .field("debesen", &self.debesen())
                .field("acesen", &self.acesen())
                .finish()
        }
    }
    #[doc = "desc ERRINTST."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Errintst(pub u16);
    impl Errintst {
        #[doc = "desc CTOE."]
        #[must_use]
        #[inline(always)]
        pub const fn ctoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTOE."]
        #[inline(always)]
        pub const fn set_ctoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc CCE."]
        #[must_use]
        #[inline(always)]
        pub const fn cce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCE."]
        #[inline(always)]
        pub const fn set_cce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc CEBE."]
        #[must_use]
        #[inline(always)]
        pub const fn cebe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CEBE."]
        #[inline(always)]
        pub const fn set_cebe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc CIE."]
        #[must_use]
        #[inline(always)]
        pub const fn cie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIE."]
        #[inline(always)]
        pub const fn set_cie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "desc DTOE."]
        #[must_use]
        #[inline(always)]
        pub const fn dtoe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTOE."]
        #[inline(always)]
        pub const fn set_dtoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc DCE."]
        #[must_use]
        #[inline(always)]
        pub const fn dce(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCE."]
        #[inline(always)]
        pub const fn set_dce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc DEBE."]
        #[must_use]
        #[inline(always)]
        pub const fn debe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc DEBE."]
        #[inline(always)]
        pub const fn set_debe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc ACE."]
        #[must_use]
        #[inline(always)]
        pub const fn ace(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACE."]
        #[inline(always)]
        pub const fn set_ace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Errintst {
        #[inline(always)]
        fn default() -> Errintst {
            Errintst(0)
        }
    }
    impl core::fmt::Debug for Errintst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Errintst")
                .field("ctoe", &self.ctoe())
                .field("cce", &self.cce())
                .field("cebe", &self.cebe())
                .field("cie", &self.cie())
                .field("dtoe", &self.dtoe())
                .field("dce", &self.dce())
                .field("debe", &self.debe())
                .field("ace", &self.ace())
                .finish()
        }
    }
    #[doc = "desc ERRINTSTEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Errintsten(pub u16);
    impl Errintsten {
        #[doc = "desc CTOEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ctoeen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTOEEN."]
        #[inline(always)]
        pub const fn set_ctoeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc CCEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cceen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCEEN."]
        #[inline(always)]
        pub const fn set_cceen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc CEBEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cebeen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CEBEEN."]
        #[inline(always)]
        pub const fn set_cebeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc CIEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cieen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIEEN."]
        #[inline(always)]
        pub const fn set_cieen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "desc DTOEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dtoeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTOEEN."]
        #[inline(always)]
        pub const fn set_dtoeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc DCEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dceen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCEEN."]
        #[inline(always)]
        pub const fn set_dceen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc DEBEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn debeen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc DEBEEN."]
        #[inline(always)]
        pub const fn set_debeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc ACEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn aceen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACEEN."]
        #[inline(always)]
        pub const fn set_aceen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Errintsten {
        #[inline(always)]
        fn default() -> Errintsten {
            Errintsten(0)
        }
    }
    impl core::fmt::Debug for Errintsten {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Errintsten")
                .field("ctoeen", &self.ctoeen())
                .field("cceen", &self.cceen())
                .field("cebeen", &self.cebeen())
                .field("cieen", &self.cieen())
                .field("dtoeen", &self.dtoeen())
                .field("dceen", &self.dceen())
                .field("debeen", &self.debeen())
                .field("aceen", &self.aceen())
                .finish()
        }
    }
    #[doc = "desc FEA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fea(pub u16);
    impl Fea {
        #[doc = "desc FNE."]
        #[must_use]
        #[inline(always)]
        pub const fn fne(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNE."]
        #[inline(always)]
        pub const fn set_fne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc FTOE."]
        #[must_use]
        #[inline(always)]
        pub const fn ftoe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FTOE."]
        #[inline(always)]
        pub const fn set_ftoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc FCE."]
        #[must_use]
        #[inline(always)]
        pub const fn fce(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCE."]
        #[inline(always)]
        pub const fn set_fce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc FEBE."]
        #[must_use]
        #[inline(always)]
        pub const fn febe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FEBE."]
        #[inline(always)]
        pub const fn set_febe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "desc FIE."]
        #[must_use]
        #[inline(always)]
        pub const fn fie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIE."]
        #[inline(always)]
        pub const fn set_fie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc FCMDE."]
        #[must_use]
        #[inline(always)]
        pub const fn fcmde(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCMDE."]
        #[inline(always)]
        pub const fn set_fcmde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
    }
    impl Default for Fea {
        #[inline(always)]
        fn default() -> Fea {
            Fea(0)
        }
    }
    impl core::fmt::Debug for Fea {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fea")
                .field("fne", &self.fne())
                .field("ftoe", &self.ftoe())
                .field("fce", &self.fce())
                .field("febe", &self.febe())
                .field("fie", &self.fie())
                .field("fcmde", &self.fcmde())
                .finish()
        }
    }
    #[doc = "desc FEE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fee(pub u16);
    impl Fee {
        #[doc = "desc FCTOE."]
        #[must_use]
        #[inline(always)]
        pub const fn fctoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCTOE."]
        #[inline(always)]
        pub const fn set_fctoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc FCCE."]
        #[must_use]
        #[inline(always)]
        pub const fn fcce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCCE."]
        #[inline(always)]
        pub const fn set_fcce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc FCEBE."]
        #[must_use]
        #[inline(always)]
        pub const fn fcebe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCEBE."]
        #[inline(always)]
        pub const fn set_fcebe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc FCIE."]
        #[must_use]
        #[inline(always)]
        pub const fn fcie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCIE."]
        #[inline(always)]
        pub const fn set_fcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "desc FDTOE."]
        #[must_use]
        #[inline(always)]
        pub const fn fdtoe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FDTOE."]
        #[inline(always)]
        pub const fn set_fdtoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc FDCE."]
        #[must_use]
        #[inline(always)]
        pub const fn fdce(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FDCE."]
        #[inline(always)]
        pub const fn set_fdce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc FDEBE."]
        #[must_use]
        #[inline(always)]
        pub const fn fdebe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FDEBE."]
        #[inline(always)]
        pub const fn set_fdebe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc FACE."]
        #[must_use]
        #[inline(always)]
        pub const fn face(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FACE."]
        #[inline(always)]
        pub const fn set_face(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Fee {
        #[inline(always)]
        fn default() -> Fee {
            Fee(0)
        }
    }
    impl core::fmt::Debug for Fee {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fee")
                .field("fctoe", &self.fctoe())
                .field("fcce", &self.fcce())
                .field("fcebe", &self.fcebe())
                .field("fcie", &self.fcie())
                .field("fdtoe", &self.fdtoe())
                .field("fdce", &self.fdce())
                .field("fdebe", &self.fdebe())
                .field("face", &self.face())
                .finish()
        }
    }
    #[doc = "desc HOSTCON."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hostcon(pub u8);
    impl Hostcon {
        #[doc = "desc DW."]
        #[must_use]
        #[inline(always)]
        pub const fn dw(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc DW."]
        #[inline(always)]
        pub const fn set_dw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc HSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn hsen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSEN."]
        #[inline(always)]
        pub const fn set_hsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc EXDW."]
        #[must_use]
        #[inline(always)]
        pub const fn exdw(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EXDW."]
        #[inline(always)]
        pub const fn set_exdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc CDTL."]
        #[must_use]
        #[inline(always)]
        pub const fn cdtl(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDTL."]
        #[inline(always)]
        pub const fn set_cdtl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc CDSS."]
        #[must_use]
        #[inline(always)]
        pub const fn cdss(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDSS."]
        #[inline(always)]
        pub const fn set_cdss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Hostcon {
        #[inline(always)]
        fn default() -> Hostcon {
            Hostcon(0)
        }
    }
    impl core::fmt::Debug for Hostcon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hostcon")
                .field("dw", &self.dw())
                .field("hsen", &self.hsen())
                .field("exdw", &self.exdw())
                .field("cdtl", &self.cdtl())
                .field("cdss", &self.cdss())
                .finish()
        }
    }
    #[doc = "desc NORINTSGEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Norintsgen(pub u16);
    impl Norintsgen {
        #[doc = "desc CCSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ccsen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCSEN."]
        #[inline(always)]
        pub const fn set_ccsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc TCSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn tcsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TCSEN."]
        #[inline(always)]
        pub const fn set_tcsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc BGESEN."]
        #[must_use]
        #[inline(always)]
        pub const fn bgesen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BGESEN."]
        #[inline(always)]
        pub const fn set_bgesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc BWRSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn bwrsen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc BWRSEN."]
        #[inline(always)]
        pub const fn set_bwrsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc BRRSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn brrsen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRRSEN."]
        #[inline(always)]
        pub const fn set_brrsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc CISTSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cistsen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CISTSEN."]
        #[inline(always)]
        pub const fn set_cistsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc CRMSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn crmsen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRMSEN."]
        #[inline(always)]
        pub const fn set_crmsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc CINTSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cintsen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINTSEN."]
        #[inline(always)]
        pub const fn set_cintsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Norintsgen {
        #[inline(always)]
        fn default() -> Norintsgen {
            Norintsgen(0)
        }
    }
    impl core::fmt::Debug for Norintsgen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Norintsgen")
                .field("ccsen", &self.ccsen())
                .field("tcsen", &self.tcsen())
                .field("bgesen", &self.bgesen())
                .field("bwrsen", &self.bwrsen())
                .field("brrsen", &self.brrsen())
                .field("cistsen", &self.cistsen())
                .field("crmsen", &self.crmsen())
                .field("cintsen", &self.cintsen())
                .finish()
        }
    }
    #[doc = "desc NORINTST."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Norintst(pub u16);
    impl Norintst {
        #[doc = "desc CC."]
        #[must_use]
        #[inline(always)]
        pub const fn cc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC."]
        #[inline(always)]
        pub const fn set_cc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc BGE."]
        #[must_use]
        #[inline(always)]
        pub const fn bge(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BGE."]
        #[inline(always)]
        pub const fn set_bge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc BWR."]
        #[must_use]
        #[inline(always)]
        pub const fn bwr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc BWR."]
        #[inline(always)]
        pub const fn set_bwr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc BRR."]
        #[must_use]
        #[inline(always)]
        pub const fn brr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR."]
        #[inline(always)]
        pub const fn set_brr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc CIST."]
        #[must_use]
        #[inline(always)]
        pub const fn cist(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIST."]
        #[inline(always)]
        pub const fn set_cist(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc CRM."]
        #[must_use]
        #[inline(always)]
        pub const fn crm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRM."]
        #[inline(always)]
        pub const fn set_crm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc CINT."]
        #[must_use]
        #[inline(always)]
        pub const fn cint(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINT."]
        #[inline(always)]
        pub const fn set_cint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc EI."]
        #[must_use]
        #[inline(always)]
        pub const fn ei(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EI."]
        #[inline(always)]
        pub const fn set_ei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Norintst {
        #[inline(always)]
        fn default() -> Norintst {
            Norintst(0)
        }
    }
    impl core::fmt::Debug for Norintst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Norintst")
                .field("cc", &self.cc())
                .field("tc", &self.tc())
                .field("bge", &self.bge())
                .field("bwr", &self.bwr())
                .field("brr", &self.brr())
                .field("cist", &self.cist())
                .field("crm", &self.crm())
                .field("cint", &self.cint())
                .field("ei", &self.ei())
                .finish()
        }
    }
    #[doc = "desc NORINTSTEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Norintsten(pub u16);
    impl Norintsten {
        #[doc = "desc CCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ccen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCEN."]
        #[inline(always)]
        pub const fn set_ccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc TCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn tcen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TCEN."]
        #[inline(always)]
        pub const fn set_tcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc BGEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn bgeen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BGEEN."]
        #[inline(always)]
        pub const fn set_bgeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc BWREN."]
        #[must_use]
        #[inline(always)]
        pub const fn bwren(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc BWREN."]
        #[inline(always)]
        pub const fn set_bwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc BRREN."]
        #[must_use]
        #[inline(always)]
        pub const fn brren(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRREN."]
        #[inline(always)]
        pub const fn set_brren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc CISTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cisten(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CISTEN."]
        #[inline(always)]
        pub const fn set_cisten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc CRMEN."]
        #[must_use]
        #[inline(always)]
        pub const fn crmen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRMEN."]
        #[inline(always)]
        pub const fn set_crmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc CINTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cinten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINTEN."]
        #[inline(always)]
        pub const fn set_cinten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Norintsten {
        #[inline(always)]
        fn default() -> Norintsten {
            Norintsten(0)
        }
    }
    impl core::fmt::Debug for Norintsten {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Norintsten")
                .field("ccen", &self.ccen())
                .field("tcen", &self.tcen())
                .field("bgeen", &self.bgeen())
                .field("bwren", &self.bwren())
                .field("brren", &self.brren())
                .field("cisten", &self.cisten())
                .field("crmen", &self.crmen())
                .field("cinten", &self.cinten())
                .finish()
        }
    }
    #[doc = "desc PSTAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pstat(pub u32);
    impl Pstat {
        #[doc = "desc CIC."]
        #[must_use]
        #[inline(always)]
        pub const fn cic(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIC."]
        #[inline(always)]
        pub const fn set_cic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CID."]
        #[must_use]
        #[inline(always)]
        pub const fn cid(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CID."]
        #[inline(always)]
        pub const fn set_cid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc DA."]
        #[must_use]
        #[inline(always)]
        pub const fn da(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc DA."]
        #[inline(always)]
        pub const fn set_da(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc WTA."]
        #[must_use]
        #[inline(always)]
        pub const fn wta(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc WTA."]
        #[inline(always)]
        pub const fn set_wta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RTA."]
        #[must_use]
        #[inline(always)]
        pub const fn rta(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTA."]
        #[inline(always)]
        pub const fn set_rta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BWE."]
        #[must_use]
        #[inline(always)]
        pub const fn bwe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BWE."]
        #[inline(always)]
        pub const fn set_bwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc BRE."]
        #[must_use]
        #[inline(always)]
        pub const fn bre(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRE."]
        #[inline(always)]
        pub const fn set_bre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc CSS."]
        #[must_use]
        #[inline(always)]
        pub const fn css(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSS."]
        #[inline(always)]
        pub const fn set_css(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc CDL."]
        #[must_use]
        #[inline(always)]
        pub const fn cdl(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDL."]
        #[inline(always)]
        pub const fn set_cdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc WPL."]
        #[must_use]
        #[inline(always)]
        pub const fn wpl(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc WPL."]
        #[inline(always)]
        pub const fn set_wpl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc DATL."]
        #[must_use]
        #[inline(always)]
        pub const fn datl(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "desc DATL."]
        #[inline(always)]
        pub const fn set_datl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "desc CMDL."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdl(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMDL."]
        #[inline(always)]
        pub const fn set_cmdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Pstat {
        #[inline(always)]
        fn default() -> Pstat {
            Pstat(0)
        }
    }
    impl core::fmt::Debug for Pstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pstat")
                .field("cic", &self.cic())
                .field("cid", &self.cid())
                .field("da", &self.da())
                .field("wta", &self.wta())
                .field("rta", &self.rta())
                .field("bwe", &self.bwe())
                .field("bre", &self.bre())
                .field("cin", &self.cin())
                .field("css", &self.css())
                .field("cdl", &self.cdl())
                .field("wpl", &self.wpl())
                .field("datl", &self.datl())
                .field("cmdl", &self.cmdl())
                .finish()
        }
    }
    #[doc = "desc PWRCON."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwrcon(pub u8);
    impl Pwrcon {
        #[doc = "desc PWON."]
        #[must_use]
        #[inline(always)]
        pub const fn pwon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWON."]
        #[inline(always)]
        pub const fn set_pwon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Pwrcon {
        #[inline(always)]
        fn default() -> Pwrcon {
            Pwrcon(0)
        }
    }
    impl core::fmt::Debug for Pwrcon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwrcon")
                .field("pwon", &self.pwon())
                .finish()
        }
    }
    #[doc = "desc SFTRST."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sftrst(pub u8);
    impl Sftrst {
        #[doc = "desc RSTA."]
        #[must_use]
        #[inline(always)]
        pub const fn rsta(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSTA."]
        #[inline(always)]
        pub const fn set_rsta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc RSTC."]
        #[must_use]
        #[inline(always)]
        pub const fn rstc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSTC."]
        #[inline(always)]
        pub const fn set_rstc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc RSTD."]
        #[must_use]
        #[inline(always)]
        pub const fn rstd(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSTD."]
        #[inline(always)]
        pub const fn set_rstd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for Sftrst {
        #[inline(always)]
        fn default() -> Sftrst {
            Sftrst(0)
        }
    }
    impl core::fmt::Debug for Sftrst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sftrst")
                .field("rsta", &self.rsta())
                .field("rstc", &self.rstc())
                .field("rstd", &self.rstd())
                .finish()
        }
    }
    #[doc = "desc TOUTCON."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Toutcon(pub u8);
    impl Toutcon {
        #[doc = "desc DTO."]
        #[must_use]
        #[inline(always)]
        pub const fn dto(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc DTO."]
        #[inline(always)]
        pub const fn set_dto(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for Toutcon {
        #[inline(always)]
        fn default() -> Toutcon {
            Toutcon(0)
        }
    }
    impl core::fmt::Debug for Toutcon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Toutcon").field("dto", &self.dto()).finish()
        }
    }
    #[doc = "desc TRANSMODE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Transmode(pub u16);
    impl Transmode {
        #[doc = "desc BCE."]
        #[must_use]
        #[inline(always)]
        pub const fn bce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc BCE."]
        #[inline(always)]
        pub const fn set_bce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc ATCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn atcen(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc ATCEN."]
        #[inline(always)]
        pub const fn set_atcen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[doc = "desc DDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn ddir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DDIR."]
        #[inline(always)]
        pub const fn set_ddir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc MULB."]
        #[must_use]
        #[inline(always)]
        pub const fn mulb(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc MULB."]
        #[inline(always)]
        pub const fn set_mulb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
    }
    impl Default for Transmode {
        #[inline(always)]
        fn default() -> Transmode {
            Transmode(0)
        }
    }
    impl core::fmt::Debug for Transmode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Transmode")
                .field("bce", &self.bce())
                .field("atcen", &self.atcen())
                .field("ddir", &self.ddir())
                .field("mulb", &self.mulb())
                .finish()
        }
    }
}
