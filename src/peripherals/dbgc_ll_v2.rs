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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc MCUSTPCTL."]
    #[inline(always)]
    pub const fn mcustpctl(self) -> crate::common::Reg<regs::Mcustpctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc MCUTRACECTL."]
    #[inline(always)]
    pub const fn mcutracectl(self) -> crate::common::Reg<regs::Mcutracectl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
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
        #[doc = "desc WDTSTP."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtstp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTSTP."]
        #[inline(always)]
        pub const fn set_wdtstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
        #[doc = "desc TMR02STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr02stp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR02STP."]
        #[inline(always)]
        pub const fn set_tmr02stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
        #[doc = "desc TMR42STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr42stp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR42STP."]
        #[inline(always)]
        pub const fn set_tmr42stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TMR43STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr43stp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR43STP."]
        #[inline(always)]
        pub const fn set_tmr43stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc TM61STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tm61stp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc TM61STP."]
        #[inline(always)]
        pub const fn set_tm61stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc TM62STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tm62stp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc TM62STP."]
        #[inline(always)]
        pub const fn set_tm62stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc TMR63STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr63stp(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR63STP."]
        #[inline(always)]
        pub const fn set_tmr63stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc TMRA1STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra1stp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA1STP."]
        #[inline(always)]
        pub const fn set_tmra1stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc TMRA2STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra2stp(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA2STP."]
        #[inline(always)]
        pub const fn set_tmra2stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc TMRA3STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra3stp(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA3STP."]
        #[inline(always)]
        pub const fn set_tmra3stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc TMRA4STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra4stp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA4STP."]
        #[inline(always)]
        pub const fn set_tmra4stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc TMRA5STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra5stp(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA5STP."]
        #[inline(always)]
        pub const fn set_tmra5stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc TMRA6STP."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra6stp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA6STP."]
        #[inline(always)]
        pub const fn set_tmra6stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("wdtstp", &self.wdtstp())
                .field("rtcstp", &self.rtcstp())
                .field("tmr01stp", &self.tmr01stp())
                .field("tmr02stp", &self.tmr02stp())
                .field("tmr41stp", &self.tmr41stp())
                .field("tmr42stp", &self.tmr42stp())
                .field("tmr43stp", &self.tmr43stp())
                .field("tm61stp", &self.tm61stp())
                .field("tm62stp", &self.tm62stp())
                .field("tmr63stp", &self.tmr63stp())
                .field("tmra1stp", &self.tmra1stp())
                .field("tmra2stp", &self.tmra2stp())
                .field("tmra3stp", &self.tmra3stp())
                .field("tmra4stp", &self.tmra4stp())
                .field("tmra5stp", &self.tmra5stp())
                .field("tmra6stp", &self.tmra6stp())
                .finish()
        }
    }
    #[doc = "desc MCUTRACECTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcutracectl(pub u32);
    impl Mcutracectl {
        #[doc = "desc TRACEMODE."]
        #[must_use]
        #[inline(always)]
        pub const fn tracemode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc TRACEMODE."]
        #[inline(always)]
        pub const fn set_tracemode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc TRACEIOEN."]
        #[must_use]
        #[inline(always)]
        pub const fn traceioen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRACEIOEN."]
        #[inline(always)]
        pub const fn set_traceioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Mcutracectl {
        #[inline(always)]
        fn default() -> Mcutracectl {
            Mcutracectl(0)
        }
    }
    impl core::fmt::Debug for Mcutracectl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcutracectl")
                .field("tracemode", &self.tracemode())
                .field("traceioen", &self.traceioen())
                .finish()
        }
    }
}
