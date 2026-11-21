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
    #[doc = "desc CHIPID."]
    #[inline(always)]
    pub const fn chipid(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc MCUDBGCSTAT."]
    #[inline(always)]
    pub const fn mcudbgcstat(self) -> crate::common::Reg<regs::Mcudbgcstat, crate::common::RW> {
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
    #[doc = "desc MCUSTPCTL2."]
    #[inline(always)]
    pub const fn mcustpctl2(self) -> crate::common::Reg<regs::Mcustpctl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc MCUDBGCSTAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcudbgcstat(pub u32);
    impl Mcudbgcstat {
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
    impl Default for Mcudbgcstat {
        #[inline(always)]
        fn default() -> Mcudbgcstat {
            Mcudbgcstat(0)
        }
    }
    impl core::fmt::Debug for Mcudbgcstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcudbgcstat")
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
        #[doc = "desc M06STP."]
        #[must_use]
        #[inline(always)]
        pub const fn m06stp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc M06STP."]
        #[inline(always)]
        pub const fn set_m06stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc M07STP."]
        #[must_use]
        #[inline(always)]
        pub const fn m07stp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc M07STP."]
        #[inline(always)]
        pub const fn set_m07stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc M12STP."]
        #[must_use]
        #[inline(always)]
        pub const fn m12stp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc M12STP."]
        #[inline(always)]
        pub const fn set_m12stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc M13STP."]
        #[must_use]
        #[inline(always)]
        pub const fn m13stp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc M13STP."]
        #[inline(always)]
        pub const fn set_m13stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc M14STP."]
        #[must_use]
        #[inline(always)]
        pub const fn m14stp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc M14STP."]
        #[inline(always)]
        pub const fn set_m14stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc M15STP."]
        #[must_use]
        #[inline(always)]
        pub const fn m15stp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc M15STP."]
        #[inline(always)]
        pub const fn set_m15stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc M16STP."]
        #[must_use]
        #[inline(always)]
        pub const fn m16stp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc M16STP."]
        #[inline(always)]
        pub const fn set_m16stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("m06stp", &self.m06stp())
                .field("m07stp", &self.m07stp())
                .field("m12stp", &self.m12stp())
                .field("m13stp", &self.m13stp())
                .field("m14stp", &self.m14stp())
                .field("m15stp", &self.m15stp())
                .field("m16stp", &self.m16stp())
                .finish()
        }
    }
    #[doc = "desc MCUSTPCTL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcustpctl2(pub u32);
    impl Mcustpctl2 {
        #[doc = "desc M32STP."]
        #[must_use]
        #[inline(always)]
        pub const fn m32stp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc M32STP."]
        #[inline(always)]
        pub const fn set_m32stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc M33STP."]
        #[must_use]
        #[inline(always)]
        pub const fn m33stp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc M33STP."]
        #[inline(always)]
        pub const fn set_m33stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc M34STP."]
        #[must_use]
        #[inline(always)]
        pub const fn m34stp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc M34STP."]
        #[inline(always)]
        pub const fn set_m34stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc M35STP."]
        #[must_use]
        #[inline(always)]
        pub const fn m35stp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc M35STP."]
        #[inline(always)]
        pub const fn set_m35stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc M36STP."]
        #[must_use]
        #[inline(always)]
        pub const fn m36stp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc M36STP."]
        #[inline(always)]
        pub const fn set_m36stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Mcustpctl2 {
        #[inline(always)]
        fn default() -> Mcustpctl2 {
            Mcustpctl2(0)
        }
    }
    impl core::fmt::Debug for Mcustpctl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcustpctl2")
                .field("m32stp", &self.m32stp())
                .field("m33stp", &self.m33stp())
                .field("m34stp", &self.m34stp())
                .field("m35stp", &self.m35stp())
                .field("m36stp", &self.m36stp())
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
