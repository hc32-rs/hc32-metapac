#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc HRPWM1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hrpwm {
    ptr: *mut u8,
}
unsafe impl Send for Hrpwm {}
unsafe impl Sync for Hrpwm {}
impl Hrpwm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CNTER."]
    #[inline(always)]
    pub const fn cnter(self) -> crate::common::Reg<regs::Cnter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc UPDAR."]
    #[inline(always)]
    pub const fn updar(self) -> crate::common::Reg<regs::Updar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc HRPERAR."]
    #[inline(always)]
    pub const fn hrperar(self) -> crate::common::Reg<regs::Hrperar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc HRPERBR."]
    #[inline(always)]
    pub const fn hrperbr(self) -> crate::common::Reg<regs::Hrperbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc HRGCMAR."]
    #[inline(always)]
    pub const fn hrgcmar(self) -> crate::common::Reg<regs::Hrgcmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "desc HRGCMBR."]
    #[inline(always)]
    pub const fn hrgcmbr(self) -> crate::common::Reg<regs::Hrgcmbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "desc HRGCMCR."]
    #[inline(always)]
    pub const fn hrgcmcr(self) -> crate::common::Reg<regs::Hrgcmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "desc HRGCMDR."]
    #[inline(always)]
    pub const fn hrgcmdr(self) -> crate::common::Reg<regs::Hrgcmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "desc HRGCMER."]
    #[inline(always)]
    pub const fn hrgcmer(self) -> crate::common::Reg<regs::Hrgcmer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "desc HRGCMFR."]
    #[inline(always)]
    pub const fn hrgcmfr(self) -> crate::common::Reg<regs::Hrgcmfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "desc HRGCMGR."]
    #[inline(always)]
    pub const fn hrgcmgr(self) -> crate::common::Reg<regs::Hrgcmgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "desc HRGCMHR."]
    #[inline(always)]
    pub const fn hrgcmhr(self) -> crate::common::Reg<regs::Hrgcmhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "desc SCMAR."]
    #[inline(always)]
    pub const fn scmar(self) -> crate::common::Reg<regs::Scmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "desc SCMBR."]
    #[inline(always)]
    pub const fn scmbr(self) -> crate::common::Reg<regs::Scmbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "desc SCMCR."]
    #[inline(always)]
    pub const fn scmcr(self) -> crate::common::Reg<regs::Scmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "desc SCMDR."]
    #[inline(always)]
    pub const fn scmdr(self) -> crate::common::Reg<regs::Scmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "desc CAPAR."]
    #[inline(always)]
    pub const fn capar(self) -> crate::common::Reg<regs::Capar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "desc CAPBR."]
    #[inline(always)]
    pub const fn capbr(self) -> crate::common::Reg<regs::Capbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "desc HRDTUAR."]
    #[inline(always)]
    pub const fn hrdtuar(self) -> crate::common::Reg<regs::Hrdtuar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "desc HRDTDAR."]
    #[inline(always)]
    pub const fn hrdtdar(self) -> crate::common::Reg<regs::Hrdtdar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "desc HRDTUBR."]
    #[inline(always)]
    pub const fn hrdtubr(self) -> crate::common::Reg<regs::Hrdtubr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "desc HRDTDBR."]
    #[inline(always)]
    pub const fn hrdtdbr(self) -> crate::common::Reg<regs::Hrdtdbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "desc GCONR."]
    #[inline(always)]
    pub const fn gconr(self) -> crate::common::Reg<regs::Gconr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "desc ICONR."]
    #[inline(always)]
    pub const fn iconr(self) -> crate::common::Reg<regs::Iconr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "desc BCONR1."]
    #[inline(always)]
    pub const fn bconr1(self) -> crate::common::Reg<regs::Bconr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "desc DCONR."]
    #[inline(always)]
    pub const fn dconr(self) -> crate::common::Reg<regs::Dconr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "desc PCNAR1."]
    #[inline(always)]
    pub const fn pcnar1(self) -> crate::common::Reg<regs::Pcnar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "desc PCNBR1."]
    #[inline(always)]
    pub const fn pcnbr1(self) -> crate::common::Reg<regs::Pcnbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "desc VPERR."]
    #[inline(always)]
    pub const fn vperr(self) -> crate::common::Reg<regs::Vperr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "desc STFLR1."]
    #[inline(always)]
    pub const fn stflr1(self) -> crate::common::Reg<regs::Stflr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "desc STFLR2."]
    #[inline(always)]
    pub const fn stflr2(self) -> crate::common::Reg<regs::Stflr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "desc HSTAR1."]
    #[inline(always)]
    pub const fn hstar1(self) -> crate::common::Reg<regs::Hstar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "desc HCLRR1."]
    #[inline(always)]
    pub const fn hclrr1(self) -> crate::common::Reg<regs::Hclrr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "desc HCPAR1."]
    #[inline(always)]
    pub const fn hcpar1(self) -> crate::common::Reg<regs::Hcpar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "desc HCPBR1."]
    #[inline(always)]
    pub const fn hcpbr1(self) -> crate::common::Reg<regs::Hcpbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "desc HCPAR2."]
    #[inline(always)]
    pub const fn hcpar2(self) -> crate::common::Reg<regs::Hcpar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "desc HCPBR2."]
    #[inline(always)]
    pub const fn hcpbr2(self) -> crate::common::Reg<regs::Hcpbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "desc BCONR2."]
    #[inline(always)]
    pub const fn bconr2(self) -> crate::common::Reg<regs::Bconr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "desc PCNAR2."]
    #[inline(always)]
    pub const fn pcnar2(self) -> crate::common::Reg<regs::Pcnar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "desc PCNBR2."]
    #[inline(always)]
    pub const fn pcnbr2(self) -> crate::common::Reg<regs::Pcnbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "desc HSTAR2."]
    #[inline(always)]
    pub const fn hstar2(self) -> crate::common::Reg<regs::Hstar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "desc HCLRR2."]
    #[inline(always)]
    pub const fn hclrr2(self) -> crate::common::Reg<regs::Hclrr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "desc EEFOFFSETAR."]
    #[inline(always)]
    pub const fn eefoffsetar(self) -> crate::common::Reg<regs::Eefoffsetar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "desc EEFOFFSETBR."]
    #[inline(always)]
    pub const fn eefoffsetbr(self) -> crate::common::Reg<regs::Eefoffsetbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "desc EEFWINAR."]
    #[inline(always)]
    pub const fn eefwinar(self) -> crate::common::Reg<regs::Eefwinar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "desc EEFWINBR."]
    #[inline(always)]
    pub const fn eefwinbr(self) -> crate::common::Reg<regs::Eefwinbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "desc EEFLTCR1."]
    #[inline(always)]
    pub const fn eefltcr1(self) -> crate::common::Reg<regs::Eefltcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "desc EEFLTCR2."]
    #[inline(always)]
    pub const fn eefltcr2(self) -> crate::common::Reg<regs::Eefltcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "desc PCNAR3."]
    #[inline(always)]
    pub const fn pcnar3(self) -> crate::common::Reg<regs::Pcnar3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "desc PCNBR3."]
    #[inline(always)]
    pub const fn pcnbr3(self) -> crate::common::Reg<regs::Pcnbr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "desc IDLECR."]
    #[inline(always)]
    pub const fn idlecr(self) -> crate::common::Reg<regs::Idlecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "desc GCONR1."]
    #[inline(always)]
    pub const fn gconr1(self) -> crate::common::Reg<regs::Gconr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "desc BICONR."]
    #[inline(always)]
    pub const fn biconr(self) -> crate::common::Reg<regs::Biconr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "desc BPCNAR1."]
    #[inline(always)]
    pub const fn bpcnar1(self) -> crate::common::Reg<regs::Bpcnar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "desc BPCNBR1."]
    #[inline(always)]
    pub const fn bpcnbr1(self) -> crate::common::Reg<regs::Bpcnbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "desc BPCNAR2."]
    #[inline(always)]
    pub const fn bpcnar2(self) -> crate::common::Reg<regs::Bpcnar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "desc BPCNBR2."]
    #[inline(always)]
    pub const fn bpcnbr2(self) -> crate::common::Reg<regs::Bpcnbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "desc BPCNAR3."]
    #[inline(always)]
    pub const fn bpcnar3(self) -> crate::common::Reg<regs::Bpcnar3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "desc BPCNBR3."]
    #[inline(always)]
    pub const fn bpcnbr3(self) -> crate::common::Reg<regs::Bpcnbr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "desc BGCONR1."]
    #[inline(always)]
    pub const fn bgconr1(self) -> crate::common::Reg<regs::Bgconr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "desc SCMASELR."]
    #[inline(always)]
    pub const fn scmaselr(self) -> crate::common::Reg<regs::Scmaselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "desc PHSCTL."]
    #[inline(always)]
    pub const fn phsctl(self) -> crate::common::Reg<regs::Phsctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "desc PHSCMP1A."]
    #[inline(always)]
    pub const fn phscmp1a(self) -> crate::common::Reg<regs::Phscmp1a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "desc PHSCMP1B."]
    #[inline(always)]
    pub const fn phscmp1b(self) -> crate::common::Reg<regs::Phscmp1b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "desc PHSCMP2A."]
    #[inline(always)]
    pub const fn phscmp2a(self) -> crate::common::Reg<regs::Phscmp2a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "desc PHSCMP2B."]
    #[inline(always)]
    pub const fn phscmp2b(self) -> crate::common::Reg<regs::Phscmp2b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "desc PHSCMP3A."]
    #[inline(always)]
    pub const fn phscmp3a(self) -> crate::common::Reg<regs::Phscmp3a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "desc PHSCMP3B."]
    #[inline(always)]
    pub const fn phscmp3b(self) -> crate::common::Reg<regs::Phscmp3b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "desc PHSCMP4A."]
    #[inline(always)]
    pub const fn phscmp4a(self) -> crate::common::Reg<regs::Phscmp4a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "desc PHSCMP4B."]
    #[inline(always)]
    pub const fn phscmp4b(self) -> crate::common::Reg<regs::Phscmp4b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "desc PHSCMP5A."]
    #[inline(always)]
    pub const fn phscmp5a(self) -> crate::common::Reg<regs::Phscmp5a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "desc PHSCMP5B."]
    #[inline(always)]
    pub const fn phscmp5b(self) -> crate::common::Reg<regs::Phscmp5b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc BCONR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bconr1(pub u32);
    impl Bconr1 {
        #[doc = "desc BENAE."]
        #[must_use]
        #[inline(always)]
        pub const fn benae(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc BENAE."]
        #[inline(always)]
        pub const fn set_benae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc BTRUAE."]
        #[must_use]
        #[inline(always)]
        pub const fn btruae(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRUAE."]
        #[inline(always)]
        pub const fn set_btruae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc BTRDAE."]
        #[must_use]
        #[inline(always)]
        pub const fn btrdae(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRDAE."]
        #[inline(always)]
        pub const fn set_btrdae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc BENBF."]
        #[must_use]
        #[inline(always)]
        pub const fn benbf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc BENBF."]
        #[inline(always)]
        pub const fn set_benbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc BTRUBF."]
        #[must_use]
        #[inline(always)]
        pub const fn btrubf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRUBF."]
        #[inline(always)]
        pub const fn set_btrubf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BTRDBF."]
        #[must_use]
        #[inline(always)]
        pub const fn btrdbf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRDBF."]
        #[inline(always)]
        pub const fn set_btrdbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BENP."]
        #[must_use]
        #[inline(always)]
        pub const fn benp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BENP."]
        #[inline(always)]
        pub const fn set_benp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc BTRUP."]
        #[must_use]
        #[inline(always)]
        pub const fn btrup(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRUP."]
        #[inline(always)]
        pub const fn set_btrup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc BTRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn btrdp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRDP."]
        #[inline(always)]
        pub const fn set_btrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BENSPA."]
        #[must_use]
        #[inline(always)]
        pub const fn benspa(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc BENSPA."]
        #[inline(always)]
        pub const fn set_benspa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc BTRUSPA."]
        #[must_use]
        #[inline(always)]
        pub const fn btruspa(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRUSPA."]
        #[inline(always)]
        pub const fn set_btruspa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc BTRDSPA."]
        #[must_use]
        #[inline(always)]
        pub const fn btrdspa(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRDSPA."]
        #[inline(always)]
        pub const fn set_btrdspa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc BENSPB."]
        #[must_use]
        #[inline(always)]
        pub const fn benspb(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc BENSPB."]
        #[inline(always)]
        pub const fn set_benspb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc BTRUSPB."]
        #[must_use]
        #[inline(always)]
        pub const fn btruspb(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRUSPB."]
        #[inline(always)]
        pub const fn set_btruspb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc BTRDSPB."]
        #[must_use]
        #[inline(always)]
        pub const fn btrdspb(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRDSPB."]
        #[inline(always)]
        pub const fn set_btrdspb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Bconr1 {
        #[inline(always)]
        fn default() -> Bconr1 {
            Bconr1(0)
        }
    }
    impl core::fmt::Debug for Bconr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bconr1")
                .field("benae", &self.benae())
                .field("btruae", &self.btruae())
                .field("btrdae", &self.btrdae())
                .field("benbf", &self.benbf())
                .field("btrubf", &self.btrubf())
                .field("btrdbf", &self.btrdbf())
                .field("benp", &self.benp())
                .field("btrup", &self.btrup())
                .field("btrdp", &self.btrdp())
                .field("benspa", &self.benspa())
                .field("btruspa", &self.btruspa())
                .field("btrdspa", &self.btrdspa())
                .field("benspb", &self.benspb())
                .field("btruspb", &self.btruspb())
                .field("btrdspb", &self.btrdspb())
                .finish()
        }
    }
    #[doc = "desc BCONR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bconr2(pub u32);
    impl Bconr2 {
        #[doc = "desc BENEEFOFF."]
        #[must_use]
        #[inline(always)]
        pub const fn beneefoff(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc BENEEFOFF."]
        #[inline(always)]
        pub const fn set_beneefoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc BTRUEEFOFF."]
        #[must_use]
        #[inline(always)]
        pub const fn btrueefoff(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRUEEFOFF."]
        #[inline(always)]
        pub const fn set_btrueefoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc BTRDEEFOFF."]
        #[must_use]
        #[inline(always)]
        pub const fn btrdeefoff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRDEEFOFF."]
        #[inline(always)]
        pub const fn set_btrdeefoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc BENEEFWIN."]
        #[must_use]
        #[inline(always)]
        pub const fn beneefwin(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc BENEEFWIN."]
        #[inline(always)]
        pub const fn set_beneefwin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc BTRUEEFWIN."]
        #[must_use]
        #[inline(always)]
        pub const fn btrueefwin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRUEEFWIN."]
        #[inline(always)]
        pub const fn set_btrueefwin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc BTRDEEFWIN."]
        #[must_use]
        #[inline(always)]
        pub const fn btrdeefwin(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRDEEFWIN."]
        #[inline(always)]
        pub const fn set_btrdeefwin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc BENCTL."]
        #[must_use]
        #[inline(always)]
        pub const fn benctl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BENCTL."]
        #[inline(always)]
        pub const fn set_benctl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc BTRUCTL."]
        #[must_use]
        #[inline(always)]
        pub const fn btructl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRUCTL."]
        #[inline(always)]
        pub const fn set_btructl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc BTRDCTL."]
        #[must_use]
        #[inline(always)]
        pub const fn btrdctl(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRDCTL."]
        #[inline(always)]
        pub const fn set_btrdctl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Bconr2 {
        #[inline(always)]
        fn default() -> Bconr2 {
            Bconr2(0)
        }
    }
    impl core::fmt::Debug for Bconr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bconr2")
                .field("beneefoff", &self.beneefoff())
                .field("btrueefoff", &self.btrueefoff())
                .field("btrdeefoff", &self.btrdeefoff())
                .field("beneefwin", &self.beneefwin())
                .field("btrueefwin", &self.btrueefwin())
                .field("btrdeefwin", &self.btrdeefwin())
                .field("benctl", &self.benctl())
                .field("btructl", &self.btructl())
                .field("btrdctl", &self.btrdctl())
                .finish()
        }
    }
    #[doc = "desc BGCONR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgconr1(pub u32);
    impl Bgconr1 {
        #[doc = "desc INVCAEN."]
        #[must_use]
        #[inline(always)]
        pub const fn invcaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVCAEN."]
        #[inline(always)]
        pub const fn set_invcaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc INVCBEN."]
        #[must_use]
        #[inline(always)]
        pub const fn invcben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVCBEN."]
        #[inline(always)]
        pub const fn set_invcben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SWAPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn swapen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWAPEN."]
        #[inline(always)]
        pub const fn set_swapen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SWAPMD."]
        #[must_use]
        #[inline(always)]
        pub const fn swapmd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWAPMD."]
        #[inline(always)]
        pub const fn set_swapmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc HALF."]
        #[must_use]
        #[inline(always)]
        pub const fn half(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc HALF."]
        #[inline(always)]
        pub const fn set_half(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc EEFM."]
        #[must_use]
        #[inline(always)]
        pub const fn eefm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EEFM."]
        #[inline(always)]
        pub const fn set_eefm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc EEFREF."]
        #[must_use]
        #[inline(always)]
        pub const fn eefref(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EEFREF."]
        #[inline(always)]
        pub const fn set_eefref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PRDSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn prdsel(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "desc PRDSEL."]
        #[inline(always)]
        pub const fn set_prdsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "desc CMSCAUEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cmscauen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSCAUEN."]
        #[inline(always)]
        pub const fn set_cmscauen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc CMSCADEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cmscaden(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSCADEN."]
        #[inline(always)]
        pub const fn set_cmscaden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc CMSCBUEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cmscbuen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSCBUEN."]
        #[inline(always)]
        pub const fn set_cmscbuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc CMSCBDEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cmscbden(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSCBDEN."]
        #[inline(always)]
        pub const fn set_cmscbden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc BFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn bfsel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "desc BFSEL."]
        #[inline(always)]
        pub const fn set_bfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for Bgconr1 {
        #[inline(always)]
        fn default() -> Bgconr1 {
            Bgconr1(0)
        }
    }
    impl core::fmt::Debug for Bgconr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgconr1")
                .field("invcaen", &self.invcaen())
                .field("invcben", &self.invcben())
                .field("swapen", &self.swapen())
                .field("swapmd", &self.swapmd())
                .field("half", &self.half())
                .field("eefm", &self.eefm())
                .field("eefref", &self.eefref())
                .field("prdsel", &self.prdsel())
                .field("cmscauen", &self.cmscauen())
                .field("cmscaden", &self.cmscaden())
                .field("cmscbuen", &self.cmscbuen())
                .field("cmscbden", &self.cmscbden())
                .field("bfsel", &self.bfsel())
                .finish()
        }
    }
    #[doc = "desc BICONR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Biconr(pub u32);
    impl Biconr {
        #[doc = "desc INTENA."]
        #[must_use]
        #[inline(always)]
        pub const fn intena(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENA."]
        #[inline(always)]
        pub const fn set_intena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc INTENB."]
        #[must_use]
        #[inline(always)]
        pub const fn intenb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENB."]
        #[inline(always)]
        pub const fn set_intenb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc INTENC."]
        #[must_use]
        #[inline(always)]
        pub const fn intenc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENC."]
        #[inline(always)]
        pub const fn set_intenc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc INTEND."]
        #[must_use]
        #[inline(always)]
        pub const fn intend(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTEND."]
        #[inline(always)]
        pub const fn set_intend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc INTENE."]
        #[must_use]
        #[inline(always)]
        pub const fn intene(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENE."]
        #[inline(always)]
        pub const fn set_intene(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INTENF."]
        #[must_use]
        #[inline(always)]
        pub const fn intenf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENF."]
        #[inline(always)]
        pub const fn set_intenf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc INTENOVF."]
        #[must_use]
        #[inline(always)]
        pub const fn intenovf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENOVF."]
        #[inline(always)]
        pub const fn set_intenovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NTENUDF."]
        #[must_use]
        #[inline(always)]
        pub const fn ntenudf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc NTENUDF."]
        #[inline(always)]
        pub const fn set_ntenudf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc INTENCAPA."]
        #[must_use]
        #[inline(always)]
        pub const fn intencapa(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENCAPA."]
        #[inline(always)]
        pub const fn set_intencapa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc INTENCAPB."]
        #[must_use]
        #[inline(always)]
        pub const fn intencapb(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENCAPB."]
        #[inline(always)]
        pub const fn set_intencapb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc INTENSAU."]
        #[must_use]
        #[inline(always)]
        pub const fn intensau(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENSAU."]
        #[inline(always)]
        pub const fn set_intensau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc INTENSAD."]
        #[must_use]
        #[inline(always)]
        pub const fn intensad(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENSAD."]
        #[inline(always)]
        pub const fn set_intensad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc INTENSBU."]
        #[must_use]
        #[inline(always)]
        pub const fn intensbu(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENSBU."]
        #[inline(always)]
        pub const fn set_intensbu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc INTENSBD."]
        #[must_use]
        #[inline(always)]
        pub const fn intensbd(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENSBD."]
        #[inline(always)]
        pub const fn set_intensbd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Biconr {
        #[inline(always)]
        fn default() -> Biconr {
            Biconr(0)
        }
    }
    impl core::fmt::Debug for Biconr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Biconr")
                .field("intena", &self.intena())
                .field("intenb", &self.intenb())
                .field("intenc", &self.intenc())
                .field("intend", &self.intend())
                .field("intene", &self.intene())
                .field("intenf", &self.intenf())
                .field("intenovf", &self.intenovf())
                .field("ntenudf", &self.ntenudf())
                .field("intencapa", &self.intencapa())
                .field("intencapb", &self.intencapb())
                .field("intensau", &self.intensau())
                .field("intensad", &self.intensad())
                .field("intensbu", &self.intensbu())
                .field("intensbd", &self.intensbd())
                .finish()
        }
    }
    #[doc = "desc BPCNAR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bpcnar1(pub u32);
    impl Bpcnar1 {
        #[doc = "desc STACA."]
        #[must_use]
        #[inline(always)]
        pub const fn staca(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc STACA."]
        #[inline(always)]
        pub const fn set_staca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc STPCA."]
        #[must_use]
        #[inline(always)]
        pub const fn stpca(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc STPCA."]
        #[inline(always)]
        pub const fn set_stpca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc OVFCA."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfca(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc OVFCA."]
        #[inline(always)]
        pub const fn set_ovfca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc UDFCA."]
        #[must_use]
        #[inline(always)]
        pub const fn udfca(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc UDFCA."]
        #[inline(always)]
        pub const fn set_udfca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc CMAUCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmauca(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMAUCA."]
        #[inline(always)]
        pub const fn set_cmauca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc CMADCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmadca(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMADCA."]
        #[inline(always)]
        pub const fn set_cmadca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc CMBUCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmbuca(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMBUCA."]
        #[inline(always)]
        pub const fn set_cmbuca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc CMBDCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmbdca(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMBDCA."]
        #[inline(always)]
        pub const fn set_cmbdca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc FORCA."]
        #[must_use]
        #[inline(always)]
        pub const fn forca(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc FORCA."]
        #[inline(always)]
        pub const fn set_forca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc EMBCA."]
        #[must_use]
        #[inline(always)]
        pub const fn embca(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc EMBCA."]
        #[inline(always)]
        pub const fn set_embca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc EMBRA."]
        #[must_use]
        #[inline(always)]
        pub const fn embra(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc EMBRA."]
        #[inline(always)]
        pub const fn set_embra(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc EMBSA."]
        #[must_use]
        #[inline(always)]
        pub const fn embsa(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc EMBSA."]
        #[inline(always)]
        pub const fn set_embsa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "desc OUTENA."]
        #[must_use]
        #[inline(always)]
        pub const fn outena(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc OUTENA."]
        #[inline(always)]
        pub const fn set_outena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Bpcnar1 {
        #[inline(always)]
        fn default() -> Bpcnar1 {
            Bpcnar1(0)
        }
    }
    impl core::fmt::Debug for Bpcnar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bpcnar1")
                .field("staca", &self.staca())
                .field("stpca", &self.stpca())
                .field("ovfca", &self.ovfca())
                .field("udfca", &self.udfca())
                .field("cmauca", &self.cmauca())
                .field("cmadca", &self.cmadca())
                .field("cmbuca", &self.cmbuca())
                .field("cmbdca", &self.cmbdca())
                .field("forca", &self.forca())
                .field("embca", &self.embca())
                .field("embra", &self.embra())
                .field("embsa", &self.embsa())
                .field("outena", &self.outena())
                .finish()
        }
    }
    #[doc = "desc BPCNAR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bpcnar2(pub u32);
    impl Bpcnar2 {
        #[doc = "desc EXEV1UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev1uca(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV1UCA."]
        #[inline(always)]
        pub const fn set_exev1uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EXEV2UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev2uca(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV2UCA."]
        #[inline(always)]
        pub const fn set_exev2uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc EXEV3UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev3uca(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV3UCA."]
        #[inline(always)]
        pub const fn set_exev3uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EXEV4UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev4uca(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV4UCA."]
        #[inline(always)]
        pub const fn set_exev4uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc EXEV5UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev5uca(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV5UCA."]
        #[inline(always)]
        pub const fn set_exev5uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc EXEV6UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev6uca(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV6UCA."]
        #[inline(always)]
        pub const fn set_exev6uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc EXEV7UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev7uca(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV7UCA."]
        #[inline(always)]
        pub const fn set_exev7uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc EXEV8UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev8uca(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV8UCA."]
        #[inline(always)]
        pub const fn set_exev8uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc EXEV9UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev9uca(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV9UCA."]
        #[inline(always)]
        pub const fn set_exev9uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc EXEV10UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev10uca(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV10UCA."]
        #[inline(always)]
        pub const fn set_exev10uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc CMEUCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmeuca(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMEUCA."]
        #[inline(always)]
        pub const fn set_cmeuca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc CMFUCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmfuca(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMFUCA."]
        #[inline(always)]
        pub const fn set_cmfuca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc SCMAUCA."]
        #[must_use]
        #[inline(always)]
        pub const fn scmauca(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMAUCA."]
        #[inline(always)]
        pub const fn set_scmauca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "desc SCMBUCA."]
        #[must_use]
        #[inline(always)]
        pub const fn scmbuca(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMBUCA."]
        #[inline(always)]
        pub const fn set_scmbuca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
    }
    impl Default for Bpcnar2 {
        #[inline(always)]
        fn default() -> Bpcnar2 {
            Bpcnar2(0)
        }
    }
    impl core::fmt::Debug for Bpcnar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bpcnar2")
                .field("exev1uca", &self.exev1uca())
                .field("exev2uca", &self.exev2uca())
                .field("exev3uca", &self.exev3uca())
                .field("exev4uca", &self.exev4uca())
                .field("exev5uca", &self.exev5uca())
                .field("exev6uca", &self.exev6uca())
                .field("exev7uca", &self.exev7uca())
                .field("exev8uca", &self.exev8uca())
                .field("exev9uca", &self.exev9uca())
                .field("exev10uca", &self.exev10uca())
                .field("cmeuca", &self.cmeuca())
                .field("cmfuca", &self.cmfuca())
                .field("scmauca", &self.scmauca())
                .field("scmbuca", &self.scmbuca())
                .finish()
        }
    }
    #[doc = "desc BPCNAR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bpcnar3(pub u32);
    impl Bpcnar3 {
        #[doc = "desc EXEV1DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev1dca(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV1DCA."]
        #[inline(always)]
        pub const fn set_exev1dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EXEV2DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev2dca(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV2DCA."]
        #[inline(always)]
        pub const fn set_exev2dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc EXEV3DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev3dca(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV3DCA."]
        #[inline(always)]
        pub const fn set_exev3dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EXEV4DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev4dca(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV4DCA."]
        #[inline(always)]
        pub const fn set_exev4dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc EXEV5DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev5dca(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV5DCA."]
        #[inline(always)]
        pub const fn set_exev5dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc EXEV6DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev6dca(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV6DCA."]
        #[inline(always)]
        pub const fn set_exev6dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc EXEV7DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev7dca(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV7DCA."]
        #[inline(always)]
        pub const fn set_exev7dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc EXEV8DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev8dca(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV8DCA."]
        #[inline(always)]
        pub const fn set_exev8dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc EXEV9DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev9dca(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV9DCA."]
        #[inline(always)]
        pub const fn set_exev9dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc EXEV10DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev10dca(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV10DCA."]
        #[inline(always)]
        pub const fn set_exev10dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc CMEDCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmedca(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMEDCA."]
        #[inline(always)]
        pub const fn set_cmedca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc CMFDCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmfdca(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMFDCA."]
        #[inline(always)]
        pub const fn set_cmfdca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc SCMADCA."]
        #[must_use]
        #[inline(always)]
        pub const fn scmadca(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMADCA."]
        #[inline(always)]
        pub const fn set_scmadca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "desc SCMBDCA."]
        #[must_use]
        #[inline(always)]
        pub const fn scmbdca(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMBDCA."]
        #[inline(always)]
        pub const fn set_scmbdca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
    }
    impl Default for Bpcnar3 {
        #[inline(always)]
        fn default() -> Bpcnar3 {
            Bpcnar3(0)
        }
    }
    impl core::fmt::Debug for Bpcnar3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bpcnar3")
                .field("exev1dca", &self.exev1dca())
                .field("exev2dca", &self.exev2dca())
                .field("exev3dca", &self.exev3dca())
                .field("exev4dca", &self.exev4dca())
                .field("exev5dca", &self.exev5dca())
                .field("exev6dca", &self.exev6dca())
                .field("exev7dca", &self.exev7dca())
                .field("exev8dca", &self.exev8dca())
                .field("exev9dca", &self.exev9dca())
                .field("exev10dca", &self.exev10dca())
                .field("cmedca", &self.cmedca())
                .field("cmfdca", &self.cmfdca())
                .field("scmadca", &self.scmadca())
                .field("scmbdca", &self.scmbdca())
                .finish()
        }
    }
    #[doc = "desc BPCNBR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bpcnbr1(pub u32);
    impl Bpcnbr1 {
        #[doc = "desc STACB."]
        #[must_use]
        #[inline(always)]
        pub const fn stacb(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc STACB."]
        #[inline(always)]
        pub const fn set_stacb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc STPCB."]
        #[must_use]
        #[inline(always)]
        pub const fn stpcb(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc STPCB."]
        #[inline(always)]
        pub const fn set_stpcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc OVFCB."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfcb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc OVFCB."]
        #[inline(always)]
        pub const fn set_ovfcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc UDFCB."]
        #[must_use]
        #[inline(always)]
        pub const fn udfcb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc UDFCB."]
        #[inline(always)]
        pub const fn set_udfcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc CMAUCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmaucb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMAUCB."]
        #[inline(always)]
        pub const fn set_cmaucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc CMADCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmadcb(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMADCB."]
        #[inline(always)]
        pub const fn set_cmadcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc CMBUCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmbucb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMBUCB."]
        #[inline(always)]
        pub const fn set_cmbucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc CMBDCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmbdcb(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMBDCB."]
        #[inline(always)]
        pub const fn set_cmbdcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc FORCB."]
        #[must_use]
        #[inline(always)]
        pub const fn forcb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc FORCB."]
        #[inline(always)]
        pub const fn set_forcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc EMBCB."]
        #[must_use]
        #[inline(always)]
        pub const fn embcb(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc EMBCB."]
        #[inline(always)]
        pub const fn set_embcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc EMBRB."]
        #[must_use]
        #[inline(always)]
        pub const fn embrb(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc EMBRB."]
        #[inline(always)]
        pub const fn set_embrb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc EMBSB."]
        #[must_use]
        #[inline(always)]
        pub const fn embsb(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc EMBSB."]
        #[inline(always)]
        pub const fn set_embsb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "desc OUTENB."]
        #[must_use]
        #[inline(always)]
        pub const fn outenb(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc OUTENB."]
        #[inline(always)]
        pub const fn set_outenb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Bpcnbr1 {
        #[inline(always)]
        fn default() -> Bpcnbr1 {
            Bpcnbr1(0)
        }
    }
    impl core::fmt::Debug for Bpcnbr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bpcnbr1")
                .field("stacb", &self.stacb())
                .field("stpcb", &self.stpcb())
                .field("ovfcb", &self.ovfcb())
                .field("udfcb", &self.udfcb())
                .field("cmaucb", &self.cmaucb())
                .field("cmadcb", &self.cmadcb())
                .field("cmbucb", &self.cmbucb())
                .field("cmbdcb", &self.cmbdcb())
                .field("forcb", &self.forcb())
                .field("embcb", &self.embcb())
                .field("embrb", &self.embrb())
                .field("embsb", &self.embsb())
                .field("outenb", &self.outenb())
                .finish()
        }
    }
    #[doc = "desc BPCNBR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bpcnbr2(pub u32);
    impl Bpcnbr2 {
        #[doc = "desc EXEV1UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev1ucb(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV1UCB."]
        #[inline(always)]
        pub const fn set_exev1ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EXEV2UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev2ucb(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV2UCB."]
        #[inline(always)]
        pub const fn set_exev2ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc EXEV3UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev3ucb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV3UCB."]
        #[inline(always)]
        pub const fn set_exev3ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EXEV4UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev4ucb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV4UCB."]
        #[inline(always)]
        pub const fn set_exev4ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc EXEV5UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev5ucb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV5UCB."]
        #[inline(always)]
        pub const fn set_exev5ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc EXEV6UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev6ucb(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV6UCB."]
        #[inline(always)]
        pub const fn set_exev6ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc EXEV7UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev7ucb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV7UCB."]
        #[inline(always)]
        pub const fn set_exev7ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc EXEV8UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev8ucb(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV8UCB."]
        #[inline(always)]
        pub const fn set_exev8ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc EXEV9UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev9ucb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV9UCB."]
        #[inline(always)]
        pub const fn set_exev9ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc EXEV10UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev10ucb(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV10UCB."]
        #[inline(always)]
        pub const fn set_exev10ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc CMEUCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmeucb(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMEUCB."]
        #[inline(always)]
        pub const fn set_cmeucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc CMFUCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmfucb(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMFUCB."]
        #[inline(always)]
        pub const fn set_cmfucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc SCMAUCB."]
        #[must_use]
        #[inline(always)]
        pub const fn scmaucb(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMAUCB."]
        #[inline(always)]
        pub const fn set_scmaucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "desc SCMBUCB."]
        #[must_use]
        #[inline(always)]
        pub const fn scmbucb(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMBUCB."]
        #[inline(always)]
        pub const fn set_scmbucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
    }
    impl Default for Bpcnbr2 {
        #[inline(always)]
        fn default() -> Bpcnbr2 {
            Bpcnbr2(0)
        }
    }
    impl core::fmt::Debug for Bpcnbr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bpcnbr2")
                .field("exev1ucb", &self.exev1ucb())
                .field("exev2ucb", &self.exev2ucb())
                .field("exev3ucb", &self.exev3ucb())
                .field("exev4ucb", &self.exev4ucb())
                .field("exev5ucb", &self.exev5ucb())
                .field("exev6ucb", &self.exev6ucb())
                .field("exev7ucb", &self.exev7ucb())
                .field("exev8ucb", &self.exev8ucb())
                .field("exev9ucb", &self.exev9ucb())
                .field("exev10ucb", &self.exev10ucb())
                .field("cmeucb", &self.cmeucb())
                .field("cmfucb", &self.cmfucb())
                .field("scmaucb", &self.scmaucb())
                .field("scmbucb", &self.scmbucb())
                .finish()
        }
    }
    #[doc = "desc BPCNBR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bpcnbr3(pub u32);
    impl Bpcnbr3 {
        #[doc = "desc EXEV1DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev1dcb(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV1DCB."]
        #[inline(always)]
        pub const fn set_exev1dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EXEV2DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev2dcb(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV2DCB."]
        #[inline(always)]
        pub const fn set_exev2dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc EXEV3DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev3dcb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV3DCB."]
        #[inline(always)]
        pub const fn set_exev3dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EXEV4DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev4dcb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV4DCB."]
        #[inline(always)]
        pub const fn set_exev4dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc EXEV5DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev5dcb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV5DCB."]
        #[inline(always)]
        pub const fn set_exev5dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc EXEV6DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev6dcb(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV6DCB."]
        #[inline(always)]
        pub const fn set_exev6dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc EXEV7DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev7dcb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV7DCB."]
        #[inline(always)]
        pub const fn set_exev7dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc EXEV8DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev8dcb(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV8DCB."]
        #[inline(always)]
        pub const fn set_exev8dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc EXEV9DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev9dcb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV9DCB."]
        #[inline(always)]
        pub const fn set_exev9dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc EXEV10DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev10dcb(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV10DCB."]
        #[inline(always)]
        pub const fn set_exev10dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc CMEDCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmedcb(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMEDCB."]
        #[inline(always)]
        pub const fn set_cmedcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc CMFDCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmfdcb(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMFDCB."]
        #[inline(always)]
        pub const fn set_cmfdcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc SCMADCB."]
        #[must_use]
        #[inline(always)]
        pub const fn scmadcb(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMADCB."]
        #[inline(always)]
        pub const fn set_scmadcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "desc SCMBDCB."]
        #[must_use]
        #[inline(always)]
        pub const fn scmbdcb(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMBDCB."]
        #[inline(always)]
        pub const fn set_scmbdcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
    }
    impl Default for Bpcnbr3 {
        #[inline(always)]
        fn default() -> Bpcnbr3 {
            Bpcnbr3(0)
        }
    }
    impl core::fmt::Debug for Bpcnbr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bpcnbr3")
                .field("exev1dcb", &self.exev1dcb())
                .field("exev2dcb", &self.exev2dcb())
                .field("exev3dcb", &self.exev3dcb())
                .field("exev4dcb", &self.exev4dcb())
                .field("exev5dcb", &self.exev5dcb())
                .field("exev6dcb", &self.exev6dcb())
                .field("exev7dcb", &self.exev7dcb())
                .field("exev8dcb", &self.exev8dcb())
                .field("exev9dcb", &self.exev9dcb())
                .field("exev10dcb", &self.exev10dcb())
                .field("cmedcb", &self.cmedcb())
                .field("cmfdcb", &self.cmfdcb())
                .field("scmadcb", &self.scmadcb())
                .field("scmbdcb", &self.scmbdcb())
                .finish()
        }
    }
    #[doc = "desc CAPAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Capar(pub u32);
    impl Capar {
        #[doc = "desc CAPA."]
        #[must_use]
        #[inline(always)]
        pub const fn capa(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc CAPA."]
        #[inline(always)]
        pub const fn set_capa(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
        #[doc = "desc CAPDIRA."]
        #[must_use]
        #[inline(always)]
        pub const fn capdira(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPDIRA."]
        #[inline(always)]
        pub const fn set_capdira(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Capar {
        #[inline(always)]
        fn default() -> Capar {
            Capar(0)
        }
    }
    impl core::fmt::Debug for Capar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Capar")
                .field("capa", &self.capa())
                .field("capdira", &self.capdira())
                .finish()
        }
    }
    #[doc = "desc CAPBR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Capbr(pub u32);
    impl Capbr {
        #[doc = "desc CAPB."]
        #[must_use]
        #[inline(always)]
        pub const fn capb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc CAPB."]
        #[inline(always)]
        pub const fn set_capb(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
        #[doc = "desc CAPDIRB."]
        #[must_use]
        #[inline(always)]
        pub const fn capdirb(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPDIRB."]
        #[inline(always)]
        pub const fn set_capdirb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Capbr {
        #[inline(always)]
        fn default() -> Capbr {
            Capbr(0)
        }
    }
    impl core::fmt::Debug for Capbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Capbr")
                .field("capb", &self.capb())
                .field("capdirb", &self.capdirb())
                .finish()
        }
    }
    #[doc = "desc CNTER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnter(pub u32);
    impl Cnter {
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Cnter {
        #[inline(always)]
        fn default() -> Cnter {
            Cnter(0)
        }
    }
    impl core::fmt::Debug for Cnter {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnter").field("cnt", &self.cnt()).finish()
        }
    }
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc DACSYNC1."]
        #[must_use]
        #[inline(always)]
        pub const fn dacsync1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc DACSYNC1."]
        #[inline(always)]
        pub const fn set_dacsync1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc DACSYNC2."]
        #[must_use]
        #[inline(always)]
        pub const fn dacsync2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc DACSYNC2."]
        #[inline(always)]
        pub const fn set_dacsync2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc DACSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn dacsrc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc DACSRC."]
        #[inline(always)]
        pub const fn set_dacsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("en", &self.en())
                .field("dacsync1", &self.dacsync1())
                .field("dacsync2", &self.dacsync2())
                .field("dacsrc", &self.dacsrc())
                .finish()
        }
    }
    #[doc = "desc DCONR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dconr(pub u32);
    impl Dconr {
        #[doc = "desc DTCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dtcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTCEN."]
        #[inline(always)]
        pub const fn set_dtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SEPA."]
        #[must_use]
        #[inline(always)]
        pub const fn sepa(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEPA."]
        #[inline(always)]
        pub const fn set_sepa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc DTBENU."]
        #[must_use]
        #[inline(always)]
        pub const fn dtbenu(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTBENU."]
        #[inline(always)]
        pub const fn set_dtbenu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DTBEND."]
        #[must_use]
        #[inline(always)]
        pub const fn dtbend(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTBEND."]
        #[inline(always)]
        pub const fn set_dtbend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc DTBTRU."]
        #[must_use]
        #[inline(always)]
        pub const fn dtbtru(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTBTRU."]
        #[inline(always)]
        pub const fn set_dtbtru(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DTBTRD."]
        #[must_use]
        #[inline(always)]
        pub const fn dtbtrd(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTBTRD."]
        #[inline(always)]
        pub const fn set_dtbtrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Dconr {
        #[inline(always)]
        fn default() -> Dconr {
            Dconr(0)
        }
    }
    impl core::fmt::Debug for Dconr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dconr")
                .field("dtcen", &self.dtcen())
                .field("sepa", &self.sepa())
                .field("dtbenu", &self.dtbenu())
                .field("dtbend", &self.dtbend())
                .field("dtbtru", &self.dtbtru())
                .field("dtbtrd", &self.dtbtrd())
                .finish()
        }
    }
    #[doc = "desc EEFLTCR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eefltcr1(pub u32);
    impl Eefltcr1 {
        #[doc = "desc EE1LAT."]
        #[must_use]
        #[inline(always)]
        pub const fn ee1lat(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE1LAT."]
        #[inline(always)]
        pub const fn set_ee1lat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EE1FM."]
        #[must_use]
        #[inline(always)]
        pub const fn ee1fm(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EE1FM."]
        #[inline(always)]
        pub const fn set_ee1fm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
        }
        #[doc = "desc EE1TMO."]
        #[must_use]
        #[inline(always)]
        pub const fn ee1tmo(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE1TMO."]
        #[inline(always)]
        pub const fn set_ee1tmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc EE2LAT."]
        #[must_use]
        #[inline(always)]
        pub const fn ee2lat(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE2LAT."]
        #[inline(always)]
        pub const fn set_ee2lat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc EE2FM."]
        #[must_use]
        #[inline(always)]
        pub const fn ee2fm(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EE2FM."]
        #[inline(always)]
        pub const fn set_ee2fm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
        #[doc = "desc EE2TMO."]
        #[must_use]
        #[inline(always)]
        pub const fn ee2tmo(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE2TMO."]
        #[inline(always)]
        pub const fn set_ee2tmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc EE3LAT."]
        #[must_use]
        #[inline(always)]
        pub const fn ee3lat(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE3LAT."]
        #[inline(always)]
        pub const fn set_ee3lat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc EE3FM."]
        #[must_use]
        #[inline(always)]
        pub const fn ee3fm(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EE3FM."]
        #[inline(always)]
        pub const fn set_ee3fm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[doc = "desc EE3TMO."]
        #[must_use]
        #[inline(always)]
        pub const fn ee3tmo(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE3TMO."]
        #[inline(always)]
        pub const fn set_ee3tmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EE4LAT."]
        #[must_use]
        #[inline(always)]
        pub const fn ee4lat(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE4LAT."]
        #[inline(always)]
        pub const fn set_ee4lat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc EE4FM."]
        #[must_use]
        #[inline(always)]
        pub const fn ee4fm(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EE4FM."]
        #[inline(always)]
        pub const fn set_ee4fm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
        }
        #[doc = "desc EE4TMO."]
        #[must_use]
        #[inline(always)]
        pub const fn ee4tmo(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE4TMO."]
        #[inline(always)]
        pub const fn set_ee4tmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc EE5LAT."]
        #[must_use]
        #[inline(always)]
        pub const fn ee5lat(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE5LAT."]
        #[inline(always)]
        pub const fn set_ee5lat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc EE5FM."]
        #[must_use]
        #[inline(always)]
        pub const fn ee5fm(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EE5FM."]
        #[inline(always)]
        pub const fn set_ee5fm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
        }
        #[doc = "desc EE5TMO."]
        #[must_use]
        #[inline(always)]
        pub const fn ee5tmo(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE5TMO."]
        #[inline(always)]
        pub const fn set_ee5tmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EEINTPOL."]
        #[must_use]
        #[inline(always)]
        pub const fn eeintpol(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EEINTPOL."]
        #[inline(always)]
        pub const fn set_eeintpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Eefltcr1 {
        #[inline(always)]
        fn default() -> Eefltcr1 {
            Eefltcr1(0)
        }
    }
    impl core::fmt::Debug for Eefltcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eefltcr1")
                .field("ee1lat", &self.ee1lat())
                .field("ee1fm", &self.ee1fm())
                .field("ee1tmo", &self.ee1tmo())
                .field("ee2lat", &self.ee2lat())
                .field("ee2fm", &self.ee2fm())
                .field("ee2tmo", &self.ee2tmo())
                .field("ee3lat", &self.ee3lat())
                .field("ee3fm", &self.ee3fm())
                .field("ee3tmo", &self.ee3tmo())
                .field("ee4lat", &self.ee4lat())
                .field("ee4fm", &self.ee4fm())
                .field("ee4tmo", &self.ee4tmo())
                .field("ee5lat", &self.ee5lat())
                .field("ee5fm", &self.ee5fm())
                .field("ee5tmo", &self.ee5tmo())
                .field("eeintpol", &self.eeintpol())
                .finish()
        }
    }
    #[doc = "desc EEFLTCR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eefltcr2(pub u32);
    impl Eefltcr2 {
        #[doc = "desc EE6LAT."]
        #[must_use]
        #[inline(always)]
        pub const fn ee6lat(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE6LAT."]
        #[inline(always)]
        pub const fn set_ee6lat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EE6FM."]
        #[must_use]
        #[inline(always)]
        pub const fn ee6fm(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EE6FM."]
        #[inline(always)]
        pub const fn set_ee6fm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
        }
        #[doc = "desc EE6TMO."]
        #[must_use]
        #[inline(always)]
        pub const fn ee6tmo(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE6TMO."]
        #[inline(always)]
        pub const fn set_ee6tmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc EE7LAT."]
        #[must_use]
        #[inline(always)]
        pub const fn ee7lat(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE7LAT."]
        #[inline(always)]
        pub const fn set_ee7lat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc EE7FM."]
        #[must_use]
        #[inline(always)]
        pub const fn ee7fm(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EE7FM."]
        #[inline(always)]
        pub const fn set_ee7fm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
        #[doc = "desc EE7TMO."]
        #[must_use]
        #[inline(always)]
        pub const fn ee7tmo(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE7TMO."]
        #[inline(always)]
        pub const fn set_ee7tmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc EE8LAT."]
        #[must_use]
        #[inline(always)]
        pub const fn ee8lat(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE8LAT."]
        #[inline(always)]
        pub const fn set_ee8lat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc EE8FM."]
        #[must_use]
        #[inline(always)]
        pub const fn ee8fm(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EE8FM."]
        #[inline(always)]
        pub const fn set_ee8fm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[doc = "desc EE8TMO."]
        #[must_use]
        #[inline(always)]
        pub const fn ee8tmo(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE8TMO."]
        #[inline(always)]
        pub const fn set_ee8tmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EE9LAT."]
        #[must_use]
        #[inline(always)]
        pub const fn ee9lat(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE9LAT."]
        #[inline(always)]
        pub const fn set_ee9lat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc EE9FM."]
        #[must_use]
        #[inline(always)]
        pub const fn ee9fm(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EE9FM."]
        #[inline(always)]
        pub const fn set_ee9fm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
        }
        #[doc = "desc EE9TMO."]
        #[must_use]
        #[inline(always)]
        pub const fn ee9tmo(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE9TMO."]
        #[inline(always)]
        pub const fn set_ee9tmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc EE10LAT."]
        #[must_use]
        #[inline(always)]
        pub const fn ee10lat(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE10LAT."]
        #[inline(always)]
        pub const fn set_ee10lat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc EE10FM."]
        #[must_use]
        #[inline(always)]
        pub const fn ee10fm(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EE10FM."]
        #[inline(always)]
        pub const fn set_ee10fm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
        }
        #[doc = "desc EE10TMO."]
        #[must_use]
        #[inline(always)]
        pub const fn ee10tmo(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc EE10TMO."]
        #[inline(always)]
        pub const fn set_ee10tmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Eefltcr2 {
        #[inline(always)]
        fn default() -> Eefltcr2 {
            Eefltcr2(0)
        }
    }
    impl core::fmt::Debug for Eefltcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eefltcr2")
                .field("ee6lat", &self.ee6lat())
                .field("ee6fm", &self.ee6fm())
                .field("ee6tmo", &self.ee6tmo())
                .field("ee7lat", &self.ee7lat())
                .field("ee7fm", &self.ee7fm())
                .field("ee7tmo", &self.ee7tmo())
                .field("ee8lat", &self.ee8lat())
                .field("ee8fm", &self.ee8fm())
                .field("ee8tmo", &self.ee8tmo())
                .field("ee9lat", &self.ee9lat())
                .field("ee9fm", &self.ee9fm())
                .field("ee9tmo", &self.ee9tmo())
                .field("ee10lat", &self.ee10lat())
                .field("ee10fm", &self.ee10fm())
                .field("ee10tmo", &self.ee10tmo())
                .finish()
        }
    }
    #[doc = "desc EEFOFFSETAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eefoffsetar(pub u32);
    impl Eefoffsetar {
        #[doc = "desc OFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc OFFSET."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
        #[doc = "desc OFFSETDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn offsetdir(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc OFFSETDIR."]
        #[inline(always)]
        pub const fn set_offsetdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Eefoffsetar {
        #[inline(always)]
        fn default() -> Eefoffsetar {
            Eefoffsetar(0)
        }
    }
    impl core::fmt::Debug for Eefoffsetar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eefoffsetar")
                .field("offset", &self.offset())
                .field("offsetdir", &self.offsetdir())
                .finish()
        }
    }
    #[doc = "desc EEFOFFSETBR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eefoffsetbr(pub u32);
    impl Eefoffsetbr {
        #[doc = "desc OFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc OFFSET."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
        #[doc = "desc OFFSETDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn offsetdir(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc OFFSETDIR."]
        #[inline(always)]
        pub const fn set_offsetdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Eefoffsetbr {
        #[inline(always)]
        fn default() -> Eefoffsetbr {
            Eefoffsetbr(0)
        }
    }
    impl core::fmt::Debug for Eefoffsetbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eefoffsetbr")
                .field("offset", &self.offset())
                .field("offsetdir", &self.offsetdir())
                .finish()
        }
    }
    #[doc = "desc EEFWINAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eefwinar(pub u32);
    impl Eefwinar {
        #[doc = "desc WIN."]
        #[must_use]
        #[inline(always)]
        pub const fn win(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc WIN."]
        #[inline(always)]
        pub const fn set_win(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
        #[doc = "desc WINDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn windir(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc WINDIR."]
        #[inline(always)]
        pub const fn set_windir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Eefwinar {
        #[inline(always)]
        fn default() -> Eefwinar {
            Eefwinar(0)
        }
    }
    impl core::fmt::Debug for Eefwinar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eefwinar")
                .field("win", &self.win())
                .field("windir", &self.windir())
                .finish()
        }
    }
    #[doc = "desc EEFWINBR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eefwinbr(pub u32);
    impl Eefwinbr {
        #[doc = "desc WIN."]
        #[must_use]
        #[inline(always)]
        pub const fn win(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc WIN."]
        #[inline(always)]
        pub const fn set_win(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
        #[doc = "desc WINDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn windir(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc WINDIR."]
        #[inline(always)]
        pub const fn set_windir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Eefwinbr {
        #[inline(always)]
        fn default() -> Eefwinbr {
            Eefwinbr(0)
        }
    }
    impl core::fmt::Debug for Eefwinbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eefwinbr")
                .field("win", &self.win())
                .field("windir", &self.windir())
                .finish()
        }
    }
    #[doc = "desc GCONR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gconr(pub u32);
    impl Gconr {
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
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc OVSTP."]
        #[must_use]
        #[inline(always)]
        pub const fn ovstp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVSTP."]
        #[inline(always)]
        pub const fn set_ovstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Gconr {
        #[inline(always)]
        fn default() -> Gconr {
            Gconr(0)
        }
    }
    impl core::fmt::Debug for Gconr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gconr")
                .field("start", &self.start())
                .field("mode", &self.mode())
                .field("ovstp", &self.ovstp())
                .finish()
        }
    }
    #[doc = "desc GCONR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gconr1(pub u32);
    impl Gconr1 {
        #[doc = "desc INVCAEN."]
        #[must_use]
        #[inline(always)]
        pub const fn invcaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVCAEN."]
        #[inline(always)]
        pub const fn set_invcaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc INVCBEN."]
        #[must_use]
        #[inline(always)]
        pub const fn invcben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVCBEN."]
        #[inline(always)]
        pub const fn set_invcben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SWAPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn swapen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWAPEN."]
        #[inline(always)]
        pub const fn set_swapen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SWAPMD."]
        #[must_use]
        #[inline(always)]
        pub const fn swapmd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWAPMD."]
        #[inline(always)]
        pub const fn set_swapmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc HALF."]
        #[must_use]
        #[inline(always)]
        pub const fn half(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc HALF."]
        #[inline(always)]
        pub const fn set_half(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc EEFM."]
        #[must_use]
        #[inline(always)]
        pub const fn eefm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EEFM."]
        #[inline(always)]
        pub const fn set_eefm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc EEFREF."]
        #[must_use]
        #[inline(always)]
        pub const fn eefref(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EEFREF."]
        #[inline(always)]
        pub const fn set_eefref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PRDSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn prdsel(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "desc PRDSEL."]
        #[inline(always)]
        pub const fn set_prdsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "desc CMSCAUEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cmscauen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSCAUEN."]
        #[inline(always)]
        pub const fn set_cmscauen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc CMSCADEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cmscaden(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSCADEN."]
        #[inline(always)]
        pub const fn set_cmscaden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc CMSCBUEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cmscbuen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSCBUEN."]
        #[inline(always)]
        pub const fn set_cmscbuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc CMSCBDEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cmscbden(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSCBDEN."]
        #[inline(always)]
        pub const fn set_cmscbden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc BFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn bfsel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "desc BFSEL."]
        #[inline(always)]
        pub const fn set_bfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for Gconr1 {
        #[inline(always)]
        fn default() -> Gconr1 {
            Gconr1(0)
        }
    }
    impl core::fmt::Debug for Gconr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gconr1")
                .field("invcaen", &self.invcaen())
                .field("invcben", &self.invcben())
                .field("swapen", &self.swapen())
                .field("swapmd", &self.swapmd())
                .field("half", &self.half())
                .field("eefm", &self.eefm())
                .field("eefref", &self.eefref())
                .field("prdsel", &self.prdsel())
                .field("cmscauen", &self.cmscauen())
                .field("cmscaden", &self.cmscaden())
                .field("cmscbuen", &self.cmscbuen())
                .field("cmscbden", &self.cmscbden())
                .field("bfsel", &self.bfsel())
                .finish()
        }
    }
    #[doc = "desc HCLRR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hclrr1(pub u32);
    impl Hclrr1 {
        #[doc = "desc CLES."]
        #[must_use]
        #[inline(always)]
        pub const fn cles(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLES."]
        #[inline(always)]
        pub const fn set_cles(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HCLE8."]
        #[must_use]
        #[inline(always)]
        pub const fn hcle8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLE8."]
        #[inline(always)]
        pub const fn set_hcle8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc HCLE9."]
        #[must_use]
        #[inline(always)]
        pub const fn hcle9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLE9."]
        #[inline(always)]
        pub const fn set_hcle9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc HCLE10."]
        #[must_use]
        #[inline(always)]
        pub const fn hcle10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLE10."]
        #[inline(always)]
        pub const fn set_hcle10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc HCLE11."]
        #[must_use]
        #[inline(always)]
        pub const fn hcle11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLE11."]
        #[inline(always)]
        pub const fn set_hcle11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc HCLE16."]
        #[must_use]
        #[inline(always)]
        pub const fn hcle16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLE16."]
        #[inline(always)]
        pub const fn set_hcle16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc HCLE17."]
        #[must_use]
        #[inline(always)]
        pub const fn hcle17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLE17."]
        #[inline(always)]
        pub const fn set_hcle17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc HCLE18."]
        #[must_use]
        #[inline(always)]
        pub const fn hcle18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLE18."]
        #[inline(always)]
        pub const fn set_hcle18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc HCLE19."]
        #[must_use]
        #[inline(always)]
        pub const fn hcle19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLE19."]
        #[inline(always)]
        pub const fn set_hcle19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc HCLE20."]
        #[must_use]
        #[inline(always)]
        pub const fn hcle20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLE20."]
        #[inline(always)]
        pub const fn set_hcle20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc HCLE21."]
        #[must_use]
        #[inline(always)]
        pub const fn hcle21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLE21."]
        #[inline(always)]
        pub const fn set_hcle21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc HCLE22."]
        #[must_use]
        #[inline(always)]
        pub const fn hcle22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLE22."]
        #[inline(always)]
        pub const fn set_hcle22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc HCLE23."]
        #[must_use]
        #[inline(always)]
        pub const fn hcle23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLE23."]
        #[inline(always)]
        pub const fn set_hcle23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Hclrr1 {
        #[inline(always)]
        fn default() -> Hclrr1 {
            Hclrr1(0)
        }
    }
    impl core::fmt::Debug for Hclrr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hclrr1")
                .field("cles", &self.cles())
                .field("hcle8", &self.hcle8())
                .field("hcle9", &self.hcle9())
                .field("hcle10", &self.hcle10())
                .field("hcle11", &self.hcle11())
                .field("hcle16", &self.hcle16())
                .field("hcle17", &self.hcle17())
                .field("hcle18", &self.hcle18())
                .field("hcle19", &self.hcle19())
                .field("hcle20", &self.hcle20())
                .field("hcle21", &self.hcle21())
                .field("hcle22", &self.hcle22())
                .field("hcle23", &self.hcle23())
                .finish()
        }
    }
    #[doc = "desc HCLRR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hclrr2(pub u32);
    impl Hclrr2 {
        #[doc = "desc HCLREXEV1."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrexev1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLREXEV1."]
        #[inline(always)]
        pub const fn set_hclrexev1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc HCLREXEV2."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrexev2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLREXEV2."]
        #[inline(always)]
        pub const fn set_hclrexev2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HCLREXEV3."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrexev3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLREXEV3."]
        #[inline(always)]
        pub const fn set_hclrexev3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc HCLREXEV4."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrexev4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLREXEV4."]
        #[inline(always)]
        pub const fn set_hclrexev4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc HCLREXEV5."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrexev5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLREXEV5."]
        #[inline(always)]
        pub const fn set_hclrexev5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc HCLREXEV6."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrexev6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLREXEV6."]
        #[inline(always)]
        pub const fn set_hclrexev6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc HCLREXEV7."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrexev7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLREXEV7."]
        #[inline(always)]
        pub const fn set_hclrexev7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc HCLREXEV8."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrexev8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLREXEV8."]
        #[inline(always)]
        pub const fn set_hclrexev8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HCLREXEV9."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrexev9(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLREXEV9."]
        #[inline(always)]
        pub const fn set_hclrexev9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc HCLREXEV10."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrexev10(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLREXEV10."]
        #[inline(always)]
        pub const fn set_hclrexev10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc HCLRGCMA1."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrgcma1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRGCMA1."]
        #[inline(always)]
        pub const fn set_hclrgcma1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc HCLRGCMB1."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrgcmb1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRGCMB1."]
        #[inline(always)]
        pub const fn set_hclrgcmb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc HCLRGCMA2."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrgcma2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRGCMA2."]
        #[inline(always)]
        pub const fn set_hclrgcma2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc HCLRGCMB2."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrgcmb2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRGCMB2."]
        #[inline(always)]
        pub const fn set_hclrgcmb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc HCLRGCMA3."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrgcma3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRGCMA3."]
        #[inline(always)]
        pub const fn set_hclrgcma3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc HCLRGCMB3."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrgcmb3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRGCMB3."]
        #[inline(always)]
        pub const fn set_hclrgcmb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc HCLRGCMA4."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrgcma4(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRGCMA4."]
        #[inline(always)]
        pub const fn set_hclrgcma4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc HCLRGCMB4."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrgcmb4(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRGCMB4."]
        #[inline(always)]
        pub const fn set_hclrgcmb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc HCLRGCMA5."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrgcma5(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRGCMA5."]
        #[inline(always)]
        pub const fn set_hclrgcma5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc HCLRGCMB5."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrgcmb5(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRGCMB5."]
        #[inline(always)]
        pub const fn set_hclrgcmb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc HCLRGCMA6."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrgcma6(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRGCMA6."]
        #[inline(always)]
        pub const fn set_hclrgcma6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc HCLRGCMB6."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrgcmb6(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRGCMB6."]
        #[inline(always)]
        pub const fn set_hclrgcmb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc HCLRSCMB1."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrscmb1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRSCMB1."]
        #[inline(always)]
        pub const fn set_hclrscmb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc HCLRSCMB2."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrscmb2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRSCMB2."]
        #[inline(always)]
        pub const fn set_hclrscmb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc HCLRSCMB3."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrscmb3(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRSCMB3."]
        #[inline(always)]
        pub const fn set_hclrscmb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc HCLRSCMB4."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrscmb4(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRSCMB4."]
        #[inline(always)]
        pub const fn set_hclrscmb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc HCLRSCMB5."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrscmb5(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRSCMB5."]
        #[inline(always)]
        pub const fn set_hclrscmb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc HCLRSCMB6."]
        #[must_use]
        #[inline(always)]
        pub const fn hclrscmb6(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLRSCMB6."]
        #[inline(always)]
        pub const fn set_hclrscmb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Hclrr2 {
        #[inline(always)]
        fn default() -> Hclrr2 {
            Hclrr2(0)
        }
    }
    impl core::fmt::Debug for Hclrr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hclrr2")
                .field("hclrexev1", &self.hclrexev1())
                .field("hclrexev2", &self.hclrexev2())
                .field("hclrexev3", &self.hclrexev3())
                .field("hclrexev4", &self.hclrexev4())
                .field("hclrexev5", &self.hclrexev5())
                .field("hclrexev6", &self.hclrexev6())
                .field("hclrexev7", &self.hclrexev7())
                .field("hclrexev8", &self.hclrexev8())
                .field("hclrexev9", &self.hclrexev9())
                .field("hclrexev10", &self.hclrexev10())
                .field("hclrgcma1", &self.hclrgcma1())
                .field("hclrgcmb1", &self.hclrgcmb1())
                .field("hclrgcma2", &self.hclrgcma2())
                .field("hclrgcmb2", &self.hclrgcmb2())
                .field("hclrgcma3", &self.hclrgcma3())
                .field("hclrgcmb3", &self.hclrgcmb3())
                .field("hclrgcma4", &self.hclrgcma4())
                .field("hclrgcmb4", &self.hclrgcmb4())
                .field("hclrgcma5", &self.hclrgcma5())
                .field("hclrgcmb5", &self.hclrgcmb5())
                .field("hclrgcma6", &self.hclrgcma6())
                .field("hclrgcmb6", &self.hclrgcmb6())
                .field("hclrscmb1", &self.hclrscmb1())
                .field("hclrscmb2", &self.hclrscmb2())
                .field("hclrscmb3", &self.hclrscmb3())
                .field("hclrscmb4", &self.hclrscmb4())
                .field("hclrscmb5", &self.hclrscmb5())
                .field("hclrscmb6", &self.hclrscmb6())
                .finish()
        }
    }
    #[doc = "desc HCPAR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcpar1(pub u32);
    impl Hcpar1 {
        #[doc = "desc HCPA8."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpa8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPA8."]
        #[inline(always)]
        pub const fn set_hcpa8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc HCPA9."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpa9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPA9."]
        #[inline(always)]
        pub const fn set_hcpa9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc HCPA10."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPA10."]
        #[inline(always)]
        pub const fn set_hcpa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc HCPA11."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPA11."]
        #[inline(always)]
        pub const fn set_hcpa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc HCPA16."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpa16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPA16."]
        #[inline(always)]
        pub const fn set_hcpa16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc HCPA17."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpa17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPA17."]
        #[inline(always)]
        pub const fn set_hcpa17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc HCPA18."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpa18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPA18."]
        #[inline(always)]
        pub const fn set_hcpa18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc HCPA19."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpa19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPA19."]
        #[inline(always)]
        pub const fn set_hcpa19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc HCPA20."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpa20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPA20."]
        #[inline(always)]
        pub const fn set_hcpa20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc HCPA21."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpa21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPA21."]
        #[inline(always)]
        pub const fn set_hcpa21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc HCPA22."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpa22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPA22."]
        #[inline(always)]
        pub const fn set_hcpa22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc HCPA23."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpa23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPA23."]
        #[inline(always)]
        pub const fn set_hcpa23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Hcpar1 {
        #[inline(always)]
        fn default() -> Hcpar1 {
            Hcpar1(0)
        }
    }
    impl core::fmt::Debug for Hcpar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcpar1")
                .field("hcpa8", &self.hcpa8())
                .field("hcpa9", &self.hcpa9())
                .field("hcpa10", &self.hcpa10())
                .field("hcpa11", &self.hcpa11())
                .field("hcpa16", &self.hcpa16())
                .field("hcpa17", &self.hcpa17())
                .field("hcpa18", &self.hcpa18())
                .field("hcpa19", &self.hcpa19())
                .field("hcpa20", &self.hcpa20())
                .field("hcpa21", &self.hcpa21())
                .field("hcpa22", &self.hcpa22())
                .field("hcpa23", &self.hcpa23())
                .finish()
        }
    }
    #[doc = "desc HCPAR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcpar2(pub u32);
    impl Hcpar2 {
        #[doc = "desc HCPAEEV1."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpaeev1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPAEEV1."]
        #[inline(always)]
        pub const fn set_hcpaeev1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc HCPAEEV2."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpaeev2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPAEEV2."]
        #[inline(always)]
        pub const fn set_hcpaeev2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HCPAEEV3."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpaeev3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPAEEV3."]
        #[inline(always)]
        pub const fn set_hcpaeev3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc HCPAEEV4."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpaeev4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPAEEV4."]
        #[inline(always)]
        pub const fn set_hcpaeev4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc HCPAEEV5."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpaeev5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPAEEV5."]
        #[inline(always)]
        pub const fn set_hcpaeev5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc HCPAEEV6."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpaeev6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPAEEV6."]
        #[inline(always)]
        pub const fn set_hcpaeev6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc HCPAEEV7."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpaeev7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPAEEV7."]
        #[inline(always)]
        pub const fn set_hcpaeev7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc HCPAEEV8."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpaeev8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPAEEV8."]
        #[inline(always)]
        pub const fn set_hcpaeev8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HCPAEEV9."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpaeev9(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPAEEV9."]
        #[inline(always)]
        pub const fn set_hcpaeev9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc HCPAEEV10."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpaeev10(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPAEEV10."]
        #[inline(always)]
        pub const fn set_hcpaeev10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc HCPAEN."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpaen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPAEN."]
        #[inline(always)]
        pub const fn set_hcpaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcpar2 {
        #[inline(always)]
        fn default() -> Hcpar2 {
            Hcpar2(0)
        }
    }
    impl core::fmt::Debug for Hcpar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcpar2")
                .field("hcpaeev1", &self.hcpaeev1())
                .field("hcpaeev2", &self.hcpaeev2())
                .field("hcpaeev3", &self.hcpaeev3())
                .field("hcpaeev4", &self.hcpaeev4())
                .field("hcpaeev5", &self.hcpaeev5())
                .field("hcpaeev6", &self.hcpaeev6())
                .field("hcpaeev7", &self.hcpaeev7())
                .field("hcpaeev8", &self.hcpaeev8())
                .field("hcpaeev9", &self.hcpaeev9())
                .field("hcpaeev10", &self.hcpaeev10())
                .field("hcpaen", &self.hcpaen())
                .finish()
        }
    }
    #[doc = "desc HCPBR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcpbr1(pub u32);
    impl Hcpbr1 {
        #[doc = "desc HCPB8."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPB8."]
        #[inline(always)]
        pub const fn set_hcpb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc HCPB9."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPB9."]
        #[inline(always)]
        pub const fn set_hcpb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc HCPB10."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPB10."]
        #[inline(always)]
        pub const fn set_hcpb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc HCPB11."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPB11."]
        #[inline(always)]
        pub const fn set_hcpb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc HCPB16."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPB16."]
        #[inline(always)]
        pub const fn set_hcpb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc HCPB17."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPB17."]
        #[inline(always)]
        pub const fn set_hcpb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc HCPB18."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPB18."]
        #[inline(always)]
        pub const fn set_hcpb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc HCPB19."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPB19."]
        #[inline(always)]
        pub const fn set_hcpb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc HCPB20."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPB20."]
        #[inline(always)]
        pub const fn set_hcpb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc HCPB21."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPB21."]
        #[inline(always)]
        pub const fn set_hcpb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc HCPB22."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPB22."]
        #[inline(always)]
        pub const fn set_hcpb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc HCPB23."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPB23."]
        #[inline(always)]
        pub const fn set_hcpb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Hcpbr1 {
        #[inline(always)]
        fn default() -> Hcpbr1 {
            Hcpbr1(0)
        }
    }
    impl core::fmt::Debug for Hcpbr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcpbr1")
                .field("hcpb8", &self.hcpb8())
                .field("hcpb9", &self.hcpb9())
                .field("hcpb10", &self.hcpb10())
                .field("hcpb11", &self.hcpb11())
                .field("hcpb16", &self.hcpb16())
                .field("hcpb17", &self.hcpb17())
                .field("hcpb18", &self.hcpb18())
                .field("hcpb19", &self.hcpb19())
                .field("hcpb20", &self.hcpb20())
                .field("hcpb21", &self.hcpb21())
                .field("hcpb22", &self.hcpb22())
                .field("hcpb23", &self.hcpb23())
                .finish()
        }
    }
    #[doc = "desc HCPBR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcpbr2(pub u32);
    impl Hcpbr2 {
        #[doc = "desc HCPBEEV1."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpbeev1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPBEEV1."]
        #[inline(always)]
        pub const fn set_hcpbeev1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc HCPBEEV2."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpbeev2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPBEEV2."]
        #[inline(always)]
        pub const fn set_hcpbeev2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HCPBEEV3."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpbeev3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPBEEV3."]
        #[inline(always)]
        pub const fn set_hcpbeev3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc HCPBEEV4."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpbeev4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPBEEV4."]
        #[inline(always)]
        pub const fn set_hcpbeev4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc HCPBEEV5."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpbeev5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPBEEV5."]
        #[inline(always)]
        pub const fn set_hcpbeev5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc HCPBEEV6."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpbeev6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPBEEV6."]
        #[inline(always)]
        pub const fn set_hcpbeev6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc HCPBEEV7."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpbeev7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPBEEV7."]
        #[inline(always)]
        pub const fn set_hcpbeev7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc HCPBEEV8."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpbeev8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPBEEV8."]
        #[inline(always)]
        pub const fn set_hcpbeev8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HCPBEEV9."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpbeev9(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPBEEV9."]
        #[inline(always)]
        pub const fn set_hcpbeev9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc HCPBEEV10."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpbeev10(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPBEEV10."]
        #[inline(always)]
        pub const fn set_hcpbeev10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc HCPBEN."]
        #[must_use]
        #[inline(always)]
        pub const fn hcpben(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCPBEN."]
        #[inline(always)]
        pub const fn set_hcpben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcpbr2 {
        #[inline(always)]
        fn default() -> Hcpbr2 {
            Hcpbr2(0)
        }
    }
    impl core::fmt::Debug for Hcpbr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcpbr2")
                .field("hcpbeev1", &self.hcpbeev1())
                .field("hcpbeev2", &self.hcpbeev2())
                .field("hcpbeev3", &self.hcpbeev3())
                .field("hcpbeev4", &self.hcpbeev4())
                .field("hcpbeev5", &self.hcpbeev5())
                .field("hcpbeev6", &self.hcpbeev6())
                .field("hcpbeev7", &self.hcpbeev7())
                .field("hcpbeev8", &self.hcpbeev8())
                .field("hcpbeev9", &self.hcpbeev9())
                .field("hcpbeev10", &self.hcpbeev10())
                .field("hcpben", &self.hcpben())
                .finish()
        }
    }
    #[doc = "desc HRDTDAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrdtdar(pub u32);
    impl Hrdtdar {
        #[doc = "desc HRDTDA."]
        #[must_use]
        #[inline(always)]
        pub const fn hrdtda(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRDTDA."]
        #[inline(always)]
        pub const fn set_hrdtda(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrdtdar {
        #[inline(always)]
        fn default() -> Hrdtdar {
            Hrdtdar(0)
        }
    }
    impl core::fmt::Debug for Hrdtdar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrdtdar")
                .field("hrdtda", &self.hrdtda())
                .finish()
        }
    }
    #[doc = "desc HRDTDBR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrdtdbr(pub u32);
    impl Hrdtdbr {
        #[doc = "desc HRDTDB."]
        #[must_use]
        #[inline(always)]
        pub const fn hrdtdb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRDTDB."]
        #[inline(always)]
        pub const fn set_hrdtdb(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrdtdbr {
        #[inline(always)]
        fn default() -> Hrdtdbr {
            Hrdtdbr(0)
        }
    }
    impl core::fmt::Debug for Hrdtdbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrdtdbr")
                .field("hrdtdb", &self.hrdtdb())
                .finish()
        }
    }
    #[doc = "desc HRDTUAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrdtuar(pub u32);
    impl Hrdtuar {
        #[doc = "desc HRDTUA."]
        #[must_use]
        #[inline(always)]
        pub const fn hrdtua(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRDTUA."]
        #[inline(always)]
        pub const fn set_hrdtua(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrdtuar {
        #[inline(always)]
        fn default() -> Hrdtuar {
            Hrdtuar(0)
        }
    }
    impl core::fmt::Debug for Hrdtuar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrdtuar")
                .field("hrdtua", &self.hrdtua())
                .finish()
        }
    }
    #[doc = "desc HRDTUBR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrdtubr(pub u32);
    impl Hrdtubr {
        #[doc = "desc HRDTUB."]
        #[must_use]
        #[inline(always)]
        pub const fn hrdtub(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRDTUB."]
        #[inline(always)]
        pub const fn set_hrdtub(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrdtubr {
        #[inline(always)]
        fn default() -> Hrdtubr {
            Hrdtubr(0)
        }
    }
    impl core::fmt::Debug for Hrdtubr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrdtubr")
                .field("hrdtub", &self.hrdtub())
                .finish()
        }
    }
    #[doc = "desc HRGCMAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrgcmar(pub u32);
    impl Hrgcmar {
        #[doc = "desc HRGCMA."]
        #[must_use]
        #[inline(always)]
        pub const fn hrgcma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRGCMA."]
        #[inline(always)]
        pub const fn set_hrgcma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrgcmar {
        #[inline(always)]
        fn default() -> Hrgcmar {
            Hrgcmar(0)
        }
    }
    impl core::fmt::Debug for Hrgcmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrgcmar")
                .field("hrgcma", &self.hrgcma())
                .finish()
        }
    }
    #[doc = "desc HRGCMBR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrgcmbr(pub u32);
    impl Hrgcmbr {
        #[doc = "desc HRGCMB."]
        #[must_use]
        #[inline(always)]
        pub const fn hrgcmb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRGCMB."]
        #[inline(always)]
        pub const fn set_hrgcmb(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrgcmbr {
        #[inline(always)]
        fn default() -> Hrgcmbr {
            Hrgcmbr(0)
        }
    }
    impl core::fmt::Debug for Hrgcmbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrgcmbr")
                .field("hrgcmb", &self.hrgcmb())
                .finish()
        }
    }
    #[doc = "desc HRGCMCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrgcmcr(pub u32);
    impl Hrgcmcr {
        #[doc = "desc HRGCMC."]
        #[must_use]
        #[inline(always)]
        pub const fn hrgcmc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRGCMC."]
        #[inline(always)]
        pub const fn set_hrgcmc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrgcmcr {
        #[inline(always)]
        fn default() -> Hrgcmcr {
            Hrgcmcr(0)
        }
    }
    impl core::fmt::Debug for Hrgcmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrgcmcr")
                .field("hrgcmc", &self.hrgcmc())
                .finish()
        }
    }
    #[doc = "desc HRGCMDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrgcmdr(pub u32);
    impl Hrgcmdr {
        #[doc = "desc HRGCMD."]
        #[must_use]
        #[inline(always)]
        pub const fn hrgcmd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRGCMD."]
        #[inline(always)]
        pub const fn set_hrgcmd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrgcmdr {
        #[inline(always)]
        fn default() -> Hrgcmdr {
            Hrgcmdr(0)
        }
    }
    impl core::fmt::Debug for Hrgcmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrgcmdr")
                .field("hrgcmd", &self.hrgcmd())
                .finish()
        }
    }
    #[doc = "desc HRGCMER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrgcmer(pub u32);
    impl Hrgcmer {
        #[doc = "desc HRGCME."]
        #[must_use]
        #[inline(always)]
        pub const fn hrgcme(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRGCME."]
        #[inline(always)]
        pub const fn set_hrgcme(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrgcmer {
        #[inline(always)]
        fn default() -> Hrgcmer {
            Hrgcmer(0)
        }
    }
    impl core::fmt::Debug for Hrgcmer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrgcmer")
                .field("hrgcme", &self.hrgcme())
                .finish()
        }
    }
    #[doc = "desc HRGCMFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrgcmfr(pub u32);
    impl Hrgcmfr {
        #[doc = "desc HRGCMF."]
        #[must_use]
        #[inline(always)]
        pub const fn hrgcmf(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRGCMF."]
        #[inline(always)]
        pub const fn set_hrgcmf(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrgcmfr {
        #[inline(always)]
        fn default() -> Hrgcmfr {
            Hrgcmfr(0)
        }
    }
    impl core::fmt::Debug for Hrgcmfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrgcmfr")
                .field("hrgcmf", &self.hrgcmf())
                .finish()
        }
    }
    #[doc = "desc HRGCMGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrgcmgr(pub u32);
    impl Hrgcmgr {
        #[doc = "desc HRGCMG."]
        #[must_use]
        #[inline(always)]
        pub const fn hrgcmg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRGCMG."]
        #[inline(always)]
        pub const fn set_hrgcmg(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrgcmgr {
        #[inline(always)]
        fn default() -> Hrgcmgr {
            Hrgcmgr(0)
        }
    }
    impl core::fmt::Debug for Hrgcmgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrgcmgr")
                .field("hrgcmg", &self.hrgcmg())
                .finish()
        }
    }
    #[doc = "desc HRGCMHR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrgcmhr(pub u32);
    impl Hrgcmhr {
        #[doc = "desc HRGCMH."]
        #[must_use]
        #[inline(always)]
        pub const fn hrgcmh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRGCMH."]
        #[inline(always)]
        pub const fn set_hrgcmh(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrgcmhr {
        #[inline(always)]
        fn default() -> Hrgcmhr {
            Hrgcmhr(0)
        }
    }
    impl core::fmt::Debug for Hrgcmhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrgcmhr")
                .field("hrgcmh", &self.hrgcmh())
                .finish()
        }
    }
    #[doc = "desc HRPERAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrperar(pub u32);
    impl Hrperar {
        #[doc = "desc HRPERA."]
        #[must_use]
        #[inline(always)]
        pub const fn hrpera(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRPERA."]
        #[inline(always)]
        pub const fn set_hrpera(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrperar {
        #[inline(always)]
        fn default() -> Hrperar {
            Hrperar(0)
        }
    }
    impl core::fmt::Debug for Hrperar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrperar")
                .field("hrpera", &self.hrpera())
                .finish()
        }
    }
    #[doc = "desc HRPERBR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrperbr(pub u32);
    impl Hrperbr {
        #[doc = "desc HRPERB."]
        #[must_use]
        #[inline(always)]
        pub const fn hrperb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc HRPERB."]
        #[inline(always)]
        pub const fn set_hrperb(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Hrperbr {
        #[inline(always)]
        fn default() -> Hrperbr {
            Hrperbr(0)
        }
    }
    impl core::fmt::Debug for Hrperbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrperbr")
                .field("hrperb", &self.hrperb())
                .finish()
        }
    }
    #[doc = "desc HSTAR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hstar1(pub u32);
    impl Hstar1 {
        #[doc = "desc STAS."]
        #[must_use]
        #[inline(always)]
        pub const fn stas(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc STAS."]
        #[inline(always)]
        pub const fn set_stas(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HSTA8."]
        #[must_use]
        #[inline(always)]
        pub const fn hsta8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTA8."]
        #[inline(always)]
        pub const fn set_hsta8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc HSTA9."]
        #[must_use]
        #[inline(always)]
        pub const fn hsta9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTA9."]
        #[inline(always)]
        pub const fn set_hsta9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc HSTA10."]
        #[must_use]
        #[inline(always)]
        pub const fn hsta10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTA10."]
        #[inline(always)]
        pub const fn set_hsta10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc HSTA11."]
        #[must_use]
        #[inline(always)]
        pub const fn hsta11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTA11."]
        #[inline(always)]
        pub const fn set_hsta11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc HSTA16."]
        #[must_use]
        #[inline(always)]
        pub const fn hsta16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTA16."]
        #[inline(always)]
        pub const fn set_hsta16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc HSTA17."]
        #[must_use]
        #[inline(always)]
        pub const fn hsta17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTA17."]
        #[inline(always)]
        pub const fn set_hsta17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc HSTA18."]
        #[must_use]
        #[inline(always)]
        pub const fn hsta18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTA18."]
        #[inline(always)]
        pub const fn set_hsta18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc HSTA19."]
        #[must_use]
        #[inline(always)]
        pub const fn hsta19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTA19."]
        #[inline(always)]
        pub const fn set_hsta19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc HSTA20."]
        #[must_use]
        #[inline(always)]
        pub const fn hsta20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTA20."]
        #[inline(always)]
        pub const fn set_hsta20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc HSTA21."]
        #[must_use]
        #[inline(always)]
        pub const fn hsta21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTA21."]
        #[inline(always)]
        pub const fn set_hsta21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc HSTA22."]
        #[must_use]
        #[inline(always)]
        pub const fn hsta22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTA22."]
        #[inline(always)]
        pub const fn set_hsta22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc HSTA23."]
        #[must_use]
        #[inline(always)]
        pub const fn hsta23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTA23."]
        #[inline(always)]
        pub const fn set_hsta23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Hstar1 {
        #[inline(always)]
        fn default() -> Hstar1 {
            Hstar1(0)
        }
    }
    impl core::fmt::Debug for Hstar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hstar1")
                .field("stas", &self.stas())
                .field("hsta8", &self.hsta8())
                .field("hsta9", &self.hsta9())
                .field("hsta10", &self.hsta10())
                .field("hsta11", &self.hsta11())
                .field("hsta16", &self.hsta16())
                .field("hsta17", &self.hsta17())
                .field("hsta18", &self.hsta18())
                .field("hsta19", &self.hsta19())
                .field("hsta20", &self.hsta20())
                .field("hsta21", &self.hsta21())
                .field("hsta22", &self.hsta22())
                .field("hsta23", &self.hsta23())
                .finish()
        }
    }
    #[doc = "desc HSTAR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hstar2(pub u32);
    impl Hstar2 {
        #[doc = "desc HSTAEXEV1."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaexev1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAEXEV1."]
        #[inline(always)]
        pub const fn set_hstaexev1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc HSTAEXEV2."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaexev2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAEXEV2."]
        #[inline(always)]
        pub const fn set_hstaexev2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HSTAEXEV3."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaexev3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAEXEV3."]
        #[inline(always)]
        pub const fn set_hstaexev3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc HSTAEXEV4."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaexev4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAEXEV4."]
        #[inline(always)]
        pub const fn set_hstaexev4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc HSTAEXEV5."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaexev5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAEXEV5."]
        #[inline(always)]
        pub const fn set_hstaexev5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc HSTAEXEV6."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaexev6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAEXEV6."]
        #[inline(always)]
        pub const fn set_hstaexev6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc HSTAEXEV7."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaexev7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAEXEV7."]
        #[inline(always)]
        pub const fn set_hstaexev7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc HSTAEXEV8."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaexev8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAEXEV8."]
        #[inline(always)]
        pub const fn set_hstaexev8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HSTAEXEV9."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaexev9(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAEXEV9."]
        #[inline(always)]
        pub const fn set_hstaexev9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc HSTAEXEV10."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaexev10(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAEXEV10."]
        #[inline(always)]
        pub const fn set_hstaexev10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc HSTAGCMA1."]
        #[must_use]
        #[inline(always)]
        pub const fn hstagcma1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAGCMA1."]
        #[inline(always)]
        pub const fn set_hstagcma1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc HSTAGCMB1."]
        #[must_use]
        #[inline(always)]
        pub const fn hstagcmb1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAGCMB1."]
        #[inline(always)]
        pub const fn set_hstagcmb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc HSTAGCMA2."]
        #[must_use]
        #[inline(always)]
        pub const fn hstagcma2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAGCMA2."]
        #[inline(always)]
        pub const fn set_hstagcma2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc HSTAGCMB2."]
        #[must_use]
        #[inline(always)]
        pub const fn hstagcmb2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAGCMB2."]
        #[inline(always)]
        pub const fn set_hstagcmb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc HSTAGCMA3."]
        #[must_use]
        #[inline(always)]
        pub const fn hstagcma3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAGCMA3."]
        #[inline(always)]
        pub const fn set_hstagcma3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc HSTAGCMB3."]
        #[must_use]
        #[inline(always)]
        pub const fn hstagcmb3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAGCMB3."]
        #[inline(always)]
        pub const fn set_hstagcmb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc HSTAGCMA4."]
        #[must_use]
        #[inline(always)]
        pub const fn hstagcma4(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAGCMA4."]
        #[inline(always)]
        pub const fn set_hstagcma4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc HSTAGCMB4."]
        #[must_use]
        #[inline(always)]
        pub const fn hstagcmb4(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAGCMB4."]
        #[inline(always)]
        pub const fn set_hstagcmb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc HSTAGCMA5."]
        #[must_use]
        #[inline(always)]
        pub const fn hstagcma5(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAGCMA5."]
        #[inline(always)]
        pub const fn set_hstagcma5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc HSTAGCMB5."]
        #[must_use]
        #[inline(always)]
        pub const fn hstagcmb5(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAGCMB5."]
        #[inline(always)]
        pub const fn set_hstagcmb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc HSTAGCMA6."]
        #[must_use]
        #[inline(always)]
        pub const fn hstagcma6(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAGCMA6."]
        #[inline(always)]
        pub const fn set_hstagcma6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc HSTAGCMB6."]
        #[must_use]
        #[inline(always)]
        pub const fn hstagcmb6(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAGCMB6."]
        #[inline(always)]
        pub const fn set_hstagcmb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc HSTASCMA1."]
        #[must_use]
        #[inline(always)]
        pub const fn hstascma1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTASCMA1."]
        #[inline(always)]
        pub const fn set_hstascma1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc HSTASCMB2."]
        #[must_use]
        #[inline(always)]
        pub const fn hstascmb2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTASCMB2."]
        #[inline(always)]
        pub const fn set_hstascmb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc HSTASCMB3."]
        #[must_use]
        #[inline(always)]
        pub const fn hstascmb3(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTASCMB3."]
        #[inline(always)]
        pub const fn set_hstascmb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc HSTASCMB4."]
        #[must_use]
        #[inline(always)]
        pub const fn hstascmb4(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTASCMB4."]
        #[inline(always)]
        pub const fn set_hstascmb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc HSTASCMB5."]
        #[must_use]
        #[inline(always)]
        pub const fn hstascmb5(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTASCMB5."]
        #[inline(always)]
        pub const fn set_hstascmb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc HSTASCMB6."]
        #[must_use]
        #[inline(always)]
        pub const fn hstascmb6(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTASCMB6."]
        #[inline(always)]
        pub const fn set_hstascmb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Hstar2 {
        #[inline(always)]
        fn default() -> Hstar2 {
            Hstar2(0)
        }
    }
    impl core::fmt::Debug for Hstar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hstar2")
                .field("hstaexev1", &self.hstaexev1())
                .field("hstaexev2", &self.hstaexev2())
                .field("hstaexev3", &self.hstaexev3())
                .field("hstaexev4", &self.hstaexev4())
                .field("hstaexev5", &self.hstaexev5())
                .field("hstaexev6", &self.hstaexev6())
                .field("hstaexev7", &self.hstaexev7())
                .field("hstaexev8", &self.hstaexev8())
                .field("hstaexev9", &self.hstaexev9())
                .field("hstaexev10", &self.hstaexev10())
                .field("hstagcma1", &self.hstagcma1())
                .field("hstagcmb1", &self.hstagcmb1())
                .field("hstagcma2", &self.hstagcma2())
                .field("hstagcmb2", &self.hstagcmb2())
                .field("hstagcma3", &self.hstagcma3())
                .field("hstagcmb3", &self.hstagcmb3())
                .field("hstagcma4", &self.hstagcma4())
                .field("hstagcmb4", &self.hstagcmb4())
                .field("hstagcma5", &self.hstagcma5())
                .field("hstagcmb5", &self.hstagcmb5())
                .field("hstagcma6", &self.hstagcma6())
                .field("hstagcmb6", &self.hstagcmb6())
                .field("hstascma1", &self.hstascma1())
                .field("hstascmb2", &self.hstascmb2())
                .field("hstascmb3", &self.hstascmb3())
                .field("hstascmb4", &self.hstascmb4())
                .field("hstascmb5", &self.hstascmb5())
                .field("hstascmb6", &self.hstascmb6())
                .finish()
        }
    }
    #[doc = "desc ICONR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iconr(pub u32);
    impl Iconr {
        #[doc = "desc INTENA."]
        #[must_use]
        #[inline(always)]
        pub const fn intena(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENA."]
        #[inline(always)]
        pub const fn set_intena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc INTENB."]
        #[must_use]
        #[inline(always)]
        pub const fn intenb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENB."]
        #[inline(always)]
        pub const fn set_intenb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc INTENC."]
        #[must_use]
        #[inline(always)]
        pub const fn intenc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENC."]
        #[inline(always)]
        pub const fn set_intenc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc INTEND."]
        #[must_use]
        #[inline(always)]
        pub const fn intend(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTEND."]
        #[inline(always)]
        pub const fn set_intend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc INTENE."]
        #[must_use]
        #[inline(always)]
        pub const fn intene(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENE."]
        #[inline(always)]
        pub const fn set_intene(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INTENF."]
        #[must_use]
        #[inline(always)]
        pub const fn intenf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENF."]
        #[inline(always)]
        pub const fn set_intenf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc INTENOVF."]
        #[must_use]
        #[inline(always)]
        pub const fn intenovf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENOVF."]
        #[inline(always)]
        pub const fn set_intenovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc INTENUDF."]
        #[must_use]
        #[inline(always)]
        pub const fn intenudf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENUDF."]
        #[inline(always)]
        pub const fn set_intenudf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc INTENCAPA."]
        #[must_use]
        #[inline(always)]
        pub const fn intencapa(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENCAPA."]
        #[inline(always)]
        pub const fn set_intencapa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc INTENCAPB."]
        #[must_use]
        #[inline(always)]
        pub const fn intencapb(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENCAPB."]
        #[inline(always)]
        pub const fn set_intencapb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc INTENSAU."]
        #[must_use]
        #[inline(always)]
        pub const fn intensau(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENSAU."]
        #[inline(always)]
        pub const fn set_intensau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc INTENSAD."]
        #[must_use]
        #[inline(always)]
        pub const fn intensad(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENSAD."]
        #[inline(always)]
        pub const fn set_intensad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc INTENSBU."]
        #[must_use]
        #[inline(always)]
        pub const fn intensbu(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENSBU."]
        #[inline(always)]
        pub const fn set_intensbu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc INTENSBD."]
        #[must_use]
        #[inline(always)]
        pub const fn intensbd(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENSBD."]
        #[inline(always)]
        pub const fn set_intensbd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Iconr {
        #[inline(always)]
        fn default() -> Iconr {
            Iconr(0)
        }
    }
    impl core::fmt::Debug for Iconr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iconr")
                .field("intena", &self.intena())
                .field("intenb", &self.intenb())
                .field("intenc", &self.intenc())
                .field("intend", &self.intend())
                .field("intene", &self.intene())
                .field("intenf", &self.intenf())
                .field("intenovf", &self.intenovf())
                .field("intenudf", &self.intenudf())
                .field("intencapa", &self.intencapa())
                .field("intencapb", &self.intencapb())
                .field("intensau", &self.intensau())
                .field("intensad", &self.intensad())
                .field("intensbu", &self.intensbu())
                .field("intensbd", &self.intensbd())
                .finish()
        }
    }
    #[doc = "desc IDLECR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idlecr(pub u32);
    impl Idlecr {
        #[doc = "desc IDLEBMA."]
        #[must_use]
        #[inline(always)]
        pub const fn idlebma(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc IDLEBMA."]
        #[inline(always)]
        pub const fn set_idlebma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc IDLESA."]
        #[must_use]
        #[inline(always)]
        pub const fn idlesa(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc IDLESA."]
        #[inline(always)]
        pub const fn set_idlesa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc DIDLA."]
        #[must_use]
        #[inline(always)]
        pub const fn didla(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIDLA."]
        #[inline(always)]
        pub const fn set_didla(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc DLYCHA."]
        #[must_use]
        #[inline(always)]
        pub const fn dlycha(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc DLYCHA."]
        #[inline(always)]
        pub const fn set_dlycha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc DLYPRTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyprten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc DLYPRTEN."]
        #[inline(always)]
        pub const fn set_dlyprten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc DLYEVSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyevsel(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "desc DLYEVSEL."]
        #[inline(always)]
        pub const fn set_dlyevsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "desc DLYSTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn dlystrg(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc DLYSTRG."]
        #[inline(always)]
        pub const fn set_dlystrg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc IDLEBMB."]
        #[must_use]
        #[inline(always)]
        pub const fn idlebmb(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc IDLEBMB."]
        #[inline(always)]
        pub const fn set_idlebmb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc IDLESB."]
        #[must_use]
        #[inline(always)]
        pub const fn idlesb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc IDLESB."]
        #[inline(always)]
        pub const fn set_idlesb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc DIDLB."]
        #[must_use]
        #[inline(always)]
        pub const fn didlb(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIDLB."]
        #[inline(always)]
        pub const fn set_didlb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc DLYCHB."]
        #[must_use]
        #[inline(always)]
        pub const fn dlychb(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc DLYCHB."]
        #[inline(always)]
        pub const fn set_dlychb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc INTENDLYPRT."]
        #[must_use]
        #[inline(always)]
        pub const fn intendlyprt(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTENDLYPRT."]
        #[inline(always)]
        pub const fn set_intendlyprt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FOLLOW."]
        #[must_use]
        #[inline(always)]
        pub const fn follow(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FOLLOW."]
        #[inline(always)]
        pub const fn set_follow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Idlecr {
        #[inline(always)]
        fn default() -> Idlecr {
            Idlecr(0)
        }
    }
    impl core::fmt::Debug for Idlecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idlecr")
                .field("idlebma", &self.idlebma())
                .field("idlesa", &self.idlesa())
                .field("didla", &self.didla())
                .field("dlycha", &self.dlycha())
                .field("dlyprten", &self.dlyprten())
                .field("dlyevsel", &self.dlyevsel())
                .field("dlystrg", &self.dlystrg())
                .field("idlebmb", &self.idlebmb())
                .field("idlesb", &self.idlesb())
                .field("didlb", &self.didlb())
                .field("dlychb", &self.dlychb())
                .field("intendlyprt", &self.intendlyprt())
                .field("follow", &self.follow())
                .finish()
        }
    }
    #[doc = "desc PCNAR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcnar1(pub u32);
    impl Pcnar1 {
        #[doc = "desc STACA."]
        #[must_use]
        #[inline(always)]
        pub const fn staca(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc STACA."]
        #[inline(always)]
        pub const fn set_staca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc STPCA."]
        #[must_use]
        #[inline(always)]
        pub const fn stpca(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc STPCA."]
        #[inline(always)]
        pub const fn set_stpca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc OVFCA."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfca(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc OVFCA."]
        #[inline(always)]
        pub const fn set_ovfca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc UDFCA."]
        #[must_use]
        #[inline(always)]
        pub const fn udfca(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc UDFCA."]
        #[inline(always)]
        pub const fn set_udfca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc CMAUCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmauca(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMAUCA."]
        #[inline(always)]
        pub const fn set_cmauca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc CMADCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmadca(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMADCA."]
        #[inline(always)]
        pub const fn set_cmadca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc CMBUCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmbuca(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMBUCA."]
        #[inline(always)]
        pub const fn set_cmbuca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc CMBDCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmbdca(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMBDCA."]
        #[inline(always)]
        pub const fn set_cmbdca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc FORCA."]
        #[must_use]
        #[inline(always)]
        pub const fn forca(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc FORCA."]
        #[inline(always)]
        pub const fn set_forca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc EMBCA."]
        #[must_use]
        #[inline(always)]
        pub const fn embca(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc EMBCA."]
        #[inline(always)]
        pub const fn set_embca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc EMBRA."]
        #[must_use]
        #[inline(always)]
        pub const fn embra(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc EMBRA."]
        #[inline(always)]
        pub const fn set_embra(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc EMBSA."]
        #[must_use]
        #[inline(always)]
        pub const fn embsa(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc EMBSA."]
        #[inline(always)]
        pub const fn set_embsa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "desc OUTENA."]
        #[must_use]
        #[inline(always)]
        pub const fn outena(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc OUTENA."]
        #[inline(always)]
        pub const fn set_outena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Pcnar1 {
        #[inline(always)]
        fn default() -> Pcnar1 {
            Pcnar1(0)
        }
    }
    impl core::fmt::Debug for Pcnar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcnar1")
                .field("staca", &self.staca())
                .field("stpca", &self.stpca())
                .field("ovfca", &self.ovfca())
                .field("udfca", &self.udfca())
                .field("cmauca", &self.cmauca())
                .field("cmadca", &self.cmadca())
                .field("cmbuca", &self.cmbuca())
                .field("cmbdca", &self.cmbdca())
                .field("forca", &self.forca())
                .field("embca", &self.embca())
                .field("embra", &self.embra())
                .field("embsa", &self.embsa())
                .field("outena", &self.outena())
                .finish()
        }
    }
    #[doc = "desc PCNAR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcnar2(pub u32);
    impl Pcnar2 {
        #[doc = "desc EXEV1UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev1uca(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV1UCA."]
        #[inline(always)]
        pub const fn set_exev1uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EXEV2UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev2uca(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV2UCA."]
        #[inline(always)]
        pub const fn set_exev2uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc EXEV3UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev3uca(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV3UCA."]
        #[inline(always)]
        pub const fn set_exev3uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EXEV4UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev4uca(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV4UCA."]
        #[inline(always)]
        pub const fn set_exev4uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc EXEV5UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev5uca(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV5UCA."]
        #[inline(always)]
        pub const fn set_exev5uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc EXEV6UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev6uca(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV6UCA."]
        #[inline(always)]
        pub const fn set_exev6uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc EXEV7UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev7uca(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV7UCA."]
        #[inline(always)]
        pub const fn set_exev7uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc EXEV8UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev8uca(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV8UCA."]
        #[inline(always)]
        pub const fn set_exev8uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc EXEV9UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev9uca(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV9UCA."]
        #[inline(always)]
        pub const fn set_exev9uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc EXEV10UCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev10uca(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV10UCA."]
        #[inline(always)]
        pub const fn set_exev10uca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc CMEUCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmeuca(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMEUCA."]
        #[inline(always)]
        pub const fn set_cmeuca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc CMFUCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmfuca(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMFUCA."]
        #[inline(always)]
        pub const fn set_cmfuca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc SCMAUCA."]
        #[must_use]
        #[inline(always)]
        pub const fn scmauca(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMAUCA."]
        #[inline(always)]
        pub const fn set_scmauca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "desc SCMBUCA."]
        #[must_use]
        #[inline(always)]
        pub const fn scmbuca(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMBUCA."]
        #[inline(always)]
        pub const fn set_scmbuca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
    }
    impl Default for Pcnar2 {
        #[inline(always)]
        fn default() -> Pcnar2 {
            Pcnar2(0)
        }
    }
    impl core::fmt::Debug for Pcnar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcnar2")
                .field("exev1uca", &self.exev1uca())
                .field("exev2uca", &self.exev2uca())
                .field("exev3uca", &self.exev3uca())
                .field("exev4uca", &self.exev4uca())
                .field("exev5uca", &self.exev5uca())
                .field("exev6uca", &self.exev6uca())
                .field("exev7uca", &self.exev7uca())
                .field("exev8uca", &self.exev8uca())
                .field("exev9uca", &self.exev9uca())
                .field("exev10uca", &self.exev10uca())
                .field("cmeuca", &self.cmeuca())
                .field("cmfuca", &self.cmfuca())
                .field("scmauca", &self.scmauca())
                .field("scmbuca", &self.scmbuca())
                .finish()
        }
    }
    #[doc = "desc PCNAR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcnar3(pub u32);
    impl Pcnar3 {
        #[doc = "desc EXEV1DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev1dca(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV1DCA."]
        #[inline(always)]
        pub const fn set_exev1dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EXEV2DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev2dca(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV2DCA."]
        #[inline(always)]
        pub const fn set_exev2dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc EXEV3DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev3dca(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV3DCA."]
        #[inline(always)]
        pub const fn set_exev3dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EXEV4DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev4dca(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV4DCA."]
        #[inline(always)]
        pub const fn set_exev4dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc EXEV5DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev5dca(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV5DCA."]
        #[inline(always)]
        pub const fn set_exev5dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc EXEV6DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev6dca(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV6DCA."]
        #[inline(always)]
        pub const fn set_exev6dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc EXEV7DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev7dca(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV7DCA."]
        #[inline(always)]
        pub const fn set_exev7dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc EXEV8DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev8dca(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV8DCA."]
        #[inline(always)]
        pub const fn set_exev8dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc EXEV9DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev9dca(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV9DCA."]
        #[inline(always)]
        pub const fn set_exev9dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc EXEV10DCA."]
        #[must_use]
        #[inline(always)]
        pub const fn exev10dca(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV10DCA."]
        #[inline(always)]
        pub const fn set_exev10dca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc CMEDCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmedca(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMEDCA."]
        #[inline(always)]
        pub const fn set_cmedca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc CMFDCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmfdca(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMFDCA."]
        #[inline(always)]
        pub const fn set_cmfdca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc SCMADCA."]
        #[must_use]
        #[inline(always)]
        pub const fn scmadca(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMADCA."]
        #[inline(always)]
        pub const fn set_scmadca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "desc SCMBDCA."]
        #[must_use]
        #[inline(always)]
        pub const fn scmbdca(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMBDCA."]
        #[inline(always)]
        pub const fn set_scmbdca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
    }
    impl Default for Pcnar3 {
        #[inline(always)]
        fn default() -> Pcnar3 {
            Pcnar3(0)
        }
    }
    impl core::fmt::Debug for Pcnar3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcnar3")
                .field("exev1dca", &self.exev1dca())
                .field("exev2dca", &self.exev2dca())
                .field("exev3dca", &self.exev3dca())
                .field("exev4dca", &self.exev4dca())
                .field("exev5dca", &self.exev5dca())
                .field("exev6dca", &self.exev6dca())
                .field("exev7dca", &self.exev7dca())
                .field("exev8dca", &self.exev8dca())
                .field("exev9dca", &self.exev9dca())
                .field("exev10dca", &self.exev10dca())
                .field("cmedca", &self.cmedca())
                .field("cmfdca", &self.cmfdca())
                .field("scmadca", &self.scmadca())
                .field("scmbdca", &self.scmbdca())
                .finish()
        }
    }
    #[doc = "desc PCNBR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcnbr1(pub u32);
    impl Pcnbr1 {
        #[doc = "desc STACB."]
        #[must_use]
        #[inline(always)]
        pub const fn stacb(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc STACB."]
        #[inline(always)]
        pub const fn set_stacb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc STPCB."]
        #[must_use]
        #[inline(always)]
        pub const fn stpcb(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc STPCB."]
        #[inline(always)]
        pub const fn set_stpcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc OVFCB."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfcb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc OVFCB."]
        #[inline(always)]
        pub const fn set_ovfcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc UDFCB."]
        #[must_use]
        #[inline(always)]
        pub const fn udfcb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc UDFCB."]
        #[inline(always)]
        pub const fn set_udfcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc CMAUCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmaucb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMAUCB."]
        #[inline(always)]
        pub const fn set_cmaucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc CMADCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmadcb(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMADCB."]
        #[inline(always)]
        pub const fn set_cmadcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc CMBUCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmbucb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMBUCB."]
        #[inline(always)]
        pub const fn set_cmbucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc CMBDCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmbdcb(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMBDCB."]
        #[inline(always)]
        pub const fn set_cmbdcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc FORCB."]
        #[must_use]
        #[inline(always)]
        pub const fn forcb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc FORCB."]
        #[inline(always)]
        pub const fn set_forcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc EMBCB."]
        #[must_use]
        #[inline(always)]
        pub const fn embcb(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc EMBCB."]
        #[inline(always)]
        pub const fn set_embcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc EMBRB."]
        #[must_use]
        #[inline(always)]
        pub const fn embrb(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc EMBRB."]
        #[inline(always)]
        pub const fn set_embrb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc EMBSB."]
        #[must_use]
        #[inline(always)]
        pub const fn embsb(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc EMBSB."]
        #[inline(always)]
        pub const fn set_embsb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "desc OUTENB."]
        #[must_use]
        #[inline(always)]
        pub const fn outenb(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc OUTENB."]
        #[inline(always)]
        pub const fn set_outenb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Pcnbr1 {
        #[inline(always)]
        fn default() -> Pcnbr1 {
            Pcnbr1(0)
        }
    }
    impl core::fmt::Debug for Pcnbr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcnbr1")
                .field("stacb", &self.stacb())
                .field("stpcb", &self.stpcb())
                .field("ovfcb", &self.ovfcb())
                .field("udfcb", &self.udfcb())
                .field("cmaucb", &self.cmaucb())
                .field("cmadcb", &self.cmadcb())
                .field("cmbucb", &self.cmbucb())
                .field("cmbdcb", &self.cmbdcb())
                .field("forcb", &self.forcb())
                .field("embcb", &self.embcb())
                .field("embrb", &self.embrb())
                .field("embsb", &self.embsb())
                .field("outenb", &self.outenb())
                .finish()
        }
    }
    #[doc = "desc PCNBR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcnbr2(pub u32);
    impl Pcnbr2 {
        #[doc = "desc EXEV1UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev1ucb(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV1UCB."]
        #[inline(always)]
        pub const fn set_exev1ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EXEV2UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev2ucb(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV2UCB."]
        #[inline(always)]
        pub const fn set_exev2ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc EXEV3UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev3ucb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV3UCB."]
        #[inline(always)]
        pub const fn set_exev3ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EXEV4UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev4ucb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV4UCB."]
        #[inline(always)]
        pub const fn set_exev4ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc EXEV5UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev5ucb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV5UCB."]
        #[inline(always)]
        pub const fn set_exev5ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc EXEV6UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev6ucb(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV6UCB."]
        #[inline(always)]
        pub const fn set_exev6ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc EXEV7UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev7ucb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV7UCB."]
        #[inline(always)]
        pub const fn set_exev7ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc EXEV8UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev8ucb(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV8UCB."]
        #[inline(always)]
        pub const fn set_exev8ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc EXEV9UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev9ucb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV9UCB."]
        #[inline(always)]
        pub const fn set_exev9ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc EXEV10UCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev10ucb(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV10UCB."]
        #[inline(always)]
        pub const fn set_exev10ucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc CMEUCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmeucb(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMEUCB."]
        #[inline(always)]
        pub const fn set_cmeucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc CMFUCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmfucb(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMFUCB."]
        #[inline(always)]
        pub const fn set_cmfucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc SCMAUCB."]
        #[must_use]
        #[inline(always)]
        pub const fn scmaucb(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMAUCB."]
        #[inline(always)]
        pub const fn set_scmaucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "desc SCMBUCB."]
        #[must_use]
        #[inline(always)]
        pub const fn scmbucb(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMBUCB."]
        #[inline(always)]
        pub const fn set_scmbucb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
    }
    impl Default for Pcnbr2 {
        #[inline(always)]
        fn default() -> Pcnbr2 {
            Pcnbr2(0)
        }
    }
    impl core::fmt::Debug for Pcnbr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcnbr2")
                .field("exev1ucb", &self.exev1ucb())
                .field("exev2ucb", &self.exev2ucb())
                .field("exev3ucb", &self.exev3ucb())
                .field("exev4ucb", &self.exev4ucb())
                .field("exev5ucb", &self.exev5ucb())
                .field("exev6ucb", &self.exev6ucb())
                .field("exev7ucb", &self.exev7ucb())
                .field("exev8ucb", &self.exev8ucb())
                .field("exev9ucb", &self.exev9ucb())
                .field("exev10ucb", &self.exev10ucb())
                .field("cmeucb", &self.cmeucb())
                .field("cmfucb", &self.cmfucb())
                .field("scmaucb", &self.scmaucb())
                .field("scmbucb", &self.scmbucb())
                .finish()
        }
    }
    #[doc = "desc PCNBR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcnbr3(pub u32);
    impl Pcnbr3 {
        #[doc = "desc EXEV1DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev1dcb(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV1DCB."]
        #[inline(always)]
        pub const fn set_exev1dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EXEV2DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev2dcb(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV2DCB."]
        #[inline(always)]
        pub const fn set_exev2dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc EXEV3DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev3dcb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV3DCB."]
        #[inline(always)]
        pub const fn set_exev3dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EXEV4DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev4dcb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV4DCB."]
        #[inline(always)]
        pub const fn set_exev4dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc EXEV5DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev5dcb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV5DCB."]
        #[inline(always)]
        pub const fn set_exev5dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc EXEV6DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev6dcb(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV6DCB."]
        #[inline(always)]
        pub const fn set_exev6dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc EXEV7DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev7dcb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV7DCB."]
        #[inline(always)]
        pub const fn set_exev7dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc EXEV8DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev8dcb(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV8DCB."]
        #[inline(always)]
        pub const fn set_exev8dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc EXEV9DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev9dcb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV9DCB."]
        #[inline(always)]
        pub const fn set_exev9dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc EXEV10DCB."]
        #[must_use]
        #[inline(always)]
        pub const fn exev10dcb(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EXEV10DCB."]
        #[inline(always)]
        pub const fn set_exev10dcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc CMEDCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmedcb(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMEDCB."]
        #[inline(always)]
        pub const fn set_cmedcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc CMFDCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmfdcb(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMFDCB."]
        #[inline(always)]
        pub const fn set_cmfdcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc SCMADCB."]
        #[must_use]
        #[inline(always)]
        pub const fn scmadcb(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMADCB."]
        #[inline(always)]
        pub const fn set_scmadcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "desc SCMBDCB."]
        #[must_use]
        #[inline(always)]
        pub const fn scmbdcb(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc SCMBDCB."]
        #[inline(always)]
        pub const fn set_scmbdcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
    }
    impl Default for Pcnbr3 {
        #[inline(always)]
        fn default() -> Pcnbr3 {
            Pcnbr3(0)
        }
    }
    impl core::fmt::Debug for Pcnbr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcnbr3")
                .field("exev1dcb", &self.exev1dcb())
                .field("exev2dcb", &self.exev2dcb())
                .field("exev3dcb", &self.exev3dcb())
                .field("exev4dcb", &self.exev4dcb())
                .field("exev5dcb", &self.exev5dcb())
                .field("exev6dcb", &self.exev6dcb())
                .field("exev7dcb", &self.exev7dcb())
                .field("exev8dcb", &self.exev8dcb())
                .field("exev9dcb", &self.exev9dcb())
                .field("exev10dcb", &self.exev10dcb())
                .field("cmedcb", &self.cmedcb())
                .field("cmfdcb", &self.cmfdcb())
                .field("scmadcb", &self.scmadcb())
                .field("scmbdcb", &self.scmbdcb())
                .finish()
        }
    }
    #[doc = "desc PHSCMP1A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phscmp1a(pub u32);
    impl Phscmp1a {
        #[doc = "desc PHSCMP1A."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp1a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "desc PHSCMP1A."]
        #[inline(always)]
        pub const fn set_phscmp1a(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for Phscmp1a {
        #[inline(always)]
        fn default() -> Phscmp1a {
            Phscmp1a(0)
        }
    }
    impl core::fmt::Debug for Phscmp1a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Phscmp1a")
                .field("phscmp1a", &self.phscmp1a())
                .finish()
        }
    }
    #[doc = "desc PHSCMP1B."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phscmp1b(pub u32);
    impl Phscmp1b {
        #[doc = "desc PHSCMP1B."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp1b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "desc PHSCMP1B."]
        #[inline(always)]
        pub const fn set_phscmp1b(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for Phscmp1b {
        #[inline(always)]
        fn default() -> Phscmp1b {
            Phscmp1b(0)
        }
    }
    impl core::fmt::Debug for Phscmp1b {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Phscmp1b")
                .field("phscmp1b", &self.phscmp1b())
                .finish()
        }
    }
    #[doc = "desc PHSCMP2A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phscmp2a(pub u32);
    impl Phscmp2a {
        #[doc = "desc PHSCMP2A."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp2a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "desc PHSCMP2A."]
        #[inline(always)]
        pub const fn set_phscmp2a(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for Phscmp2a {
        #[inline(always)]
        fn default() -> Phscmp2a {
            Phscmp2a(0)
        }
    }
    impl core::fmt::Debug for Phscmp2a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Phscmp2a")
                .field("phscmp2a", &self.phscmp2a())
                .finish()
        }
    }
    #[doc = "desc PHSCMP2B."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phscmp2b(pub u32);
    impl Phscmp2b {
        #[doc = "desc PHSCMP2B."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp2b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "desc PHSCMP2B."]
        #[inline(always)]
        pub const fn set_phscmp2b(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for Phscmp2b {
        #[inline(always)]
        fn default() -> Phscmp2b {
            Phscmp2b(0)
        }
    }
    impl core::fmt::Debug for Phscmp2b {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Phscmp2b")
                .field("phscmp2b", &self.phscmp2b())
                .finish()
        }
    }
    #[doc = "desc PHSCMP3A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phscmp3a(pub u32);
    impl Phscmp3a {
        #[doc = "desc PHSCMP3A."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp3a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "desc PHSCMP3A."]
        #[inline(always)]
        pub const fn set_phscmp3a(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for Phscmp3a {
        #[inline(always)]
        fn default() -> Phscmp3a {
            Phscmp3a(0)
        }
    }
    impl core::fmt::Debug for Phscmp3a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Phscmp3a")
                .field("phscmp3a", &self.phscmp3a())
                .finish()
        }
    }
    #[doc = "desc PHSCMP3B."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phscmp3b(pub u32);
    impl Phscmp3b {
        #[doc = "desc PHSCMP3B."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp3b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "desc PHSCMP3B."]
        #[inline(always)]
        pub const fn set_phscmp3b(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for Phscmp3b {
        #[inline(always)]
        fn default() -> Phscmp3b {
            Phscmp3b(0)
        }
    }
    impl core::fmt::Debug for Phscmp3b {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Phscmp3b")
                .field("phscmp3b", &self.phscmp3b())
                .finish()
        }
    }
    #[doc = "desc PHSCMP4A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phscmp4a(pub u32);
    impl Phscmp4a {
        #[doc = "desc PHSCMP4A."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp4a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "desc PHSCMP4A."]
        #[inline(always)]
        pub const fn set_phscmp4a(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for Phscmp4a {
        #[inline(always)]
        fn default() -> Phscmp4a {
            Phscmp4a(0)
        }
    }
    impl core::fmt::Debug for Phscmp4a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Phscmp4a")
                .field("phscmp4a", &self.phscmp4a())
                .finish()
        }
    }
    #[doc = "desc PHSCMP4B."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phscmp4b(pub u32);
    impl Phscmp4b {
        #[doc = "desc PHSCMP4B."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp4b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "desc PHSCMP4B."]
        #[inline(always)]
        pub const fn set_phscmp4b(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for Phscmp4b {
        #[inline(always)]
        fn default() -> Phscmp4b {
            Phscmp4b(0)
        }
    }
    impl core::fmt::Debug for Phscmp4b {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Phscmp4b")
                .field("phscmp4b", &self.phscmp4b())
                .finish()
        }
    }
    #[doc = "desc PHSCMP5A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phscmp5a(pub u32);
    impl Phscmp5a {
        #[doc = "desc PHSCMP5A."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp5a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "desc PHSCMP5A."]
        #[inline(always)]
        pub const fn set_phscmp5a(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for Phscmp5a {
        #[inline(always)]
        fn default() -> Phscmp5a {
            Phscmp5a(0)
        }
    }
    impl core::fmt::Debug for Phscmp5a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Phscmp5a")
                .field("phscmp5a", &self.phscmp5a())
                .finish()
        }
    }
    #[doc = "desc PHSCMP5B."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phscmp5b(pub u32);
    impl Phscmp5b {
        #[doc = "desc PHSCMP5B."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp5b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "desc PHSCMP5B."]
        #[inline(always)]
        pub const fn set_phscmp5b(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for Phscmp5b {
        #[inline(always)]
        fn default() -> Phscmp5b {
            Phscmp5b(0)
        }
    }
    impl core::fmt::Debug for Phscmp5b {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Phscmp5b")
                .field("phscmp5b", &self.phscmp5b())
                .finish()
        }
    }
    #[doc = "desc PHSCTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phsctl(pub u32);
    impl Phsctl {
        #[doc = "desc BENPHS."]
        #[must_use]
        #[inline(always)]
        pub const fn benphs(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BENPHS."]
        #[inline(always)]
        pub const fn set_benphs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc BTRUPHS."]
        #[must_use]
        #[inline(always)]
        pub const fn btruphs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRUPHS."]
        #[inline(always)]
        pub const fn set_btruphs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BTRDPHS."]
        #[must_use]
        #[inline(always)]
        pub const fn btrdphs(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTRDPHS."]
        #[inline(always)]
        pub const fn set_btrdphs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Phsctl {
        #[inline(always)]
        fn default() -> Phsctl {
            Phsctl(0)
        }
    }
    impl core::fmt::Debug for Phsctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Phsctl")
                .field("benphs", &self.benphs())
                .field("btruphs", &self.btruphs())
                .field("btrdphs", &self.btrdphs())
                .finish()
        }
    }
    #[doc = "desc SCMAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scmar(pub u32);
    impl Scmar {
        #[doc = "desc SCMA."]
        #[must_use]
        #[inline(always)]
        pub const fn scma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc SCMA."]
        #[inline(always)]
        pub const fn set_scma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Scmar {
        #[inline(always)]
        fn default() -> Scmar {
            Scmar(0)
        }
    }
    impl core::fmt::Debug for Scmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scmar").field("scma", &self.scma()).finish()
        }
    }
    #[doc = "desc SCMASELR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scmaselr(pub u32);
    impl Scmaselr {
        #[doc = "desc CMSA1."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsa1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSA1."]
        #[inline(always)]
        pub const fn set_cmsa1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CMSB1."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSB1."]
        #[inline(always)]
        pub const fn set_cmsb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc OVF1."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF1."]
        #[inline(always)]
        pub const fn set_ovf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc UDF1."]
        #[must_use]
        #[inline(always)]
        pub const fn udf1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDF1."]
        #[inline(always)]
        pub const fn set_udf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CMSA2."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsa2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSA2."]
        #[inline(always)]
        pub const fn set_cmsa2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CMSB2."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsb2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSB2."]
        #[inline(always)]
        pub const fn set_cmsb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc OVF2."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF2."]
        #[inline(always)]
        pub const fn set_ovf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc UDF2."]
        #[must_use]
        #[inline(always)]
        pub const fn udf2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDF2."]
        #[inline(always)]
        pub const fn set_udf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CMSA3."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsa3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSA3."]
        #[inline(always)]
        pub const fn set_cmsa3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CMSB3."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsb3(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSB3."]
        #[inline(always)]
        pub const fn set_cmsb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc OVF3."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF3."]
        #[inline(always)]
        pub const fn set_ovf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc UDF3."]
        #[must_use]
        #[inline(always)]
        pub const fn udf3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDF3."]
        #[inline(always)]
        pub const fn set_udf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CMSA4."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsa4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSA4."]
        #[inline(always)]
        pub const fn set_cmsa4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc CMSB4."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsb4(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSB4."]
        #[inline(always)]
        pub const fn set_cmsb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc OVF4."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf4(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF4."]
        #[inline(always)]
        pub const fn set_ovf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc UDF4."]
        #[must_use]
        #[inline(always)]
        pub const fn udf4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDF4."]
        #[inline(always)]
        pub const fn set_udf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc CMSA5."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsa5(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSA5."]
        #[inline(always)]
        pub const fn set_cmsa5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc CMSB5."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsb5(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSB5."]
        #[inline(always)]
        pub const fn set_cmsb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc OVF5."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf5(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF5."]
        #[inline(always)]
        pub const fn set_ovf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc UDF5."]
        #[must_use]
        #[inline(always)]
        pub const fn udf5(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDF5."]
        #[inline(always)]
        pub const fn set_udf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc CMSA6."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsa6(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSA6."]
        #[inline(always)]
        pub const fn set_cmsa6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc CMSB6."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsb6(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSB6."]
        #[inline(always)]
        pub const fn set_cmsb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc OVF6."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF6."]
        #[inline(always)]
        pub const fn set_ovf6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc UDF6."]
        #[must_use]
        #[inline(always)]
        pub const fn udf6(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDF6."]
        #[inline(always)]
        pub const fn set_udf6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc EEVDET."]
        #[must_use]
        #[inline(always)]
        pub const fn eevdet(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc EEVDET."]
        #[inline(always)]
        pub const fn set_eevdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Scmaselr {
        #[inline(always)]
        fn default() -> Scmaselr {
            Scmaselr(0)
        }
    }
    impl core::fmt::Debug for Scmaselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scmaselr")
                .field("cmsa1", &self.cmsa1())
                .field("cmsb1", &self.cmsb1())
                .field("ovf1", &self.ovf1())
                .field("udf1", &self.udf1())
                .field("cmsa2", &self.cmsa2())
                .field("cmsb2", &self.cmsb2())
                .field("ovf2", &self.ovf2())
                .field("udf2", &self.udf2())
                .field("cmsa3", &self.cmsa3())
                .field("cmsb3", &self.cmsb3())
                .field("ovf3", &self.ovf3())
                .field("udf3", &self.udf3())
                .field("cmsa4", &self.cmsa4())
                .field("cmsb4", &self.cmsb4())
                .field("ovf4", &self.ovf4())
                .field("udf4", &self.udf4())
                .field("cmsa5", &self.cmsa5())
                .field("cmsb5", &self.cmsb5())
                .field("ovf5", &self.ovf5())
                .field("udf5", &self.udf5())
                .field("cmsa6", &self.cmsa6())
                .field("cmsb6", &self.cmsb6())
                .field("ovf6", &self.ovf6())
                .field("udf6", &self.udf6())
                .field("eevdet", &self.eevdet())
                .finish()
        }
    }
    #[doc = "desc SCMBR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scmbr(pub u32);
    impl Scmbr {
        #[doc = "desc SCMB."]
        #[must_use]
        #[inline(always)]
        pub const fn scmb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc SCMB."]
        #[inline(always)]
        pub const fn set_scmb(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Scmbr {
        #[inline(always)]
        fn default() -> Scmbr {
            Scmbr(0)
        }
    }
    impl core::fmt::Debug for Scmbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scmbr").field("scmb", &self.scmb()).finish()
        }
    }
    #[doc = "desc SCMCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scmcr(pub u32);
    impl Scmcr {
        #[doc = "desc SCMC."]
        #[must_use]
        #[inline(always)]
        pub const fn scmc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc SCMC."]
        #[inline(always)]
        pub const fn set_scmc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Scmcr {
        #[inline(always)]
        fn default() -> Scmcr {
            Scmcr(0)
        }
    }
    impl core::fmt::Debug for Scmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scmcr").field("scmc", &self.scmc()).finish()
        }
    }
    #[doc = "desc SCMDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scmdr(pub u32);
    impl Scmdr {
        #[doc = "desc SCMD."]
        #[must_use]
        #[inline(always)]
        pub const fn scmd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc SCMD."]
        #[inline(always)]
        pub const fn set_scmd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Scmdr {
        #[inline(always)]
        fn default() -> Scmdr {
            Scmdr(0)
        }
    }
    impl core::fmt::Debug for Scmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scmdr").field("scmd", &self.scmd()).finish()
        }
    }
    #[doc = "desc STFLR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stflr1(pub u32);
    impl Stflr1 {
        #[doc = "desc CMAF."]
        #[must_use]
        #[inline(always)]
        pub const fn cmaf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMAF."]
        #[inline(always)]
        pub const fn set_cmaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CMBF."]
        #[must_use]
        #[inline(always)]
        pub const fn cmbf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMBF."]
        #[inline(always)]
        pub const fn set_cmbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CMCF."]
        #[must_use]
        #[inline(always)]
        pub const fn cmcf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMCF."]
        #[inline(always)]
        pub const fn set_cmcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CMDF."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMDF."]
        #[inline(always)]
        pub const fn set_cmdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CMEF."]
        #[must_use]
        #[inline(always)]
        pub const fn cmef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMEF."]
        #[inline(always)]
        pub const fn set_cmef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CMFF."]
        #[must_use]
        #[inline(always)]
        pub const fn cmff(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMFF."]
        #[inline(always)]
        pub const fn set_cmff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc OVFF."]
        #[must_use]
        #[inline(always)]
        pub const fn ovff(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVFF."]
        #[inline(always)]
        pub const fn set_ovff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc UDFF."]
        #[must_use]
        #[inline(always)]
        pub const fn udff(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDFF."]
        #[inline(always)]
        pub const fn set_udff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CMSAUF."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsauf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSAUF."]
        #[inline(always)]
        pub const fn set_cmsauf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CMSADF."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsadf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSADF."]
        #[inline(always)]
        pub const fn set_cmsadf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc CMSBUF."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsbuf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSBUF."]
        #[inline(always)]
        pub const fn set_cmsbuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CMSBDF."]
        #[must_use]
        #[inline(always)]
        pub const fn cmsbdf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMSBDF."]
        #[inline(always)]
        pub const fn set_cmsbdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VPERNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn vpernum(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x1f;
            val as u8
        }
        #[doc = "desc VPERNUM."]
        #[inline(always)]
        pub const fn set_vpernum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
        }
        #[doc = "desc CAPAF."]
        #[must_use]
        #[inline(always)]
        pub const fn capaf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPAF."]
        #[inline(always)]
        pub const fn set_capaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc CAPBF."]
        #[must_use]
        #[inline(always)]
        pub const fn capbf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPBF."]
        #[inline(always)]
        pub const fn set_capbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc DIRF."]
        #[must_use]
        #[inline(always)]
        pub const fn dirf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIRF."]
        #[inline(always)]
        pub const fn set_dirf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Stflr1 {
        #[inline(always)]
        fn default() -> Stflr1 {
            Stflr1(0)
        }
    }
    impl core::fmt::Debug for Stflr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stflr1")
                .field("cmaf", &self.cmaf())
                .field("cmbf", &self.cmbf())
                .field("cmcf", &self.cmcf())
                .field("cmdf", &self.cmdf())
                .field("cmef", &self.cmef())
                .field("cmff", &self.cmff())
                .field("ovff", &self.ovff())
                .field("udff", &self.udff())
                .field("cmsauf", &self.cmsauf())
                .field("cmsadf", &self.cmsadf())
                .field("cmsbuf", &self.cmsbuf())
                .field("cmsbdf", &self.cmsbdf())
                .field("vpernum", &self.vpernum())
                .field("capaf", &self.capaf())
                .field("capbf", &self.capbf())
                .field("dirf", &self.dirf())
                .finish()
        }
    }
    #[doc = "desc STFLR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stflr2(pub u32);
    impl Stflr2 {
        #[doc = "desc DLYIDLEA."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyidlea(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc DLYIDLEA."]
        #[inline(always)]
        pub const fn set_dlyidlea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc DLYIDLEB."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyidleb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc DLYIDLEB."]
        #[inline(always)]
        pub const fn set_dlyidleb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc DLYPRT."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyprt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc DLYPRT."]
        #[inline(always)]
        pub const fn set_dlyprt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc OASTAT."]
        #[must_use]
        #[inline(always)]
        pub const fn oastat(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OASTAT."]
        #[inline(always)]
        pub const fn set_oastat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc OBSTAT."]
        #[must_use]
        #[inline(always)]
        pub const fn obstat(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc OBSTAT."]
        #[inline(always)]
        pub const fn set_obstat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PHSCMP1F."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp1f(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PHSCMP1F."]
        #[inline(always)]
        pub const fn set_phscmp1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PHSCMP2F."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp2f(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PHSCMP2F."]
        #[inline(always)]
        pub const fn set_phscmp2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PHSCMP3F."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp3f(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PHSCMP3F."]
        #[inline(always)]
        pub const fn set_phscmp3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PHSCMP4F."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp4f(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PHSCMP4F."]
        #[inline(always)]
        pub const fn set_phscmp4f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PHSCMP5F."]
        #[must_use]
        #[inline(always)]
        pub const fn phscmp5f(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PHSCMP5F."]
        #[inline(always)]
        pub const fn set_phscmp5f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc OSTOVF."]
        #[must_use]
        #[inline(always)]
        pub const fn ostovf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc OSTOVF."]
        #[inline(always)]
        pub const fn set_ostovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Stflr2 {
        #[inline(always)]
        fn default() -> Stflr2 {
            Stflr2(0)
        }
    }
    impl core::fmt::Debug for Stflr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stflr2")
                .field("dlyidlea", &self.dlyidlea())
                .field("dlyidleb", &self.dlyidleb())
                .field("dlyprt", &self.dlyprt())
                .field("oastat", &self.oastat())
                .field("obstat", &self.obstat())
                .field("phscmp1f", &self.phscmp1f())
                .field("phscmp2f", &self.phscmp2f())
                .field("phscmp3f", &self.phscmp3f())
                .field("phscmp4f", &self.phscmp4f())
                .field("phscmp5f", &self.phscmp5f())
                .field("ostovf", &self.ostovf())
                .finish()
        }
    }
    #[doc = "desc UPDAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Updar(pub u32);
    impl Updar {
        #[doc = "desc UPDA."]
        #[must_use]
        #[inline(always)]
        pub const fn upda(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "desc UPDA."]
        #[inline(always)]
        pub const fn set_upda(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Updar {
        #[inline(always)]
        fn default() -> Updar {
            Updar(0)
        }
    }
    impl core::fmt::Debug for Updar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Updar").field("upda", &self.upda()).finish()
        }
    }
    #[doc = "desc VPERR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vperr(pub u32);
    impl Vperr {
        #[doc = "desc SPPERIA."]
        #[must_use]
        #[inline(always)]
        pub const fn spperia(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPPERIA."]
        #[inline(always)]
        pub const fn set_spperia(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SPPERIB."]
        #[must_use]
        #[inline(always)]
        pub const fn spperib(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPPERIB."]
        #[inline(always)]
        pub const fn set_spperib(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PCNTE."]
        #[must_use]
        #[inline(always)]
        pub const fn pcnte(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc PCNTE."]
        #[inline(always)]
        pub const fn set_pcnte(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc PCNTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pcnts(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[doc = "desc PCNTS."]
        #[inline(always)]
        pub const fn set_pcnts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
    }
    impl Default for Vperr {
        #[inline(always)]
        fn default() -> Vperr {
            Vperr(0)
        }
    }
    impl core::fmt::Debug for Vperr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vperr")
                .field("spperia", &self.spperia())
                .field("spperib", &self.spperib())
                .field("pcnte", &self.pcnte())
                .field("pcnts", &self.pcnts())
                .finish()
        }
    }
}
