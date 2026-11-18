#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc DBGC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgc {
    ptr: *mut u8,
}
unsafe impl Send for Dbgc {}
unsafe impl Sync for Dbgc {}
impl Dbgc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc MCUDBGSTAT."]
    #[inline(always)]
    pub const fn mcudbgstat(self) -> crate::common::Reg<regs::Mcudbgstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc MCUSTPCTL."]
    #[inline(always)]
    pub const fn mcustpctl(self) -> crate::common::Reg<regs::Mcustpctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc MCUDBGSTAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcudbgstat(pub u32);
    impl Mcudbgstat {
        #[doc = "desc CDBGPWRUPREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn cdbgpwrupreq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDBGPWRUPREQ."]
        #[inline(always)]
        pub const fn set_cdbgpwrupreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CDBGPWRUPACK."]
        #[must_use]
        #[inline(always)]
        pub const fn cdbgpwrupack(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDBGPWRUPACK."]
        #[inline(always)]
        pub const fn set_cdbgpwrupack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Mcudbgstat {
        #[inline(always)]
        fn default() -> Mcudbgstat {
            Mcudbgstat(0)
        }
    }
    impl core::fmt::Debug for Mcudbgstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcudbgstat")
                .field("cdbgpwrupreq", &self.cdbgpwrupreq())
                .field("cdbgpwrupack", &self.cdbgpwrupack())
                .finish()
        }
    }
    #[doc = "desc MCUSTPCTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcustpctl(pub u32);
    impl Mcustpctl {
        #[doc = "desc SWDTSTP."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtstp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTSTP."]
        #[inline(always)]
        pub const fn set_swdtstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RTCSTP."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcstp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTCSTP."]
        #[inline(always)]
        pub const fn set_rtcstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc LVD1STP."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1stp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1STP."]
        #[inline(always)]
        pub const fn set_lvd1stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc LVD2STP."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd2stp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD2STP."]
        #[inline(always)]
        pub const fn set_lvd2stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TMR01STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr01stp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR01STP."]
        #[inline(always)]
        pub const fn set_tmr01stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc TMR21STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr21stp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR21STP."]
        #[inline(always)]
        pub const fn set_tmr21stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc TMR22STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr22stp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR22STP."]
        #[inline(always)]
        pub const fn set_tmr22stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc TMR41STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr41stp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR41STP."]
        #[inline(always)]
        pub const fn set_tmr41stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc TMRA1STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra1stp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA1STP."]
        #[inline(always)]
        pub const fn set_tmra1stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Mcustpctl {
        #[inline(always)]
        fn default() -> Mcustpctl {
            Mcustpctl(0)
        }
    }
    impl core::fmt::Debug for Mcustpctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcustpctl")
                .field("swdtstp", &self.swdtstp())
                .field("rtcstp", &self.rtcstp())
                .field("lvd1stp", &self.lvd1stp())
                .field("lvd2stp", &self.lvd2stp())
                .field("tmr01stp", &self.tmr01stp())
                .field("tmr21stp", &self.tmr21stp())
                .field("tmr22stp", &self.tmr22stp())
                .field("tmr41stp", &self.tmr41stp())
                .field("tmra1stp", &self.tmra1stp())
                .finish()
        }
    }
}
