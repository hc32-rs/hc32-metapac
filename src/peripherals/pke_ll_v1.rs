#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc PKE."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pke {
    ptr: *mut u8,
}
unsafe impl Send for Pke {}
unsafe impl Sync for Pke {}
impl Pke {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc RAM0_CKCR."]
    #[inline(always)]
    pub const fn ram0_ckcr(self) -> crate::common::Reg<regs::Ram0Ckcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc RAM0_CKSR."]
    #[inline(always)]
    pub const fn ram0_cksr(self) -> crate::common::Reg<regs::Ram0Cksr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc RAM0_EIEN."]
    #[inline(always)]
    pub const fn ram0_eien(self) -> crate::common::Reg<regs::Ram0Eien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc RAM0_EIBIT0."]
    #[inline(always)]
    pub const fn ram0_eibit0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc RAM0_EIBIT1."]
    #[inline(always)]
    pub const fn ram0_eibit1(self) -> crate::common::Reg<regs::Ram0Eibit1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc RAM0_ECCERRADDR."]
    #[inline(always)]
    pub const fn ram0_eccerraddr(
        self,
    ) -> crate::common::Reg<regs::Ram0Eccerraddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc RAM1_CKCR."]
    #[inline(always)]
    pub const fn ram1_ckcr(self) -> crate::common::Reg<regs::Ram1Ckcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc RAM1_CKSR."]
    #[inline(always)]
    pub const fn ram1_cksr(self) -> crate::common::Reg<regs::Ram1Cksr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc RAM1_EIEN."]
    #[inline(always)]
    pub const fn ram1_eien(self) -> crate::common::Reg<regs::Ram1Eien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc RAM1_EIBIT0."]
    #[inline(always)]
    pub const fn ram1_eibit0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc RAM1_EIBIT1."]
    #[inline(always)]
    pub const fn ram1_eibit1(self) -> crate::common::Reg<regs::Ram1Eibit1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc RAM1_ECCERRADDR."]
    #[inline(always)]
    pub const fn ram1_eccerraddr(
        self,
    ) -> crate::common::Reg<regs::Ram1Eccerraddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc CTRL."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "desc CFG."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "desc MC_PTR."]
    #[inline(always)]
    pub const fn mc_ptr(self) -> crate::common::Reg<regs::McPtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "desc RISR."]
    #[inline(always)]
    pub const fn risr(self) -> crate::common::Reg<regs::Risr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "desc IMCR."]
    #[inline(always)]
    pub const fn imcr(self) -> crate::common::Reg<regs::Imcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "desc MISR."]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "desc RT_CODE."]
    #[inline(always)]
    pub const fn rt_code(self) -> crate::common::Reg<regs::RtCode, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "desc EXE_CFG."]
    #[inline(always)]
    pub const fn exe_cfg(self) -> crate::common::Reg<regs::ExeCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "desc VERSION."]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10fcusize) as _) }
    }
    #[doc = "desc MEM_RAM0."]
    #[inline(always)]
    pub const fn mem_ram0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1400usize) as _) }
    }
    #[doc = "desc MEM_RAM1."]
    #[inline(always)]
    pub const fn mem_ram1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "desc PARTIAL_RADIX."]
        #[must_use]
        #[inline(always)]
        pub const fn partial_radix(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc PARTIAL_RADIX."]
        #[inline(always)]
        pub const fn set_partial_radix(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc BASE_RADIX."]
        #[must_use]
        #[inline(always)]
        pub const fn base_radix(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc BASE_RADIX."]
        #[inline(always)]
        pub const fn set_base_radix(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    impl core::fmt::Debug for Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg")
                .field("partial_radix", &self.partial_radix())
                .field("base_radix", &self.base_radix())
                .finish()
        }
    }
    #[doc = "desc CTRL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "desc START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
                .field("start", &self.start())
                .finish()
        }
    }
    #[doc = "desc EXE_CFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExeCfg(pub u32);
    impl ExeCfg {
        #[doc = "desc IAFF_R0."]
        #[must_use]
        #[inline(always)]
        pub const fn iaff_r0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc IAFF_R0."]
        #[inline(always)]
        pub const fn set_iaff_r0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc IMON_R0."]
        #[must_use]
        #[inline(always)]
        pub const fn imon_r0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc IMON_R0."]
        #[inline(always)]
        pub const fn set_imon_r0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc IAFF_R1."]
        #[must_use]
        #[inline(always)]
        pub const fn iaff_r1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc IAFF_R1."]
        #[inline(always)]
        pub const fn set_iaff_r1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc IMON_R1."]
        #[must_use]
        #[inline(always)]
        pub const fn imon_r1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc IMON_R1."]
        #[inline(always)]
        pub const fn set_imon_r1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OAFF."]
        #[must_use]
        #[inline(always)]
        pub const fn oaff(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OAFF."]
        #[inline(always)]
        pub const fn set_oaff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc OMON."]
        #[must_use]
        #[inline(always)]
        pub const fn omon(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc OMON."]
        #[inline(always)]
        pub const fn set_omon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for ExeCfg {
        #[inline(always)]
        fn default() -> ExeCfg {
            ExeCfg(0)
        }
    }
    impl core::fmt::Debug for ExeCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExeCfg")
                .field("iaff_r0", &self.iaff_r0())
                .field("imon_r0", &self.imon_r0())
                .field("iaff_r1", &self.iaff_r1())
                .field("imon_r1", &self.imon_r1())
                .field("oaff", &self.oaff())
                .field("omon", &self.omon())
                .finish()
        }
    }
    #[doc = "desc IMCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imcr(pub u32);
    impl Imcr {
        #[doc = "desc CORE_IRQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn core_irqen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CORE_IRQEN."]
        #[inline(always)]
        pub const fn set_core_irqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Imcr {
        #[inline(always)]
        fn default() -> Imcr {
            Imcr(0)
        }
    }
    impl core::fmt::Debug for Imcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Imcr")
                .field("core_irqen", &self.core_irqen())
                .finish()
        }
    }
    #[doc = "desc MC_PTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McPtr(pub u32);
    impl McPtr {
        #[doc = "desc ADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ADDR."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for McPtr {
        #[inline(always)]
        fn default() -> McPtr {
            McPtr(0)
        }
    }
    impl core::fmt::Debug for McPtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("McPtr").field("addr", &self.addr()).finish()
        }
    }
    #[doc = "desc MISR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "desc CORE_MI."]
        #[must_use]
        #[inline(always)]
        pub const fn core_mi(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CORE_MI."]
        #[inline(always)]
        pub const fn set_core_mi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    impl core::fmt::Debug for Misr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Misr")
                .field("core_mi", &self.core_mi())
                .finish()
        }
    }
    #[doc = "desc RAM0_CKCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram0Ckcr(pub u32);
    impl Ram0Ckcr {
        #[doc = "desc ECCMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn eccmod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc ECCMOD."]
        #[inline(always)]
        pub const fn set_eccmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Ram0Ckcr {
        #[inline(always)]
        fn default() -> Ram0Ckcr {
            Ram0Ckcr(0)
        }
    }
    impl core::fmt::Debug for Ram0Ckcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram0Ckcr")
                .field("eccmod", &self.eccmod())
                .finish()
        }
    }
    #[doc = "desc RAM0_CKSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram0Cksr(pub u32);
    impl Ram0Cksr {
        #[doc = "desc RAM_1ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ram_1err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAM_1ERR."]
        #[inline(always)]
        pub const fn set_ram_1err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RAM_2ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ram_2err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAM_2ERR."]
        #[inline(always)]
        pub const fn set_ram_2err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ram0Cksr {
        #[inline(always)]
        fn default() -> Ram0Cksr {
            Ram0Cksr(0)
        }
    }
    impl core::fmt::Debug for Ram0Cksr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram0Cksr")
                .field("ram_1err", &self.ram_1err())
                .field("ram_2err", &self.ram_2err())
                .finish()
        }
    }
    #[doc = "desc RAM0_ECCERRADDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram0Eccerraddr(pub u32);
    impl Ram0Eccerraddr {
        #[doc = "desc ECCERRADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn eccerraddr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc ECCERRADDR."]
        #[inline(always)]
        pub const fn set_eccerraddr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Ram0Eccerraddr {
        #[inline(always)]
        fn default() -> Ram0Eccerraddr {
            Ram0Eccerraddr(0)
        }
    }
    impl core::fmt::Debug for Ram0Eccerraddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram0Eccerraddr")
                .field("eccerraddr", &self.eccerraddr())
                .finish()
        }
    }
    #[doc = "desc RAM0_EIBIT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram0Eibit1(pub u32);
    impl Ram0Eibit1 {
        #[doc = "desc EIBIT1."]
        #[must_use]
        #[inline(always)]
        pub const fn eibit1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc EIBIT1."]
        #[inline(always)]
        pub const fn set_eibit1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Ram0Eibit1 {
        #[inline(always)]
        fn default() -> Ram0Eibit1 {
            Ram0Eibit1(0)
        }
    }
    impl core::fmt::Debug for Ram0Eibit1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram0Eibit1")
                .field("eibit1", &self.eibit1())
                .finish()
        }
    }
    #[doc = "desc RAM0_EIEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram0Eien(pub u32);
    impl Ram0Eien {
        #[doc = "desc EIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn eien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIEN."]
        #[inline(always)]
        pub const fn set_eien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ram0Eien {
        #[inline(always)]
        fn default() -> Ram0Eien {
            Ram0Eien(0)
        }
    }
    impl core::fmt::Debug for Ram0Eien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram0Eien")
                .field("eien", &self.eien())
                .finish()
        }
    }
    #[doc = "desc RAM1_CKCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram1Ckcr(pub u32);
    impl Ram1Ckcr {
        #[doc = "desc ECCMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn eccmod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc ECCMOD."]
        #[inline(always)]
        pub const fn set_eccmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Ram1Ckcr {
        #[inline(always)]
        fn default() -> Ram1Ckcr {
            Ram1Ckcr(0)
        }
    }
    impl core::fmt::Debug for Ram1Ckcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram1Ckcr")
                .field("eccmod", &self.eccmod())
                .finish()
        }
    }
    #[doc = "desc RAM1_CKSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram1Cksr(pub u32);
    impl Ram1Cksr {
        #[doc = "desc RAM_1ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ram_1err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAM_1ERR."]
        #[inline(always)]
        pub const fn set_ram_1err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RAM_2ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ram_2err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAM_2ERR."]
        #[inline(always)]
        pub const fn set_ram_2err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ram1Cksr {
        #[inline(always)]
        fn default() -> Ram1Cksr {
            Ram1Cksr(0)
        }
    }
    impl core::fmt::Debug for Ram1Cksr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram1Cksr")
                .field("ram_1err", &self.ram_1err())
                .field("ram_2err", &self.ram_2err())
                .finish()
        }
    }
    #[doc = "desc RAM1_ECCERRADDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram1Eccerraddr(pub u32);
    impl Ram1Eccerraddr {
        #[doc = "desc ECCERRADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn eccerraddr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc ECCERRADDR."]
        #[inline(always)]
        pub const fn set_eccerraddr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Ram1Eccerraddr {
        #[inline(always)]
        fn default() -> Ram1Eccerraddr {
            Ram1Eccerraddr(0)
        }
    }
    impl core::fmt::Debug for Ram1Eccerraddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram1Eccerraddr")
                .field("eccerraddr", &self.eccerraddr())
                .finish()
        }
    }
    #[doc = "desc RAM1_EIBIT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram1Eibit1(pub u32);
    impl Ram1Eibit1 {
        #[doc = "desc EIBIT1."]
        #[must_use]
        #[inline(always)]
        pub const fn eibit1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc EIBIT1."]
        #[inline(always)]
        pub const fn set_eibit1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Ram1Eibit1 {
        #[inline(always)]
        fn default() -> Ram1Eibit1 {
            Ram1Eibit1(0)
        }
    }
    impl core::fmt::Debug for Ram1Eibit1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram1Eibit1")
                .field("eibit1", &self.eibit1())
                .finish()
        }
    }
    #[doc = "desc RAM1_EIEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram1Eien(pub u32);
    impl Ram1Eien {
        #[doc = "desc EIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn eien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIEN."]
        #[inline(always)]
        pub const fn set_eien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ram1Eien {
        #[inline(always)]
        fn default() -> Ram1Eien {
            Ram1Eien(0)
        }
    }
    impl core::fmt::Debug for Ram1Eien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram1Eien")
                .field("eien", &self.eien())
                .finish()
        }
    }
    #[doc = "desc RISR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Risr(pub u32);
    impl Risr {
        #[doc = "desc CORE_RIS."]
        #[must_use]
        #[inline(always)]
        pub const fn core_ris(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CORE_RIS."]
        #[inline(always)]
        pub const fn set_core_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Risr {
        #[inline(always)]
        fn default() -> Risr {
            Risr(0)
        }
    }
    impl core::fmt::Debug for Risr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Risr")
                .field("core_ris", &self.core_ris())
                .finish()
        }
    }
    #[doc = "desc RT_CODE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtCode(pub u32);
    impl RtCode {
        #[doc = "desc STOP_LOG."]
        #[must_use]
        #[inline(always)]
        pub const fn stop_log(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc STOP_LOG."]
        #[inline(always)]
        pub const fn set_stop_log(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for RtCode {
        #[inline(always)]
        fn default() -> RtCode {
            RtCode(0)
        }
    }
    impl core::fmt::Debug for RtCode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtCode")
                .field("stop_log", &self.stop_log())
                .finish()
        }
    }
    #[doc = "desc VERSION."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Version(pub u32);
    impl Version {
        #[doc = "desc MIR."]
        #[must_use]
        #[inline(always)]
        pub const fn mir(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MIR."]
        #[inline(always)]
        pub const fn set_mir(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc MAR."]
        #[must_use]
        #[inline(always)]
        pub const fn mar(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MAR."]
        #[inline(always)]
        pub const fn set_mar(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc PROJECT."]
        #[must_use]
        #[inline(always)]
        pub const fn project(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PROJECT."]
        #[inline(always)]
        pub const fn set_project(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Version {
        #[inline(always)]
        fn default() -> Version {
            Version(0)
        }
    }
    impl core::fmt::Debug for Version {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Version")
                .field("mir", &self.mir())
                .field("mar", &self.mar())
                .field("project", &self.project())
                .finish()
        }
    }
}
