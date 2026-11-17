#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc DEBUG_ACTIVE."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugActive {
    ptr: *mut u8,
}
unsafe impl Send for DebugActive {}
unsafe impl Sync for DebugActive {}
impl DebugActive {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc DEBUG_ACTIVE."]
    #[inline(always)]
    pub const fn debug_active(self) -> crate::common::Reg<regs::DebugActive, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc DEBUG_ACTIVE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DebugActive(pub u32);
    impl DebugActive {
        #[doc = "desc TIM0."]
        #[must_use]
        #[inline(always)]
        pub const fn tim0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM0."]
        #[inline(always)]
        pub const fn set_tim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TIM1."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM1."]
        #[inline(always)]
        pub const fn set_tim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TIM2."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM2."]
        #[inline(always)]
        pub const fn set_tim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TIM4."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM4."]
        #[inline(always)]
        pub const fn set_tim4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TIM5."]
        #[must_use]
        #[inline(always)]
        pub const fn tim5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM5."]
        #[inline(always)]
        pub const fn set_tim5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TIM6."]
        #[must_use]
        #[inline(always)]
        pub const fn tim6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM6."]
        #[inline(always)]
        pub const fn set_tim6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PCA."]
        #[must_use]
        #[inline(always)]
        pub const fn pca(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCA."]
        #[inline(always)]
        pub const fn set_pca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc WDT."]
        #[must_use]
        #[inline(always)]
        pub const fn wdt(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDT."]
        #[inline(always)]
        pub const fn set_wdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc TIM3."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM3."]
        #[inline(always)]
        pub const fn set_tim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
    impl Default for DebugActive {
        #[inline(always)]
        fn default() -> DebugActive {
            DebugActive(0)
        }
    }
    impl core::fmt::Debug for DebugActive {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DebugActive")
                .field("tim0", &self.tim0())
                .field("tim1", &self.tim1())
                .field("tim2", &self.tim2())
                .field("tim4", &self.tim4())
                .field("tim5", &self.tim5())
                .field("tim6", &self.tim6())
                .field("pca", &self.pca())
                .field("wdt", &self.wdt())
                .field("tim3", &self.tim3())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
