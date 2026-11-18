#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc DCU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcu {
    ptr: *mut u8,
}
unsafe impl Send for Dcu {}
unsafe impl Sync for Dcu {}
impl Dcu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CTL."]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc FLAG."]
    #[inline(always)]
    pub const fn flag(self) -> crate::common::Reg<regs::Flag, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc DATA0."]
    #[inline(always)]
    pub const fn data0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc DATA1."]
    #[inline(always)]
    pub const fn data1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc DATA2."]
    #[inline(always)]
    pub const fn data2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc FLAGCLR."]
    #[inline(always)]
    pub const fn flagclr(self) -> crate::common::Reg<regs::Flagclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc INTEVTSEL."]
    #[inline(always)]
    pub const fn intevtsel(self) -> crate::common::Reg<regs::Intevtsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctl(pub u32);
    impl Ctl {
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc DATASIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn datasize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc DATASIZE."]
        #[inline(always)]
        pub const fn set_datasize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc COMPTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn comptrg(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPTRG."]
        #[inline(always)]
        pub const fn set_comptrg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc INTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn inten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTEN."]
        #[inline(always)]
        pub const fn set_inten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctl {
        #[inline(always)]
        fn default() -> Ctl {
            Ctl(0)
        }
    }
    impl core::fmt::Debug for Ctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctl")
                .field("mode", &self.mode())
                .field("datasize", &self.datasize())
                .field("comptrg", &self.comptrg())
                .field("inten", &self.inten())
                .finish()
        }
    }
    #[doc = "desc FLAG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flag(pub u32);
    impl Flag {
        #[doc = "desc FLAG_OP."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_op(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLAG_OP."]
        #[inline(always)]
        pub const fn set_flag_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FLAG_LS2."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_ls2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLAG_LS2."]
        #[inline(always)]
        pub const fn set_flag_ls2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FLAG_EQ2."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_eq2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLAG_EQ2."]
        #[inline(always)]
        pub const fn set_flag_eq2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FLAG_GT2."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_gt2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLAG_GT2."]
        #[inline(always)]
        pub const fn set_flag_gt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc FLAG_LS1."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_ls1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLAG_LS1."]
        #[inline(always)]
        pub const fn set_flag_ls1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc FLAG_EQ1."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_eq1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLAG_EQ1."]
        #[inline(always)]
        pub const fn set_flag_eq1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc FLAG_GT1."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_gt1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLAG_GT1."]
        #[inline(always)]
        pub const fn set_flag_gt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc FLAG_RLD."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_rld(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLAG_RLD."]
        #[inline(always)]
        pub const fn set_flag_rld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc FLAG_BTM."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_btm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLAG_BTM."]
        #[inline(always)]
        pub const fn set_flag_btm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc FLAG_TOP."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_top(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLAG_TOP."]
        #[inline(always)]
        pub const fn set_flag_top(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Flag {
        #[inline(always)]
        fn default() -> Flag {
            Flag(0)
        }
    }
    impl core::fmt::Debug for Flag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Flag")
                .field("flag_op", &self.flag_op())
                .field("flag_ls2", &self.flag_ls2())
                .field("flag_eq2", &self.flag_eq2())
                .field("flag_gt2", &self.flag_gt2())
                .field("flag_ls1", &self.flag_ls1())
                .field("flag_eq1", &self.flag_eq1())
                .field("flag_gt1", &self.flag_gt1())
                .field("flag_rld", &self.flag_rld())
                .field("flag_btm", &self.flag_btm())
                .field("flag_top", &self.flag_top())
                .finish()
        }
    }
    #[doc = "desc FLAGCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flagclr(pub u32);
    impl Flagclr {
        #[doc = "desc CLR_OP."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_op(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLR_OP."]
        #[inline(always)]
        pub const fn set_clr_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CLR_LS2."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_ls2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLR_LS2."]
        #[inline(always)]
        pub const fn set_clr_ls2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CLR_EQ2."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_eq2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLR_EQ2."]
        #[inline(always)]
        pub const fn set_clr_eq2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CLR_GT2."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_gt2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLR_GT2."]
        #[inline(always)]
        pub const fn set_clr_gt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CLR_LS1."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_ls1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLR_LS1."]
        #[inline(always)]
        pub const fn set_clr_ls1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CLR_EQ1."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_eq1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLR_EQ1."]
        #[inline(always)]
        pub const fn set_clr_eq1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc CLR_GT1."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_gt1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLR_GT1."]
        #[inline(always)]
        pub const fn set_clr_gt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc CLR_RLD."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_rld(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLR_RLD."]
        #[inline(always)]
        pub const fn set_clr_rld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CLR_BTM."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_btm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLR_BTM."]
        #[inline(always)]
        pub const fn set_clr_btm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc CLR_TOP."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_top(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLR_TOP."]
        #[inline(always)]
        pub const fn set_clr_top(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Flagclr {
        #[inline(always)]
        fn default() -> Flagclr {
            Flagclr(0)
        }
    }
    impl core::fmt::Debug for Flagclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Flagclr")
                .field("clr_op", &self.clr_op())
                .field("clr_ls2", &self.clr_ls2())
                .field("clr_eq2", &self.clr_eq2())
                .field("clr_gt2", &self.clr_gt2())
                .field("clr_ls1", &self.clr_ls1())
                .field("clr_eq1", &self.clr_eq1())
                .field("clr_gt1", &self.clr_gt1())
                .field("clr_rld", &self.clr_rld())
                .field("clr_btm", &self.clr_btm())
                .field("clr_top", &self.clr_top())
                .finish()
        }
    }
    #[doc = "desc INTEVTSEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intevtsel(pub u32);
    impl Intevtsel {
        #[doc = "desc SEL_OP."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_op(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEL_OP."]
        #[inline(always)]
        pub const fn set_sel_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SEL_LS2."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_ls2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEL_LS2."]
        #[inline(always)]
        pub const fn set_sel_ls2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SEL_EQ2."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_eq2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEL_EQ2."]
        #[inline(always)]
        pub const fn set_sel_eq2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SEL_GT2."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_gt2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEL_GT2."]
        #[inline(always)]
        pub const fn set_sel_gt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SEL_LS1."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_ls1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEL_LS1."]
        #[inline(always)]
        pub const fn set_sel_ls1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SEL_EQ1."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_eq1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEL_EQ1."]
        #[inline(always)]
        pub const fn set_sel_eq1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SEL_GT1."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_gt1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEL_GT1."]
        #[inline(always)]
        pub const fn set_sel_gt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SEL_WIN."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_win(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "desc SEL_WIN."]
        #[inline(always)]
        pub const fn set_sel_win(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "desc SEL_RLD."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_rld(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEL_RLD."]
        #[inline(always)]
        pub const fn set_sel_rld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc SEL_BTM."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_btm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEL_BTM."]
        #[inline(always)]
        pub const fn set_sel_btm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc SEL_TOP."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_top(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEL_TOP."]
        #[inline(always)]
        pub const fn set_sel_top(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Intevtsel {
        #[inline(always)]
        fn default() -> Intevtsel {
            Intevtsel(0)
        }
    }
    impl core::fmt::Debug for Intevtsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intevtsel")
                .field("sel_op", &self.sel_op())
                .field("sel_ls2", &self.sel_ls2())
                .field("sel_eq2", &self.sel_eq2())
                .field("sel_gt2", &self.sel_gt2())
                .field("sel_ls1", &self.sel_ls1())
                .field("sel_eq1", &self.sel_eq1())
                .field("sel_gt1", &self.sel_gt1())
                .field("sel_win", &self.sel_win())
                .field("sel_rld", &self.sel_rld())
                .field("sel_btm", &self.sel_btm())
                .field("sel_top", &self.sel_top())
                .finish()
        }
    }
}
