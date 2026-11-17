#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc FLASH."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc TPNVS."]
    #[inline(always)]
    pub const fn tpnvs(self) -> crate::common::Reg<regs::Tpnvs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc TPGS."]
    #[inline(always)]
    pub const fn tpgs(self) -> crate::common::Reg<regs::Tpgs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc TPROG."]
    #[inline(always)]
    pub const fn tprog(self) -> crate::common::Reg<regs::Tprog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc TSERASE."]
    #[inline(always)]
    pub const fn tserase(self) -> crate::common::Reg<regs::Tserase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc TMERASE."]
    #[inline(always)]
    pub const fn tmerase(self) -> crate::common::Reg<regs::Tmerase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc TPRCV."]
    #[inline(always)]
    pub const fn tprcv(self) -> crate::common::Reg<regs::Tprcv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc TSRCV."]
    #[inline(always)]
    pub const fn tsrcv(self) -> crate::common::Reg<regs::Tsrcv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc TMRCV."]
    #[inline(always)]
    pub const fn tmrcv(self) -> crate::common::Reg<regs::Tmrcv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc IFR."]
    #[inline(always)]
    pub const fn ifr(self) -> crate::common::Reg<regs::Ifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc ICLR."]
    #[inline(always)]
    pub const fn iclr(self) -> crate::common::Reg<regs::Iclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc BYPASS."]
    #[inline(always)]
    pub const fn bypass(self) -> crate::common::Reg<regs::Bypass, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc SLOCK0."]
    #[inline(always)]
    pub const fn slock0(self) -> crate::common::Reg<regs::Slock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc SLOCK1."]
    #[inline(always)]
    pub const fn slock1(self) -> crate::common::Reg<regs::Slock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc SLOCK2."]
    #[inline(always)]
    pub const fn slock2(self) -> crate::common::Reg<regs::Slock2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc SLOCK3."]
    #[inline(always)]
    pub const fn slock3(self) -> crate::common::Reg<regs::Slock3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc TSNVS."]
    #[inline(always)]
    pub const fn tsnvs(self) -> crate::common::Reg<regs::Tsnvs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc TMNVS."]
    #[inline(always)]
    pub const fn tmnvs(self) -> crate::common::Reg<regs::Tmnvs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc TADS."]
    #[inline(always)]
    pub const fn tads(self) -> crate::common::Reg<regs::Tads, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc TADH."]
    #[inline(always)]
    pub const fn tadh(self) -> crate::common::Reg<regs::Tadh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc TPGH."]
    #[inline(always)]
    pub const fn tpgh(self) -> crate::common::Reg<regs::Tpgh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc TRW1."]
    #[inline(always)]
    pub const fn trw1(self) -> crate::common::Reg<regs::Trw1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "desc TRW2."]
    #[inline(always)]
    pub const fn trw2(self) -> crate::common::Reg<regs::Trw2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "desc LOCK_ST."]
    #[inline(always)]
    pub const fn lock_st(self) -> crate::common::Reg<regs::LockSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc BYPASS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bypass(pub u32);
    impl Bypass {
        #[doc = "desc BYSEQ."]
        #[must_use]
        #[inline(always)]
        pub const fn byseq(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc BYSEQ."]
        #[inline(always)]
        pub const fn set_byseq(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Bypass {
        #[inline(always)]
        fn default() -> Bypass {
            Bypass(0)
        }
    }
    impl core::fmt::Debug for Bypass {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bypass")
                .field("byseq", &self.byseq())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc OP."]
        #[must_use]
        #[inline(always)]
        pub const fn op(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc OP."]
        #[inline(always)]
        pub const fn set_op(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc WAIT."]
        #[must_use]
        #[inline(always)]
        pub const fn wait(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc WAIT."]
        #[inline(always)]
        pub const fn set_wait(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc BUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSY."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "desc DPSTB_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn dpstb_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPSTB_EN."]
        #[inline(always)]
        pub const fn set_dpstb_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CONTP."]
        #[must_use]
        #[inline(always)]
        pub const fn contp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CONTP."]
        #[inline(always)]
        pub const fn set_contp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RO."]
        #[must_use]
        #[inline(always)]
        pub const fn ro(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RO."]
        #[inline(always)]
        pub const fn set_ro(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
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
                .field("op", &self.op())
                .field("wait", &self.wait())
                .field("busy", &self.busy())
                .field("ie", &self.ie())
                .field("dpstb_en", &self.dpstb_en())
                .field("contp", &self.contp())
                .field("ro", &self.ro())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc ICLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iclr(pub u32);
    impl Iclr {
        #[doc = "desc PC."]
        #[must_use]
        #[inline(always)]
        pub const fn pc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC."]
        #[inline(always)]
        pub const fn set_pc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PROT."]
        #[must_use]
        #[inline(always)]
        pub const fn prot(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PROT."]
        #[inline(always)]
        pub const fn set_prot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Iclr {
        #[inline(always)]
        fn default() -> Iclr {
            Iclr(0)
        }
    }
    impl core::fmt::Debug for Iclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iclr")
                .field("pc", &self.pc())
                .field("prot", &self.prot())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc IFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifr(pub u32);
    impl Ifr {
        #[doc = "desc PC."]
        #[must_use]
        #[inline(always)]
        pub const fn pc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC."]
        #[inline(always)]
        pub const fn set_pc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PROT."]
        #[must_use]
        #[inline(always)]
        pub const fn prot(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PROT."]
        #[inline(always)]
        pub const fn set_prot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
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
                .field("pc", &self.pc())
                .field("prot", &self.prot())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc LOCK_ST."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LockSt(pub u32);
    impl LockSt {
        #[doc = "desc LOCK_ST."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_st(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc LOCK_ST."]
        #[inline(always)]
        pub const fn set_lock_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for LockSt {
        #[inline(always)]
        fn default() -> LockSt {
            LockSt(0)
        }
    }
    impl core::fmt::Debug for LockSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LockSt")
                .field("lock_st", &self.lock_st())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SLOCK0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Slock0(pub u32);
    impl Slock0 {
        #[doc = "desc SLOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn slock(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SLOCK."]
        #[inline(always)]
        pub const fn set_slock(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Slock0 {
        #[inline(always)]
        fn default() -> Slock0 {
            Slock0(0)
        }
    }
    impl core::fmt::Debug for Slock0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Slock0")
                .field("slock", &self.slock())
                .finish()
        }
    }
    #[doc = "desc SLOCK1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Slock1(pub u32);
    impl Slock1 {
        #[doc = "desc SLOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn slock(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SLOCK."]
        #[inline(always)]
        pub const fn set_slock(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Slock1 {
        #[inline(always)]
        fn default() -> Slock1 {
            Slock1(0)
        }
    }
    impl core::fmt::Debug for Slock1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Slock1")
                .field("slock", &self.slock())
                .finish()
        }
    }
    #[doc = "desc SLOCK2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Slock2(pub u32);
    impl Slock2 {
        #[doc = "desc SLOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn slock(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SLOCK."]
        #[inline(always)]
        pub const fn set_slock(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Slock2 {
        #[inline(always)]
        fn default() -> Slock2 {
            Slock2(0)
        }
    }
    impl core::fmt::Debug for Slock2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Slock2")
                .field("slock", &self.slock())
                .finish()
        }
    }
    #[doc = "desc SLOCK3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Slock3(pub u32);
    impl Slock3 {
        #[doc = "desc SLOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn slock(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SLOCK."]
        #[inline(always)]
        pub const fn set_slock(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Slock3 {
        #[inline(always)]
        fn default() -> Slock3 {
            Slock3(0)
        }
    }
    impl core::fmt::Debug for Slock3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Slock3")
                .field("slock", &self.slock())
                .finish()
        }
    }
    #[doc = "desc TADH."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tadh(pub u32);
    impl Tadh {
        #[doc = "desc TADH."]
        #[must_use]
        #[inline(always)]
        pub const fn tadh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc TADH."]
        #[inline(always)]
        pub const fn set_tadh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 6usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
        }
    }
    impl Default for Tadh {
        #[inline(always)]
        fn default() -> Tadh {
            Tadh(0)
        }
    }
    impl core::fmt::Debug for Tadh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tadh")
                .field("tadh", &self.tadh())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TADS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tads(pub u32);
    impl Tads {
        #[doc = "desc TADS."]
        #[must_use]
        #[inline(always)]
        pub const fn tads(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc TADS."]
        #[inline(always)]
        pub const fn set_tads(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 6usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
        }
    }
    impl Default for Tads {
        #[inline(always)]
        fn default() -> Tads {
            Tads(0)
        }
    }
    impl core::fmt::Debug for Tads {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tads")
                .field("tads", &self.tads())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TMERASE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmerase(pub u32);
    impl Tmerase {
        #[doc = "desc TMERASE."]
        #[must_use]
        #[inline(always)]
        pub const fn tmerase(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x001f_ffff;
            val as u32
        }
        #[doc = "desc TMERASE."]
        #[inline(always)]
        pub const fn set_tmerase(&mut self, val: u32) {
            self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u16 {
            let val = (self.0 >> 21usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
        }
    }
    impl Default for Tmerase {
        #[inline(always)]
        fn default() -> Tmerase {
            Tmerase(0)
        }
    }
    impl core::fmt::Debug for Tmerase {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmerase")
                .field("tmerase", &self.tmerase())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TMNVS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmnvs(pub u32);
    impl Tmnvs {
        #[doc = "desc TMNVS."]
        #[must_use]
        #[inline(always)]
        pub const fn tmnvs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc TMNVS."]
        #[inline(always)]
        pub const fn set_tmnvs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 13usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x0007_ffff << 13usize)) | (((val as u32) & 0x0007_ffff) << 13usize);
        }
    }
    impl Default for Tmnvs {
        #[inline(always)]
        fn default() -> Tmnvs {
            Tmnvs(0)
        }
    }
    impl core::fmt::Debug for Tmnvs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmnvs")
                .field("tmnvs", &self.tmnvs())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TMRCV."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmrcv(pub u32);
    impl Tmrcv {
        #[doc = "desc TMRCV."]
        #[must_use]
        #[inline(always)]
        pub const fn tmrcv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "desc TMRCV."]
        #[inline(always)]
        pub const fn set_tmrcv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 14usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x0003_ffff << 14usize)) | (((val as u32) & 0x0003_ffff) << 14usize);
        }
    }
    impl Default for Tmrcv {
        #[inline(always)]
        fn default() -> Tmrcv {
            Tmrcv(0)
        }
    }
    impl core::fmt::Debug for Tmrcv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmrcv")
                .field("tmrcv", &self.tmrcv())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TPGH."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tpgh(pub u32);
    impl Tpgh {
        #[doc = "desc TPGH."]
        #[must_use]
        #[inline(always)]
        pub const fn tpgh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc TPGH."]
        #[inline(always)]
        pub const fn set_tpgh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 6usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
        }
    }
    impl Default for Tpgh {
        #[inline(always)]
        fn default() -> Tpgh {
            Tpgh(0)
        }
    }
    impl core::fmt::Debug for Tpgh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tpgh")
                .field("tpgh", &self.tpgh())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TPGS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tpgs(pub u32);
    impl Tpgs {
        #[doc = "desc TPGS."]
        #[must_use]
        #[inline(always)]
        pub const fn tpgs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc TPGS."]
        #[inline(always)]
        pub const fn set_tpgs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Tpgs {
        #[inline(always)]
        fn default() -> Tpgs {
            Tpgs(0)
        }
    }
    impl core::fmt::Debug for Tpgs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tpgs")
                .field("tpgs", &self.tpgs())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TPNVS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tpnvs(pub u32);
    impl Tpnvs {
        #[doc = "desc TPNVS."]
        #[must_use]
        #[inline(always)]
        pub const fn tpnvs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc TPNVS."]
        #[inline(always)]
        pub const fn set_tpnvs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 11usize) & 0x001f_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
        }
    }
    impl Default for Tpnvs {
        #[inline(always)]
        fn default() -> Tpnvs {
            Tpnvs(0)
        }
    }
    impl core::fmt::Debug for Tpnvs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tpnvs")
                .field("tpnvs", &self.tpnvs())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TPRCV."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tprcv(pub u32);
    impl Tprcv {
        #[doc = "desc TPRCV."]
        #[must_use]
        #[inline(always)]
        pub const fn tprcv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc TPRCV."]
        #[inline(always)]
        pub const fn set_tprcv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Tprcv {
        #[inline(always)]
        fn default() -> Tprcv {
            Tprcv(0)
        }
    }
    impl core::fmt::Debug for Tprcv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tprcv")
                .field("tprcv", &self.tprcv())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TPROG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tprog(pub u32);
    impl Tprog {
        #[doc = "desc TPROG."]
        #[must_use]
        #[inline(always)]
        pub const fn tprog(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TPROG."]
        #[inline(always)]
        pub const fn set_tprog(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 9usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
        }
    }
    impl Default for Tprog {
        #[inline(always)]
        fn default() -> Tprog {
            Tprog(0)
        }
    }
    impl core::fmt::Debug for Tprog {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tprog")
                .field("tprog", &self.tprog())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TRW1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trw1(pub u32);
    impl Trw1 {
        #[doc = "desc TRW1."]
        #[must_use]
        #[inline(always)]
        pub const fn trw1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc TRW1."]
        #[inline(always)]
        pub const fn set_trw1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 6usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
        }
    }
    impl Default for Trw1 {
        #[inline(always)]
        fn default() -> Trw1 {
            Trw1(0)
        }
    }
    impl core::fmt::Debug for Trw1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trw1")
                .field("trw1", &self.trw1())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TRW2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trw2(pub u32);
    impl Trw2 {
        #[doc = "desc TRW2."]
        #[must_use]
        #[inline(always)]
        pub const fn trw2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc TRW2."]
        #[inline(always)]
        pub const fn set_trw2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
        }
    }
    impl Default for Trw2 {
        #[inline(always)]
        fn default() -> Trw2 {
            Trw2(0)
        }
    }
    impl core::fmt::Debug for Trw2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trw2")
                .field("trw2", &self.trw2())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TSERASE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tserase(pub u32);
    impl Tserase {
        #[doc = "desc TSERASE."]
        #[must_use]
        #[inline(always)]
        pub const fn tserase(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "desc TSERASE."]
        #[inline(always)]
        pub const fn set_tserase(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x7fff;
            val as u16
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
        }
    }
    impl Default for Tserase {
        #[inline(always)]
        fn default() -> Tserase {
            Tserase(0)
        }
    }
    impl core::fmt::Debug for Tserase {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tserase")
                .field("tserase", &self.tserase())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TSNVS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsnvs(pub u32);
    impl Tsnvs {
        #[doc = "desc TSNVS."]
        #[must_use]
        #[inline(always)]
        pub const fn tsnvs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc TSNVS."]
        #[inline(always)]
        pub const fn set_tsnvs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 11usize) & 0x001f_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
        }
    }
    impl Default for Tsnvs {
        #[inline(always)]
        fn default() -> Tsnvs {
            Tsnvs(0)
        }
    }
    impl core::fmt::Debug for Tsnvs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tsnvs")
                .field("tsnvs", &self.tsnvs())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TSRCV."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsrcv(pub u32);
    impl Tsrcv {
        #[doc = "desc TSRCV."]
        #[must_use]
        #[inline(always)]
        pub const fn tsrcv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc TSRCV."]
        #[inline(always)]
        pub const fn set_tsrcv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Tsrcv {
        #[inline(always)]
        fn default() -> Tsrcv {
            Tsrcv(0)
        }
    }
    impl core::fmt::Debug for Tsrcv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tsrcv")
                .field("tsrcv", &self.tsrcv())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
