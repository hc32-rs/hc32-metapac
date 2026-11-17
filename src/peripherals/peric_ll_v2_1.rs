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
    #[doc = "desc SMC_ENAR."]
    #[inline(always)]
    pub const fn smc_enar(self) -> crate::common::Reg<regs::SmcEnar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc CAN_SYCTLREG."]
    #[inline(always)]
    pub const fn can_syctlreg(self) -> crate::common::Reg<regs::CanSyctlreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc USART1_NFC."]
    #[inline(always)]
    pub const fn usart1_nfc(self) -> crate::common::Reg<regs::Usart1Nfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CAN_SYCTLREG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CanSyctlreg(pub u32);
    impl CanSyctlreg {
        #[doc = "desc CAN1FDE."]
        #[must_use]
        #[inline(always)]
        pub const fn can1fde(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAN1FDE."]
        #[inline(always)]
        pub const fn set_can1fde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CAN2FDE."]
        #[must_use]
        #[inline(always)]
        pub const fn can2fde(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAN2FDE."]
        #[inline(always)]
        pub const fn set_can2fde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CAN3FDE."]
        #[must_use]
        #[inline(always)]
        pub const fn can3fde(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAN3FDE."]
        #[inline(always)]
        pub const fn set_can3fde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for CanSyctlreg {
        #[inline(always)]
        fn default() -> CanSyctlreg {
            CanSyctlreg(0)
        }
    }
    impl core::fmt::Debug for CanSyctlreg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CanSyctlreg")
                .field("can1fde", &self.can1fde())
                .field("can2fde", &self.can2fde())
                .field("can3fde", &self.can3fde())
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
        #[doc = "desc USBFS_WKUP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbfs_wkup(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBFS_WKUP."]
        #[inline(always)]
        pub const fn set_usbfs_wkup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc USBFS_DMPD."]
        #[must_use]
        #[inline(always)]
        pub const fn usbfs_dmpd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBFS_DMPD."]
        #[inline(always)]
        pub const fn set_usbfs_dmpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc USBFS_DPPD."]
        #[must_use]
        #[inline(always)]
        pub const fn usbfs_dppd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBFS_DPPD."]
        #[inline(always)]
        pub const fn set_usbfs_dppd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc USBFS_PDEN."]
        #[must_use]
        #[inline(always)]
        pub const fn usbfs_pden(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBFS_PDEN."]
        #[inline(always)]
        pub const fn set_usbfs_pden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("usbfs_wkup", &self.usbfs_wkup())
                .field("usbfs_dmpd", &self.usbfs_dmpd())
                .field("usbfs_dppd", &self.usbfs_dppd())
                .field("usbfs_pden", &self.usbfs_pden())
                .field("usbfs_nfs", &self.usbfs_nfs())
                .field("usbfs_nfe", &self.usbfs_nfe())
                .finish()
        }
    }
}
