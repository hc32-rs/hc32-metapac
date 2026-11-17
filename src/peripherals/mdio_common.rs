#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc MDIO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdio {
    ptr: *mut u8,
}
unsafe impl Send for Mdio {}
unsafe impl Sync for Mdio {}
impl Mdio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc MDCON."]
    #[inline(always)]
    pub const fn mdcon(self) -> crate::common::Reg<regs::Mdcon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc MDCON1."]
    #[inline(always)]
    pub const fn mdcon1(self) -> crate::common::Reg<regs::Mdcon1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc MDFRM."]
    #[inline(always)]
    pub const fn mdfrm(self) -> crate::common::Reg<regs::Mdfrm, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc MDRXD."]
    #[inline(always)]
    pub const fn mdrxd(self) -> crate::common::Reg<regs::Mdrxd, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc MDADR."]
    #[inline(always)]
    pub const fn mdadr(self) -> crate::common::Reg<regs::Mdadr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc MDTXD."]
    #[inline(always)]
    pub const fn mdtxd(self) -> crate::common::Reg<regs::Mdtxd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc MDPHY."]
    #[inline(always)]
    pub const fn mdphy(self) -> crate::common::Reg<regs::Mdphy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc MDSTA."]
    #[inline(always)]
    pub const fn mdsta(self) -> crate::common::Reg<regs::Mdsta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc MDIEN."]
    #[inline(always)]
    pub const fn mdien(self) -> crate::common::Reg<regs::Mdien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc MDPIN."]
    #[inline(always)]
    pub const fn mdpin(self) -> crate::common::Reg<regs::Mdpin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc MDTO."]
    #[inline(always)]
    pub const fn mdto(self) -> crate::common::Reg<regs::Mdto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc MDADR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdadr(pub u32);
    impl Mdadr {
        #[doc = "desc MD_ADR."]
        #[must_use]
        #[inline(always)]
        pub const fn md_adr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc MD_ADR."]
        #[inline(always)]
        pub const fn set_md_adr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mdadr {
        #[inline(always)]
        fn default() -> Mdadr {
            Mdadr(0)
        }
    }
    impl core::fmt::Debug for Mdadr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdadr")
                .field("md_adr", &self.md_adr())
                .finish()
        }
    }
    #[doc = "desc MDCON."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdcon(pub u32);
    impl Mdcon {
        #[doc = "desc MD_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn md_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_RST."]
        #[inline(always)]
        pub const fn set_md_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc MD_PHYB."]
        #[must_use]
        #[inline(always)]
        pub const fn md_phyb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_PHYB."]
        #[inline(always)]
        pub const fn set_md_phyb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Mdcon {
        #[inline(always)]
        fn default() -> Mdcon {
            Mdcon(0)
        }
    }
    impl core::fmt::Debug for Mdcon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdcon")
                .field("md_rst", &self.md_rst())
                .field("md_phyb", &self.md_phyb())
                .finish()
        }
    }
    #[doc = "desc MDCON1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdcon1(pub u32);
    impl Mdcon1 {
        #[doc = "desc MD_MSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn md_msdev(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_MSDEV."]
        #[inline(always)]
        pub const fn set_md_msdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SEL_EDGE."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_edge(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEL_EDGE."]
        #[inline(always)]
        pub const fn set_sel_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MD_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn md_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_EN."]
        #[inline(always)]
        pub const fn set_md_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Mdcon1 {
        #[inline(always)]
        fn default() -> Mdcon1 {
            Mdcon1(0)
        }
    }
    impl core::fmt::Debug for Mdcon1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdcon1")
                .field("md_msdev", &self.md_msdev())
                .field("sel_edge", &self.sel_edge())
                .field("md_en", &self.md_en())
                .finish()
        }
    }
    #[doc = "desc MDFRM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdfrm(pub u32);
    impl Mdfrm {
        #[doc = "desc MD_ROP."]
        #[must_use]
        #[inline(always)]
        pub const fn md_rop(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc MD_ROP."]
        #[inline(always)]
        pub const fn set_md_rop(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc MD_RPHYAD."]
        #[must_use]
        #[inline(always)]
        pub const fn md_rphyad(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MD_RPHYAD."]
        #[inline(always)]
        pub const fn set_md_rphyad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
        }
        #[doc = "desc MD_RDEVAD."]
        #[must_use]
        #[inline(always)]
        pub const fn md_rdevad(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MD_RDEVAD."]
        #[inline(always)]
        pub const fn set_md_rdevad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
        }
        #[doc = "desc MD_RTA."]
        #[must_use]
        #[inline(always)]
        pub const fn md_rta(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc MD_RTA."]
        #[inline(always)]
        pub const fn set_md_rta(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for Mdfrm {
        #[inline(always)]
        fn default() -> Mdfrm {
            Mdfrm(0)
        }
    }
    impl core::fmt::Debug for Mdfrm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdfrm")
                .field("md_rop", &self.md_rop())
                .field("md_rphyad", &self.md_rphyad())
                .field("md_rdevad", &self.md_rdevad())
                .field("md_rta", &self.md_rta())
                .finish()
        }
    }
    #[doc = "desc MDIEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdien(pub u32);
    impl Mdien {
        #[doc = "desc MD_WRFIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_wrfie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_WRFIE."]
        #[inline(always)]
        pub const fn set_md_wrfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc MD_ADRFIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_adrfie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_ADRFIE."]
        #[inline(always)]
        pub const fn set_md_adrfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc MD_INCFIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_incfie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_INCFIE."]
        #[inline(always)]
        pub const fn set_md_incfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MD_RDFIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_rdfie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_RDFIE."]
        #[inline(always)]
        pub const fn set_md_rdfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc MD_DEVMIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_devmie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_DEVMIE."]
        #[inline(always)]
        pub const fn set_md_devmie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc MD_DEVNIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_devnie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_DEVNIE."]
        #[inline(always)]
        pub const fn set_md_devnie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc MD_PHYMIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_phymie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_PHYMIE."]
        #[inline(always)]
        pub const fn set_md_phymie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc MD_PHYNIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_phynie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_PHYNIE."]
        #[inline(always)]
        pub const fn set_md_phynie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc MD_TANMIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_tanmie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_TANMIE."]
        #[inline(always)]
        pub const fn set_md_tanmie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc MD_TOIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_toie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_TOIE."]
        #[inline(always)]
        pub const fn set_md_toie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc MD_UDRIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_udrie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_UDRIE."]
        #[inline(always)]
        pub const fn set_md_udrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc MD_OVRIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_ovrie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_OVRIE."]
        #[inline(always)]
        pub const fn set_md_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc MD_RBNEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_rbneie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_RBNEIE."]
        #[inline(always)]
        pub const fn set_md_rbneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Mdien {
        #[inline(always)]
        fn default() -> Mdien {
            Mdien(0)
        }
    }
    impl core::fmt::Debug for Mdien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdien")
                .field("md_wrfie", &self.md_wrfie())
                .field("md_adrfie", &self.md_adrfie())
                .field("md_incfie", &self.md_incfie())
                .field("md_rdfie", &self.md_rdfie())
                .field("md_devmie", &self.md_devmie())
                .field("md_devnie", &self.md_devnie())
                .field("md_phymie", &self.md_phymie())
                .field("md_phynie", &self.md_phynie())
                .field("md_tanmie", &self.md_tanmie())
                .field("md_toie", &self.md_toie())
                .field("md_udrie", &self.md_udrie())
                .field("md_ovrie", &self.md_ovrie())
                .field("md_rbneie", &self.md_rbneie())
                .finish()
        }
    }
    #[doc = "desc MDPHY."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdphy(pub u32);
    impl Mdphy {
        #[doc = "desc MD_PHYSW."]
        #[must_use]
        #[inline(always)]
        pub const fn md_physw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MD_PHYSW."]
        #[inline(always)]
        pub const fn set_md_physw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MD_PHYSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn md_physel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MD_PHYSEL."]
        #[inline(always)]
        pub const fn set_md_physel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "desc MD_DEVADD."]
        #[must_use]
        #[inline(always)]
        pub const fn md_devadd(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MD_DEVADD."]
        #[inline(always)]
        pub const fn set_md_devadd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
    }
    impl Default for Mdphy {
        #[inline(always)]
        fn default() -> Mdphy {
            Mdphy(0)
        }
    }
    impl core::fmt::Debug for Mdphy {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdphy")
                .field("md_physw", &self.md_physw())
                .field("md_physel", &self.md_physel())
                .field("md_devadd", &self.md_devadd())
                .finish()
        }
    }
    #[doc = "desc MDPIN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdpin(pub u32);
    impl Mdpin {
        #[doc = "desc MD_PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn md_pin(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MD_PIN."]
        #[inline(always)]
        pub const fn set_md_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Mdpin {
        #[inline(always)]
        fn default() -> Mdpin {
            Mdpin(0)
        }
    }
    impl core::fmt::Debug for Mdpin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdpin")
                .field("md_pin", &self.md_pin())
                .finish()
        }
    }
    #[doc = "desc MDRXD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdrxd(pub u32);
    impl Mdrxd {
        #[doc = "desc MD_RXD."]
        #[must_use]
        #[inline(always)]
        pub const fn md_rxd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc MD_RXD."]
        #[inline(always)]
        pub const fn set_md_rxd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mdrxd {
        #[inline(always)]
        fn default() -> Mdrxd {
            Mdrxd(0)
        }
    }
    impl core::fmt::Debug for Mdrxd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdrxd")
                .field("md_rxd", &self.md_rxd())
                .finish()
        }
    }
    #[doc = "desc MDSTA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdsta(pub u32);
    impl Mdsta {
        #[doc = "desc MD_WRF."]
        #[must_use]
        #[inline(always)]
        pub const fn md_wrf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_WRF."]
        #[inline(always)]
        pub const fn set_md_wrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc MD_ADRF."]
        #[must_use]
        #[inline(always)]
        pub const fn md_adrf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_ADRF."]
        #[inline(always)]
        pub const fn set_md_adrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc MD_INCF."]
        #[must_use]
        #[inline(always)]
        pub const fn md_incf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_INCF."]
        #[inline(always)]
        pub const fn set_md_incf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MD_RDF."]
        #[must_use]
        #[inline(always)]
        pub const fn md_rdf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_RDF."]
        #[inline(always)]
        pub const fn set_md_rdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc MD_DEVM."]
        #[must_use]
        #[inline(always)]
        pub const fn md_devm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_DEVM."]
        #[inline(always)]
        pub const fn set_md_devm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc MD_DEVN."]
        #[must_use]
        #[inline(always)]
        pub const fn md_devn(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_DEVN."]
        #[inline(always)]
        pub const fn set_md_devn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc MD_PHYM."]
        #[must_use]
        #[inline(always)]
        pub const fn md_phym(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_PHYM."]
        #[inline(always)]
        pub const fn set_md_phym(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc MD_PHYN."]
        #[must_use]
        #[inline(always)]
        pub const fn md_phyn(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_PHYN."]
        #[inline(always)]
        pub const fn set_md_phyn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc MD_TANM."]
        #[must_use]
        #[inline(always)]
        pub const fn md_tanm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_TANM."]
        #[inline(always)]
        pub const fn set_md_tanm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc MD_TO."]
        #[must_use]
        #[inline(always)]
        pub const fn md_to(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_TO."]
        #[inline(always)]
        pub const fn set_md_to(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc MD_UDR."]
        #[must_use]
        #[inline(always)]
        pub const fn md_udr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_UDR."]
        #[inline(always)]
        pub const fn set_md_udr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc MD_OVR."]
        #[must_use]
        #[inline(always)]
        pub const fn md_ovr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_OVR."]
        #[inline(always)]
        pub const fn set_md_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc MD_RBNE."]
        #[must_use]
        #[inline(always)]
        pub const fn md_rbne(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_RBNE."]
        #[inline(always)]
        pub const fn set_md_rbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc MD_BUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn md_busy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD_BUSY."]
        #[inline(always)]
        pub const fn set_md_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Mdsta {
        #[inline(always)]
        fn default() -> Mdsta {
            Mdsta(0)
        }
    }
    impl core::fmt::Debug for Mdsta {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdsta")
                .field("md_wrf", &self.md_wrf())
                .field("md_adrf", &self.md_adrf())
                .field("md_incf", &self.md_incf())
                .field("md_rdf", &self.md_rdf())
                .field("md_devm", &self.md_devm())
                .field("md_devn", &self.md_devn())
                .field("md_phym", &self.md_phym())
                .field("md_phyn", &self.md_phyn())
                .field("md_tanm", &self.md_tanm())
                .field("md_to", &self.md_to())
                .field("md_udr", &self.md_udr())
                .field("md_ovr", &self.md_ovr())
                .field("md_rbne", &self.md_rbne())
                .field("md_busy", &self.md_busy())
                .finish()
        }
    }
    #[doc = "desc MDTO."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdto(pub u32);
    impl Mdto {
        #[doc = "desc TOCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TOCNT."]
        #[inline(always)]
        pub const fn set_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc TOEN."]
        #[must_use]
        #[inline(always)]
        pub const fn toen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOEN."]
        #[inline(always)]
        pub const fn set_toen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Mdto {
        #[inline(always)]
        fn default() -> Mdto {
            Mdto(0)
        }
    }
    impl core::fmt::Debug for Mdto {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdto")
                .field("tocnt", &self.tocnt())
                .field("toen", &self.toen())
                .finish()
        }
    }
    #[doc = "desc MDTXD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdtxd(pub u32);
    impl Mdtxd {
        #[doc = "desc MD_TXD."]
        #[must_use]
        #[inline(always)]
        pub const fn md_txd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc MD_TXD."]
        #[inline(always)]
        pub const fn set_md_txd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mdtxd {
        #[inline(always)]
        fn default() -> Mdtxd {
            Mdtxd(0)
        }
    }
    impl core::fmt::Debug for Mdtxd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdtxd")
                .field("md_txd", &self.md_txd())
                .finish()
        }
    }
}
