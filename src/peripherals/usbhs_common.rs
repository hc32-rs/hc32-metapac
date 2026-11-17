#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc USBHS."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhs {
    ptr: *mut u8,
}
unsafe impl Send for Usbhs {}
unsafe impl Sync for Usbhs {}
impl Usbhs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc GVBUSCFG."]
    #[inline(always)]
    pub const fn gvbuscfg(self) -> crate::common::Reg<regs::Gvbuscfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc GAHBCFG."]
    #[inline(always)]
    pub const fn gahbcfg(self) -> crate::common::Reg<regs::Gahbcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc GUSBCFG."]
    #[inline(always)]
    pub const fn gusbcfg(self) -> crate::common::Reg<regs::Gusbcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc GRSTCTL."]
    #[inline(always)]
    pub const fn grstctl(self) -> crate::common::Reg<regs::Grstctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc GINTSTS."]
    #[inline(always)]
    pub const fn gintsts(self) -> crate::common::Reg<regs::Gintsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc GINTMSK."]
    #[inline(always)]
    pub const fn gintmsk(self) -> crate::common::Reg<regs::Gintmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc GRXSTSR."]
    #[inline(always)]
    pub const fn grxstsr(self) -> crate::common::Reg<regs::Grxstsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc GRXSTSP."]
    #[inline(always)]
    pub const fn grxstsp(self) -> crate::common::Reg<regs::Grxstsp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc GRXFSIZ."]
    #[inline(always)]
    pub const fn grxfsiz(self) -> crate::common::Reg<regs::Grxfsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc HNPTXFSIZ."]
    #[inline(always)]
    pub const fn hnptxfsiz(self) -> crate::common::Reg<regs::Hnptxfsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc HNPTXSTS."]
    #[inline(always)]
    pub const fn hnptxsts(self) -> crate::common::Reg<regs::Hnptxsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc CID."]
    #[inline(always)]
    pub const fn cid(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc GLPMCFG."]
    #[inline(always)]
    pub const fn glpmcfg(self) -> crate::common::Reg<regs::Glpmcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc HPTXFSIZ."]
    #[inline(always)]
    pub const fn hptxfsiz(self) -> crate::common::Reg<regs::Hptxfsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "desc DIEPTXF1."]
    #[inline(always)]
    pub const fn dieptxf1(self) -> crate::common::Reg<regs::Dieptxf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "desc DIEPTXF2."]
    #[inline(always)]
    pub const fn dieptxf2(self) -> crate::common::Reg<regs::Dieptxf2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "desc DIEPTXF3."]
    #[inline(always)]
    pub const fn dieptxf3(self) -> crate::common::Reg<regs::Dieptxf3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "desc DIEPTXF4."]
    #[inline(always)]
    pub const fn dieptxf4(self) -> crate::common::Reg<regs::Dieptxf4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "desc DIEPTXF5."]
    #[inline(always)]
    pub const fn dieptxf5(self) -> crate::common::Reg<regs::Dieptxf5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "desc DIEPTXF6."]
    #[inline(always)]
    pub const fn dieptxf6(self) -> crate::common::Reg<regs::Dieptxf6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "desc DIEPTXF7."]
    #[inline(always)]
    pub const fn dieptxf7(self) -> crate::common::Reg<regs::Dieptxf7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "desc DIEPTXF8."]
    #[inline(always)]
    pub const fn dieptxf8(self) -> crate::common::Reg<regs::Dieptxf8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "desc DIEPTXF9."]
    #[inline(always)]
    pub const fn dieptxf9(self) -> crate::common::Reg<regs::Dieptxf9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "desc DIEPTXF10."]
    #[inline(always)]
    pub const fn dieptxf10(self) -> crate::common::Reg<regs::Dieptxf10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "desc DIEPTXF11."]
    #[inline(always)]
    pub const fn dieptxf11(self) -> crate::common::Reg<regs::Dieptxf11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "desc DIEPTXF12."]
    #[inline(always)]
    pub const fn dieptxf12(self) -> crate::common::Reg<regs::Dieptxf12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "desc DIEPTXF13."]
    #[inline(always)]
    pub const fn dieptxf13(self) -> crate::common::Reg<regs::Dieptxf13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "desc DIEPTXF14."]
    #[inline(always)]
    pub const fn dieptxf14(self) -> crate::common::Reg<regs::Dieptxf14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "desc DIEPTXF15."]
    #[inline(always)]
    pub const fn dieptxf15(self) -> crate::common::Reg<regs::Dieptxf15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "desc HCFG."]
    #[inline(always)]
    pub const fn hcfg(self) -> crate::common::Reg<regs::Hcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "desc HFIR."]
    #[inline(always)]
    pub const fn hfir(self) -> crate::common::Reg<regs::Hfir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "desc HFNUM."]
    #[inline(always)]
    pub const fn hfnum(self) -> crate::common::Reg<regs::Hfnum, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "desc HPTXSTS."]
    #[inline(always)]
    pub const fn hptxsts(self) -> crate::common::Reg<regs::Hptxsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "desc HAINT."]
    #[inline(always)]
    pub const fn haint(self) -> crate::common::Reg<regs::Haint, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "desc HAINTMSK."]
    #[inline(always)]
    pub const fn haintmsk(self) -> crate::common::Reg<regs::Haintmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "desc HPRT."]
    #[inline(always)]
    pub const fn hprt(self) -> crate::common::Reg<regs::Hprt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "desc HCCHAR0."]
    #[inline(always)]
    pub const fn hcchar0(self) -> crate::common::Reg<regs::Hcchar0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "desc HCSPLT0."]
    #[inline(always)]
    pub const fn hcsplt0(self) -> crate::common::Reg<regs::Hcsplt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "desc HCINT0."]
    #[inline(always)]
    pub const fn hcint0(self) -> crate::common::Reg<regs::Hcint0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "desc HCINTMSK0."]
    #[inline(always)]
    pub const fn hcintmsk0(self) -> crate::common::Reg<regs::Hcintmsk0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "desc HCTSIZ0."]
    #[inline(always)]
    pub const fn hctsiz0(self) -> crate::common::Reg<regs::Hctsiz0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "desc HCDMA0."]
    #[inline(always)]
    pub const fn hcdma0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "desc HCCHAR1."]
    #[inline(always)]
    pub const fn hcchar1(self) -> crate::common::Reg<regs::Hcchar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "desc HCSPLT1."]
    #[inline(always)]
    pub const fn hcsplt1(self) -> crate::common::Reg<regs::Hcsplt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "desc HCINT1."]
    #[inline(always)]
    pub const fn hcint1(self) -> crate::common::Reg<regs::Hcint1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "desc HCINTMSK1."]
    #[inline(always)]
    pub const fn hcintmsk1(self) -> crate::common::Reg<regs::Hcintmsk1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "desc HCTSIZ1."]
    #[inline(always)]
    pub const fn hctsiz1(self) -> crate::common::Reg<regs::Hctsiz1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "desc HCDMA1."]
    #[inline(always)]
    pub const fn hcdma1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "desc HCCHAR2."]
    #[inline(always)]
    pub const fn hcchar2(self) -> crate::common::Reg<regs::Hcchar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "desc HCSPLT2."]
    #[inline(always)]
    pub const fn hcsplt2(self) -> crate::common::Reg<regs::Hcsplt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "desc HCINT2."]
    #[inline(always)]
    pub const fn hcint2(self) -> crate::common::Reg<regs::Hcint2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "desc HCINTMSK2."]
    #[inline(always)]
    pub const fn hcintmsk2(self) -> crate::common::Reg<regs::Hcintmsk2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "desc HCTSIZ2."]
    #[inline(always)]
    pub const fn hctsiz2(self) -> crate::common::Reg<regs::Hctsiz2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "desc HCDMA2."]
    #[inline(always)]
    pub const fn hcdma2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "desc HCCHAR3."]
    #[inline(always)]
    pub const fn hcchar3(self) -> crate::common::Reg<regs::Hcchar3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "desc HCSPLT3."]
    #[inline(always)]
    pub const fn hcsplt3(self) -> crate::common::Reg<regs::Hcsplt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "desc HCINT3."]
    #[inline(always)]
    pub const fn hcint3(self) -> crate::common::Reg<regs::Hcint3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "desc HCINTMSK3."]
    #[inline(always)]
    pub const fn hcintmsk3(self) -> crate::common::Reg<regs::Hcintmsk3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "desc HCTSIZ3."]
    #[inline(always)]
    pub const fn hctsiz3(self) -> crate::common::Reg<regs::Hctsiz3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "desc HCDMA3."]
    #[inline(always)]
    pub const fn hcdma3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
    }
    #[doc = "desc HCCHAR4."]
    #[inline(always)]
    pub const fn hcchar4(self) -> crate::common::Reg<regs::Hcchar4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "desc HCSPLT4."]
    #[inline(always)]
    pub const fn hcsplt4(self) -> crate::common::Reg<regs::Hcsplt4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "desc HCINT4."]
    #[inline(always)]
    pub const fn hcint4(self) -> crate::common::Reg<regs::Hcint4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "desc HCINTMSK4."]
    #[inline(always)]
    pub const fn hcintmsk4(self) -> crate::common::Reg<regs::Hcintmsk4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "desc HCTSIZ4."]
    #[inline(always)]
    pub const fn hctsiz4(self) -> crate::common::Reg<regs::Hctsiz4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "desc HCDMA4."]
    #[inline(always)]
    pub const fn hcdma4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "desc HCCHAR5."]
    #[inline(always)]
    pub const fn hcchar5(self) -> crate::common::Reg<regs::Hcchar5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "desc HCSPLT5."]
    #[inline(always)]
    pub const fn hcsplt5(self) -> crate::common::Reg<regs::Hcsplt5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "desc HCINT5."]
    #[inline(always)]
    pub const fn hcint5(self) -> crate::common::Reg<regs::Hcint5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "desc HCINTMSK5."]
    #[inline(always)]
    pub const fn hcintmsk5(self) -> crate::common::Reg<regs::Hcintmsk5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "desc HCTSIZ5."]
    #[inline(always)]
    pub const fn hctsiz5(self) -> crate::common::Reg<regs::Hctsiz5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "desc HCDMA5."]
    #[inline(always)]
    pub const fn hcdma5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "desc HCCHAR6."]
    #[inline(always)]
    pub const fn hcchar6(self) -> crate::common::Reg<regs::Hcchar6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "desc HCSPLT6."]
    #[inline(always)]
    pub const fn hcsplt6(self) -> crate::common::Reg<regs::Hcsplt6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "desc HCINT6."]
    #[inline(always)]
    pub const fn hcint6(self) -> crate::common::Reg<regs::Hcint6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "desc HCINTMSK6."]
    #[inline(always)]
    pub const fn hcintmsk6(self) -> crate::common::Reg<regs::Hcintmsk6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ccusize) as _) }
    }
    #[doc = "desc HCTSIZ6."]
    #[inline(always)]
    pub const fn hctsiz6(self) -> crate::common::Reg<regs::Hctsiz6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "desc HCDMA6."]
    #[inline(always)]
    pub const fn hcdma6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "desc HCCHAR7."]
    #[inline(always)]
    pub const fn hcchar7(self) -> crate::common::Reg<regs::Hcchar7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "desc HCSPLT7."]
    #[inline(always)]
    pub const fn hcsplt7(self) -> crate::common::Reg<regs::Hcsplt7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e4usize) as _) }
    }
    #[doc = "desc HCINT7."]
    #[inline(always)]
    pub const fn hcint7(self) -> crate::common::Reg<regs::Hcint7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e8usize) as _) }
    }
    #[doc = "desc HCINTMSK7."]
    #[inline(always)]
    pub const fn hcintmsk7(self) -> crate::common::Reg<regs::Hcintmsk7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ecusize) as _) }
    }
    #[doc = "desc HCTSIZ7."]
    #[inline(always)]
    pub const fn hctsiz7(self) -> crate::common::Reg<regs::Hctsiz7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "desc HCDMA7."]
    #[inline(always)]
    pub const fn hcdma7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
    #[doc = "desc HCCHAR8."]
    #[inline(always)]
    pub const fn hcchar8(self) -> crate::common::Reg<regs::Hcchar8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "desc HCSPLT8."]
    #[inline(always)]
    pub const fn hcsplt8(self) -> crate::common::Reg<regs::Hcsplt8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "desc HCINT8."]
    #[inline(always)]
    pub const fn hcint8(self) -> crate::common::Reg<regs::Hcint8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "desc HCINTMSK8."]
    #[inline(always)]
    pub const fn hcintmsk8(self) -> crate::common::Reg<regs::Hcintmsk8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "desc HCTSIZ8."]
    #[inline(always)]
    pub const fn hctsiz8(self) -> crate::common::Reg<regs::Hctsiz8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "desc HCDMA8."]
    #[inline(always)]
    pub const fn hcdma8(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "desc HCCHAR9."]
    #[inline(always)]
    pub const fn hcchar9(self) -> crate::common::Reg<regs::Hcchar9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "desc HCSPLT9."]
    #[inline(always)]
    pub const fn hcsplt9(self) -> crate::common::Reg<regs::Hcsplt9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
    #[doc = "desc HCINT9."]
    #[inline(always)]
    pub const fn hcint9(self) -> crate::common::Reg<regs::Hcint9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0628usize) as _) }
    }
    #[doc = "desc HCINTMSK9."]
    #[inline(always)]
    pub const fn hcintmsk9(self) -> crate::common::Reg<regs::Hcintmsk9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x062cusize) as _) }
    }
    #[doc = "desc HCTSIZ9."]
    #[inline(always)]
    pub const fn hctsiz9(self) -> crate::common::Reg<regs::Hctsiz9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0630usize) as _) }
    }
    #[doc = "desc HCDMA9."]
    #[inline(always)]
    pub const fn hcdma9(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0634usize) as _) }
    }
    #[doc = "desc HCCHAR10."]
    #[inline(always)]
    pub const fn hcchar10(self) -> crate::common::Reg<regs::Hcchar10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
    }
    #[doc = "desc HCSPLT10."]
    #[inline(always)]
    pub const fn hcsplt10(self) -> crate::common::Reg<regs::Hcsplt10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0644usize) as _) }
    }
    #[doc = "desc HCINT10."]
    #[inline(always)]
    pub const fn hcint10(self) -> crate::common::Reg<regs::Hcint10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0648usize) as _) }
    }
    #[doc = "desc HCINTMSK10."]
    #[inline(always)]
    pub const fn hcintmsk10(self) -> crate::common::Reg<regs::Hcintmsk10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x064cusize) as _) }
    }
    #[doc = "desc HCTSIZ10."]
    #[inline(always)]
    pub const fn hctsiz10(self) -> crate::common::Reg<regs::Hctsiz10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0650usize) as _) }
    }
    #[doc = "desc HCDMA10."]
    #[inline(always)]
    pub const fn hcdma10(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0654usize) as _) }
    }
    #[doc = "desc HCCHAR11."]
    #[inline(always)]
    pub const fn hcchar11(self) -> crate::common::Reg<regs::Hcchar11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    #[doc = "desc HCSPLT11."]
    #[inline(always)]
    pub const fn hcsplt11(self) -> crate::common::Reg<regs::Hcsplt11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0664usize) as _) }
    }
    #[doc = "desc HCINT11."]
    #[inline(always)]
    pub const fn hcint11(self) -> crate::common::Reg<regs::Hcint11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0668usize) as _) }
    }
    #[doc = "desc HCINTMSK11."]
    #[inline(always)]
    pub const fn hcintmsk11(self) -> crate::common::Reg<regs::Hcintmsk11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x066cusize) as _) }
    }
    #[doc = "desc HCTSIZ11."]
    #[inline(always)]
    pub const fn hctsiz11(self) -> crate::common::Reg<regs::Hctsiz11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0670usize) as _) }
    }
    #[doc = "desc HCDMA11."]
    #[inline(always)]
    pub const fn hcdma11(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0674usize) as _) }
    }
    #[doc = "desc HCCHAR12."]
    #[inline(always)]
    pub const fn hcchar12(self) -> crate::common::Reg<regs::Hcchar12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize) as _) }
    }
    #[doc = "desc HCSPLT12."]
    #[inline(always)]
    pub const fn hcsplt12(self) -> crate::common::Reg<regs::Hcsplt12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0684usize) as _) }
    }
    #[doc = "desc HCINT12."]
    #[inline(always)]
    pub const fn hcint12(self) -> crate::common::Reg<regs::Hcint12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0688usize) as _) }
    }
    #[doc = "desc HCINTMSK12."]
    #[inline(always)]
    pub const fn hcintmsk12(self) -> crate::common::Reg<regs::Hcintmsk12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x068cusize) as _) }
    }
    #[doc = "desc HCTSIZ12."]
    #[inline(always)]
    pub const fn hctsiz12(self) -> crate::common::Reg<regs::Hctsiz12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0690usize) as _) }
    }
    #[doc = "desc HCDMA12."]
    #[inline(always)]
    pub const fn hcdma12(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0694usize) as _) }
    }
    #[doc = "desc HCCHAR13."]
    #[inline(always)]
    pub const fn hcchar13(self) -> crate::common::Reg<regs::Hcchar13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a0usize) as _) }
    }
    #[doc = "desc HCSPLT13."]
    #[inline(always)]
    pub const fn hcsplt13(self) -> crate::common::Reg<regs::Hcsplt13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a4usize) as _) }
    }
    #[doc = "desc HCINT13."]
    #[inline(always)]
    pub const fn hcint13(self) -> crate::common::Reg<regs::Hcint13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a8usize) as _) }
    }
    #[doc = "desc HCINTMSK13."]
    #[inline(always)]
    pub const fn hcintmsk13(self) -> crate::common::Reg<regs::Hcintmsk13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06acusize) as _) }
    }
    #[doc = "desc HCTSIZ13."]
    #[inline(always)]
    pub const fn hctsiz13(self) -> crate::common::Reg<regs::Hctsiz13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b0usize) as _) }
    }
    #[doc = "desc HCDMA13."]
    #[inline(always)]
    pub const fn hcdma13(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b4usize) as _) }
    }
    #[doc = "desc HCCHAR14."]
    #[inline(always)]
    pub const fn hcchar14(self) -> crate::common::Reg<regs::Hcchar14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c0usize) as _) }
    }
    #[doc = "desc HCSPLT14."]
    #[inline(always)]
    pub const fn hcsplt14(self) -> crate::common::Reg<regs::Hcsplt14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c4usize) as _) }
    }
    #[doc = "desc HCINT14."]
    #[inline(always)]
    pub const fn hcint14(self) -> crate::common::Reg<regs::Hcint14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c8usize) as _) }
    }
    #[doc = "desc HCINTMSK14."]
    #[inline(always)]
    pub const fn hcintmsk14(self) -> crate::common::Reg<regs::Hcintmsk14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06ccusize) as _) }
    }
    #[doc = "desc HCTSIZ14."]
    #[inline(always)]
    pub const fn hctsiz14(self) -> crate::common::Reg<regs::Hctsiz14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d0usize) as _) }
    }
    #[doc = "desc HCDMA14."]
    #[inline(always)]
    pub const fn hcdma14(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d4usize) as _) }
    }
    #[doc = "desc HCCHAR15."]
    #[inline(always)]
    pub const fn hcchar15(self) -> crate::common::Reg<regs::Hcchar15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize) as _) }
    }
    #[doc = "desc HCSPLT15."]
    #[inline(always)]
    pub const fn hcsplt15(self) -> crate::common::Reg<regs::Hcsplt15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e4usize) as _) }
    }
    #[doc = "desc HCINT15."]
    #[inline(always)]
    pub const fn hcint15(self) -> crate::common::Reg<regs::Hcint15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e8usize) as _) }
    }
    #[doc = "desc HCINTMSK15."]
    #[inline(always)]
    pub const fn hcintmsk15(self) -> crate::common::Reg<regs::Hcintmsk15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06ecusize) as _) }
    }
    #[doc = "desc HCTSIZ15."]
    #[inline(always)]
    pub const fn hctsiz15(self) -> crate::common::Reg<regs::Hctsiz15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f0usize) as _) }
    }
    #[doc = "desc HCDMA15."]
    #[inline(always)]
    pub const fn hcdma15(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f4usize) as _) }
    }
    #[doc = "desc DCFG."]
    #[inline(always)]
    pub const fn dcfg(self) -> crate::common::Reg<regs::Dcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "desc DCTL."]
    #[inline(always)]
    pub const fn dctl(self) -> crate::common::Reg<regs::Dctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "desc DSTS."]
    #[inline(always)]
    pub const fn dsts(self) -> crate::common::Reg<regs::Dsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0808usize) as _) }
    }
    #[doc = "desc DIEPMSK."]
    #[inline(always)]
    pub const fn diepmsk(self) -> crate::common::Reg<regs::Diepmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0810usize) as _) }
    }
    #[doc = "desc DOEPMSK."]
    #[inline(always)]
    pub const fn doepmsk(self) -> crate::common::Reg<regs::Doepmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0814usize) as _) }
    }
    #[doc = "desc DAINT."]
    #[inline(always)]
    pub const fn daint(self) -> crate::common::Reg<regs::Daint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0818usize) as _) }
    }
    #[doc = "desc DAINTMSK."]
    #[inline(always)]
    pub const fn daintmsk(self) -> crate::common::Reg<regs::Daintmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x081cusize) as _) }
    }
    #[doc = "desc DTHRCTL."]
    #[inline(always)]
    pub const fn dthrctl(self) -> crate::common::Reg<regs::Dthrctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0830usize) as _) }
    }
    #[doc = "desc DIEPEMPMSK."]
    #[inline(always)]
    pub const fn diepempmsk(self) -> crate::common::Reg<regs::Diepempmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0834usize) as _) }
    }
    #[doc = "desc DEACHINT."]
    #[inline(always)]
    pub const fn deachint(self) -> crate::common::Reg<regs::Deachint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0838usize) as _) }
    }
    #[doc = "desc DEACHINTMSK."]
    #[inline(always)]
    pub const fn deachintmsk(self) -> crate::common::Reg<regs::Deachintmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x083cusize) as _) }
    }
    #[doc = "desc DIEPEACHMSK1."]
    #[inline(always)]
    pub const fn diepeachmsk1(self) -> crate::common::Reg<regs::Diepeachmsk1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0844usize) as _) }
    }
    #[doc = "desc DOEPEACHMSK1."]
    #[inline(always)]
    pub const fn doepeachmsk1(self) -> crate::common::Reg<regs::Doepeachmsk1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0884usize) as _) }
    }
    #[doc = "desc DIEPCTL0."]
    #[inline(always)]
    pub const fn diepctl0(self) -> crate::common::Reg<regs::Diepctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize) as _) }
    }
    #[doc = "desc DIEPINT0."]
    #[inline(always)]
    pub const fn diepint0(self) -> crate::common::Reg<regs::Diepint0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0908usize) as _) }
    }
    #[doc = "desc DIEPTSIZ0."]
    #[inline(always)]
    pub const fn dieptsiz0(self) -> crate::common::Reg<regs::Dieptsiz0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0910usize) as _) }
    }
    #[doc = "desc DIEPDMA0."]
    #[inline(always)]
    pub const fn diepdma0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0914usize) as _) }
    }
    #[doc = "desc DTXFSTS0."]
    #[inline(always)]
    pub const fn dtxfsts0(self) -> crate::common::Reg<regs::Dtxfsts0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0918usize) as _) }
    }
    #[doc = "desc DIEPCTL1."]
    #[inline(always)]
    pub const fn diepctl1(self) -> crate::common::Reg<regs::Diepctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0920usize) as _) }
    }
    #[doc = "desc DIEPINT1."]
    #[inline(always)]
    pub const fn diepint1(self) -> crate::common::Reg<regs::Diepint1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0928usize) as _) }
    }
    #[doc = "desc DIEPTSIZ1."]
    #[inline(always)]
    pub const fn dieptsiz1(self) -> crate::common::Reg<regs::Dieptsiz1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0930usize) as _) }
    }
    #[doc = "desc DIEPDMA1."]
    #[inline(always)]
    pub const fn diepdma1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0934usize) as _) }
    }
    #[doc = "desc DTXFSTS1."]
    #[inline(always)]
    pub const fn dtxfsts1(self) -> crate::common::Reg<regs::Dtxfsts1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0938usize) as _) }
    }
    #[doc = "desc DIEPCTL2."]
    #[inline(always)]
    pub const fn diepctl2(self) -> crate::common::Reg<regs::Diepctl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0940usize) as _) }
    }
    #[doc = "desc DIEPINT2."]
    #[inline(always)]
    pub const fn diepint2(self) -> crate::common::Reg<regs::Diepint2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0948usize) as _) }
    }
    #[doc = "desc DIEPTSIZ2."]
    #[inline(always)]
    pub const fn dieptsiz2(self) -> crate::common::Reg<regs::Dieptsiz2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0950usize) as _) }
    }
    #[doc = "desc DIEPDMA2."]
    #[inline(always)]
    pub const fn diepdma2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0954usize) as _) }
    }
    #[doc = "desc DTXFSTS2."]
    #[inline(always)]
    pub const fn dtxfsts2(self) -> crate::common::Reg<regs::Dtxfsts2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0958usize) as _) }
    }
    #[doc = "desc DIEPCTL3."]
    #[inline(always)]
    pub const fn diepctl3(self) -> crate::common::Reg<regs::Diepctl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0960usize) as _) }
    }
    #[doc = "desc DIEPINT3."]
    #[inline(always)]
    pub const fn diepint3(self) -> crate::common::Reg<regs::Diepint3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0968usize) as _) }
    }
    #[doc = "desc DIEPTSIZ3."]
    #[inline(always)]
    pub const fn dieptsiz3(self) -> crate::common::Reg<regs::Dieptsiz3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0970usize) as _) }
    }
    #[doc = "desc DIEPDMA3."]
    #[inline(always)]
    pub const fn diepdma3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0974usize) as _) }
    }
    #[doc = "desc DTXFSTS3."]
    #[inline(always)]
    pub const fn dtxfsts3(self) -> crate::common::Reg<regs::Dtxfsts3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0978usize) as _) }
    }
    #[doc = "desc DIEPCTL4."]
    #[inline(always)]
    pub const fn diepctl4(self) -> crate::common::Reg<regs::Diepctl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0980usize) as _) }
    }
    #[doc = "desc DIEPINT4."]
    #[inline(always)]
    pub const fn diepint4(self) -> crate::common::Reg<regs::Diepint4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0988usize) as _) }
    }
    #[doc = "desc DIEPTSIZ4."]
    #[inline(always)]
    pub const fn dieptsiz4(self) -> crate::common::Reg<regs::Dieptsiz4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0990usize) as _) }
    }
    #[doc = "desc DIEPDMA4."]
    #[inline(always)]
    pub const fn diepdma4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0994usize) as _) }
    }
    #[doc = "desc DTXFSTS4."]
    #[inline(always)]
    pub const fn dtxfsts4(self) -> crate::common::Reg<regs::Dtxfsts4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0998usize) as _) }
    }
    #[doc = "desc DIEPCTL5."]
    #[inline(always)]
    pub const fn diepctl5(self) -> crate::common::Reg<regs::Diepctl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09a0usize) as _) }
    }
    #[doc = "desc DIEPINT5."]
    #[inline(always)]
    pub const fn diepint5(self) -> crate::common::Reg<regs::Diepint5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09a8usize) as _) }
    }
    #[doc = "desc DIEPTSIZ5."]
    #[inline(always)]
    pub const fn dieptsiz5(self) -> crate::common::Reg<regs::Dieptsiz5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09b0usize) as _) }
    }
    #[doc = "desc DIEPDMA5."]
    #[inline(always)]
    pub const fn diepdma5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09b4usize) as _) }
    }
    #[doc = "desc DTXFSTS5."]
    #[inline(always)]
    pub const fn dtxfsts5(self) -> crate::common::Reg<regs::Dtxfsts5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09b8usize) as _) }
    }
    #[doc = "desc DIEPCTL6."]
    #[inline(always)]
    pub const fn diepctl6(self) -> crate::common::Reg<regs::Diepctl6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09c0usize) as _) }
    }
    #[doc = "desc DIEPINT6."]
    #[inline(always)]
    pub const fn diepint6(self) -> crate::common::Reg<regs::Diepint6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09c8usize) as _) }
    }
    #[doc = "desc DIEPTSIZ6."]
    #[inline(always)]
    pub const fn dieptsiz6(self) -> crate::common::Reg<regs::Dieptsiz6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d0usize) as _) }
    }
    #[doc = "desc DIEPDMA6."]
    #[inline(always)]
    pub const fn diepdma6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d4usize) as _) }
    }
    #[doc = "desc DTXFSTS6."]
    #[inline(always)]
    pub const fn dtxfsts6(self) -> crate::common::Reg<regs::Dtxfsts6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d8usize) as _) }
    }
    #[doc = "desc DIEPCTL7."]
    #[inline(always)]
    pub const fn diepctl7(self) -> crate::common::Reg<regs::Diepctl7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e0usize) as _) }
    }
    #[doc = "desc DIEPINT7."]
    #[inline(always)]
    pub const fn diepint7(self) -> crate::common::Reg<regs::Diepint7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e8usize) as _) }
    }
    #[doc = "desc DIEPTSIZ7."]
    #[inline(always)]
    pub const fn dieptsiz7(self) -> crate::common::Reg<regs::Dieptsiz7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f0usize) as _) }
    }
    #[doc = "desc DIEPDMA7."]
    #[inline(always)]
    pub const fn diepdma7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f4usize) as _) }
    }
    #[doc = "desc DTXFSTS7."]
    #[inline(always)]
    pub const fn dtxfsts7(self) -> crate::common::Reg<regs::Dtxfsts7, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f8usize) as _) }
    }
    #[doc = "desc DIEPCTL8."]
    #[inline(always)]
    pub const fn diepctl8(self) -> crate::common::Reg<regs::Diepctl8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a00usize) as _) }
    }
    #[doc = "desc DIEPINT8."]
    #[inline(always)]
    pub const fn diepint8(self) -> crate::common::Reg<regs::Diepint8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a08usize) as _) }
    }
    #[doc = "desc DIEPTSIZ8."]
    #[inline(always)]
    pub const fn dieptsiz8(self) -> crate::common::Reg<regs::Dieptsiz8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a10usize) as _) }
    }
    #[doc = "desc DIEPDMA8."]
    #[inline(always)]
    pub const fn diepdma8(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a14usize) as _) }
    }
    #[doc = "desc DTXFSTS8."]
    #[inline(always)]
    pub const fn dtxfsts8(self) -> crate::common::Reg<regs::Dtxfsts8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a18usize) as _) }
    }
    #[doc = "desc DIEPCTL9."]
    #[inline(always)]
    pub const fn diepctl9(self) -> crate::common::Reg<regs::Diepctl9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a20usize) as _) }
    }
    #[doc = "desc DIEPINT9."]
    #[inline(always)]
    pub const fn diepint9(self) -> crate::common::Reg<regs::Diepint9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a28usize) as _) }
    }
    #[doc = "desc DIEPTSIZ9."]
    #[inline(always)]
    pub const fn dieptsiz9(self) -> crate::common::Reg<regs::Dieptsiz9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a30usize) as _) }
    }
    #[doc = "desc DIEPDMA9."]
    #[inline(always)]
    pub const fn diepdma9(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a34usize) as _) }
    }
    #[doc = "desc DTXFSTS9."]
    #[inline(always)]
    pub const fn dtxfsts9(self) -> crate::common::Reg<regs::Dtxfsts9, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a38usize) as _) }
    }
    #[doc = "desc DIEPCTL10."]
    #[inline(always)]
    pub const fn diepctl10(self) -> crate::common::Reg<regs::Diepctl10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a40usize) as _) }
    }
    #[doc = "desc DIEPINT10."]
    #[inline(always)]
    pub const fn diepint10(self) -> crate::common::Reg<regs::Diepint10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a48usize) as _) }
    }
    #[doc = "desc DIEPTSIZ10."]
    #[inline(always)]
    pub const fn dieptsiz10(self) -> crate::common::Reg<regs::Dieptsiz10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a50usize) as _) }
    }
    #[doc = "desc DIEPDMA10."]
    #[inline(always)]
    pub const fn diepdma10(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a54usize) as _) }
    }
    #[doc = "desc DTXFSTS10."]
    #[inline(always)]
    pub const fn dtxfsts10(self) -> crate::common::Reg<regs::Dtxfsts10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a58usize) as _) }
    }
    #[doc = "desc DIEPCTL11."]
    #[inline(always)]
    pub const fn diepctl11(self) -> crate::common::Reg<regs::Diepctl11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a60usize) as _) }
    }
    #[doc = "desc DIEPINT11."]
    #[inline(always)]
    pub const fn diepint11(self) -> crate::common::Reg<regs::Diepint11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a68usize) as _) }
    }
    #[doc = "desc DIEPTSIZ11."]
    #[inline(always)]
    pub const fn dieptsiz11(self) -> crate::common::Reg<regs::Dieptsiz11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a70usize) as _) }
    }
    #[doc = "desc DIEPDMA11."]
    #[inline(always)]
    pub const fn diepdma11(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a74usize) as _) }
    }
    #[doc = "desc DTXFSTS11."]
    #[inline(always)]
    pub const fn dtxfsts11(self) -> crate::common::Reg<regs::Dtxfsts11, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a78usize) as _) }
    }
    #[doc = "desc DIEPCTL12."]
    #[inline(always)]
    pub const fn diepctl12(self) -> crate::common::Reg<regs::Diepctl12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a80usize) as _) }
    }
    #[doc = "desc DIEPINT12."]
    #[inline(always)]
    pub const fn diepint12(self) -> crate::common::Reg<regs::Diepint12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a88usize) as _) }
    }
    #[doc = "desc DIEPTSIZ12."]
    #[inline(always)]
    pub const fn dieptsiz12(self) -> crate::common::Reg<regs::Dieptsiz12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a90usize) as _) }
    }
    #[doc = "desc DIEPDMA12."]
    #[inline(always)]
    pub const fn diepdma12(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a94usize) as _) }
    }
    #[doc = "desc DTXFSTS12."]
    #[inline(always)]
    pub const fn dtxfsts12(self) -> crate::common::Reg<regs::Dtxfsts12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a98usize) as _) }
    }
    #[doc = "desc DIEPCTL13."]
    #[inline(always)]
    pub const fn diepctl13(self) -> crate::common::Reg<regs::Diepctl13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0aa0usize) as _) }
    }
    #[doc = "desc DIEPINT13."]
    #[inline(always)]
    pub const fn diepint13(self) -> crate::common::Reg<regs::Diepint13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0aa8usize) as _) }
    }
    #[doc = "desc DIEPTSIZ13."]
    #[inline(always)]
    pub const fn dieptsiz13(self) -> crate::common::Reg<regs::Dieptsiz13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ab0usize) as _) }
    }
    #[doc = "desc DIEPDMA13."]
    #[inline(always)]
    pub const fn diepdma13(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ab4usize) as _) }
    }
    #[doc = "desc DTXFSTS13."]
    #[inline(always)]
    pub const fn dtxfsts13(self) -> crate::common::Reg<regs::Dtxfsts13, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ab8usize) as _) }
    }
    #[doc = "desc DIEPCTL14."]
    #[inline(always)]
    pub const fn diepctl14(self) -> crate::common::Reg<regs::Diepctl14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ac0usize) as _) }
    }
    #[doc = "desc DIEPINT14."]
    #[inline(always)]
    pub const fn diepint14(self) -> crate::common::Reg<regs::Diepint14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ac8usize) as _) }
    }
    #[doc = "desc DIEPTSIZ14."]
    #[inline(always)]
    pub const fn dieptsiz14(self) -> crate::common::Reg<regs::Dieptsiz14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ad0usize) as _) }
    }
    #[doc = "desc DIEPDMA14."]
    #[inline(always)]
    pub const fn diepdma14(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ad4usize) as _) }
    }
    #[doc = "desc DTXFSTS14."]
    #[inline(always)]
    pub const fn dtxfsts14(self) -> crate::common::Reg<regs::Dtxfsts14, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ad8usize) as _) }
    }
    #[doc = "desc DIEPCTL15."]
    #[inline(always)]
    pub const fn diepctl15(self) -> crate::common::Reg<regs::Diepctl15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ae0usize) as _) }
    }
    #[doc = "desc DIEPINT15."]
    #[inline(always)]
    pub const fn diepint15(self) -> crate::common::Reg<regs::Diepint15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ae8usize) as _) }
    }
    #[doc = "desc DIEPTSIZ15."]
    #[inline(always)]
    pub const fn dieptsiz15(self) -> crate::common::Reg<regs::Dieptsiz15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0af0usize) as _) }
    }
    #[doc = "desc DIEPDMA15."]
    #[inline(always)]
    pub const fn diepdma15(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0af4usize) as _) }
    }
    #[doc = "desc DTXFSTS15."]
    #[inline(always)]
    pub const fn dtxfsts15(self) -> crate::common::Reg<regs::Dtxfsts15, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0af8usize) as _) }
    }
    #[doc = "desc DOEPCTL0."]
    #[inline(always)]
    pub const fn doepctl0(self) -> crate::common::Reg<regs::Doepctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b00usize) as _) }
    }
    #[doc = "desc DOEPINT0."]
    #[inline(always)]
    pub const fn doepint0(self) -> crate::common::Reg<regs::Doepint0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b08usize) as _) }
    }
    #[doc = "desc DOEPTSIZ0."]
    #[inline(always)]
    pub const fn doeptsiz0(self) -> crate::common::Reg<regs::Doeptsiz0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b10usize) as _) }
    }
    #[doc = "desc DOEPDMA0."]
    #[inline(always)]
    pub const fn doepdma0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b14usize) as _) }
    }
    #[doc = "desc DOEPCTL1."]
    #[inline(always)]
    pub const fn doepctl1(self) -> crate::common::Reg<regs::Doepctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b20usize) as _) }
    }
    #[doc = "desc DOEPINT1."]
    #[inline(always)]
    pub const fn doepint1(self) -> crate::common::Reg<regs::Doepint1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b28usize) as _) }
    }
    #[doc = "desc DOEPTSIZ1."]
    #[inline(always)]
    pub const fn doeptsiz1(self) -> crate::common::Reg<regs::Doeptsiz1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b30usize) as _) }
    }
    #[doc = "desc DOEPDMA1."]
    #[inline(always)]
    pub const fn doepdma1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b34usize) as _) }
    }
    #[doc = "desc DOEPCTL2."]
    #[inline(always)]
    pub const fn doepctl2(self) -> crate::common::Reg<regs::Doepctl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b40usize) as _) }
    }
    #[doc = "desc DOEPINT2."]
    #[inline(always)]
    pub const fn doepint2(self) -> crate::common::Reg<regs::Doepint2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b48usize) as _) }
    }
    #[doc = "desc DOEPTSIZ2."]
    #[inline(always)]
    pub const fn doeptsiz2(self) -> crate::common::Reg<regs::Doeptsiz2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b50usize) as _) }
    }
    #[doc = "desc DOEPDMA2."]
    #[inline(always)]
    pub const fn doepdma2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b54usize) as _) }
    }
    #[doc = "desc DOEPCTL3."]
    #[inline(always)]
    pub const fn doepctl3(self) -> crate::common::Reg<regs::Doepctl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b60usize) as _) }
    }
    #[doc = "desc DOEPINT3."]
    #[inline(always)]
    pub const fn doepint3(self) -> crate::common::Reg<regs::Doepint3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b68usize) as _) }
    }
    #[doc = "desc DOEPTSIZ3."]
    #[inline(always)]
    pub const fn doeptsiz3(self) -> crate::common::Reg<regs::Doeptsiz3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b70usize) as _) }
    }
    #[doc = "desc DOEPDMA3."]
    #[inline(always)]
    pub const fn doepdma3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b74usize) as _) }
    }
    #[doc = "desc DOEPCTL4."]
    #[inline(always)]
    pub const fn doepctl4(self) -> crate::common::Reg<regs::Doepctl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b80usize) as _) }
    }
    #[doc = "desc DOEPINT4."]
    #[inline(always)]
    pub const fn doepint4(self) -> crate::common::Reg<regs::Doepint4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b88usize) as _) }
    }
    #[doc = "desc DOEPTSIZ4."]
    #[inline(always)]
    pub const fn doeptsiz4(self) -> crate::common::Reg<regs::Doeptsiz4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b90usize) as _) }
    }
    #[doc = "desc DOEPDMA4."]
    #[inline(always)]
    pub const fn doepdma4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b94usize) as _) }
    }
    #[doc = "desc DOEPCTL5."]
    #[inline(always)]
    pub const fn doepctl5(self) -> crate::common::Reg<regs::Doepctl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ba0usize) as _) }
    }
    #[doc = "desc DOEPINT5."]
    #[inline(always)]
    pub const fn doepint5(self) -> crate::common::Reg<regs::Doepint5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ba8usize) as _) }
    }
    #[doc = "desc DOEPTSIZ5."]
    #[inline(always)]
    pub const fn doeptsiz5(self) -> crate::common::Reg<regs::Doeptsiz5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bb0usize) as _) }
    }
    #[doc = "desc DOEPDMA5."]
    #[inline(always)]
    pub const fn doepdma5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bb4usize) as _) }
    }
    #[doc = "desc DOEPCTL6."]
    #[inline(always)]
    pub const fn doepctl6(self) -> crate::common::Reg<regs::Doepctl6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bc0usize) as _) }
    }
    #[doc = "desc DOEPINT6."]
    #[inline(always)]
    pub const fn doepint6(self) -> crate::common::Reg<regs::Doepint6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bc8usize) as _) }
    }
    #[doc = "desc DOEPTSIZ6."]
    #[inline(always)]
    pub const fn doeptsiz6(self) -> crate::common::Reg<regs::Doeptsiz6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bd0usize) as _) }
    }
    #[doc = "desc DOEPDMA6."]
    #[inline(always)]
    pub const fn doepdma6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bd4usize) as _) }
    }
    #[doc = "desc DOEPCTL7."]
    #[inline(always)]
    pub const fn doepctl7(self) -> crate::common::Reg<regs::Doepctl7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0be0usize) as _) }
    }
    #[doc = "desc DOEPINT7."]
    #[inline(always)]
    pub const fn doepint7(self) -> crate::common::Reg<regs::Doepint7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0be8usize) as _) }
    }
    #[doc = "desc DOEPTSIZ7."]
    #[inline(always)]
    pub const fn doeptsiz7(self) -> crate::common::Reg<regs::Doeptsiz7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bf0usize) as _) }
    }
    #[doc = "desc DOEPDMA7."]
    #[inline(always)]
    pub const fn doepdma7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bf4usize) as _) }
    }
    #[doc = "desc DOEPCTL8."]
    #[inline(always)]
    pub const fn doepctl8(self) -> crate::common::Reg<regs::Doepctl8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "desc DOEPINT8."]
    #[inline(always)]
    pub const fn doepint8(self) -> crate::common::Reg<regs::Doepint8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c08usize) as _) }
    }
    #[doc = "desc DOEPTSIZ8."]
    #[inline(always)]
    pub const fn doeptsiz8(self) -> crate::common::Reg<regs::Doeptsiz8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c10usize) as _) }
    }
    #[doc = "desc DOEPDMA8."]
    #[inline(always)]
    pub const fn doepdma8(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c14usize) as _) }
    }
    #[doc = "desc DOEPCTL9."]
    #[inline(always)]
    pub const fn doepctl9(self) -> crate::common::Reg<regs::Doepctl9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c20usize) as _) }
    }
    #[doc = "desc DOEPINT9."]
    #[inline(always)]
    pub const fn doepint9(self) -> crate::common::Reg<regs::Doepint9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c28usize) as _) }
    }
    #[doc = "desc DOEPTSIZ9."]
    #[inline(always)]
    pub const fn doeptsiz9(self) -> crate::common::Reg<regs::Doeptsiz9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c30usize) as _) }
    }
    #[doc = "desc DOEPDMA9."]
    #[inline(always)]
    pub const fn doepdma9(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c34usize) as _) }
    }
    #[doc = "desc DOEPCTL10."]
    #[inline(always)]
    pub const fn doepctl10(self) -> crate::common::Reg<regs::Doepctl10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c40usize) as _) }
    }
    #[doc = "desc DOEPINT10."]
    #[inline(always)]
    pub const fn doepint10(self) -> crate::common::Reg<regs::Doepint10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c48usize) as _) }
    }
    #[doc = "desc DOEPTSIZ10."]
    #[inline(always)]
    pub const fn doeptsiz10(self) -> crate::common::Reg<regs::Doeptsiz10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c50usize) as _) }
    }
    #[doc = "desc DOEPDMA10."]
    #[inline(always)]
    pub const fn doepdma10(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c54usize) as _) }
    }
    #[doc = "desc DOEPCTL11."]
    #[inline(always)]
    pub const fn doepctl11(self) -> crate::common::Reg<regs::Doepctl11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c60usize) as _) }
    }
    #[doc = "desc DOEPINT11."]
    #[inline(always)]
    pub const fn doepint11(self) -> crate::common::Reg<regs::Doepint11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c68usize) as _) }
    }
    #[doc = "desc DOEPTSIZ11."]
    #[inline(always)]
    pub const fn doeptsiz11(self) -> crate::common::Reg<regs::Doeptsiz11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c70usize) as _) }
    }
    #[doc = "desc DOEPDMA11."]
    #[inline(always)]
    pub const fn doepdma11(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c74usize) as _) }
    }
    #[doc = "desc DOEPCTL12."]
    #[inline(always)]
    pub const fn doepctl12(self) -> crate::common::Reg<regs::Doepctl12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c80usize) as _) }
    }
    #[doc = "desc DOEPINT12."]
    #[inline(always)]
    pub const fn doepint12(self) -> crate::common::Reg<regs::Doepint12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c88usize) as _) }
    }
    #[doc = "desc DOEPTSIZ12."]
    #[inline(always)]
    pub const fn doeptsiz12(self) -> crate::common::Reg<regs::Doeptsiz12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c90usize) as _) }
    }
    #[doc = "desc DOEPDMA12."]
    #[inline(always)]
    pub const fn doepdma12(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c94usize) as _) }
    }
    #[doc = "desc DOEPCTL13."]
    #[inline(always)]
    pub const fn doepctl13(self) -> crate::common::Reg<regs::Doepctl13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ca0usize) as _) }
    }
    #[doc = "desc DOEPINT13."]
    #[inline(always)]
    pub const fn doepint13(self) -> crate::common::Reg<regs::Doepint13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ca8usize) as _) }
    }
    #[doc = "desc DOEPTSIZ13."]
    #[inline(always)]
    pub const fn doeptsiz13(self) -> crate::common::Reg<regs::Doeptsiz13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cb0usize) as _) }
    }
    #[doc = "desc DOEPDMA13."]
    #[inline(always)]
    pub const fn doepdma13(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cb4usize) as _) }
    }
    #[doc = "desc DOEPCTL14."]
    #[inline(always)]
    pub const fn doepctl14(self) -> crate::common::Reg<regs::Doepctl14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cc0usize) as _) }
    }
    #[doc = "desc DOEPINT14."]
    #[inline(always)]
    pub const fn doepint14(self) -> crate::common::Reg<regs::Doepint14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cc8usize) as _) }
    }
    #[doc = "desc DOEPTSIZ14."]
    #[inline(always)]
    pub const fn doeptsiz14(self) -> crate::common::Reg<regs::Doeptsiz14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cd0usize) as _) }
    }
    #[doc = "desc DOEPDMA14."]
    #[inline(always)]
    pub const fn doepdma14(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cd4usize) as _) }
    }
    #[doc = "desc DOEPCTL15."]
    #[inline(always)]
    pub const fn doepctl15(self) -> crate::common::Reg<regs::Doepctl15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ce0usize) as _) }
    }
    #[doc = "desc DOEPINT15."]
    #[inline(always)]
    pub const fn doepint15(self) -> crate::common::Reg<regs::Doepint15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ce8usize) as _) }
    }
    #[doc = "desc DOEPTSIZ15."]
    #[inline(always)]
    pub const fn doeptsiz15(self) -> crate::common::Reg<regs::Doeptsiz15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cf0usize) as _) }
    }
    #[doc = "desc DOEPDMA15."]
    #[inline(always)]
    pub const fn doepdma15(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cf4usize) as _) }
    }
    #[doc = "desc GCCTL."]
    #[inline(always)]
    pub const fn gcctl(self) -> crate::common::Reg<regs::Gcctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc DAINT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Daint(pub u32);
    impl Daint {
        #[doc = "desc IEPINT."]
        #[must_use]
        #[inline(always)]
        pub const fn iepint(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc IEPINT."]
        #[inline(always)]
        pub const fn set_iepint(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc OEPINT."]
        #[must_use]
        #[inline(always)]
        pub const fn oepint(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc OEPINT."]
        #[inline(always)]
        pub const fn set_oepint(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Daint {
        #[inline(always)]
        fn default() -> Daint {
            Daint(0)
        }
    }
    impl core::fmt::Debug for Daint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Daint")
                .field("iepint", &self.iepint())
                .field("oepint", &self.oepint())
                .finish()
        }
    }
    #[doc = "desc DAINTMSK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Daintmsk(pub u32);
    impl Daintmsk {
        #[doc = "desc IEPINTM."]
        #[must_use]
        #[inline(always)]
        pub const fn iepintm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc IEPINTM."]
        #[inline(always)]
        pub const fn set_iepintm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc OEPINTM."]
        #[must_use]
        #[inline(always)]
        pub const fn oepintm(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc OEPINTM."]
        #[inline(always)]
        pub const fn set_oepintm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Daintmsk {
        #[inline(always)]
        fn default() -> Daintmsk {
            Daintmsk(0)
        }
    }
    impl core::fmt::Debug for Daintmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Daintmsk")
                .field("iepintm", &self.iepintm())
                .field("oepintm", &self.oepintm())
                .finish()
        }
    }
    #[doc = "desc DCFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcfg(pub u32);
    impl Dcfg {
        #[doc = "desc DSPD."]
        #[must_use]
        #[inline(always)]
        pub const fn dspd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc DSPD."]
        #[inline(always)]
        pub const fn set_dspd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc NZLSOHSK."]
        #[must_use]
        #[inline(always)]
        pub const fn nzlsohsk(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NZLSOHSK."]
        #[inline(always)]
        pub const fn set_nzlsohsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 4usize)) | (((val as u32) & 0x7f) << 4usize);
        }
        #[doc = "desc PFIVL."]
        #[must_use]
        #[inline(always)]
        pub const fn pfivl(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "desc PFIVL."]
        #[inline(always)]
        pub const fn set_pfivl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
    }
    impl Default for Dcfg {
        #[inline(always)]
        fn default() -> Dcfg {
            Dcfg(0)
        }
    }
    impl core::fmt::Debug for Dcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcfg")
                .field("dspd", &self.dspd())
                .field("nzlsohsk", &self.nzlsohsk())
                .field("dad", &self.dad())
                .field("pfivl", &self.pfivl())
                .finish()
        }
    }
    #[doc = "desc DCTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dctl(pub u32);
    impl Dctl {
        #[doc = "desc RWUSIG."]
        #[must_use]
        #[inline(always)]
        pub const fn rwusig(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RWUSIG."]
        #[inline(always)]
        pub const fn set_rwusig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn sdis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SDIS."]
        #[inline(always)]
        pub const fn set_sdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc GINSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn ginsts(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc GINSTS."]
        #[inline(always)]
        pub const fn set_ginsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc GONSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn gonsts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc GONSTS."]
        #[inline(always)]
        pub const fn set_gonsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TCTL."]
        #[must_use]
        #[inline(always)]
        pub const fn tctl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc TCTL."]
        #[inline(always)]
        pub const fn set_tctl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc SGINAK."]
        #[must_use]
        #[inline(always)]
        pub const fn sginak(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SGINAK."]
        #[inline(always)]
        pub const fn set_sginak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CGINAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cginak(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CGINAK."]
        #[inline(always)]
        pub const fn set_cginak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SGONAK."]
        #[must_use]
        #[inline(always)]
        pub const fn sgonak(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc SGONAK."]
        #[inline(always)]
        pub const fn set_sgonak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CGONAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cgonak(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CGONAK."]
        #[inline(always)]
        pub const fn set_cgonak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POPRGDNE."]
        #[must_use]
        #[inline(always)]
        pub const fn poprgdne(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POPRGDNE."]
        #[inline(always)]
        pub const fn set_poprgdne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Dctl {
        #[inline(always)]
        fn default() -> Dctl {
            Dctl(0)
        }
    }
    impl core::fmt::Debug for Dctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dctl")
                .field("rwusig", &self.rwusig())
                .field("sdis", &self.sdis())
                .field("ginsts", &self.ginsts())
                .field("gonsts", &self.gonsts())
                .field("tctl", &self.tctl())
                .field("sginak", &self.sginak())
                .field("cginak", &self.cginak())
                .field("sgonak", &self.sgonak())
                .field("cgonak", &self.cgonak())
                .field("poprgdne", &self.poprgdne())
                .finish()
        }
    }
    #[doc = "desc DEACHINT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Deachint(pub u32);
    impl Deachint {
        #[doc = "desc IEP1INT."]
        #[must_use]
        #[inline(always)]
        pub const fn iep1int(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEP1INT."]
        #[inline(always)]
        pub const fn set_iep1int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc OEP1INT."]
        #[must_use]
        #[inline(always)]
        pub const fn oep1int(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc OEP1INT."]
        #[inline(always)]
        pub const fn set_oep1int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Deachint {
        #[inline(always)]
        fn default() -> Deachint {
            Deachint(0)
        }
    }
    impl core::fmt::Debug for Deachint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Deachint")
                .field("iep1int", &self.iep1int())
                .field("oep1int", &self.oep1int())
                .finish()
        }
    }
    #[doc = "desc DEACHINTMSK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Deachintmsk(pub u32);
    impl Deachintmsk {
        #[doc = "desc IEP1INTM."]
        #[must_use]
        #[inline(always)]
        pub const fn iep1intm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEP1INTM."]
        #[inline(always)]
        pub const fn set_iep1intm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc OEP1INTM."]
        #[must_use]
        #[inline(always)]
        pub const fn oep1intm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc OEP1INTM."]
        #[inline(always)]
        pub const fn set_oep1intm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Deachintmsk {
        #[inline(always)]
        fn default() -> Deachintmsk {
            Deachintmsk(0)
        }
    }
    impl core::fmt::Debug for Deachintmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Deachintmsk")
                .field("iep1intm", &self.iep1intm())
                .field("oep1intm", &self.oep1intm())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl0(pub u32);
    impl Diepctl0 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl0 {
        #[inline(always)]
        fn default() -> Diepctl0 {
            Diepctl0(0)
        }
    }
    impl core::fmt::Debug for Diepctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl0")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl1(pub u32);
    impl Diepctl1 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl1 {
        #[inline(always)]
        fn default() -> Diepctl1 {
            Diepctl1(0)
        }
    }
    impl core::fmt::Debug for Diepctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl1")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl10(pub u32);
    impl Diepctl10 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl10 {
        #[inline(always)]
        fn default() -> Diepctl10 {
            Diepctl10(0)
        }
    }
    impl core::fmt::Debug for Diepctl10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl10")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl11(pub u32);
    impl Diepctl11 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl11 {
        #[inline(always)]
        fn default() -> Diepctl11 {
            Diepctl11(0)
        }
    }
    impl core::fmt::Debug for Diepctl11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl11")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl12(pub u32);
    impl Diepctl12 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl12 {
        #[inline(always)]
        fn default() -> Diepctl12 {
            Diepctl12(0)
        }
    }
    impl core::fmt::Debug for Diepctl12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl12")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl13(pub u32);
    impl Diepctl13 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl13 {
        #[inline(always)]
        fn default() -> Diepctl13 {
            Diepctl13(0)
        }
    }
    impl core::fmt::Debug for Diepctl13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl13")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl14(pub u32);
    impl Diepctl14 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl14 {
        #[inline(always)]
        fn default() -> Diepctl14 {
            Diepctl14(0)
        }
    }
    impl core::fmt::Debug for Diepctl14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl14")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl15(pub u32);
    impl Diepctl15 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl15 {
        #[inline(always)]
        fn default() -> Diepctl15 {
            Diepctl15(0)
        }
    }
    impl core::fmt::Debug for Diepctl15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl15")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl2(pub u32);
    impl Diepctl2 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl2 {
        #[inline(always)]
        fn default() -> Diepctl2 {
            Diepctl2(0)
        }
    }
    impl core::fmt::Debug for Diepctl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl2")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl3(pub u32);
    impl Diepctl3 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl3 {
        #[inline(always)]
        fn default() -> Diepctl3 {
            Diepctl3(0)
        }
    }
    impl core::fmt::Debug for Diepctl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl3")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl4(pub u32);
    impl Diepctl4 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl4 {
        #[inline(always)]
        fn default() -> Diepctl4 {
            Diepctl4(0)
        }
    }
    impl core::fmt::Debug for Diepctl4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl4")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl5(pub u32);
    impl Diepctl5 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl5 {
        #[inline(always)]
        fn default() -> Diepctl5 {
            Diepctl5(0)
        }
    }
    impl core::fmt::Debug for Diepctl5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl5")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl6(pub u32);
    impl Diepctl6 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl6 {
        #[inline(always)]
        fn default() -> Diepctl6 {
            Diepctl6(0)
        }
    }
    impl core::fmt::Debug for Diepctl6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl6")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl7(pub u32);
    impl Diepctl7 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl7 {
        #[inline(always)]
        fn default() -> Diepctl7 {
            Diepctl7(0)
        }
    }
    impl core::fmt::Debug for Diepctl7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl7")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl8(pub u32);
    impl Diepctl8 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl8 {
        #[inline(always)]
        fn default() -> Diepctl8 {
            Diepctl8(0)
        }
    }
    impl core::fmt::Debug for Diepctl8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl8")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPCTL9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl9(pub u32);
    impl Diepctl9 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EONUM_DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EONUM_DPID."]
        #[inline(always)]
        pub const fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID_SEVNFRM."]
        #[inline(always)]
        pub const fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SODDFRM."]
        #[inline(always)]
        pub const fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl9 {
        #[inline(always)]
        fn default() -> Diepctl9 {
            Diepctl9(0)
        }
    }
    impl core::fmt::Debug for Diepctl9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl9")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DIEPEACHMSK1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepeachmsk1(pub u32);
    impl Diepeachmsk1 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDM."]
        #[must_use]
        #[inline(always)]
        pub const fn epdm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDM."]
        #[inline(always)]
        pub const fn set_epdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOM."]
        #[must_use]
        #[inline(always)]
        pub const fn tom(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOM."]
        #[inline(always)]
        pub const fn set_tom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFEMSK."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfemsk(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFEMSK."]
        #[inline(always)]
        pub const fn set_ttxfemsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNMM."]
        #[must_use]
        #[inline(always)]
        pub const fn inepnmm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNMM."]
        #[inline(always)]
        pub const fn set_inepnmm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc INEPNEM."]
        #[must_use]
        #[inline(always)]
        pub const fn inepnem(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNEM."]
        #[inline(always)]
        pub const fn set_inepnem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFURM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfurm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFURM."]
        #[inline(always)]
        pub const fn set_txfurm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepeachmsk1 {
        #[inline(always)]
        fn default() -> Diepeachmsk1 {
            Diepeachmsk1(0)
        }
    }
    impl core::fmt::Debug for Diepeachmsk1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepeachmsk1")
                .field("xfrcm", &self.xfrcm())
                .field("epdm", &self.epdm())
                .field("tom", &self.tom())
                .field("ttxfemsk", &self.ttxfemsk())
                .field("inepnmm", &self.inepnmm())
                .field("inepnem", &self.inepnem())
                .field("txfurm", &self.txfurm())
                .field("nakm", &self.nakm())
                .finish()
        }
    }
    #[doc = "desc DIEPEMPMSK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepempmsk(pub u32);
    impl Diepempmsk {
        #[doc = "desc INEPTXFEM."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfem(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFEM."]
        #[inline(always)]
        pub const fn set_ineptxfem(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Diepempmsk {
        #[inline(always)]
        fn default() -> Diepempmsk {
            Diepempmsk(0)
        }
    }
    impl core::fmt::Debug for Diepempmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepempmsk")
                .field("ineptxfem", &self.ineptxfem())
                .finish()
        }
    }
    #[doc = "desc DIEPINT0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint0(pub u32);
    impl Diepint0 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint0 {
        #[inline(always)]
        fn default() -> Diepint0 {
            Diepint0(0)
        }
    }
    impl core::fmt::Debug for Diepint0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint0")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint1(pub u32);
    impl Diepint1 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint1 {
        #[inline(always)]
        fn default() -> Diepint1 {
            Diepint1(0)
        }
    }
    impl core::fmt::Debug for Diepint1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint1")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint10(pub u32);
    impl Diepint10 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint10 {
        #[inline(always)]
        fn default() -> Diepint10 {
            Diepint10(0)
        }
    }
    impl core::fmt::Debug for Diepint10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint10")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint11(pub u32);
    impl Diepint11 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint11 {
        #[inline(always)]
        fn default() -> Diepint11 {
            Diepint11(0)
        }
    }
    impl core::fmt::Debug for Diepint11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint11")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint12(pub u32);
    impl Diepint12 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint12 {
        #[inline(always)]
        fn default() -> Diepint12 {
            Diepint12(0)
        }
    }
    impl core::fmt::Debug for Diepint12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint12")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint13(pub u32);
    impl Diepint13 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint13 {
        #[inline(always)]
        fn default() -> Diepint13 {
            Diepint13(0)
        }
    }
    impl core::fmt::Debug for Diepint13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint13")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint14(pub u32);
    impl Diepint14 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint14 {
        #[inline(always)]
        fn default() -> Diepint14 {
            Diepint14(0)
        }
    }
    impl core::fmt::Debug for Diepint14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint14")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint15(pub u32);
    impl Diepint15 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint15 {
        #[inline(always)]
        fn default() -> Diepint15 {
            Diepint15(0)
        }
    }
    impl core::fmt::Debug for Diepint15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint15")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint2(pub u32);
    impl Diepint2 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint2 {
        #[inline(always)]
        fn default() -> Diepint2 {
            Diepint2(0)
        }
    }
    impl core::fmt::Debug for Diepint2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint2")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint3(pub u32);
    impl Diepint3 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint3 {
        #[inline(always)]
        fn default() -> Diepint3 {
            Diepint3(0)
        }
    }
    impl core::fmt::Debug for Diepint3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint3")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint4(pub u32);
    impl Diepint4 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint4 {
        #[inline(always)]
        fn default() -> Diepint4 {
            Diepint4(0)
        }
    }
    impl core::fmt::Debug for Diepint4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint4")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint5(pub u32);
    impl Diepint5 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint5 {
        #[inline(always)]
        fn default() -> Diepint5 {
            Diepint5(0)
        }
    }
    impl core::fmt::Debug for Diepint5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint5")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint6(pub u32);
    impl Diepint6 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint6 {
        #[inline(always)]
        fn default() -> Diepint6 {
            Diepint6(0)
        }
    }
    impl core::fmt::Debug for Diepint6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint6")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint7(pub u32);
    impl Diepint7 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint7 {
        #[inline(always)]
        fn default() -> Diepint7 {
            Diepint7(0)
        }
    }
    impl core::fmt::Debug for Diepint7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint7")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint8(pub u32);
    impl Diepint8 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint8 {
        #[inline(always)]
        fn default() -> Diepint8 {
            Diepint8(0)
        }
    }
    impl core::fmt::Debug for Diepint8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint8")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPINT9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint9(pub u32);
    impl Diepint9 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOC."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOC."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFE."]
        #[inline(always)]
        pub const fn set_ttxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNE."]
        #[must_use]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNE."]
        #[inline(always)]
        pub const fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFE."]
        #[inline(always)]
        pub const fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXFIFOUDRN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoudrn(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFOUDRN."]
        #[inline(always)]
        pub const fn set_txfifoudrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PKTDRPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktdrpsts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTDRPSTS."]
        #[inline(always)]
        pub const fn set_pktdrpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BERR."]
        #[must_use]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BERR."]
        #[inline(always)]
        pub const fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepint9 {
        #[inline(always)]
        fn default() -> Diepint9 {
            Diepint9(0)
        }
    }
    impl core::fmt::Debug for Diepint9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint9")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ttxfe", &self.ttxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .field("txfifoudrn", &self.txfifoudrn())
                .field("pktdrpsts", &self.pktdrpsts())
                .field("berr", &self.berr())
                .field("nak", &self.nak())
                .finish()
        }
    }
    #[doc = "desc DIEPMSK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepmsk(pub u32);
    impl Diepmsk {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDM."]
        #[must_use]
        #[inline(always)]
        pub const fn epdm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDM."]
        #[inline(always)]
        pub const fn set_epdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOM."]
        #[must_use]
        #[inline(always)]
        pub const fn tom(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOM."]
        #[inline(always)]
        pub const fn set_tom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TTXFEMSK."]
        #[must_use]
        #[inline(always)]
        pub const fn ttxfemsk(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTXFEMSK."]
        #[inline(always)]
        pub const fn set_ttxfemsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INEPNMM."]
        #[must_use]
        #[inline(always)]
        pub const fn inepnmm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNMM."]
        #[inline(always)]
        pub const fn set_inepnmm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc INEPNEM."]
        #[must_use]
        #[inline(always)]
        pub const fn inepnem(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INEPNEM."]
        #[inline(always)]
        pub const fn set_inepnem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXFURM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfurm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFURM."]
        #[inline(always)]
        pub const fn set_txfurm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Diepmsk {
        #[inline(always)]
        fn default() -> Diepmsk {
            Diepmsk(0)
        }
    }
    impl core::fmt::Debug for Diepmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepmsk")
                .field("xfrcm", &self.xfrcm())
                .field("epdm", &self.epdm())
                .field("tom", &self.tom())
                .field("ttxfemsk", &self.ttxfemsk())
                .field("inepnmm", &self.inepnmm())
                .field("inepnem", &self.inepnem())
                .field("txfurm", &self.txfurm())
                .field("nakm", &self.nakm())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz0(pub u32);
    impl Dieptsiz0 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
    }
    impl Default for Dieptsiz0 {
        #[inline(always)]
        fn default() -> Dieptsiz0 {
            Dieptsiz0(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz0")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz1(pub u32);
    impl Dieptsiz1 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz1 {
        #[inline(always)]
        fn default() -> Dieptsiz1 {
            Dieptsiz1(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz1")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz10(pub u32);
    impl Dieptsiz10 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz10 {
        #[inline(always)]
        fn default() -> Dieptsiz10 {
            Dieptsiz10(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz10")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz11(pub u32);
    impl Dieptsiz11 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz11 {
        #[inline(always)]
        fn default() -> Dieptsiz11 {
            Dieptsiz11(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz11")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz12(pub u32);
    impl Dieptsiz12 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz12 {
        #[inline(always)]
        fn default() -> Dieptsiz12 {
            Dieptsiz12(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz12")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz13(pub u32);
    impl Dieptsiz13 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz13 {
        #[inline(always)]
        fn default() -> Dieptsiz13 {
            Dieptsiz13(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz13")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz14(pub u32);
    impl Dieptsiz14 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz14 {
        #[inline(always)]
        fn default() -> Dieptsiz14 {
            Dieptsiz14(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz14")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz15(pub u32);
    impl Dieptsiz15 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz15 {
        #[inline(always)]
        fn default() -> Dieptsiz15 {
            Dieptsiz15(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz15")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz2(pub u32);
    impl Dieptsiz2 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz2 {
        #[inline(always)]
        fn default() -> Dieptsiz2 {
            Dieptsiz2(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz2")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz3(pub u32);
    impl Dieptsiz3 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz3 {
        #[inline(always)]
        fn default() -> Dieptsiz3 {
            Dieptsiz3(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz3")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz4(pub u32);
    impl Dieptsiz4 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz4 {
        #[inline(always)]
        fn default() -> Dieptsiz4 {
            Dieptsiz4(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz4")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz5(pub u32);
    impl Dieptsiz5 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz5 {
        #[inline(always)]
        fn default() -> Dieptsiz5 {
            Dieptsiz5(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz5")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz6(pub u32);
    impl Dieptsiz6 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz6 {
        #[inline(always)]
        fn default() -> Dieptsiz6 {
            Dieptsiz6(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz6")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz7(pub u32);
    impl Dieptsiz7 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz7 {
        #[inline(always)]
        fn default() -> Dieptsiz7 {
            Dieptsiz7(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz7")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz8(pub u32);
    impl Dieptsiz8 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz8 {
        #[inline(always)]
        fn default() -> Dieptsiz8 {
            Dieptsiz8(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz8")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTSIZ9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz9(pub u32);
    impl Dieptsiz9 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc MCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc MCNT."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz9 {
        #[inline(always)]
        fn default() -> Dieptsiz9 {
            Dieptsiz9(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz9")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf1(pub u32);
    impl Dieptxf1 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf1 {
        #[inline(always)]
        fn default() -> Dieptxf1 {
            Dieptxf1(0)
        }
    }
    impl core::fmt::Debug for Dieptxf1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf1")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf10(pub u32);
    impl Dieptxf10 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf10 {
        #[inline(always)]
        fn default() -> Dieptxf10 {
            Dieptxf10(0)
        }
    }
    impl core::fmt::Debug for Dieptxf10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf10")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf11(pub u32);
    impl Dieptxf11 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf11 {
        #[inline(always)]
        fn default() -> Dieptxf11 {
            Dieptxf11(0)
        }
    }
    impl core::fmt::Debug for Dieptxf11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf11")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf12(pub u32);
    impl Dieptxf12 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf12 {
        #[inline(always)]
        fn default() -> Dieptxf12 {
            Dieptxf12(0)
        }
    }
    impl core::fmt::Debug for Dieptxf12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf12")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf13(pub u32);
    impl Dieptxf13 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf13 {
        #[inline(always)]
        fn default() -> Dieptxf13 {
            Dieptxf13(0)
        }
    }
    impl core::fmt::Debug for Dieptxf13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf13")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf14(pub u32);
    impl Dieptxf14 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf14 {
        #[inline(always)]
        fn default() -> Dieptxf14 {
            Dieptxf14(0)
        }
    }
    impl core::fmt::Debug for Dieptxf14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf14")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf15(pub u32);
    impl Dieptxf15 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf15 {
        #[inline(always)]
        fn default() -> Dieptxf15 {
            Dieptxf15(0)
        }
    }
    impl core::fmt::Debug for Dieptxf15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf15")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf2(pub u32);
    impl Dieptxf2 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf2 {
        #[inline(always)]
        fn default() -> Dieptxf2 {
            Dieptxf2(0)
        }
    }
    impl core::fmt::Debug for Dieptxf2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf2")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf3(pub u32);
    impl Dieptxf3 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf3 {
        #[inline(always)]
        fn default() -> Dieptxf3 {
            Dieptxf3(0)
        }
    }
    impl core::fmt::Debug for Dieptxf3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf3")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf4(pub u32);
    impl Dieptxf4 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf4 {
        #[inline(always)]
        fn default() -> Dieptxf4 {
            Dieptxf4(0)
        }
    }
    impl core::fmt::Debug for Dieptxf4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf4")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf5(pub u32);
    impl Dieptxf5 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf5 {
        #[inline(always)]
        fn default() -> Dieptxf5 {
            Dieptxf5(0)
        }
    }
    impl core::fmt::Debug for Dieptxf5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf5")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf6(pub u32);
    impl Dieptxf6 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf6 {
        #[inline(always)]
        fn default() -> Dieptxf6 {
            Dieptxf6(0)
        }
    }
    impl core::fmt::Debug for Dieptxf6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf6")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf7(pub u32);
    impl Dieptxf7 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf7 {
        #[inline(always)]
        fn default() -> Dieptxf7 {
            Dieptxf7(0)
        }
    }
    impl core::fmt::Debug for Dieptxf7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf7")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf8(pub u32);
    impl Dieptxf8 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf8 {
        #[inline(always)]
        fn default() -> Dieptxf8 {
            Dieptxf8(0)
        }
    }
    impl core::fmt::Debug for Dieptxf8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf8")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DIEPTXF9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptxf9(pub u32);
    impl Dieptxf9 {
        #[doc = "desc INEPTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXSA."]
        #[inline(always)]
        pub const fn set_ineptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc INEPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTXFD."]
        #[inline(always)]
        pub const fn set_ineptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dieptxf9 {
        #[inline(always)]
        fn default() -> Dieptxf9 {
            Dieptxf9(0)
        }
    }
    impl core::fmt::Debug for Dieptxf9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptxf9")
                .field("ineptxsa", &self.ineptxsa())
                .field("ineptxfd", &self.ineptxfd())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl0(pub u32);
    impl Doepctl0 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl0 {
        #[inline(always)]
        fn default() -> Doepctl0 {
            Doepctl0(0)
        }
    }
    impl core::fmt::Debug for Doepctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl0")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl1(pub u32);
    impl Doepctl1 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl1 {
        #[inline(always)]
        fn default() -> Doepctl1 {
            Doepctl1(0)
        }
    }
    impl core::fmt::Debug for Doepctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl1")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl10(pub u32);
    impl Doepctl10 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl10 {
        #[inline(always)]
        fn default() -> Doepctl10 {
            Doepctl10(0)
        }
    }
    impl core::fmt::Debug for Doepctl10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl10")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl11(pub u32);
    impl Doepctl11 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl11 {
        #[inline(always)]
        fn default() -> Doepctl11 {
            Doepctl11(0)
        }
    }
    impl core::fmt::Debug for Doepctl11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl11")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl12(pub u32);
    impl Doepctl12 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl12 {
        #[inline(always)]
        fn default() -> Doepctl12 {
            Doepctl12(0)
        }
    }
    impl core::fmt::Debug for Doepctl12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl12")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl13(pub u32);
    impl Doepctl13 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl13 {
        #[inline(always)]
        fn default() -> Doepctl13 {
            Doepctl13(0)
        }
    }
    impl core::fmt::Debug for Doepctl13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl13")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl14(pub u32);
    impl Doepctl14 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl14 {
        #[inline(always)]
        fn default() -> Doepctl14 {
            Doepctl14(0)
        }
    }
    impl core::fmt::Debug for Doepctl14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl14")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl15(pub u32);
    impl Doepctl15 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl15 {
        #[inline(always)]
        fn default() -> Doepctl15 {
            Doepctl15(0)
        }
    }
    impl core::fmt::Debug for Doepctl15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl15")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl2(pub u32);
    impl Doepctl2 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl2 {
        #[inline(always)]
        fn default() -> Doepctl2 {
            Doepctl2(0)
        }
    }
    impl core::fmt::Debug for Doepctl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl2")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl3(pub u32);
    impl Doepctl3 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl3 {
        #[inline(always)]
        fn default() -> Doepctl3 {
            Doepctl3(0)
        }
    }
    impl core::fmt::Debug for Doepctl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl3")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl4(pub u32);
    impl Doepctl4 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl4 {
        #[inline(always)]
        fn default() -> Doepctl4 {
            Doepctl4(0)
        }
    }
    impl core::fmt::Debug for Doepctl4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl4")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl5(pub u32);
    impl Doepctl5 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl5 {
        #[inline(always)]
        fn default() -> Doepctl5 {
            Doepctl5(0)
        }
    }
    impl core::fmt::Debug for Doepctl5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl5")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl6(pub u32);
    impl Doepctl6 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl6 {
        #[inline(always)]
        fn default() -> Doepctl6 {
            Doepctl6(0)
        }
    }
    impl core::fmt::Debug for Doepctl6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl6")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl7(pub u32);
    impl Doepctl7 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl7 {
        #[inline(always)]
        fn default() -> Doepctl7 {
            Doepctl7(0)
        }
    }
    impl core::fmt::Debug for Doepctl7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl7")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl8(pub u32);
    impl Doepctl8 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl8 {
        #[inline(always)]
        fn default() -> Doepctl8 {
            Doepctl8(0)
        }
    }
    impl core::fmt::Debug for Doepctl8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl8")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPCTL9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl9(pub u32);
    impl Doepctl9 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc USBAEP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBAEP."]
        #[inline(always)]
        pub const fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc NAKSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKSTS."]
        #[inline(always)]
        pub const fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc SNPM."]
        #[must_use]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNPM."]
        #[inline(always)]
        pub const fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNAK."]
        #[inline(always)]
        pub const fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SNAK."]
        #[must_use]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNAK."]
        #[inline(always)]
        pub const fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SD0PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd0pid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD0PID."]
        #[inline(always)]
        pub const fn set_sd0pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SD1PID."]
        #[must_use]
        #[inline(always)]
        pub const fn sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SD1PID."]
        #[inline(always)]
        pub const fn set_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIS."]
        #[inline(always)]
        pub const fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EPENA."]
        #[must_use]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPENA."]
        #[inline(always)]
        pub const fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl9 {
        #[inline(always)]
        fn default() -> Doepctl9 {
            Doepctl9(0)
        }
    }
    impl core::fmt::Debug for Doepctl9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl9")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("dpid", &self.dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid", &self.sd0pid())
                .field("sd1pid", &self.sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[doc = "desc DOEPEACHMSK1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepeachmsk1(pub u32);
    impl Doepeachmsk1 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDM."]
        #[must_use]
        #[inline(always)]
        pub const fn epdm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDM."]
        #[inline(always)]
        pub const fn set_epdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUPM."]
        #[must_use]
        #[inline(always)]
        pub const fn stupm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUPM."]
        #[inline(always)]
        pub const fn set_stupm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDM."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDM."]
        #[inline(always)]
        pub const fn set_otepdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc OPEM."]
        #[must_use]
        #[inline(always)]
        pub const fn opem(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc OPEM."]
        #[inline(always)]
        pub const fn set_opem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepeachmsk1 {
        #[inline(always)]
        fn default() -> Doepeachmsk1 {
            Doepeachmsk1(0)
        }
    }
    impl core::fmt::Debug for Doepeachmsk1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepeachmsk1")
                .field("xfrcm", &self.xfrcm())
                .field("epdm", &self.epdm())
                .field("stupm", &self.stupm())
                .field("otepdm", &self.otepdm())
                .field("b2bstup", &self.b2bstup())
                .field("opem", &self.opem())
                .field("nyetm", &self.nyetm())
                .finish()
        }
    }
    #[doc = "desc DOEPINT0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint0(pub u32);
    impl Doepint0 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint0 {
        #[inline(always)]
        fn default() -> Doepint0 {
            Doepint0(0)
        }
    }
    impl core::fmt::Debug for Doepint0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint0")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint1(pub u32);
    impl Doepint1 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint1 {
        #[inline(always)]
        fn default() -> Doepint1 {
            Doepint1(0)
        }
    }
    impl core::fmt::Debug for Doepint1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint1")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint10(pub u32);
    impl Doepint10 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint10 {
        #[inline(always)]
        fn default() -> Doepint10 {
            Doepint10(0)
        }
    }
    impl core::fmt::Debug for Doepint10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint10")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint11(pub u32);
    impl Doepint11 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint11 {
        #[inline(always)]
        fn default() -> Doepint11 {
            Doepint11(0)
        }
    }
    impl core::fmt::Debug for Doepint11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint11")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint12(pub u32);
    impl Doepint12 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint12 {
        #[inline(always)]
        fn default() -> Doepint12 {
            Doepint12(0)
        }
    }
    impl core::fmt::Debug for Doepint12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint12")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint13(pub u32);
    impl Doepint13 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint13 {
        #[inline(always)]
        fn default() -> Doepint13 {
            Doepint13(0)
        }
    }
    impl core::fmt::Debug for Doepint13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint13")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint14(pub u32);
    impl Doepint14 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint14 {
        #[inline(always)]
        fn default() -> Doepint14 {
            Doepint14(0)
        }
    }
    impl core::fmt::Debug for Doepint14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint14")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint15(pub u32);
    impl Doepint15 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint15 {
        #[inline(always)]
        fn default() -> Doepint15 {
            Doepint15(0)
        }
    }
    impl core::fmt::Debug for Doepint15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint15")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint2(pub u32);
    impl Doepint2 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint2 {
        #[inline(always)]
        fn default() -> Doepint2 {
            Doepint2(0)
        }
    }
    impl core::fmt::Debug for Doepint2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint2")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint3(pub u32);
    impl Doepint3 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint3 {
        #[inline(always)]
        fn default() -> Doepint3 {
            Doepint3(0)
        }
    }
    impl core::fmt::Debug for Doepint3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint3")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint4(pub u32);
    impl Doepint4 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint4 {
        #[inline(always)]
        fn default() -> Doepint4 {
            Doepint4(0)
        }
    }
    impl core::fmt::Debug for Doepint4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint4")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint5(pub u32);
    impl Doepint5 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint5 {
        #[inline(always)]
        fn default() -> Doepint5 {
            Doepint5(0)
        }
    }
    impl core::fmt::Debug for Doepint5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint5")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint6(pub u32);
    impl Doepint6 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint6 {
        #[inline(always)]
        fn default() -> Doepint6 {
            Doepint6(0)
        }
    }
    impl core::fmt::Debug for Doepint6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint6")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint7(pub u32);
    impl Doepint7 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint7 {
        #[inline(always)]
        fn default() -> Doepint7 {
            Doepint7(0)
        }
    }
    impl core::fmt::Debug for Doepint7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint7")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint8(pub u32);
    impl Doepint8 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint8 {
        #[inline(always)]
        fn default() -> Doepint8 {
            Doepint8(0)
        }
    }
    impl core::fmt::Debug for Doepint8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint8")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPINT9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint9(pub u32);
    impl Doepint9 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDISD."]
        #[must_use]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDISD."]
        #[inline(always)]
        pub const fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUP."]
        #[must_use]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUP."]
        #[inline(always)]
        pub const fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDIS."]
        #[inline(always)]
        pub const fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepint9 {
        #[inline(always)]
        fn default() -> Doepint9 {
            Doepint9(0)
        }
    }
    impl core::fmt::Debug for Doepint9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint9")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .field("nyet", &self.nyet())
                .finish()
        }
    }
    #[doc = "desc DOEPMSK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepmsk(pub u32);
    impl Doepmsk {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EPDM."]
        #[must_use]
        #[inline(always)]
        pub const fn epdm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDM."]
        #[inline(always)]
        pub const fn set_epdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc STUPM."]
        #[must_use]
        #[inline(always)]
        pub const fn stupm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STUPM."]
        #[inline(always)]
        pub const fn set_stupm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OTEPDM."]
        #[must_use]
        #[inline(always)]
        pub const fn otepdm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTEPDM."]
        #[inline(always)]
        pub const fn set_otepdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc B2BSTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2BSTUP."]
        #[inline(always)]
        pub const fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc OPEM."]
        #[must_use]
        #[inline(always)]
        pub const fn opem(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc OPEM."]
        #[inline(always)]
        pub const fn set_opem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Doepmsk {
        #[inline(always)]
        fn default() -> Doepmsk {
            Doepmsk(0)
        }
    }
    impl core::fmt::Debug for Doepmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepmsk")
                .field("xfrcm", &self.xfrcm())
                .field("epdm", &self.epdm())
                .field("stupm", &self.stupm())
                .field("otepdm", &self.otepdm())
                .field("b2bstup", &self.b2bstup())
                .field("opem", &self.opem())
                .field("nyetm", &self.nyetm())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz0(pub u32);
    impl Doeptsiz0 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc STUPCNT."]
        #[inline(always)]
        pub const fn set_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz0 {
        #[inline(always)]
        fn default() -> Doeptsiz0 {
            Doeptsiz0(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz0")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("stupcnt", &self.stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz1(pub u32);
    impl Doeptsiz1 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz1 {
        #[inline(always)]
        fn default() -> Doeptsiz1 {
            Doeptsiz1(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz1")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz10(pub u32);
    impl Doeptsiz10 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz10 {
        #[inline(always)]
        fn default() -> Doeptsiz10 {
            Doeptsiz10(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz10")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz11(pub u32);
    impl Doeptsiz11 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz11 {
        #[inline(always)]
        fn default() -> Doeptsiz11 {
            Doeptsiz11(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz11")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz12(pub u32);
    impl Doeptsiz12 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz12 {
        #[inline(always)]
        fn default() -> Doeptsiz12 {
            Doeptsiz12(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz12")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz13(pub u32);
    impl Doeptsiz13 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz13 {
        #[inline(always)]
        fn default() -> Doeptsiz13 {
            Doeptsiz13(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz13")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz14(pub u32);
    impl Doeptsiz14 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz14 {
        #[inline(always)]
        fn default() -> Doeptsiz14 {
            Doeptsiz14(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz14")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz15(pub u32);
    impl Doeptsiz15 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz15 {
        #[inline(always)]
        fn default() -> Doeptsiz15 {
            Doeptsiz15(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz15")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz2(pub u32);
    impl Doeptsiz2 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz2 {
        #[inline(always)]
        fn default() -> Doeptsiz2 {
            Doeptsiz2(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz2")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz3(pub u32);
    impl Doeptsiz3 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz3 {
        #[inline(always)]
        fn default() -> Doeptsiz3 {
            Doeptsiz3(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz3")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz4(pub u32);
    impl Doeptsiz4 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz4 {
        #[inline(always)]
        fn default() -> Doeptsiz4 {
            Doeptsiz4(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz4")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz5(pub u32);
    impl Doeptsiz5 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz5 {
        #[inline(always)]
        fn default() -> Doeptsiz5 {
            Doeptsiz5(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz5")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz6(pub u32);
    impl Doeptsiz6 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz6 {
        #[inline(always)]
        fn default() -> Doeptsiz6 {
            Doeptsiz6(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz6")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz7(pub u32);
    impl Doeptsiz7 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz7 {
        #[inline(always)]
        fn default() -> Doeptsiz7 {
            Doeptsiz7(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz7")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz8(pub u32);
    impl Doeptsiz8 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz8 {
        #[inline(always)]
        fn default() -> Doeptsiz8 {
            Doeptsiz8(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz8")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DOEPTSIZ9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz9(pub u32);
    impl Doeptsiz9 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXDPID_STUPCNT."]
        #[inline(always)]
        pub const fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz9 {
        #[inline(always)]
        fn default() -> Doeptsiz9 {
            Doeptsiz9(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz9")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[doc = "desc DSTS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dsts(pub u32);
    impl Dsts {
        #[doc = "desc SUSPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn suspsts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SUSPSTS."]
        #[inline(always)]
        pub const fn set_suspsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ENUMSPD."]
        #[must_use]
        #[inline(always)]
        pub const fn enumspd(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "desc ENUMSPD."]
        #[inline(always)]
        pub const fn set_enumspd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "desc EERR."]
        #[must_use]
        #[inline(always)]
        pub const fn eerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc EERR."]
        #[inline(always)]
        pub const fn set_eerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc FNSOF."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsof(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x3fff;
            val as u16
        }
        #[doc = "desc FNSOF."]
        #[inline(always)]
        pub const fn set_fnsof(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 8usize)) | (((val as u32) & 0x3fff) << 8usize);
        }
        #[doc = "desc LNSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn lnsts(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc LNSTS."]
        #[inline(always)]
        pub const fn set_lnsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
    }
    impl Default for Dsts {
        #[inline(always)]
        fn default() -> Dsts {
            Dsts(0)
        }
    }
    impl core::fmt::Debug for Dsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dsts")
                .field("suspsts", &self.suspsts())
                .field("enumspd", &self.enumspd())
                .field("eerr", &self.eerr())
                .field("fnsof", &self.fnsof())
                .field("lnsts", &self.lnsts())
                .finish()
        }
    }
    #[doc = "desc DTHRCTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dthrctl(pub u32);
    impl Dthrctl {
        #[doc = "desc NONISOTHREN."]
        #[must_use]
        #[inline(always)]
        pub const fn nonisothren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc NONISOTHREN."]
        #[inline(always)]
        pub const fn set_nonisothren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ISOTHREN."]
        #[must_use]
        #[inline(always)]
        pub const fn isothren(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ISOTHREN."]
        #[inline(always)]
        pub const fn set_isothren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TXTHRLEN."]
        #[must_use]
        #[inline(always)]
        pub const fn txthrlen(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TXTHRLEN."]
        #[inline(always)]
        pub const fn set_txthrlen(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 2usize)) | (((val as u32) & 0x01ff) << 2usize);
        }
        #[doc = "desc RXTHREN."]
        #[must_use]
        #[inline(always)]
        pub const fn rxthren(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXTHREN."]
        #[inline(always)]
        pub const fn set_rxthren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc RXTHRLEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rxthrlen(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc RXTHRLEN."]
        #[inline(always)]
        pub const fn set_rxthrlen(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 17usize)) | (((val as u32) & 0x01ff) << 17usize);
        }
        #[doc = "desc ARPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn arpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc ARPEN."]
        #[inline(always)]
        pub const fn set_arpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Dthrctl {
        #[inline(always)]
        fn default() -> Dthrctl {
            Dthrctl(0)
        }
    }
    impl core::fmt::Debug for Dthrctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dthrctl")
                .field("nonisothren", &self.nonisothren())
                .field("isothren", &self.isothren())
                .field("txthrlen", &self.txthrlen())
                .field("rxthren", &self.rxthren())
                .field("rxthrlen", &self.rxthrlen())
                .field("arpen", &self.arpen())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts0(pub u32);
    impl Dtxfsts0 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts0 {
        #[inline(always)]
        fn default() -> Dtxfsts0 {
            Dtxfsts0(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts0")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts1(pub u32);
    impl Dtxfsts1 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts1 {
        #[inline(always)]
        fn default() -> Dtxfsts1 {
            Dtxfsts1(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts1")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts10(pub u32);
    impl Dtxfsts10 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts10 {
        #[inline(always)]
        fn default() -> Dtxfsts10 {
            Dtxfsts10(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts10")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts11(pub u32);
    impl Dtxfsts11 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts11 {
        #[inline(always)]
        fn default() -> Dtxfsts11 {
            Dtxfsts11(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts11")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts12(pub u32);
    impl Dtxfsts12 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts12 {
        #[inline(always)]
        fn default() -> Dtxfsts12 {
            Dtxfsts12(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts12")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts13(pub u32);
    impl Dtxfsts13 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts13 {
        #[inline(always)]
        fn default() -> Dtxfsts13 {
            Dtxfsts13(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts13")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts14(pub u32);
    impl Dtxfsts14 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts14 {
        #[inline(always)]
        fn default() -> Dtxfsts14 {
            Dtxfsts14(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts14")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts15(pub u32);
    impl Dtxfsts15 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts15 {
        #[inline(always)]
        fn default() -> Dtxfsts15 {
            Dtxfsts15(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts15")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts2(pub u32);
    impl Dtxfsts2 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts2 {
        #[inline(always)]
        fn default() -> Dtxfsts2 {
            Dtxfsts2(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts2")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts3(pub u32);
    impl Dtxfsts3 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts3 {
        #[inline(always)]
        fn default() -> Dtxfsts3 {
            Dtxfsts3(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts3")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts4(pub u32);
    impl Dtxfsts4 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts4 {
        #[inline(always)]
        fn default() -> Dtxfsts4 {
            Dtxfsts4(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts4")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts5(pub u32);
    impl Dtxfsts5 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts5 {
        #[inline(always)]
        fn default() -> Dtxfsts5 {
            Dtxfsts5(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts5")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts6(pub u32);
    impl Dtxfsts6 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts6 {
        #[inline(always)]
        fn default() -> Dtxfsts6 {
            Dtxfsts6(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts6")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts7(pub u32);
    impl Dtxfsts7 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts7 {
        #[inline(always)]
        fn default() -> Dtxfsts7 {
            Dtxfsts7(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts7")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts8(pub u32);
    impl Dtxfsts8 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts8 {
        #[inline(always)]
        fn default() -> Dtxfsts8 {
            Dtxfsts8(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts8")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc DTXFSTS9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts9(pub u32);
    impl Dtxfsts9 {
        #[doc = "desc INEPTFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc INEPTFSAV."]
        #[inline(always)]
        pub const fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts9 {
        #[inline(always)]
        fn default() -> Dtxfsts9 {
            Dtxfsts9(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts9")
                .field("ineptfsav", &self.ineptfsav())
                .finish()
        }
    }
    #[doc = "desc GAHBCFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gahbcfg(pub u32);
    impl Gahbcfg {
        #[doc = "desc GINTMSK."]
        #[must_use]
        #[inline(always)]
        pub const fn gintmsk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc GINTMSK."]
        #[inline(always)]
        pub const fn set_gintmsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc HBSTLEN."]
        #[must_use]
        #[inline(always)]
        pub const fn hbstlen(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x0f;
            val as u8
        }
        #[doc = "desc HBSTLEN."]
        #[inline(always)]
        pub const fn set_hbstlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
        }
        #[doc = "desc DMAEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAEN."]
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TXFELVL."]
        #[must_use]
        #[inline(always)]
        pub const fn txfelvl(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFELVL."]
        #[inline(always)]
        pub const fn set_txfelvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PTXFELVL."]
        #[must_use]
        #[inline(always)]
        pub const fn ptxfelvl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PTXFELVL."]
        #[inline(always)]
        pub const fn set_ptxfelvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Gahbcfg {
        #[inline(always)]
        fn default() -> Gahbcfg {
            Gahbcfg(0)
        }
    }
    impl core::fmt::Debug for Gahbcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gahbcfg")
                .field("gintmsk", &self.gintmsk())
                .field("hbstlen", &self.hbstlen())
                .field("dmaen", &self.dmaen())
                .field("txfelvl", &self.txfelvl())
                .field("ptxfelvl", &self.ptxfelvl())
                .finish()
        }
    }
    #[doc = "desc GCCTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcctl(pub u32);
    impl Gcctl {
        #[doc = "desc STPPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn stppclk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc STPPCLK."]
        #[inline(always)]
        pub const fn set_stppclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc GATEHCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn gatehclk(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc GATEHCLK."]
        #[inline(always)]
        pub const fn set_gatehclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ENL1GTG."]
        #[must_use]
        #[inline(always)]
        pub const fn enl1gtg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENL1GTG."]
        #[inline(always)]
        pub const fn set_enl1gtg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PHYSLEEP."]
        #[must_use]
        #[inline(always)]
        pub const fn physleep(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PHYSLEEP."]
        #[inline(always)]
        pub const fn set_physleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SUSP."]
        #[must_use]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SUSP."]
        #[inline(always)]
        pub const fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Gcctl {
        #[inline(always)]
        fn default() -> Gcctl {
            Gcctl(0)
        }
    }
    impl core::fmt::Debug for Gcctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcctl")
                .field("stppclk", &self.stppclk())
                .field("gatehclk", &self.gatehclk())
                .field("enl1gtg", &self.enl1gtg())
                .field("physleep", &self.physleep())
                .field("susp", &self.susp())
                .finish()
        }
    }
    #[doc = "desc GINTMSK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gintmsk(pub u32);
    impl Gintmsk {
        #[doc = "desc MMISM."]
        #[must_use]
        #[inline(always)]
        pub const fn mmism(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc MMISM."]
        #[inline(always)]
        pub const fn set_mmism(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SOFM."]
        #[must_use]
        #[inline(always)]
        pub const fn sofm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SOFM."]
        #[inline(always)]
        pub const fn set_sofm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RXFNEM."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfnem(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXFNEM."]
        #[inline(always)]
        pub const fn set_rxfnem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc NPTXFEM."]
        #[must_use]
        #[inline(always)]
        pub const fn nptxfem(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc NPTXFEM."]
        #[inline(always)]
        pub const fn set_nptxfem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc GINAKEFFM."]
        #[must_use]
        #[inline(always)]
        pub const fn ginakeffm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc GINAKEFFM."]
        #[inline(always)]
        pub const fn set_ginakeffm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc GONAKEFFM."]
        #[must_use]
        #[inline(always)]
        pub const fn gonakeffm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc GONAKEFFM."]
        #[inline(always)]
        pub const fn set_gonakeffm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc ESUSPM."]
        #[must_use]
        #[inline(always)]
        pub const fn esuspm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc ESUSPM."]
        #[inline(always)]
        pub const fn set_esuspm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc USBSUSPM."]
        #[must_use]
        #[inline(always)]
        pub const fn usbsuspm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBSUSPM."]
        #[inline(always)]
        pub const fn set_usbsuspm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc USBRSTM."]
        #[must_use]
        #[inline(always)]
        pub const fn usbrstm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBRSTM."]
        #[inline(always)]
        pub const fn set_usbrstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ENUMDNEM."]
        #[must_use]
        #[inline(always)]
        pub const fn enumdnem(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENUMDNEM."]
        #[inline(always)]
        pub const fn set_enumdnem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc ISOODRPM."]
        #[must_use]
        #[inline(always)]
        pub const fn isoodrpm(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc ISOODRPM."]
        #[inline(always)]
        pub const fn set_isoodrpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc EOPFM."]
        #[must_use]
        #[inline(always)]
        pub const fn eopfm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOPFM."]
        #[inline(always)]
        pub const fn set_eopfm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc IEPIM."]
        #[must_use]
        #[inline(always)]
        pub const fn iepim(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEPIM."]
        #[inline(always)]
        pub const fn set_iepim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc OEPIM."]
        #[must_use]
        #[inline(always)]
        pub const fn oepim(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc OEPIM."]
        #[inline(always)]
        pub const fn set_oepim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc IISOIXFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn iisoixfrm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc IISOIXFRM."]
        #[inline(always)]
        pub const fn set_iisoixfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc IPXFRM_INCOMPISOOUTM."]
        #[must_use]
        #[inline(always)]
        pub const fn ipxfrm_incompisooutm(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc IPXFRM_INCOMPISOOUTM."]
        #[inline(always)]
        pub const fn set_ipxfrm_incompisooutm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc DATAFSUSPM."]
        #[must_use]
        #[inline(always)]
        pub const fn datafsuspm(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc DATAFSUSPM."]
        #[inline(always)]
        pub const fn set_datafsuspm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc HPRTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn hprtim(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc HPRTIM."]
        #[inline(always)]
        pub const fn set_hprtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc HCIM."]
        #[must_use]
        #[inline(always)]
        pub const fn hcim(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCIM."]
        #[inline(always)]
        pub const fn set_hcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc PTXFEM."]
        #[must_use]
        #[inline(always)]
        pub const fn ptxfem(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc PTXFEM."]
        #[inline(always)]
        pub const fn set_ptxfem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc LPMINTM."]
        #[must_use]
        #[inline(always)]
        pub const fn lpmintm(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPMINTM."]
        #[inline(always)]
        pub const fn set_lpmintm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc CIDSCHGM."]
        #[must_use]
        #[inline(always)]
        pub const fn cidschgm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIDSCHGM."]
        #[inline(always)]
        pub const fn set_cidschgm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc DISCIM."]
        #[must_use]
        #[inline(always)]
        pub const fn discim(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc DISCIM."]
        #[inline(always)]
        pub const fn set_discim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VBUSVIM."]
        #[must_use]
        #[inline(always)]
        pub const fn vbusvim(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VBUSVIM."]
        #[inline(always)]
        pub const fn set_vbusvim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc WKUIM."]
        #[must_use]
        #[inline(always)]
        pub const fn wkuim(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKUIM."]
        #[inline(always)]
        pub const fn set_wkuim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gintmsk {
        #[inline(always)]
        fn default() -> Gintmsk {
            Gintmsk(0)
        }
    }
    impl core::fmt::Debug for Gintmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gintmsk")
                .field("mmism", &self.mmism())
                .field("sofm", &self.sofm())
                .field("rxfnem", &self.rxfnem())
                .field("nptxfem", &self.nptxfem())
                .field("ginakeffm", &self.ginakeffm())
                .field("gonakeffm", &self.gonakeffm())
                .field("esuspm", &self.esuspm())
                .field("usbsuspm", &self.usbsuspm())
                .field("usbrstm", &self.usbrstm())
                .field("enumdnem", &self.enumdnem())
                .field("isoodrpm", &self.isoodrpm())
                .field("eopfm", &self.eopfm())
                .field("iepim", &self.iepim())
                .field("oepim", &self.oepim())
                .field("iisoixfrm", &self.iisoixfrm())
                .field("ipxfrm_incompisooutm", &self.ipxfrm_incompisooutm())
                .field("datafsuspm", &self.datafsuspm())
                .field("hprtim", &self.hprtim())
                .field("hcim", &self.hcim())
                .field("ptxfem", &self.ptxfem())
                .field("lpmintm", &self.lpmintm())
                .field("cidschgm", &self.cidschgm())
                .field("discim", &self.discim())
                .field("vbusvim", &self.vbusvim())
                .field("wkuim", &self.wkuim())
                .finish()
        }
    }
    #[doc = "desc GINTSTS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gintsts(pub u32);
    impl Gintsts {
        #[doc = "desc CMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn cmod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMOD."]
        #[inline(always)]
        pub const fn set_cmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc MMIS."]
        #[must_use]
        #[inline(always)]
        pub const fn mmis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc MMIS."]
        #[inline(always)]
        pub const fn set_mmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SOF."]
        #[must_use]
        #[inline(always)]
        pub const fn sof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SOF."]
        #[inline(always)]
        pub const fn set_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RXFNE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfne(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXFNE."]
        #[inline(always)]
        pub const fn set_rxfne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc NPTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn nptxfe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc NPTXFE."]
        #[inline(always)]
        pub const fn set_nptxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc GINAKEFF."]
        #[must_use]
        #[inline(always)]
        pub const fn ginakeff(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc GINAKEFF."]
        #[inline(always)]
        pub const fn set_ginakeff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc GONAKEFF."]
        #[must_use]
        #[inline(always)]
        pub const fn gonakeff(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc GONAKEFF."]
        #[inline(always)]
        pub const fn set_gonakeff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc ESUSP."]
        #[must_use]
        #[inline(always)]
        pub const fn esusp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc ESUSP."]
        #[inline(always)]
        pub const fn set_esusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc USBSUSP."]
        #[must_use]
        #[inline(always)]
        pub const fn usbsusp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBSUSP."]
        #[inline(always)]
        pub const fn set_usbsusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc USBRST."]
        #[must_use]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBRST."]
        #[inline(always)]
        pub const fn set_usbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ENUMDNE."]
        #[must_use]
        #[inline(always)]
        pub const fn enumdne(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENUMDNE."]
        #[inline(always)]
        pub const fn set_enumdne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc ISOODRP."]
        #[must_use]
        #[inline(always)]
        pub const fn isoodrp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc ISOODRP."]
        #[inline(always)]
        pub const fn set_isoodrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc EOPF."]
        #[must_use]
        #[inline(always)]
        pub const fn eopf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOPF."]
        #[inline(always)]
        pub const fn set_eopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc IEPINT."]
        #[must_use]
        #[inline(always)]
        pub const fn iepint(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEPINT."]
        #[inline(always)]
        pub const fn set_iepint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc OEPINT."]
        #[must_use]
        #[inline(always)]
        pub const fn oepint(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc OEPINT."]
        #[inline(always)]
        pub const fn set_oepint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc IISOIXFR."]
        #[must_use]
        #[inline(always)]
        pub const fn iisoixfr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc IISOIXFR."]
        #[inline(always)]
        pub const fn set_iisoixfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc IPXFR_INCOMPISOOUT."]
        #[must_use]
        #[inline(always)]
        pub const fn ipxfr_incompisoout(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc IPXFR_INCOMPISOOUT."]
        #[inline(always)]
        pub const fn set_ipxfr_incompisoout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc DATAFSUSP."]
        #[must_use]
        #[inline(always)]
        pub const fn datafsusp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc DATAFSUSP."]
        #[inline(always)]
        pub const fn set_datafsusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc HPRTINT."]
        #[must_use]
        #[inline(always)]
        pub const fn hprtint(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc HPRTINT."]
        #[inline(always)]
        pub const fn set_hprtint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc HCINT."]
        #[must_use]
        #[inline(always)]
        pub const fn hcint(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCINT."]
        #[inline(always)]
        pub const fn set_hcint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc PTXFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ptxfe(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc PTXFE."]
        #[inline(always)]
        pub const fn set_ptxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc LPMINT."]
        #[must_use]
        #[inline(always)]
        pub const fn lpmint(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPMINT."]
        #[inline(always)]
        pub const fn set_lpmint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc CIDSCHG."]
        #[must_use]
        #[inline(always)]
        pub const fn cidschg(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIDSCHG."]
        #[inline(always)]
        pub const fn set_cidschg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc DISCINT."]
        #[must_use]
        #[inline(always)]
        pub const fn discint(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc DISCINT."]
        #[inline(always)]
        pub const fn set_discint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VBUSVINT."]
        #[must_use]
        #[inline(always)]
        pub const fn vbusvint(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VBUSVINT."]
        #[inline(always)]
        pub const fn set_vbusvint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc WKUINT."]
        #[must_use]
        #[inline(always)]
        pub const fn wkuint(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKUINT."]
        #[inline(always)]
        pub const fn set_wkuint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gintsts {
        #[inline(always)]
        fn default() -> Gintsts {
            Gintsts(0)
        }
    }
    impl core::fmt::Debug for Gintsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gintsts")
                .field("cmod", &self.cmod())
                .field("mmis", &self.mmis())
                .field("sof", &self.sof())
                .field("rxfne", &self.rxfne())
                .field("nptxfe", &self.nptxfe())
                .field("ginakeff", &self.ginakeff())
                .field("gonakeff", &self.gonakeff())
                .field("esusp", &self.esusp())
                .field("usbsusp", &self.usbsusp())
                .field("usbrst", &self.usbrst())
                .field("enumdne", &self.enumdne())
                .field("isoodrp", &self.isoodrp())
                .field("eopf", &self.eopf())
                .field("iepint", &self.iepint())
                .field("oepint", &self.oepint())
                .field("iisoixfr", &self.iisoixfr())
                .field("ipxfr_incompisoout", &self.ipxfr_incompisoout())
                .field("datafsusp", &self.datafsusp())
                .field("hprtint", &self.hprtint())
                .field("hcint", &self.hcint())
                .field("ptxfe", &self.ptxfe())
                .field("lpmint", &self.lpmint())
                .field("cidschg", &self.cidschg())
                .field("discint", &self.discint())
                .field("vbusvint", &self.vbusvint())
                .field("wkuint", &self.wkuint())
                .finish()
        }
    }
    #[doc = "desc GLPMCFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Glpmcfg(pub u32);
    impl Glpmcfg {
        #[doc = "desc LPMEN."]
        #[must_use]
        #[inline(always)]
        pub const fn lpmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPMEN."]
        #[inline(always)]
        pub const fn set_lpmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc LPMACK."]
        #[must_use]
        #[inline(always)]
        pub const fn lpmack(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPMACK."]
        #[inline(always)]
        pub const fn set_lpmack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc BSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn bsel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "desc BSEL."]
        #[inline(always)]
        pub const fn set_bsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "desc REMWAKE."]
        #[must_use]
        #[inline(always)]
        pub const fn remwake(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc REMWAKE."]
        #[inline(always)]
        pub const fn set_remwake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc L1SSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn l1ssen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc L1SSEN."]
        #[inline(always)]
        pub const fn set_l1ssen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BESLTHRS."]
        #[must_use]
        #[inline(always)]
        pub const fn beslthrs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc BESLTHRS."]
        #[inline(always)]
        pub const fn set_beslthrs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc L1DSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn l1dsen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc L1DSEN."]
        #[inline(always)]
        pub const fn set_l1dsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc LPMRSP."]
        #[must_use]
        #[inline(always)]
        pub const fn lpmrsp(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "desc LPMRSP."]
        #[inline(always)]
        pub const fn set_lpmrsp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "desc SLPSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn slpsts(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc SLPSTS."]
        #[inline(always)]
        pub const fn set_slpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc L1RSMOK."]
        #[must_use]
        #[inline(always)]
        pub const fn l1rsmok(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc L1RSMOK."]
        #[inline(always)]
        pub const fn set_l1rsmok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc LPMCHIDX."]
        #[must_use]
        #[inline(always)]
        pub const fn lpmchidx(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x0f;
            val as u8
        }
        #[doc = "desc LPMCHIDX."]
        #[inline(always)]
        pub const fn set_lpmchidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
        }
        #[doc = "desc LPMRCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn lpmrcnt(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "desc LPMRCNT."]
        #[inline(always)]
        pub const fn set_lpmrcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "desc SENDLPM."]
        #[must_use]
        #[inline(always)]
        pub const fn sendlpm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc SENDLPM."]
        #[inline(always)]
        pub const fn set_sendlpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc LPMRCNTSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn lpmrcntsts(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x07;
            val as u8
        }
        #[doc = "desc LPMRCNTSTS."]
        #[inline(always)]
        pub const fn set_lpmrcntsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
        }
        #[doc = "desc ENBSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn enbsel(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENBSEL."]
        #[inline(always)]
        pub const fn set_enbsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Glpmcfg {
        #[inline(always)]
        fn default() -> Glpmcfg {
            Glpmcfg(0)
        }
    }
    impl core::fmt::Debug for Glpmcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Glpmcfg")
                .field("lpmen", &self.lpmen())
                .field("lpmack", &self.lpmack())
                .field("bsel", &self.bsel())
                .field("remwake", &self.remwake())
                .field("l1ssen", &self.l1ssen())
                .field("beslthrs", &self.beslthrs())
                .field("l1dsen", &self.l1dsen())
                .field("lpmrsp", &self.lpmrsp())
                .field("slpsts", &self.slpsts())
                .field("l1rsmok", &self.l1rsmok())
                .field("lpmchidx", &self.lpmchidx())
                .field("lpmrcnt", &self.lpmrcnt())
                .field("sendlpm", &self.sendlpm())
                .field("lpmrcntsts", &self.lpmrcntsts())
                .field("enbsel", &self.enbsel())
                .finish()
        }
    }
    #[doc = "desc GRSTCTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grstctl(pub u32);
    impl Grstctl {
        #[doc = "desc CSRST."]
        #[must_use]
        #[inline(always)]
        pub const fn csrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSRST."]
        #[inline(always)]
        pub const fn set_csrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc HSRST."]
        #[must_use]
        #[inline(always)]
        pub const fn hsrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSRST."]
        #[inline(always)]
        pub const fn set_hsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FCRST."]
        #[must_use]
        #[inline(always)]
        pub const fn fcrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCRST."]
        #[inline(always)]
        pub const fn set_fcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RXFFLSH."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfflsh(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXFFLSH."]
        #[inline(always)]
        pub const fn set_rxfflsh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TXFFLSH."]
        #[must_use]
        #[inline(always)]
        pub const fn txfflsh(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFFLSH."]
        #[inline(always)]
        pub const fn set_txfflsh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TXFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "desc TXFNUM."]
        #[inline(always)]
        pub const fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "desc DMAREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn dmareq(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAREQ."]
        #[inline(always)]
        pub const fn set_dmareq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc AHBIDL."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbidl(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBIDL."]
        #[inline(always)]
        pub const fn set_ahbidl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Grstctl {
        #[inline(always)]
        fn default() -> Grstctl {
            Grstctl(0)
        }
    }
    impl core::fmt::Debug for Grstctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grstctl")
                .field("csrst", &self.csrst())
                .field("hsrst", &self.hsrst())
                .field("fcrst", &self.fcrst())
                .field("rxfflsh", &self.rxfflsh())
                .field("txfflsh", &self.txfflsh())
                .field("txfnum", &self.txfnum())
                .field("dmareq", &self.dmareq())
                .field("ahbidl", &self.ahbidl())
                .finish()
        }
    }
    #[doc = "desc GRXFSIZ."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grxfsiz(pub u32);
    impl Grxfsiz {
        #[doc = "desc RXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RXFD."]
        #[inline(always)]
        pub const fn set_rxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Grxfsiz {
        #[inline(always)]
        fn default() -> Grxfsiz {
            Grxfsiz(0)
        }
    }
    impl core::fmt::Debug for Grxfsiz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grxfsiz")
                .field("rxfd", &self.rxfd())
                .finish()
        }
    }
    #[doc = "desc GRXSTSP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grxstsp(pub u32);
    impl Grxstsp {
        #[doc = "desc CHNUM_EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn chnum_epnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CHNUM_EPNUM."]
        #[inline(always)]
        pub const fn set_chnum_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc BCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn bcnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc BCNT."]
        #[inline(always)]
        pub const fn set_bcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
        }
        #[doc = "desc PKTSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktsts(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PKTSTS."]
        #[inline(always)]
        pub const fn set_pktsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
        }
    }
    impl Default for Grxstsp {
        #[inline(always)]
        fn default() -> Grxstsp {
            Grxstsp(0)
        }
    }
    impl core::fmt::Debug for Grxstsp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grxstsp")
                .field("chnum_epnum", &self.chnum_epnum())
                .field("bcnt", &self.bcnt())
                .field("dpid", &self.dpid())
                .field("pktsts", &self.pktsts())
                .finish()
        }
    }
    #[doc = "desc GRXSTSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grxstsr(pub u32);
    impl Grxstsr {
        #[doc = "desc CHNUM_EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn chnum_epnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CHNUM_EPNUM."]
        #[inline(always)]
        pub const fn set_chnum_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc BCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn bcnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc BCNT."]
        #[inline(always)]
        pub const fn set_bcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
        }
        #[doc = "desc PKTSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pktsts(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PKTSTS."]
        #[inline(always)]
        pub const fn set_pktsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
        }
    }
    impl Default for Grxstsr {
        #[inline(always)]
        fn default() -> Grxstsr {
            Grxstsr(0)
        }
    }
    impl core::fmt::Debug for Grxstsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grxstsr")
                .field("chnum_epnum", &self.chnum_epnum())
                .field("bcnt", &self.bcnt())
                .field("dpid", &self.dpid())
                .field("pktsts", &self.pktsts())
                .finish()
        }
    }
    #[doc = "desc GUSBCFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gusbcfg(pub u32);
    impl Gusbcfg {
        #[doc = "desc TOCAL."]
        #[must_use]
        #[inline(always)]
        pub const fn tocal(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc TOCAL."]
        #[inline(always)]
        pub const fn set_tocal(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc PHYSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn physel(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PHYSEL."]
        #[inline(always)]
        pub const fn set_physel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TRDT."]
        #[must_use]
        #[inline(always)]
        pub const fn trdt(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TRDT."]
        #[inline(always)]
        pub const fn set_trdt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[doc = "desc PHYLPCS."]
        #[must_use]
        #[inline(always)]
        pub const fn phylpcs(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PHYLPCS."]
        #[inline(always)]
        pub const fn set_phylpcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc ULFSLS."]
        #[must_use]
        #[inline(always)]
        pub const fn ulfsls(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc ULFSLS."]
        #[inline(always)]
        pub const fn set_ulfsls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc ULPIAR."]
        #[must_use]
        #[inline(always)]
        pub const fn ulpiar(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc ULPIAR."]
        #[inline(always)]
        pub const fn set_ulpiar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc ULPICSM."]
        #[must_use]
        #[inline(always)]
        pub const fn ulpicsm(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc ULPICSM."]
        #[inline(always)]
        pub const fn set_ulpicsm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc ULPIEVBUSD."]
        #[must_use]
        #[inline(always)]
        pub const fn ulpievbusd(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc ULPIEVBUSD."]
        #[inline(always)]
        pub const fn set_ulpievbusd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc ULPIEVBUSI."]
        #[must_use]
        #[inline(always)]
        pub const fn ulpievbusi(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc ULPIEVBUSI."]
        #[inline(always)]
        pub const fn set_ulpievbusi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc PCCI."]
        #[must_use]
        #[inline(always)]
        pub const fn pcci(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCCI."]
        #[inline(always)]
        pub const fn set_pcci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc PTCI."]
        #[must_use]
        #[inline(always)]
        pub const fn ptci(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc PTCI."]
        #[inline(always)]
        pub const fn set_ptci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc ULPIPD."]
        #[must_use]
        #[inline(always)]
        pub const fn ulpipd(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc ULPIPD."]
        #[inline(always)]
        pub const fn set_ulpipd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc FHMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn fhmod(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc FHMOD."]
        #[inline(always)]
        pub const fn set_fhmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc FDMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn fdmod(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc FDMOD."]
        #[inline(always)]
        pub const fn set_fdmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Gusbcfg {
        #[inline(always)]
        fn default() -> Gusbcfg {
            Gusbcfg(0)
        }
    }
    impl core::fmt::Debug for Gusbcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gusbcfg")
                .field("tocal", &self.tocal())
                .field("physel", &self.physel())
                .field("trdt", &self.trdt())
                .field("phylpcs", &self.phylpcs())
                .field("ulfsls", &self.ulfsls())
                .field("ulpiar", &self.ulpiar())
                .field("ulpicsm", &self.ulpicsm())
                .field("ulpievbusd", &self.ulpievbusd())
                .field("ulpievbusi", &self.ulpievbusi())
                .field("pcci", &self.pcci())
                .field("ptci", &self.ptci())
                .field("ulpipd", &self.ulpipd())
                .field("fhmod", &self.fhmod())
                .field("fdmod", &self.fdmod())
                .finish()
        }
    }
    #[doc = "desc GVBUSCFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gvbuscfg(pub u32);
    impl Gvbuscfg {
        #[doc = "desc VBUSOVEN."]
        #[must_use]
        #[inline(always)]
        pub const fn vbusoven(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VBUSOVEN."]
        #[inline(always)]
        pub const fn set_vbusoven(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VBUSVAL."]
        #[must_use]
        #[inline(always)]
        pub const fn vbusval(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VBUSVAL."]
        #[inline(always)]
        pub const fn set_vbusval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Gvbuscfg {
        #[inline(always)]
        fn default() -> Gvbuscfg {
            Gvbuscfg(0)
        }
    }
    impl core::fmt::Debug for Gvbuscfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gvbuscfg")
                .field("vbusoven", &self.vbusoven())
                .field("vbusval", &self.vbusval())
                .finish()
        }
    }
    #[doc = "desc HAINT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Haint(pub u32);
    impl Haint {
        #[doc = "desc HAINT."]
        #[must_use]
        #[inline(always)]
        pub const fn haint(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc HAINT."]
        #[inline(always)]
        pub const fn set_haint(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Haint {
        #[inline(always)]
        fn default() -> Haint {
            Haint(0)
        }
    }
    impl core::fmt::Debug for Haint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Haint")
                .field("haint", &self.haint())
                .finish()
        }
    }
    #[doc = "desc HAINTMSK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Haintmsk(pub u32);
    impl Haintmsk {
        #[doc = "desc HAINTM."]
        #[must_use]
        #[inline(always)]
        pub const fn haintm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc HAINTM."]
        #[inline(always)]
        pub const fn set_haintm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Haintmsk {
        #[inline(always)]
        fn default() -> Haintmsk {
            Haintmsk(0)
        }
    }
    impl core::fmt::Debug for Haintmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Haintmsk")
                .field("haintm", &self.haintm())
                .finish()
        }
    }
    #[doc = "desc HCCHAR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar0(pub u32);
    impl Hcchar0 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar0 {
        #[inline(always)]
        fn default() -> Hcchar0 {
            Hcchar0(0)
        }
    }
    impl core::fmt::Debug for Hcchar0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar0")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar1(pub u32);
    impl Hcchar1 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar1 {
        #[inline(always)]
        fn default() -> Hcchar1 {
            Hcchar1(0)
        }
    }
    impl core::fmt::Debug for Hcchar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar1")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar10(pub u32);
    impl Hcchar10 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar10 {
        #[inline(always)]
        fn default() -> Hcchar10 {
            Hcchar10(0)
        }
    }
    impl core::fmt::Debug for Hcchar10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar10")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar11(pub u32);
    impl Hcchar11 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar11 {
        #[inline(always)]
        fn default() -> Hcchar11 {
            Hcchar11(0)
        }
    }
    impl core::fmt::Debug for Hcchar11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar11")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar12(pub u32);
    impl Hcchar12 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar12 {
        #[inline(always)]
        fn default() -> Hcchar12 {
            Hcchar12(0)
        }
    }
    impl core::fmt::Debug for Hcchar12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar12")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar13(pub u32);
    impl Hcchar13 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar13 {
        #[inline(always)]
        fn default() -> Hcchar13 {
            Hcchar13(0)
        }
    }
    impl core::fmt::Debug for Hcchar13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar13")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar14(pub u32);
    impl Hcchar14 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar14 {
        #[inline(always)]
        fn default() -> Hcchar14 {
            Hcchar14(0)
        }
    }
    impl core::fmt::Debug for Hcchar14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar14")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar15(pub u32);
    impl Hcchar15 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar15 {
        #[inline(always)]
        fn default() -> Hcchar15 {
            Hcchar15(0)
        }
    }
    impl core::fmt::Debug for Hcchar15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar15")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar2(pub u32);
    impl Hcchar2 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar2 {
        #[inline(always)]
        fn default() -> Hcchar2 {
            Hcchar2(0)
        }
    }
    impl core::fmt::Debug for Hcchar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar2")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar3(pub u32);
    impl Hcchar3 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar3 {
        #[inline(always)]
        fn default() -> Hcchar3 {
            Hcchar3(0)
        }
    }
    impl core::fmt::Debug for Hcchar3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar3")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar4(pub u32);
    impl Hcchar4 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar4 {
        #[inline(always)]
        fn default() -> Hcchar4 {
            Hcchar4(0)
        }
    }
    impl core::fmt::Debug for Hcchar4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar4")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar5(pub u32);
    impl Hcchar5 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar5 {
        #[inline(always)]
        fn default() -> Hcchar5 {
            Hcchar5(0)
        }
    }
    impl core::fmt::Debug for Hcchar5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar5")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar6(pub u32);
    impl Hcchar6 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar6 {
        #[inline(always)]
        fn default() -> Hcchar6 {
            Hcchar6(0)
        }
    }
    impl core::fmt::Debug for Hcchar6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar6")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar7(pub u32);
    impl Hcchar7 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar7 {
        #[inline(always)]
        fn default() -> Hcchar7 {
            Hcchar7(0)
        }
    }
    impl core::fmt::Debug for Hcchar7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar7")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar8(pub u32);
    impl Hcchar8 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar8 {
        #[inline(always)]
        fn default() -> Hcchar8 {
            Hcchar8(0)
        }
    }
    impl core::fmt::Debug for Hcchar8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar8")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCCHAR9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar9(pub u32);
    impl Hcchar9 {
        #[doc = "desc MPSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc MPSIZ."]
        #[inline(always)]
        pub const fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc EPNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EPNUM."]
        #[inline(always)]
        pub const fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc EPDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPDIR."]
        #[inline(always)]
        pub const fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LSDEV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSDEV."]
        #[inline(always)]
        pub const fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EPTYP."]
        #[must_use]
        #[inline(always)]
        pub const fn eptyp(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc EPTYP."]
        #[inline(always)]
        pub const fn set_eptyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc MC."]
        #[must_use]
        #[inline(always)]
        pub const fn mc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc MC."]
        #[inline(always)]
        pub const fn set_mc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc DAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DAD."]
        #[inline(always)]
        pub const fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "desc ODDFRM."]
        #[must_use]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODDFRM."]
        #[inline(always)]
        pub const fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc CHDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHDIS."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CHENA."]
        #[must_use]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHENA."]
        #[inline(always)]
        pub const fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar9 {
        #[inline(always)]
        fn default() -> Hcchar9 {
            Hcchar9(0)
        }
    }
    impl core::fmt::Debug for Hcchar9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar9")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mc", &self.mc())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[doc = "desc HCFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcfg(pub u32);
    impl Hcfg {
        #[doc = "desc FSLSPCS."]
        #[must_use]
        #[inline(always)]
        pub const fn fslspcs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc FSLSPCS."]
        #[inline(always)]
        pub const fn set_fslspcs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc FSLSS."]
        #[must_use]
        #[inline(always)]
        pub const fn fslss(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FSLSS."]
        #[inline(always)]
        pub const fn set_fslss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Hcfg {
        #[inline(always)]
        fn default() -> Hcfg {
            Hcfg(0)
        }
    }
    impl core::fmt::Debug for Hcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcfg")
                .field("fslspcs", &self.fslspcs())
                .field("fslss", &self.fslss())
                .finish()
        }
    }
    #[doc = "desc HCINT0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint0(pub u32);
    impl Hcint0 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint0 {
        #[inline(always)]
        fn default() -> Hcint0 {
            Hcint0(0)
        }
    }
    impl core::fmt::Debug for Hcint0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint0")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint1(pub u32);
    impl Hcint1 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint1 {
        #[inline(always)]
        fn default() -> Hcint1 {
            Hcint1(0)
        }
    }
    impl core::fmt::Debug for Hcint1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint1")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint10(pub u32);
    impl Hcint10 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint10 {
        #[inline(always)]
        fn default() -> Hcint10 {
            Hcint10(0)
        }
    }
    impl core::fmt::Debug for Hcint10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint10")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint11(pub u32);
    impl Hcint11 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint11 {
        #[inline(always)]
        fn default() -> Hcint11 {
            Hcint11(0)
        }
    }
    impl core::fmt::Debug for Hcint11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint11")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint12(pub u32);
    impl Hcint12 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint12 {
        #[inline(always)]
        fn default() -> Hcint12 {
            Hcint12(0)
        }
    }
    impl core::fmt::Debug for Hcint12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint12")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint13(pub u32);
    impl Hcint13 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint13 {
        #[inline(always)]
        fn default() -> Hcint13 {
            Hcint13(0)
        }
    }
    impl core::fmt::Debug for Hcint13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint13")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint14(pub u32);
    impl Hcint14 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint14 {
        #[inline(always)]
        fn default() -> Hcint14 {
            Hcint14(0)
        }
    }
    impl core::fmt::Debug for Hcint14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint14")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint15(pub u32);
    impl Hcint15 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint15 {
        #[inline(always)]
        fn default() -> Hcint15 {
            Hcint15(0)
        }
    }
    impl core::fmt::Debug for Hcint15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint15")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint2(pub u32);
    impl Hcint2 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint2 {
        #[inline(always)]
        fn default() -> Hcint2 {
            Hcint2(0)
        }
    }
    impl core::fmt::Debug for Hcint2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint2")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint3(pub u32);
    impl Hcint3 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint3 {
        #[inline(always)]
        fn default() -> Hcint3 {
            Hcint3(0)
        }
    }
    impl core::fmt::Debug for Hcint3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint3")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint4(pub u32);
    impl Hcint4 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint4 {
        #[inline(always)]
        fn default() -> Hcint4 {
            Hcint4(0)
        }
    }
    impl core::fmt::Debug for Hcint4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint4")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint5(pub u32);
    impl Hcint5 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint5 {
        #[inline(always)]
        fn default() -> Hcint5 {
            Hcint5(0)
        }
    }
    impl core::fmt::Debug for Hcint5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint5")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint6(pub u32);
    impl Hcint6 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint6 {
        #[inline(always)]
        fn default() -> Hcint6 {
            Hcint6(0)
        }
    }
    impl core::fmt::Debug for Hcint6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint6")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint7(pub u32);
    impl Hcint7 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint7 {
        #[inline(always)]
        fn default() -> Hcint7 {
            Hcint7(0)
        }
    }
    impl core::fmt::Debug for Hcint7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint7")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint8(pub u32);
    impl Hcint8 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint8 {
        #[inline(always)]
        fn default() -> Hcint8 {
            Hcint8(0)
        }
    }
    impl core::fmt::Debug for Hcint8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint8")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINT9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint9(pub u32);
    impl Hcint9 {
        #[doc = "desc XFRC."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRC."]
        #[inline(always)]
        pub const fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHH."]
        #[must_use]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHH."]
        #[inline(always)]
        pub const fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERR."]
        #[inline(always)]
        pub const fn set_ahberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALL."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALL."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAK."]
        #[inline(always)]
        pub const fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACK."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYET."]
        #[must_use]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYET."]
        #[inline(always)]
        pub const fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERR."]
        #[must_use]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERR."]
        #[inline(always)]
        pub const fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMOR."]
        #[must_use]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMOR."]
        #[inline(always)]
        pub const fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERR."]
        #[inline(always)]
        pub const fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint9 {
        #[inline(always)]
        fn default() -> Hcint9 {
            Hcint9(0)
        }
    }
    impl core::fmt::Debug for Hcint9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint9")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("ahberr", &self.ahberr())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("nyet", &self.nyet())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk0(pub u32);
    impl Hcintmsk0 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk0 {
        #[inline(always)]
        fn default() -> Hcintmsk0 {
            Hcintmsk0(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk0")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk1(pub u32);
    impl Hcintmsk1 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk1 {
        #[inline(always)]
        fn default() -> Hcintmsk1 {
            Hcintmsk1(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk1")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk10(pub u32);
    impl Hcintmsk10 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk10 {
        #[inline(always)]
        fn default() -> Hcintmsk10 {
            Hcintmsk10(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk10")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk11(pub u32);
    impl Hcintmsk11 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk11 {
        #[inline(always)]
        fn default() -> Hcintmsk11 {
            Hcintmsk11(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk11")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk12(pub u32);
    impl Hcintmsk12 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk12 {
        #[inline(always)]
        fn default() -> Hcintmsk12 {
            Hcintmsk12(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk12")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk13(pub u32);
    impl Hcintmsk13 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk13 {
        #[inline(always)]
        fn default() -> Hcintmsk13 {
            Hcintmsk13(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk13")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk14(pub u32);
    impl Hcintmsk14 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk14 {
        #[inline(always)]
        fn default() -> Hcintmsk14 {
            Hcintmsk14(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk14")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk15(pub u32);
    impl Hcintmsk15 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk15 {
        #[inline(always)]
        fn default() -> Hcintmsk15 {
            Hcintmsk15(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk15")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk2(pub u32);
    impl Hcintmsk2 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk2 {
        #[inline(always)]
        fn default() -> Hcintmsk2 {
            Hcintmsk2(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk2")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk3(pub u32);
    impl Hcintmsk3 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk3 {
        #[inline(always)]
        fn default() -> Hcintmsk3 {
            Hcintmsk3(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk3")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk4(pub u32);
    impl Hcintmsk4 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk4 {
        #[inline(always)]
        fn default() -> Hcintmsk4 {
            Hcintmsk4(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk4")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk5(pub u32);
    impl Hcintmsk5 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk5 {
        #[inline(always)]
        fn default() -> Hcintmsk5 {
            Hcintmsk5(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk5")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk6(pub u32);
    impl Hcintmsk6 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk6 {
        #[inline(always)]
        fn default() -> Hcintmsk6 {
            Hcintmsk6(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk6")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk7(pub u32);
    impl Hcintmsk7 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk7 {
        #[inline(always)]
        fn default() -> Hcintmsk7 {
            Hcintmsk7(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk7")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk8(pub u32);
    impl Hcintmsk8 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk8 {
        #[inline(always)]
        fn default() -> Hcintmsk8 {
            Hcintmsk8(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk8")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCINTMSK9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk9(pub u32);
    impl Hcintmsk9 {
        #[doc = "desc XFRCM."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XFRCM."]
        #[inline(always)]
        pub const fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHHM."]
        #[must_use]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHHM."]
        #[inline(always)]
        pub const fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AHBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn ahberrm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHBERRM."]
        #[inline(always)]
        pub const fn set_ahberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc STALLM."]
        #[must_use]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc STALLM."]
        #[inline(always)]
        pub const fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NAKM."]
        #[must_use]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NAKM."]
        #[inline(always)]
        pub const fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ACKM."]
        #[must_use]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKM."]
        #[inline(always)]
        pub const fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc NYETM."]
        #[must_use]
        #[inline(always)]
        pub const fn nyetm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc NYETM."]
        #[inline(always)]
        pub const fn set_nyetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERRM."]
        #[inline(always)]
        pub const fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BBERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BBERRM."]
        #[inline(always)]
        pub const fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FRMORM."]
        #[must_use]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRMORM."]
        #[inline(always)]
        pub const fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DTERRM."]
        #[must_use]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTERRM."]
        #[inline(always)]
        pub const fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk9 {
        #[inline(always)]
        fn default() -> Hcintmsk9 {
            Hcintmsk9(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk9")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("ahberrm", &self.ahberrm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyetm", &self.nyetm())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[doc = "desc HCSPLT0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt0(pub u32);
    impl Hcsplt0 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt0 {
        #[inline(always)]
        fn default() -> Hcsplt0 {
            Hcsplt0(0)
        }
    }
    impl core::fmt::Debug for Hcsplt0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt0")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt1(pub u32);
    impl Hcsplt1 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt1 {
        #[inline(always)]
        fn default() -> Hcsplt1 {
            Hcsplt1(0)
        }
    }
    impl core::fmt::Debug for Hcsplt1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt1")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt10(pub u32);
    impl Hcsplt10 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt10 {
        #[inline(always)]
        fn default() -> Hcsplt10 {
            Hcsplt10(0)
        }
    }
    impl core::fmt::Debug for Hcsplt10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt10")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt11(pub u32);
    impl Hcsplt11 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt11 {
        #[inline(always)]
        fn default() -> Hcsplt11 {
            Hcsplt11(0)
        }
    }
    impl core::fmt::Debug for Hcsplt11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt11")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt12(pub u32);
    impl Hcsplt12 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt12 {
        #[inline(always)]
        fn default() -> Hcsplt12 {
            Hcsplt12(0)
        }
    }
    impl core::fmt::Debug for Hcsplt12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt12")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt13(pub u32);
    impl Hcsplt13 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt13 {
        #[inline(always)]
        fn default() -> Hcsplt13 {
            Hcsplt13(0)
        }
    }
    impl core::fmt::Debug for Hcsplt13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt13")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt14(pub u32);
    impl Hcsplt14 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt14 {
        #[inline(always)]
        fn default() -> Hcsplt14 {
            Hcsplt14(0)
        }
    }
    impl core::fmt::Debug for Hcsplt14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt14")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt15(pub u32);
    impl Hcsplt15 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt15 {
        #[inline(always)]
        fn default() -> Hcsplt15 {
            Hcsplt15(0)
        }
    }
    impl core::fmt::Debug for Hcsplt15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt15")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt2(pub u32);
    impl Hcsplt2 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt2 {
        #[inline(always)]
        fn default() -> Hcsplt2 {
            Hcsplt2(0)
        }
    }
    impl core::fmt::Debug for Hcsplt2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt2")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt3(pub u32);
    impl Hcsplt3 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt3 {
        #[inline(always)]
        fn default() -> Hcsplt3 {
            Hcsplt3(0)
        }
    }
    impl core::fmt::Debug for Hcsplt3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt3")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt4(pub u32);
    impl Hcsplt4 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt4 {
        #[inline(always)]
        fn default() -> Hcsplt4 {
            Hcsplt4(0)
        }
    }
    impl core::fmt::Debug for Hcsplt4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt4")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt5(pub u32);
    impl Hcsplt5 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt5 {
        #[inline(always)]
        fn default() -> Hcsplt5 {
            Hcsplt5(0)
        }
    }
    impl core::fmt::Debug for Hcsplt5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt5")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt6(pub u32);
    impl Hcsplt6 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt6 {
        #[inline(always)]
        fn default() -> Hcsplt6 {
            Hcsplt6(0)
        }
    }
    impl core::fmt::Debug for Hcsplt6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt6")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt7(pub u32);
    impl Hcsplt7 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt7 {
        #[inline(always)]
        fn default() -> Hcsplt7 {
            Hcsplt7(0)
        }
    }
    impl core::fmt::Debug for Hcsplt7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt7")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt8(pub u32);
    impl Hcsplt8 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt8 {
        #[inline(always)]
        fn default() -> Hcsplt8 {
            Hcsplt8(0)
        }
    }
    impl core::fmt::Debug for Hcsplt8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt8")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCSPLT9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt9(pub u32);
    impl Hcsplt9 {
        #[doc = "desc PRTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn prtaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc PRTADDR."]
        #[inline(always)]
        pub const fn set_prtaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc HUBADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn hubaddr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x7f;
            val as u8
        }
        #[doc = "desc HUBADDR."]
        #[inline(always)]
        pub const fn set_hubaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
        }
        #[doc = "desc XACTPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn xactpos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc XACTPOS."]
        #[inline(always)]
        pub const fn set_xactpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc COMPLSPLT."]
        #[must_use]
        #[inline(always)]
        pub const fn complsplt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPLSPLT."]
        #[inline(always)]
        pub const fn set_complsplt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPLITEN."]
        #[must_use]
        #[inline(always)]
        pub const fn spliten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPLITEN."]
        #[inline(always)]
        pub const fn set_spliten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcsplt9 {
        #[inline(always)]
        fn default() -> Hcsplt9 {
            Hcsplt9(0)
        }
    }
    impl core::fmt::Debug for Hcsplt9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcsplt9")
                .field("prtaddr", &self.prtaddr())
                .field("hubaddr", &self.hubaddr())
                .field("xactpos", &self.xactpos())
                .field("complsplt", &self.complsplt())
                .field("spliten", &self.spliten())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz0(pub u32);
    impl Hctsiz0 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz0 {
        #[inline(always)]
        fn default() -> Hctsiz0 {
            Hctsiz0(0)
        }
    }
    impl core::fmt::Debug for Hctsiz0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz0")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz1(pub u32);
    impl Hctsiz1 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz1 {
        #[inline(always)]
        fn default() -> Hctsiz1 {
            Hctsiz1(0)
        }
    }
    impl core::fmt::Debug for Hctsiz1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz1")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz10(pub u32);
    impl Hctsiz10 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz10 {
        #[inline(always)]
        fn default() -> Hctsiz10 {
            Hctsiz10(0)
        }
    }
    impl core::fmt::Debug for Hctsiz10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz10")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz11(pub u32);
    impl Hctsiz11 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz11 {
        #[inline(always)]
        fn default() -> Hctsiz11 {
            Hctsiz11(0)
        }
    }
    impl core::fmt::Debug for Hctsiz11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz11")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz12(pub u32);
    impl Hctsiz12 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz12 {
        #[inline(always)]
        fn default() -> Hctsiz12 {
            Hctsiz12(0)
        }
    }
    impl core::fmt::Debug for Hctsiz12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz12")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz13(pub u32);
    impl Hctsiz13 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz13 {
        #[inline(always)]
        fn default() -> Hctsiz13 {
            Hctsiz13(0)
        }
    }
    impl core::fmt::Debug for Hctsiz13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz13")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz14(pub u32);
    impl Hctsiz14 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz14 {
        #[inline(always)]
        fn default() -> Hctsiz14 {
            Hctsiz14(0)
        }
    }
    impl core::fmt::Debug for Hctsiz14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz14")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz15(pub u32);
    impl Hctsiz15 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz15 {
        #[inline(always)]
        fn default() -> Hctsiz15 {
            Hctsiz15(0)
        }
    }
    impl core::fmt::Debug for Hctsiz15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz15")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz2(pub u32);
    impl Hctsiz2 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz2 {
        #[inline(always)]
        fn default() -> Hctsiz2 {
            Hctsiz2(0)
        }
    }
    impl core::fmt::Debug for Hctsiz2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz2")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz3(pub u32);
    impl Hctsiz3 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz3 {
        #[inline(always)]
        fn default() -> Hctsiz3 {
            Hctsiz3(0)
        }
    }
    impl core::fmt::Debug for Hctsiz3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz3")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz4(pub u32);
    impl Hctsiz4 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz4 {
        #[inline(always)]
        fn default() -> Hctsiz4 {
            Hctsiz4(0)
        }
    }
    impl core::fmt::Debug for Hctsiz4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz4")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz5(pub u32);
    impl Hctsiz5 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz5 {
        #[inline(always)]
        fn default() -> Hctsiz5 {
            Hctsiz5(0)
        }
    }
    impl core::fmt::Debug for Hctsiz5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz5")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz6(pub u32);
    impl Hctsiz6 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz6 {
        #[inline(always)]
        fn default() -> Hctsiz6 {
            Hctsiz6(0)
        }
    }
    impl core::fmt::Debug for Hctsiz6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz6")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz7(pub u32);
    impl Hctsiz7 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz7 {
        #[inline(always)]
        fn default() -> Hctsiz7 {
            Hctsiz7(0)
        }
    }
    impl core::fmt::Debug for Hctsiz7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz7")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz8(pub u32);
    impl Hctsiz8 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz8 {
        #[inline(always)]
        fn default() -> Hctsiz8 {
            Hctsiz8(0)
        }
    }
    impl core::fmt::Debug for Hctsiz8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz8")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HCTSIZ9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz9(pub u32);
    impl Hctsiz9 {
        #[doc = "desc XFRSIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "desc XFRSIZ."]
        #[inline(always)]
        pub const fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "desc PKTCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc PKTCNT."]
        #[inline(always)]
        pub const fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "desc DPID."]
        #[must_use]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPID."]
        #[inline(always)]
        pub const fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "desc DOPING."]
        #[must_use]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOPING."]
        #[inline(always)]
        pub const fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz9 {
        #[inline(always)]
        fn default() -> Hctsiz9 {
            Hctsiz9(0)
        }
    }
    impl core::fmt::Debug for Hctsiz9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz9")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[doc = "desc HFIR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfir(pub u32);
    impl Hfir {
        #[doc = "desc FRIVL."]
        #[must_use]
        #[inline(always)]
        pub const fn frivl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc FRIVL."]
        #[inline(always)]
        pub const fn set_frivl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Hfir {
        #[inline(always)]
        fn default() -> Hfir {
            Hfir(0)
        }
    }
    impl core::fmt::Debug for Hfir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfir")
                .field("frivl", &self.frivl())
                .finish()
        }
    }
    #[doc = "desc HFNUM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfnum(pub u32);
    impl Hfnum {
        #[doc = "desc FRNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn frnum(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc FRNUM."]
        #[inline(always)]
        pub const fn set_frnum(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc FTREM."]
        #[must_use]
        #[inline(always)]
        pub const fn ftrem(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc FTREM."]
        #[inline(always)]
        pub const fn set_ftrem(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Hfnum {
        #[inline(always)]
        fn default() -> Hfnum {
            Hfnum(0)
        }
    }
    impl core::fmt::Debug for Hfnum {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfnum")
                .field("frnum", &self.frnum())
                .field("ftrem", &self.ftrem())
                .finish()
        }
    }
    #[doc = "desc HNPTXFSIZ."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hnptxfsiz(pub u32);
    impl Hnptxfsiz {
        #[doc = "desc NPTXFSA."]
        #[must_use]
        #[inline(always)]
        pub const fn nptxfsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc NPTXFSA."]
        #[inline(always)]
        pub const fn set_nptxfsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc NPTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn nptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc NPTXFD."]
        #[inline(always)]
        pub const fn set_nptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Hnptxfsiz {
        #[inline(always)]
        fn default() -> Hnptxfsiz {
            Hnptxfsiz(0)
        }
    }
    impl core::fmt::Debug for Hnptxfsiz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hnptxfsiz")
                .field("nptxfsa", &self.nptxfsa())
                .field("nptxfd", &self.nptxfd())
                .finish()
        }
    }
    #[doc = "desc HNPTXSTS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hnptxsts(pub u32);
    impl Hnptxsts {
        #[doc = "desc NPTXFSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn nptxfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc NPTXFSAV."]
        #[inline(always)]
        pub const fn set_nptxfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc NPTQXSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn nptqxsav(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NPTQXSAV."]
        #[inline(always)]
        pub const fn set_nptqxsav(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "desc NPTXQTOP."]
        #[must_use]
        #[inline(always)]
        pub const fn nptxqtop(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "desc NPTXQTOP."]
        #[inline(always)]
        pub const fn set_nptxqtop(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Hnptxsts {
        #[inline(always)]
        fn default() -> Hnptxsts {
            Hnptxsts(0)
        }
    }
    impl core::fmt::Debug for Hnptxsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hnptxsts")
                .field("nptxfsav", &self.nptxfsav())
                .field("nptqxsav", &self.nptqxsav())
                .field("nptxqtop", &self.nptxqtop())
                .finish()
        }
    }
    #[doc = "desc HPRT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hprt(pub u32);
    impl Hprt {
        #[doc = "desc PCSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pcsts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCSTS."]
        #[inline(always)]
        pub const fn set_pcsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PCDET."]
        #[must_use]
        #[inline(always)]
        pub const fn pcdet(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCDET."]
        #[inline(always)]
        pub const fn set_pcdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PENA."]
        #[must_use]
        #[inline(always)]
        pub const fn pena(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PENA."]
        #[inline(always)]
        pub const fn set_pena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PENCHNG."]
        #[must_use]
        #[inline(always)]
        pub const fn penchng(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PENCHNG."]
        #[inline(always)]
        pub const fn set_penchng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POCA."]
        #[must_use]
        #[inline(always)]
        pub const fn poca(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POCA."]
        #[inline(always)]
        pub const fn set_poca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POCCHNG."]
        #[must_use]
        #[inline(always)]
        pub const fn pocchng(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POCCHNG."]
        #[inline(always)]
        pub const fn set_pocchng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PRES."]
        #[must_use]
        #[inline(always)]
        pub const fn pres(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRES."]
        #[inline(always)]
        pub const fn set_pres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PSUSP."]
        #[must_use]
        #[inline(always)]
        pub const fn psusp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PSUSP."]
        #[inline(always)]
        pub const fn set_psusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PRST."]
        #[must_use]
        #[inline(always)]
        pub const fn prst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRST."]
        #[inline(always)]
        pub const fn set_prst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PLSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn plsts(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc PLSTS."]
        #[inline(always)]
        pub const fn set_plsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc PWPR."]
        #[must_use]
        #[inline(always)]
        pub const fn pwpr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWPR."]
        #[inline(always)]
        pub const fn set_pwpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PTCTL."]
        #[must_use]
        #[inline(always)]
        pub const fn ptctl(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PTCTL."]
        #[inline(always)]
        pub const fn set_ptctl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[doc = "desc PSPD."]
        #[must_use]
        #[inline(always)]
        pub const fn pspd(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "desc PSPD."]
        #[inline(always)]
        pub const fn set_pspd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
    }
    impl Default for Hprt {
        #[inline(always)]
        fn default() -> Hprt {
            Hprt(0)
        }
    }
    impl core::fmt::Debug for Hprt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hprt")
                .field("pcsts", &self.pcsts())
                .field("pcdet", &self.pcdet())
                .field("pena", &self.pena())
                .field("penchng", &self.penchng())
                .field("poca", &self.poca())
                .field("pocchng", &self.pocchng())
                .field("pres", &self.pres())
                .field("psusp", &self.psusp())
                .field("prst", &self.prst())
                .field("plsts", &self.plsts())
                .field("pwpr", &self.pwpr())
                .field("ptctl", &self.ptctl())
                .field("pspd", &self.pspd())
                .finish()
        }
    }
    #[doc = "desc HPTXFSIZ."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hptxfsiz(pub u32);
    impl Hptxfsiz {
        #[doc = "desc PTXSA."]
        #[must_use]
        #[inline(always)]
        pub const fn ptxsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PTXSA."]
        #[inline(always)]
        pub const fn set_ptxsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc PTXFD."]
        #[must_use]
        #[inline(always)]
        pub const fn ptxfd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PTXFD."]
        #[inline(always)]
        pub const fn set_ptxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Hptxfsiz {
        #[inline(always)]
        fn default() -> Hptxfsiz {
            Hptxfsiz(0)
        }
    }
    impl core::fmt::Debug for Hptxfsiz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hptxfsiz")
                .field("ptxsa", &self.ptxsa())
                .field("ptxfd", &self.ptxfd())
                .finish()
        }
    }
    #[doc = "desc HPTXSTS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hptxsts(pub u32);
    impl Hptxsts {
        #[doc = "desc PTXFSAVL."]
        #[must_use]
        #[inline(always)]
        pub const fn ptxfsavl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PTXFSAVL."]
        #[inline(always)]
        pub const fn set_ptxfsavl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc PTXQSAV."]
        #[must_use]
        #[inline(always)]
        pub const fn ptxqsav(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc PTXQSAV."]
        #[inline(always)]
        pub const fn set_ptxqsav(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "desc PTXQTOP."]
        #[must_use]
        #[inline(always)]
        pub const fn ptxqtop(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "desc PTXQTOP."]
        #[inline(always)]
        pub const fn set_ptxqtop(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Hptxsts {
        #[inline(always)]
        fn default() -> Hptxsts {
            Hptxsts(0)
        }
    }
    impl core::fmt::Debug for Hptxsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hptxsts")
                .field("ptxfsavl", &self.ptxfsavl())
                .field("ptxqsav", &self.ptxqsav())
                .field("ptxqtop", &self.ptxqtop())
                .finish()
        }
    }
}
