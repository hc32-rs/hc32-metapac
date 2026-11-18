#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc PERIC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peric {
    ptr: *mut u8,
}
unsafe impl Send for Peric {}
unsafe impl Sync for Peric {}
impl Peric {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc USB_SYCTLREG."]
    #[inline(always)]
    pub const fn usb_syctlreg(self) -> crate::common::Reg<regs::UsbSyctlreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc SDIOC_SYCTLREG."]
    #[inline(always)]
    pub const fn sdioc_syctlreg(
        self,
    ) -> crate::common::Reg<regs::SdiocSyctlreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc NFC_STCR."]
    #[inline(always)]
    pub const fn nfc_stcr(self) -> crate::common::Reg<regs::NfcStcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc DMC_ENAR."]
    #[inline(always)]
    pub const fn dmc_enar(self) -> crate::common::Reg<regs::DmcEnar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc NFC_ENAR."]
    #[inline(always)]
    pub const fn nfc_enar(self) -> crate::common::Reg<regs::NfcEnar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc SMC_ENAR."]
    #[inline(always)]
    pub const fn smc_enar(self) -> crate::common::Reg<regs::SmcEnar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc USART1_NFC."]
    #[inline(always)]
    pub const fn usart1_nfc(self) -> crate::common::Reg<regs::Usart1Nfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc NFC_STSR."]
    #[inline(always)]
    pub const fn nfc_stsr(self) -> crate::common::Reg<regs::NfcStsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc DMC_ENAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmcEnar(pub u32);
    impl DmcEnar {
        #[doc = "desc DMCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dmcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMCEN."]
        #[inline(always)]
        pub const fn set_dmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for DmcEnar {
        #[inline(always)]
        fn default() -> DmcEnar {
            DmcEnar(0)
        }
    }
    impl core::fmt::Debug for DmcEnar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmcEnar")
                .field("dmcen", &self.dmcen())
                .finish()
        }
    }
    #[doc = "desc NFC_ENAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NfcEnar(pub u32);
    impl NfcEnar {
        #[doc = "desc NFCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nfcen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NFCEN."]
        #[inline(always)]
        pub const fn set_nfcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for NfcEnar {
        #[inline(always)]
        fn default() -> NfcEnar {
            NfcEnar(0)
        }
    }
    impl core::fmt::Debug for NfcEnar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NfcEnar")
                .field("nfcen", &self.nfcen())
                .finish()
        }
    }
    #[doc = "desc NFC_STCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NfcStcr(pub u32);
    impl NfcStcr {
        #[doc = "desc OPENP."]
        #[must_use]
        #[inline(always)]
        pub const fn openp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc OPENP."]
        #[inline(always)]
        pub const fn set_openp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for NfcStcr {
        #[inline(always)]
        fn default() -> NfcStcr {
            NfcStcr(0)
        }
    }
    impl core::fmt::Debug for NfcStcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NfcStcr")
                .field("openp", &self.openp())
                .finish()
        }
    }
    #[doc = "desc NFC_STSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NfcStsr(pub u32);
    impl NfcStsr {
        #[doc = "desc CHIP_BUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn chip_busy(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CHIP_BUSY."]
        #[inline(always)]
        pub const fn set_chip_busy(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc PECC."]
        #[must_use]
        #[inline(always)]
        pub const fn pecc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PECC."]
        #[inline(always)]
        pub const fn set_pecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for NfcStsr {
        #[inline(always)]
        fn default() -> NfcStsr {
            NfcStsr(0)
        }
    }
    impl core::fmt::Debug for NfcStsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NfcStsr")
                .field("chip_busy", &self.chip_busy())
                .field("pecc", &self.pecc())
                .finish()
        }
    }
    #[doc = "desc SDIOC_SYCTLREG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdiocSyctlreg(pub u32);
    impl SdiocSyctlreg {
        #[doc = "desc SELMMC1."]
        #[must_use]
        #[inline(always)]
        pub const fn selmmc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SELMMC1."]
        #[inline(always)]
        pub const fn set_selmmc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SELMMC2."]
        #[must_use]
        #[inline(always)]
        pub const fn selmmc2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SELMMC2."]
        #[inline(always)]
        pub const fn set_selmmc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for SdiocSyctlreg {
        #[inline(always)]
        fn default() -> SdiocSyctlreg {
            SdiocSyctlreg(0)
        }
    }
    impl core::fmt::Debug for SdiocSyctlreg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdiocSyctlreg")
                .field("selmmc1", &self.selmmc1())
                .field("selmmc2", &self.selmmc2())
                .finish()
        }
    }
    #[doc = "desc SMC_ENAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SmcEnar(pub u32);
    impl SmcEnar {
        #[doc = "desc SMCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn smcen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMCEN."]
        #[inline(always)]
        pub const fn set_smcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SmcEnar {
        #[inline(always)]
        fn default() -> SmcEnar {
            SmcEnar(0)
        }
    }
    impl core::fmt::Debug for SmcEnar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SmcEnar")
                .field("smcen", &self.smcen())
                .finish()
        }
    }
    #[doc = "desc USART1_NFC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart1Nfc(pub u32);
    impl Usart1Nfc {
        #[doc = "desc USASRT1_NFS."]
        #[must_use]
        #[inline(always)]
        pub const fn usasrt1_nfs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc USASRT1_NFS."]
        #[inline(always)]
        pub const fn set_usasrt1_nfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc USART1_NFE."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1_nfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART1_NFE."]
        #[inline(always)]
        pub const fn set_usart1_nfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Usart1Nfc {
        #[inline(always)]
        fn default() -> Usart1Nfc {
            Usart1Nfc(0)
        }
    }
    impl core::fmt::Debug for Usart1Nfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart1Nfc")
                .field("usasrt1_nfs", &self.usasrt1_nfs())
                .field("usart1_nfe", &self.usart1_nfe())
                .finish()
        }
    }
    #[doc = "desc USB_SYCTLREG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbSyctlreg(pub u32);
    impl UsbSyctlreg {
        #[doc = "desc USBFS_DFB."]
        #[must_use]
        #[inline(always)]
        pub const fn usbfs_dfb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBFS_DFB."]
        #[inline(always)]
        pub const fn set_usbfs_dfb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc USBFS_SOFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn usbfs_sofen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBFS_SOFEN."]
        #[inline(always)]
        pub const fn set_usbfs_sofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc USBHS_DFB."]
        #[must_use]
        #[inline(always)]
        pub const fn usbhs_dfb(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBHS_DFB."]
        #[inline(always)]
        pub const fn set_usbhs_dfb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc USBHS_SOFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn usbhs_sofen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBHS_SOFEN."]
        #[inline(always)]
        pub const fn set_usbhs_sofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc USBHS_FSPHYE."]
        #[must_use]
        #[inline(always)]
        pub const fn usbhs_fsphye(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBHS_FSPHYE."]
        #[inline(always)]
        pub const fn set_usbhs_fsphye(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc USBFS_NFS."]
        #[must_use]
        #[inline(always)]
        pub const fn usbfs_nfs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc USBFS_NFS."]
        #[inline(always)]
        pub const fn set_usbfs_nfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc USBFS_NFE."]
        #[must_use]
        #[inline(always)]
        pub const fn usbfs_nfe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBFS_NFE."]
        #[inline(always)]
        pub const fn set_usbfs_nfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc USBHS_NFS."]
        #[must_use]
        #[inline(always)]
        pub const fn usbhs_nfs(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "desc USBHS_NFS."]
        #[inline(always)]
        pub const fn set_usbhs_nfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "desc USBHS_NFE."]
        #[must_use]
        #[inline(always)]
        pub const fn usbhs_nfe(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBHS_NFE."]
        #[inline(always)]
        pub const fn set_usbhs_nfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for UsbSyctlreg {
        #[inline(always)]
        fn default() -> UsbSyctlreg {
            UsbSyctlreg(0)
        }
    }
    impl core::fmt::Debug for UsbSyctlreg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbSyctlreg")
                .field("usbfs_dfb", &self.usbfs_dfb())
                .field("usbfs_sofen", &self.usbfs_sofen())
                .field("usbhs_dfb", &self.usbhs_dfb())
                .field("usbhs_sofen", &self.usbhs_sofen())
                .field("usbhs_fsphye", &self.usbhs_fsphye())
                .field("usbfs_nfs", &self.usbfs_nfs())
                .field("usbfs_nfe", &self.usbfs_nfe())
                .field("usbhs_nfs", &self.usbhs_nfs())
                .field("usbhs_nfe", &self.usbhs_nfe())
                .finish()
        }
    }
}
