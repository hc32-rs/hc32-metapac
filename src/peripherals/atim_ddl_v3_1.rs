#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc ATIM0MODE23."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atim {
    ptr: *mut u8,
}
unsafe impl Send for Atim {}
unsafe impl Sync for Atim {}
impl Atim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc ARR."]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CNT."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc M23CR."]
    #[inline(always)]
    pub const fn m23cr(self) -> crate::common::Reg<regs::M23cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc IFR."]
    #[inline(always)]
    pub const fn ifr(self) -> crate::common::Reg<regs::Ifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc ICLR."]
    #[inline(always)]
    pub const fn iclr(self) -> crate::common::Reg<regs::Iclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc MSCR."]
    #[inline(always)]
    pub const fn mscr(self) -> crate::common::Reg<regs::Mscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc FLTR."]
    #[inline(always)]
    pub const fn fltr(self) -> crate::common::Reg<regs::Fltr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc ADTR."]
    #[inline(always)]
    pub const fn adtr(self) -> crate::common::Reg<regs::Adtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc CRCH0."]
    #[inline(always)]
    pub const fn crch0(self) -> crate::common::Reg<regs::Crch0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc DTR."]
    #[inline(always)]
    pub const fn dtr(self) -> crate::common::Reg<regs::Dtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc RCR."]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc CCR0A."]
    #[inline(always)]
    pub const fn ccr0a(self) -> crate::common::Reg<regs::Ccr0a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc CCR0B."]
    #[inline(always)]
    pub const fn ccr0b(self) -> crate::common::Reg<regs::Ccr0b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ADTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adtr(pub u32);
    impl Adtr {
        #[doc = "desc UEV."]
        #[must_use]
        #[inline(always)]
        pub const fn uev(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc UEV."]
        #[inline(always)]
        pub const fn set_uev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CMA0."]
        #[must_use]
        #[inline(always)]
        pub const fn cma0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMA0."]
        #[inline(always)]
        pub const fn set_cma0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CMB0."]
        #[must_use]
        #[inline(always)]
        pub const fn cmb0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMB0."]
        #[inline(always)]
        pub const fn set_cmb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ADT."]
        #[must_use]
        #[inline(always)]
        pub const fn adt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADT."]
        #[inline(always)]
        pub const fn set_adt(&mut self, val: bool) {
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
    impl Default for Adtr {
        #[inline(always)]
        fn default() -> Adtr {
            Adtr(0)
        }
    }
    impl core::fmt::Debug for Adtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adtr")
                .field("uev", &self.uev())
                .field("cma0", &self.cma0())
                .field("cmb0", &self.cmb0())
                .field("adt", &self.adt())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc ARR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Arr(pub u32);
    impl Arr {
        #[doc = "desc ARR."]
        #[must_use]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc ARR."]
        #[inline(always)]
        pub const fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
    impl Default for Arr {
        #[inline(always)]
        fn default() -> Arr {
            Arr(0)
        }
    }
    impl core::fmt::Debug for Arr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Arr")
                .field("arr", &self.arr())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCR0A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr0a(pub u32);
    impl Ccr0a {
        #[doc = "desc CCR0A."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr0a(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR0A."]
        #[inline(always)]
        pub const fn set_ccr0a(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
    impl Default for Ccr0a {
        #[inline(always)]
        fn default() -> Ccr0a {
            Ccr0a(0)
        }
    }
    impl core::fmt::Debug for Ccr0a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr0a")
                .field("ccr0a", &self.ccr0a())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCR0B."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr0b(pub u32);
    impl Ccr0b {
        #[doc = "desc CCR0B."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr0b(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR0B."]
        #[inline(always)]
        pub const fn set_ccr0b(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
    impl Default for Ccr0b {
        #[inline(always)]
        fn default() -> Ccr0b {
            Ccr0b(0)
        }
    }
    impl core::fmt::Debug for Ccr0b {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr0b")
                .field("ccr0b", &self.ccr0b())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CNT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    impl core::fmt::Debug for Cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt")
                .field("cnt", &self.cnt())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CRCH0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crch0(pub u32);
    impl Crch0 {
        #[doc = "desc BKSACFACRA."]
        #[must_use]
        #[inline(always)]
        pub const fn bksacfacra(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc BKSACFACRA."]
        #[inline(always)]
        pub const fn set_bksacfacra(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc BKSBCFBCRB."]
        #[must_use]
        #[inline(always)]
        pub const fn bksbcfbcrb(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc BKSBCFBCRB."]
        #[inline(always)]
        pub const fn set_bksbcfbcrb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc CSA."]
        #[must_use]
        #[inline(always)]
        pub const fn csa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSA."]
        #[inline(always)]
        pub const fn set_csa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CSB."]
        #[must_use]
        #[inline(always)]
        pub const fn csb(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSB."]
        #[inline(always)]
        pub const fn set_csb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc BUFEA."]
        #[must_use]
        #[inline(always)]
        pub const fn bufea(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUFEA."]
        #[inline(always)]
        pub const fn set_bufea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BUFEB."]
        #[must_use]
        #[inline(always)]
        pub const fn bufeb(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUFEB."]
        #[inline(always)]
        pub const fn set_bufeb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CIEA."]
        #[must_use]
        #[inline(always)]
        pub const fn ciea(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIEA."]
        #[inline(always)]
        pub const fn set_ciea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CIEB."]
        #[must_use]
        #[inline(always)]
        pub const fn cieb(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIEB."]
        #[inline(always)]
        pub const fn set_cieb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CDEA."]
        #[must_use]
        #[inline(always)]
        pub const fn cdea(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDEA."]
        #[inline(always)]
        pub const fn set_cdea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc CDEB."]
        #[must_use]
        #[inline(always)]
        pub const fn cdeb(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDEB."]
        #[inline(always)]
        pub const fn set_cdeb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CISB."]
        #[must_use]
        #[inline(always)]
        pub const fn cisb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc CISB."]
        #[inline(always)]
        pub const fn set_cisb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc CCGA."]
        #[must_use]
        #[inline(always)]
        pub const fn ccga(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCGA."]
        #[inline(always)]
        pub const fn set_ccga(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc CCGB."]
        #[must_use]
        #[inline(always)]
        pub const fn ccgb(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCGB."]
        #[inline(always)]
        pub const fn set_ccgb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
    impl Default for Crch0 {
        #[inline(always)]
        fn default() -> Crch0 {
            Crch0(0)
        }
    }
    impl core::fmt::Debug for Crch0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crch0")
                .field("bksacfacra", &self.bksacfacra())
                .field("bksbcfbcrb", &self.bksbcfbcrb())
                .field("csa", &self.csa())
                .field("csb", &self.csb())
                .field("bufea", &self.bufea())
                .field("bufeb", &self.bufeb())
                .field("ciea", &self.ciea())
                .field("cieb", &self.cieb())
                .field("cdea", &self.cdea())
                .field("cdeb", &self.cdeb())
                .field("cisb", &self.cisb())
                .field("ccga", &self.ccga())
                .field("ccgb", &self.ccgb())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc DTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtr(pub u32);
    impl Dtr {
        #[doc = "desc DTR."]
        #[must_use]
        #[inline(always)]
        pub const fn dtr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DTR."]
        #[inline(always)]
        pub const fn set_dtr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc BKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn bksel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKSEL."]
        #[inline(always)]
        pub const fn set_bksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc DT."]
        #[must_use]
        #[inline(always)]
        pub const fn dt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc DT."]
        #[inline(always)]
        pub const fn set_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BK."]
        #[must_use]
        #[inline(always)]
        pub const fn bk(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BK."]
        #[inline(always)]
        pub const fn set_bk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc AO."]
        #[must_use]
        #[inline(always)]
        pub const fn ao(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc AO."]
        #[inline(always)]
        pub const fn set_ao(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc MO."]
        #[must_use]
        #[inline(always)]
        pub const fn mo(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc MO."]
        #[inline(always)]
        pub const fn set_mo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc SAFE."]
        #[must_use]
        #[inline(always)]
        pub const fn safe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc SAFE."]
        #[inline(always)]
        pub const fn set_safe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VC."]
        #[must_use]
        #[inline(always)]
        pub const fn vc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VC."]
        #[inline(always)]
        pub const fn set_vc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
    impl Default for Dtr {
        #[inline(always)]
        fn default() -> Dtr {
            Dtr(0)
        }
    }
    impl core::fmt::Debug for Dtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtr")
                .field("dtr", &self.dtr())
                .field("bksel", &self.bksel())
                .field("dt", &self.dt())
                .field("bk", &self.bk())
                .field("ao", &self.ao())
                .field("mo", &self.mo())
                .field("safe", &self.safe())
                .field("vc", &self.vc())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc FLTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fltr(pub u32);
    impl Fltr {
        #[doc = "desc OCMA0FLTA0."]
        #[must_use]
        #[inline(always)]
        pub const fn ocma0flta0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc OCMA0FLTA0."]
        #[inline(always)]
        pub const fn set_ocma0flta0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc CCPA0."]
        #[must_use]
        #[inline(always)]
        pub const fn ccpa0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCPA0."]
        #[inline(always)]
        pub const fn set_ccpa0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OCMB0FLTB0."]
        #[must_use]
        #[inline(always)]
        pub const fn ocmb0fltb0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc OCMB0FLTB0."]
        #[inline(always)]
        pub const fn set_ocmb0fltb0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc CCPB0."]
        #[must_use]
        #[inline(always)]
        pub const fn ccpb0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCPB0."]
        #[inline(always)]
        pub const fn set_ccpb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc FLTBK."]
        #[must_use]
        #[inline(always)]
        pub const fn fltbk(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc FLTBK."]
        #[inline(always)]
        pub const fn set_fltbk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "desc BKP."]
        #[must_use]
        #[inline(always)]
        pub const fn bkp(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKP."]
        #[inline(always)]
        pub const fn set_bkp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc FLTET."]
        #[must_use]
        #[inline(always)]
        pub const fn fltet(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "desc FLTET."]
        #[inline(always)]
        pub const fn set_fltet(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[doc = "desc ETP."]
        #[must_use]
        #[inline(always)]
        pub const fn etp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc ETP."]
        #[inline(always)]
        pub const fn set_etp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Fltr {
        #[inline(always)]
        fn default() -> Fltr {
            Fltr(0)
        }
    }
    impl core::fmt::Debug for Fltr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fltr")
                .field("ocma0flta0", &self.ocma0flta0())
                .field("ccpa0", &self.ccpa0())
                .field("ocmb0fltb0", &self.ocmb0fltb0())
                .field("ccpb0", &self.ccpb0())
                .field("fltbk", &self.fltbk())
                .field("bkp", &self.bkp())
                .field("fltet", &self.fltet())
                .field("etp", &self.etp())
                .finish()
        }
    }
    #[doc = "desc ICLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iclr(pub u32);
    impl Iclr {
        #[doc = "desc UIF."]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc UIF."]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CA0F."]
        #[must_use]
        #[inline(always)]
        pub const fn ca0f(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CA0F."]
        #[inline(always)]
        pub const fn set_ca0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CB0F."]
        #[must_use]
        #[inline(always)]
        pub const fn cb0f(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CB0F."]
        #[inline(always)]
        pub const fn set_cb0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc CA0E."]
        #[must_use]
        #[inline(always)]
        pub const fn ca0e(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CA0E."]
        #[inline(always)]
        pub const fn set_ca0e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CB0E."]
        #[must_use]
        #[inline(always)]
        pub const fn cb0e(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CB0E."]
        #[inline(always)]
        pub const fn set_cb0e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BI."]
        #[must_use]
        #[inline(always)]
        pub const fn bi(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc BI."]
        #[inline(always)]
        pub const fn set_bi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc TI."]
        #[must_use]
        #[inline(always)]
        pub const fn ti(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc TI."]
        #[inline(always)]
        pub const fn set_ti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc OV."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc OV."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc UND."]
        #[must_use]
        #[inline(always)]
        pub const fn und(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc UND."]
        #[inline(always)]
        pub const fn set_und(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc CA3."]
        #[must_use]
        #[inline(always)]
        pub const fn ca3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc CA3."]
        #[inline(always)]
        pub const fn set_ca3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
    impl Default for Iclr {
        #[inline(always)]
        fn default() -> Iclr {
            Iclr(0)
        }
    }
    impl core::fmt::Debug for Iclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iclr")
                .field("uif", &self.uif())
                .field("ca0f", &self.ca0f())
                .field("cb0f", &self.cb0f())
                .field("ca0e", &self.ca0e())
                .field("cb0e", &self.cb0e())
                .field("bi", &self.bi())
                .field("ti", &self.ti())
                .field("ov", &self.ov())
                .field("und", &self.und())
                .field("ca3", &self.ca3())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc IFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifr(pub u32);
    impl Ifr {
        #[doc = "desc UIF."]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc UIF."]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CA0F."]
        #[must_use]
        #[inline(always)]
        pub const fn ca0f(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CA0F."]
        #[inline(always)]
        pub const fn set_ca0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CB0F."]
        #[must_use]
        #[inline(always)]
        pub const fn cb0f(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CB0F."]
        #[inline(always)]
        pub const fn set_cb0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc CA0E."]
        #[must_use]
        #[inline(always)]
        pub const fn ca0e(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CA0E."]
        #[inline(always)]
        pub const fn set_ca0e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CB0E."]
        #[must_use]
        #[inline(always)]
        pub const fn cb0e(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CB0E."]
        #[inline(always)]
        pub const fn set_cb0e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BI."]
        #[must_use]
        #[inline(always)]
        pub const fn bi(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc BI."]
        #[inline(always)]
        pub const fn set_bi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc TI."]
        #[must_use]
        #[inline(always)]
        pub const fn ti(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc TI."]
        #[inline(always)]
        pub const fn set_ti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc OV."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc OV."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc UND."]
        #[must_use]
        #[inline(always)]
        pub const fn und(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc UND."]
        #[inline(always)]
        pub const fn set_und(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc CA3."]
        #[must_use]
        #[inline(always)]
        pub const fn ca3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc CA3."]
        #[inline(always)]
        pub const fn set_ca3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("uif", &self.uif())
                .field("ca0f", &self.ca0f())
                .field("cb0f", &self.cb0f())
                .field("ca0e", &self.ca0e())
                .field("cb0e", &self.cb0e())
                .field("bi", &self.bi())
                .field("ti", &self.ti())
                .field("ov", &self.ov())
                .field("und", &self.und())
                .field("ca3", &self.ca3())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc M23CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M23cr(pub u32);
    impl M23cr {
        #[doc = "desc CTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTEN."]
        #[inline(always)]
        pub const fn set_cten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc COMP."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMP."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CT."]
        #[must_use]
        #[inline(always)]
        pub const fn ct(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CT."]
        #[inline(always)]
        pub const fn set_ct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PWM2S."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm2s(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWM2S."]
        #[inline(always)]
        pub const fn set_pwm2s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PRS."]
        #[must_use]
        #[inline(always)]
        pub const fn prs(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc PRS."]
        #[inline(always)]
        pub const fn set_prs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc BUFP."]
        #[must_use]
        #[inline(always)]
        pub const fn bufp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUFP."]
        #[inline(always)]
        pub const fn set_bufp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CRG."]
        #[must_use]
        #[inline(always)]
        pub const fn crg(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRG."]
        #[inline(always)]
        pub const fn set_crg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CFG."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc UI."]
        #[must_use]
        #[inline(always)]
        pub const fn ui(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc UI."]
        #[inline(always)]
        pub const fn set_ui(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc UD."]
        #[must_use]
        #[inline(always)]
        pub const fn ud(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc UD."]
        #[inline(always)]
        pub const fn set_ud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc ONESHOT."]
        #[must_use]
        #[inline(always)]
        pub const fn oneshot(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc ONESHOT."]
        #[inline(always)]
        pub const fn set_oneshot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc CSG."]
        #[must_use]
        #[inline(always)]
        pub const fn csg(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSG."]
        #[inline(always)]
        pub const fn set_csg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc OCCS."]
        #[must_use]
        #[inline(always)]
        pub const fn occs(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc OCCS."]
        #[inline(always)]
        pub const fn set_occs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc URS."]
        #[must_use]
        #[inline(always)]
        pub const fn urs(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc URS."]
        #[inline(always)]
        pub const fn set_urs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc TD."]
        #[must_use]
        #[inline(always)]
        pub const fn td(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc TD."]
        #[inline(always)]
        pub const fn set_td(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc TI."]
        #[must_use]
        #[inline(always)]
        pub const fn ti(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc TI."]
        #[inline(always)]
        pub const fn set_ti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc BI."]
        #[must_use]
        #[inline(always)]
        pub const fn bi(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc BI."]
        #[inline(always)]
        pub const fn set_bi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc CIS."]
        #[must_use]
        #[inline(always)]
        pub const fn cis(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "desc CIS."]
        #[inline(always)]
        pub const fn set_cis(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "desc OCC."]
        #[must_use]
        #[inline(always)]
        pub const fn occ(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc OCC."]
        #[inline(always)]
        pub const fn set_occ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc TG."]
        #[must_use]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc TG."]
        #[inline(always)]
        pub const fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc UG."]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc UG."]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc BG."]
        #[must_use]
        #[inline(always)]
        pub const fn bg(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc BG."]
        #[inline(always)]
        pub const fn set_bg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc DIR."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIR."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc OV."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc OV."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc UND."]
        #[must_use]
        #[inline(always)]
        pub const fn und(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc UND."]
        #[inline(always)]
        pub const fn set_und(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
    impl Default for M23cr {
        #[inline(always)]
        fn default() -> M23cr {
            M23cr(0)
        }
    }
    impl core::fmt::Debug for M23cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M23cr")
                .field("cten", &self.cten())
                .field("comp", &self.comp())
                .field("ct", &self.ct())
                .field("pwm2s", &self.pwm2s())
                .field("prs", &self.prs())
                .field("bufp", &self.bufp())
                .field("crg", &self.crg())
                .field("cfg", &self.cfg())
                .field("ui", &self.ui())
                .field("ud", &self.ud())
                .field("mode", &self.mode())
                .field("oneshot", &self.oneshot())
                .field("csg", &self.csg())
                .field("occs", &self.occs())
                .field("urs", &self.urs())
                .field("td", &self.td())
                .field("ti", &self.ti())
                .field("bi", &self.bi())
                .field("cis", &self.cis())
                .field("occ", &self.occ())
                .field("tg", &self.tg())
                .field("ug", &self.ug())
                .field("bg", &self.bg())
                .field("dir", &self.dir())
                .field("ov", &self.ov())
                .field("und", &self.und())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc MSCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mscr(pub u32);
    impl Mscr {
        #[doc = "desc MMS."]
        #[must_use]
        #[inline(always)]
        pub const fn mms(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc MMS."]
        #[inline(always)]
        pub const fn set_mms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc CCDS."]
        #[must_use]
        #[inline(always)]
        pub const fn ccds(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCDS."]
        #[inline(always)]
        pub const fn set_ccds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc MSM."]
        #[must_use]
        #[inline(always)]
        pub const fn msm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSM."]
        #[inline(always)]
        pub const fn set_msm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TS."]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "desc TS."]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "desc SMS."]
        #[must_use]
        #[inline(always)]
        pub const fn sms(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc SMS."]
        #[inline(always)]
        pub const fn set_sms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc IA0S."]
        #[must_use]
        #[inline(always)]
        pub const fn ia0s(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc IA0S."]
        #[inline(always)]
        pub const fn set_ia0s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IB0S."]
        #[must_use]
        #[inline(always)]
        pub const fn ib0s(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc IB0S."]
        #[inline(always)]
        pub const fn set_ib0s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
    impl Default for Mscr {
        #[inline(always)]
        fn default() -> Mscr {
            Mscr(0)
        }
    }
    impl core::fmt::Debug for Mscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mscr")
                .field("mms", &self.mms())
                .field("ccds", &self.ccds())
                .field("msm", &self.msm())
                .field("ts", &self.ts())
                .field("sms", &self.sms())
                .field("ia0s", &self.ia0s())
                .field("ib0s", &self.ib0s())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc RCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcr(pub u32);
    impl Rcr {
        #[doc = "desc RCR."]
        #[must_use]
        #[inline(always)]
        pub const fn rcr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc RCR."]
        #[inline(always)]
        pub const fn set_rcr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc OV."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc OV."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc UD."]
        #[must_use]
        #[inline(always)]
        pub const fn ud(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc UD."]
        #[inline(always)]
        pub const fn set_ud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
    impl Default for Rcr {
        #[inline(always)]
        fn default() -> Rcr {
            Rcr(0)
        }
    }
    impl core::fmt::Debug for Rcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rcr")
                .field("rcr", &self.rcr())
                .field("ov", &self.ov())
                .field("ud", &self.ud())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
