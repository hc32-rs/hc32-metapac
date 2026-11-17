#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc GPIO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc PIDRA."]
    #[inline(always)]
    pub const fn pidra(self) -> crate::common::Reg<regs::Pidra, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc PIDRB."]
    #[inline(always)]
    pub const fn pidrb(self) -> crate::common::Reg<regs::Pidrb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc PIDRC."]
    #[inline(always)]
    pub const fn pidrc(self) -> crate::common::Reg<regs::Pidrc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc PIDRD."]
    #[inline(always)]
    pub const fn pidrd(self) -> crate::common::Reg<regs::Pidrd, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc PIDRE."]
    #[inline(always)]
    pub const fn pidre(self) -> crate::common::Reg<regs::Pidre, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc PODRA."]
    #[inline(always)]
    pub const fn podra(self) -> crate::common::Reg<regs::Podra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc PODRB."]
    #[inline(always)]
    pub const fn podrb(self) -> crate::common::Reg<regs::Podrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc PODRC."]
    #[inline(always)]
    pub const fn podrc(self) -> crate::common::Reg<regs::Podrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc PODRD."]
    #[inline(always)]
    pub const fn podrd(self) -> crate::common::Reg<regs::Podrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc PODRE."]
    #[inline(always)]
    pub const fn podre(self) -> crate::common::Reg<regs::Podre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc POERA."]
    #[inline(always)]
    pub const fn poera(self) -> crate::common::Reg<regs::Poera, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc POERB."]
    #[inline(always)]
    pub const fn poerb(self) -> crate::common::Reg<regs::Poerb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc POERC."]
    #[inline(always)]
    pub const fn poerc(self) -> crate::common::Reg<regs::Poerc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc POERD."]
    #[inline(always)]
    pub const fn poerd(self) -> crate::common::Reg<regs::Poerd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc POERE."]
    #[inline(always)]
    pub const fn poere(self) -> crate::common::Reg<regs::Poere, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc POSRA."]
    #[inline(always)]
    pub const fn posra(self) -> crate::common::Reg<regs::Posra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc POSRB."]
    #[inline(always)]
    pub const fn posrb(self) -> crate::common::Reg<regs::Posrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "desc POSRC."]
    #[inline(always)]
    pub const fn posrc(self) -> crate::common::Reg<regs::Posrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "desc POSRD."]
    #[inline(always)]
    pub const fn posrd(self) -> crate::common::Reg<regs::Posrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "desc POSRE."]
    #[inline(always)]
    pub const fn posre(self) -> crate::common::Reg<regs::Posre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "desc PORRA."]
    #[inline(always)]
    pub const fn porra(self) -> crate::common::Reg<regs::Porra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "desc PORRB."]
    #[inline(always)]
    pub const fn porrb(self) -> crate::common::Reg<regs::Porrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "desc PORRC."]
    #[inline(always)]
    pub const fn porrc(self) -> crate::common::Reg<regs::Porrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "desc PORRD."]
    #[inline(always)]
    pub const fn porrd(self) -> crate::common::Reg<regs::Porrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "desc PORRE."]
    #[inline(always)]
    pub const fn porre(self) -> crate::common::Reg<regs::Porre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "desc POTRA."]
    #[inline(always)]
    pub const fn potra(self) -> crate::common::Reg<regs::Potra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "desc POTRB."]
    #[inline(always)]
    pub const fn potrb(self) -> crate::common::Reg<regs::Potrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "desc POTRC."]
    #[inline(always)]
    pub const fn potrc(self) -> crate::common::Reg<regs::Potrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "desc POTRD."]
    #[inline(always)]
    pub const fn potrd(self) -> crate::common::Reg<regs::Potrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "desc POTRE."]
    #[inline(always)]
    pub const fn potre(self) -> crate::common::Reg<regs::Potre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "desc PCRA0."]
    #[inline(always)]
    pub const fn pcra0(self) -> crate::common::Reg<regs::Pcra0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "desc PFSRA0."]
    #[inline(always)]
    pub const fn pfsra0(self) -> crate::common::Reg<regs::Pfsra0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0402usize) as _) }
    }
    #[doc = "desc PCRA1."]
    #[inline(always)]
    pub const fn pcra1(self) -> crate::common::Reg<regs::Pcra1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "desc PFSRA1."]
    #[inline(always)]
    pub const fn pfsra1(self) -> crate::common::Reg<regs::Pfsra1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0406usize) as _) }
    }
    #[doc = "desc PCRA2."]
    #[inline(always)]
    pub const fn pcra2(self) -> crate::common::Reg<regs::Pcra2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "desc PFSRA2."]
    #[inline(always)]
    pub const fn pfsra2(self) -> crate::common::Reg<regs::Pfsra2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040ausize) as _) }
    }
    #[doc = "desc PCRA3."]
    #[inline(always)]
    pub const fn pcra3(self) -> crate::common::Reg<regs::Pcra3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "desc PFSRA3."]
    #[inline(always)]
    pub const fn pfsra3(self) -> crate::common::Reg<regs::Pfsra3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040eusize) as _) }
    }
    #[doc = "desc PCRA4."]
    #[inline(always)]
    pub const fn pcra4(self) -> crate::common::Reg<regs::Pcra4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "desc PFSRA4."]
    #[inline(always)]
    pub const fn pfsra4(self) -> crate::common::Reg<regs::Pfsra4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0412usize) as _) }
    }
    #[doc = "desc PCRA5."]
    #[inline(always)]
    pub const fn pcra5(self) -> crate::common::Reg<regs::Pcra5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "desc PFSRA5."]
    #[inline(always)]
    pub const fn pfsra5(self) -> crate::common::Reg<regs::Pfsra5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0416usize) as _) }
    }
    #[doc = "desc PCRA6."]
    #[inline(always)]
    pub const fn pcra6(self) -> crate::common::Reg<regs::Pcra6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "desc PFSRA6."]
    #[inline(always)]
    pub const fn pfsra6(self) -> crate::common::Reg<regs::Pfsra6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041ausize) as _) }
    }
    #[doc = "desc PCRA7."]
    #[inline(always)]
    pub const fn pcra7(self) -> crate::common::Reg<regs::Pcra7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "desc PFSRA7."]
    #[inline(always)]
    pub const fn pfsra7(self) -> crate::common::Reg<regs::Pfsra7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041eusize) as _) }
    }
    #[doc = "desc PCRA10."]
    #[inline(always)]
    pub const fn pcra10(self) -> crate::common::Reg<regs::Pcra10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "desc PFSRA10."]
    #[inline(always)]
    pub const fn pfsra10(self) -> crate::common::Reg<regs::Pfsra10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042ausize) as _) }
    }
    #[doc = "desc PCRA11."]
    #[inline(always)]
    pub const fn pcra11(self) -> crate::common::Reg<regs::Pcra11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "desc PFSRA11."]
    #[inline(always)]
    pub const fn pfsra11(self) -> crate::common::Reg<regs::Pfsra11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042eusize) as _) }
    }
    #[doc = "desc PCRA12."]
    #[inline(always)]
    pub const fn pcra12(self) -> crate::common::Reg<regs::Pcra12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "desc PFSRA12."]
    #[inline(always)]
    pub const fn pfsra12(self) -> crate::common::Reg<regs::Pfsra12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0432usize) as _) }
    }
    #[doc = "desc PCRA13."]
    #[inline(always)]
    pub const fn pcra13(self) -> crate::common::Reg<regs::Pcra13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "desc PFSRA13."]
    #[inline(always)]
    pub const fn pfsra13(self) -> crate::common::Reg<regs::Pfsra13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0436usize) as _) }
    }
    #[doc = "desc PCRB0."]
    #[inline(always)]
    pub const fn pcrb0(self) -> crate::common::Reg<regs::Pcrb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "desc PFSRB0."]
    #[inline(always)]
    pub const fn pfsrb0(self) -> crate::common::Reg<regs::Pfsrb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0482usize) as _) }
    }
    #[doc = "desc PCRB1."]
    #[inline(always)]
    pub const fn pcrb1(self) -> crate::common::Reg<regs::Pcrb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0484usize) as _) }
    }
    #[doc = "desc PFSRB1."]
    #[inline(always)]
    pub const fn pfsrb1(self) -> crate::common::Reg<regs::Pfsrb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0486usize) as _) }
    }
    #[doc = "desc PCRB2."]
    #[inline(always)]
    pub const fn pcrb2(self) -> crate::common::Reg<regs::Pcrb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0488usize) as _) }
    }
    #[doc = "desc PFSRB2."]
    #[inline(always)]
    pub const fn pfsrb2(self) -> crate::common::Reg<regs::Pfsrb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x048ausize) as _) }
    }
    #[doc = "desc PCRB3."]
    #[inline(always)]
    pub const fn pcrb3(self) -> crate::common::Reg<regs::Pcrb3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x048cusize) as _) }
    }
    #[doc = "desc PFSRB3."]
    #[inline(always)]
    pub const fn pfsrb3(self) -> crate::common::Reg<regs::Pfsrb3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x048eusize) as _) }
    }
    #[doc = "desc PCRB4."]
    #[inline(always)]
    pub const fn pcrb4(self) -> crate::common::Reg<regs::Pcrb4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0490usize) as _) }
    }
    #[doc = "desc PFSRB4."]
    #[inline(always)]
    pub const fn pfsrb4(self) -> crate::common::Reg<regs::Pfsrb4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0492usize) as _) }
    }
    #[doc = "desc PCRB5."]
    #[inline(always)]
    pub const fn pcrb5(self) -> crate::common::Reg<regs::Pcrb5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0494usize) as _) }
    }
    #[doc = "desc PFSRB5."]
    #[inline(always)]
    pub const fn pfsrb5(self) -> crate::common::Reg<regs::Pfsrb5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0496usize) as _) }
    }
    #[doc = "desc PCRB6."]
    #[inline(always)]
    pub const fn pcrb6(self) -> crate::common::Reg<regs::Pcrb6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0498usize) as _) }
    }
    #[doc = "desc PFSRB6."]
    #[inline(always)]
    pub const fn pfsrb6(self) -> crate::common::Reg<regs::Pfsrb6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x049ausize) as _) }
    }
    #[doc = "desc PCRB7."]
    #[inline(always)]
    pub const fn pcrb7(self) -> crate::common::Reg<regs::Pcrb7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x049cusize) as _) }
    }
    #[doc = "desc PFSRB7."]
    #[inline(always)]
    pub const fn pfsrb7(self) -> crate::common::Reg<regs::Pfsrb7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x049eusize) as _) }
    }
    #[doc = "desc PCRB12."]
    #[inline(always)]
    pub const fn pcrb12(self) -> crate::common::Reg<regs::Pcrb12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "desc PFSRB12."]
    #[inline(always)]
    pub const fn pfsrb12(self) -> crate::common::Reg<regs::Pfsrb12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b2usize) as _) }
    }
    #[doc = "desc PCRB13."]
    #[inline(always)]
    pub const fn pcrb13(self) -> crate::common::Reg<regs::Pcrb13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b4usize) as _) }
    }
    #[doc = "desc PFSRB13."]
    #[inline(always)]
    pub const fn pfsrb13(self) -> crate::common::Reg<regs::Pfsrb13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b6usize) as _) }
    }
    #[doc = "desc PCRC0."]
    #[inline(always)]
    pub const fn pcrc0(self) -> crate::common::Reg<regs::Pcrc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "desc PFSRC0."]
    #[inline(always)]
    pub const fn pfsrc0(self) -> crate::common::Reg<regs::Pfsrc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0502usize) as _) }
    }
    #[doc = "desc PCRC1."]
    #[inline(always)]
    pub const fn pcrc1(self) -> crate::common::Reg<regs::Pcrc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "desc PFSRC1."]
    #[inline(always)]
    pub const fn pfsrc1(self) -> crate::common::Reg<regs::Pfsrc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0506usize) as _) }
    }
    #[doc = "desc PCRC2."]
    #[inline(always)]
    pub const fn pcrc2(self) -> crate::common::Reg<regs::Pcrc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "desc PFSRC2."]
    #[inline(always)]
    pub const fn pfsrc2(self) -> crate::common::Reg<regs::Pfsrc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050ausize) as _) }
    }
    #[doc = "desc PCRC3."]
    #[inline(always)]
    pub const fn pcrc3(self) -> crate::common::Reg<regs::Pcrc3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "desc PFSRC3."]
    #[inline(always)]
    pub const fn pfsrc3(self) -> crate::common::Reg<regs::Pfsrc3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050eusize) as _) }
    }
    #[doc = "desc PCRC4."]
    #[inline(always)]
    pub const fn pcrc4(self) -> crate::common::Reg<regs::Pcrc4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "desc PFSRC4."]
    #[inline(always)]
    pub const fn pfsrc4(self) -> crate::common::Reg<regs::Pfsrc4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0512usize) as _) }
    }
    #[doc = "desc PCRC5."]
    #[inline(always)]
    pub const fn pcrc5(self) -> crate::common::Reg<regs::Pcrc5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "desc PFSRC5."]
    #[inline(always)]
    pub const fn pfsrc5(self) -> crate::common::Reg<regs::Pfsrc5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0516usize) as _) }
    }
    #[doc = "desc PCRC6."]
    #[inline(always)]
    pub const fn pcrc6(self) -> crate::common::Reg<regs::Pcrc6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "desc PFSRC6."]
    #[inline(always)]
    pub const fn pfsrc6(self) -> crate::common::Reg<regs::Pfsrc6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051ausize) as _) }
    }
    #[doc = "desc PCRC7."]
    #[inline(always)]
    pub const fn pcrc7(self) -> crate::common::Reg<regs::Pcrc7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "desc PFSRC7."]
    #[inline(always)]
    pub const fn pfsrc7(self) -> crate::common::Reg<regs::Pfsrc7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051eusize) as _) }
    }
    #[doc = "desc PCRC8."]
    #[inline(always)]
    pub const fn pcrc8(self) -> crate::common::Reg<regs::Pcrc8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "desc PFSRC8."]
    #[inline(always)]
    pub const fn pfsrc8(self) -> crate::common::Reg<regs::Pfsrc8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0522usize) as _) }
    }
    #[doc = "desc PCRC9."]
    #[inline(always)]
    pub const fn pcrc9(self) -> crate::common::Reg<regs::Pcrc9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "desc PFSRC9."]
    #[inline(always)]
    pub const fn pfsrc9(self) -> crate::common::Reg<regs::Pfsrc9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0526usize) as _) }
    }
    #[doc = "desc PCRC14."]
    #[inline(always)]
    pub const fn pcrc14(self) -> crate::common::Reg<regs::Pcrc14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "desc PFSRC14."]
    #[inline(always)]
    pub const fn pfsrc14(self) -> crate::common::Reg<regs::Pfsrc14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053ausize) as _) }
    }
    #[doc = "desc PCRC15."]
    #[inline(always)]
    pub const fn pcrc15(self) -> crate::common::Reg<regs::Pcrc15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "desc PFSRC15."]
    #[inline(always)]
    pub const fn pfsrc15(self) -> crate::common::Reg<regs::Pfsrc15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053eusize) as _) }
    }
    #[doc = "desc PCRC16."]
    #[inline(always)]
    pub const fn pcrc16(self) -> crate::common::Reg<regs::Pcrc16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "desc PFSRC16."]
    #[inline(always)]
    pub const fn pfsrc16(self) -> crate::common::Reg<regs::Pfsrc16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0542usize) as _) }
    }
    #[doc = "desc PCRC17."]
    #[inline(always)]
    pub const fn pcrc17(self) -> crate::common::Reg<regs::Pcrc17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "desc PFSRC17."]
    #[inline(always)]
    pub const fn pfsrc17(self) -> crate::common::Reg<regs::Pfsrc17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0546usize) as _) }
    }
    #[doc = "desc PCRD0."]
    #[inline(always)]
    pub const fn pcrd0(self) -> crate::common::Reg<regs::Pcrd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "desc PFSRD0."]
    #[inline(always)]
    pub const fn pfsrd0(self) -> crate::common::Reg<regs::Pfsrd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0582usize) as _) }
    }
    #[doc = "desc PCRD1."]
    #[inline(always)]
    pub const fn pcrd1(self) -> crate::common::Reg<regs::Pcrd1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "desc PFSRD1."]
    #[inline(always)]
    pub const fn pfsrd1(self) -> crate::common::Reg<regs::Pfsrd1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0586usize) as _) }
    }
    #[doc = "desc PCRD2."]
    #[inline(always)]
    pub const fn pcrd2(self) -> crate::common::Reg<regs::Pcrd2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "desc PFSRD2."]
    #[inline(always)]
    pub const fn pfsrd2(self) -> crate::common::Reg<regs::Pfsrd2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058ausize) as _) }
    }
    #[doc = "desc PCRD3."]
    #[inline(always)]
    pub const fn pcrd3(self) -> crate::common::Reg<regs::Pcrd3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "desc PFSRD3."]
    #[inline(always)]
    pub const fn pfsrd3(self) -> crate::common::Reg<regs::Pfsrd3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058eusize) as _) }
    }
    #[doc = "desc PCRD4."]
    #[inline(always)]
    pub const fn pcrd4(self) -> crate::common::Reg<regs::Pcrd4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "desc PFSRD4."]
    #[inline(always)]
    pub const fn pfsrd4(self) -> crate::common::Reg<regs::Pfsrd4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0592usize) as _) }
    }
    #[doc = "desc PCRD5."]
    #[inline(always)]
    pub const fn pcrd5(self) -> crate::common::Reg<regs::Pcrd5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "desc PFSRD5."]
    #[inline(always)]
    pub const fn pfsrd5(self) -> crate::common::Reg<regs::Pfsrd5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0596usize) as _) }
    }
    #[doc = "desc PCRD6."]
    #[inline(always)]
    pub const fn pcrd6(self) -> crate::common::Reg<regs::Pcrd6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0598usize) as _) }
    }
    #[doc = "desc PFSRD6."]
    #[inline(always)]
    pub const fn pfsrd6(self) -> crate::common::Reg<regs::Pfsrd6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x059ausize) as _) }
    }
    #[doc = "desc PCRD7."]
    #[inline(always)]
    pub const fn pcrd7(self) -> crate::common::Reg<regs::Pcrd7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x059cusize) as _) }
    }
    #[doc = "desc PFSRD7."]
    #[inline(always)]
    pub const fn pfsrd7(self) -> crate::common::Reg<regs::Pfsrd7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x059eusize) as _) }
    }
    #[doc = "desc PCRD15."]
    #[inline(always)]
    pub const fn pcrd15(self) -> crate::common::Reg<regs::Pcrd15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05bcusize) as _) }
    }
    #[doc = "desc PFSRD15."]
    #[inline(always)]
    pub const fn pfsrd15(self) -> crate::common::Reg<regs::Pfsrd15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05beusize) as _) }
    }
    #[doc = "desc PCRD16."]
    #[inline(always)]
    pub const fn pcrd16(self) -> crate::common::Reg<regs::Pcrd16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "desc PFSRD16."]
    #[inline(always)]
    pub const fn pfsrd16(self) -> crate::common::Reg<regs::Pfsrd16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c2usize) as _) }
    }
    #[doc = "desc PCRE0."]
    #[inline(always)]
    pub const fn pcre0(self) -> crate::common::Reg<regs::Pcre0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "desc PFSRE0."]
    #[inline(always)]
    pub const fn pfsre0(self) -> crate::common::Reg<regs::Pfsre0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0602usize) as _) }
    }
    #[doc = "desc PCRE1."]
    #[inline(always)]
    pub const fn pcre1(self) -> crate::common::Reg<regs::Pcre1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "desc PFSRE1."]
    #[inline(always)]
    pub const fn pfsre1(self) -> crate::common::Reg<regs::Pfsre1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0606usize) as _) }
    }
    #[doc = "desc PCRE2."]
    #[inline(always)]
    pub const fn pcre2(self) -> crate::common::Reg<regs::Pcre2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "desc PFSRE2."]
    #[inline(always)]
    pub const fn pfsre2(self) -> crate::common::Reg<regs::Pfsre2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060ausize) as _) }
    }
    #[doc = "desc PCRE3."]
    #[inline(always)]
    pub const fn pcre3(self) -> crate::common::Reg<regs::Pcre3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "desc PFSRE3."]
    #[inline(always)]
    pub const fn pfsre3(self) -> crate::common::Reg<regs::Pfsre3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060eusize) as _) }
    }
    #[doc = "desc PCRE4."]
    #[inline(always)]
    pub const fn pcre4(self) -> crate::common::Reg<regs::Pcre4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "desc PFSRE4."]
    #[inline(always)]
    pub const fn pfsre4(self) -> crate::common::Reg<regs::Pfsre4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0612usize) as _) }
    }
    #[doc = "desc PCRE5."]
    #[inline(always)]
    pub const fn pcre5(self) -> crate::common::Reg<regs::Pcre5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "desc PFSRE5."]
    #[inline(always)]
    pub const fn pfsre5(self) -> crate::common::Reg<regs::Pfsre5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0616usize) as _) }
    }
    #[doc = "desc PCRE6."]
    #[inline(always)]
    pub const fn pcre6(self) -> crate::common::Reg<regs::Pcre6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "desc PFSRE6."]
    #[inline(always)]
    pub const fn pfsre6(self) -> crate::common::Reg<regs::Pfsre6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061ausize) as _) }
    }
    #[doc = "desc PCRE7."]
    #[inline(always)]
    pub const fn pcre7(self) -> crate::common::Reg<regs::Pcre7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "desc PFSRE7."]
    #[inline(always)]
    pub const fn pfsre7(self) -> crate::common::Reg<regs::Pfsre7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061eusize) as _) }
    }
    #[doc = "desc PCRE8."]
    #[inline(always)]
    pub const fn pcre8(self) -> crate::common::Reg<regs::Pcre8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "desc PFSRE8."]
    #[inline(always)]
    pub const fn pfsre8(self) -> crate::common::Reg<regs::Pfsre8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0622usize) as _) }
    }
    #[doc = "desc PCRE9."]
    #[inline(always)]
    pub const fn pcre9(self) -> crate::common::Reg<regs::Pcre9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
    #[doc = "desc PFSRE9."]
    #[inline(always)]
    pub const fn pfsre9(self) -> crate::common::Reg<regs::Pfsre9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0626usize) as _) }
    }
    #[doc = "desc PCRE10."]
    #[inline(always)]
    pub const fn pcre10(self) -> crate::common::Reg<regs::Pcre10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0628usize) as _) }
    }
    #[doc = "desc PFSRE10."]
    #[inline(always)]
    pub const fn pfsre10(self) -> crate::common::Reg<regs::Pfsre10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x062ausize) as _) }
    }
    #[doc = "desc PCRE11."]
    #[inline(always)]
    pub const fn pcre11(self) -> crate::common::Reg<regs::Pcre11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x062cusize) as _) }
    }
    #[doc = "desc PFSRE11."]
    #[inline(always)]
    pub const fn pfsre11(self) -> crate::common::Reg<regs::Pfsre11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x062eusize) as _) }
    }
    #[doc = "desc PSPCR."]
    #[inline(always)]
    pub const fn pspcr(self) -> crate::common::Reg<regs::Pspcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "desc PCCR."]
    #[inline(always)]
    pub const fn pccr(self) -> crate::common::Reg<regs::Pccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "desc PINAER."]
    #[inline(always)]
    pub const fn pinaer(self) -> crate::common::Reg<regs::Pinaer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[doc = "desc PWPR."]
    #[inline(always)]
    pub const fn pwpr(self) -> crate::common::Reg<regs::Pwpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x070cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc PCCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pccr(pub u16);
    impl Pccr {
        #[doc = "desc BFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn bfsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc BFSEL."]
        #[inline(always)]
        pub const fn set_bfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc RDWT."]
        #[must_use]
        #[inline(always)]
        pub const fn rdwt(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc RDWT."]
        #[inline(always)]
        pub const fn set_rdwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
        }
    }
    impl Default for Pccr {
        #[inline(always)]
        fn default() -> Pccr {
            Pccr(0)
        }
    }
    impl core::fmt::Debug for Pccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pccr")
                .field("bfsel", &self.bfsel())
                .field("rdwt", &self.rdwt())
                .finish()
        }
    }
    #[doc = "desc PCRA0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcra0(pub u16);
    impl Pcra0 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcra0 {
        #[inline(always)]
        fn default() -> Pcra0 {
            Pcra0(0)
        }
    }
    impl core::fmt::Debug for Pcra0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcra0")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRA1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcra1(pub u16);
    impl Pcra1 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcra1 {
        #[inline(always)]
        fn default() -> Pcra1 {
            Pcra1(0)
        }
    }
    impl core::fmt::Debug for Pcra1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcra1")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRA10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcra10(pub u16);
    impl Pcra10 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcra10 {
        #[inline(always)]
        fn default() -> Pcra10 {
            Pcra10(0)
        }
    }
    impl core::fmt::Debug for Pcra10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcra10")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRA11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcra11(pub u16);
    impl Pcra11 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcra11 {
        #[inline(always)]
        fn default() -> Pcra11 {
            Pcra11(0)
        }
    }
    impl core::fmt::Debug for Pcra11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcra11")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRA12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcra12(pub u16);
    impl Pcra12 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcra12 {
        #[inline(always)]
        fn default() -> Pcra12 {
            Pcra12(0)
        }
    }
    impl core::fmt::Debug for Pcra12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcra12")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRA13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcra13(pub u16);
    impl Pcra13 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcra13 {
        #[inline(always)]
        fn default() -> Pcra13 {
            Pcra13(0)
        }
    }
    impl core::fmt::Debug for Pcra13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcra13")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRA2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcra2(pub u16);
    impl Pcra2 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcra2 {
        #[inline(always)]
        fn default() -> Pcra2 {
            Pcra2(0)
        }
    }
    impl core::fmt::Debug for Pcra2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcra2")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRA3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcra3(pub u16);
    impl Pcra3 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcra3 {
        #[inline(always)]
        fn default() -> Pcra3 {
            Pcra3(0)
        }
    }
    impl core::fmt::Debug for Pcra3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcra3")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRA4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcra4(pub u16);
    impl Pcra4 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcra4 {
        #[inline(always)]
        fn default() -> Pcra4 {
            Pcra4(0)
        }
    }
    impl core::fmt::Debug for Pcra4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcra4")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRA5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcra5(pub u16);
    impl Pcra5 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcra5 {
        #[inline(always)]
        fn default() -> Pcra5 {
            Pcra5(0)
        }
    }
    impl core::fmt::Debug for Pcra5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcra5")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRA6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcra6(pub u16);
    impl Pcra6 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcra6 {
        #[inline(always)]
        fn default() -> Pcra6 {
            Pcra6(0)
        }
    }
    impl core::fmt::Debug for Pcra6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcra6")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRA7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcra7(pub u16);
    impl Pcra7 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcra7 {
        #[inline(always)]
        fn default() -> Pcra7 {
            Pcra7(0)
        }
    }
    impl core::fmt::Debug for Pcra7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcra7")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRB0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrb0(pub u16);
    impl Pcrb0 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrb0 {
        #[inline(always)]
        fn default() -> Pcrb0 {
            Pcrb0(0)
        }
    }
    impl core::fmt::Debug for Pcrb0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrb0")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRB1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrb1(pub u16);
    impl Pcrb1 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrb1 {
        #[inline(always)]
        fn default() -> Pcrb1 {
            Pcrb1(0)
        }
    }
    impl core::fmt::Debug for Pcrb1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrb1")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRB12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrb12(pub u16);
    impl Pcrb12 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrb12 {
        #[inline(always)]
        fn default() -> Pcrb12 {
            Pcrb12(0)
        }
    }
    impl core::fmt::Debug for Pcrb12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrb12")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRB13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrb13(pub u16);
    impl Pcrb13 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrb13 {
        #[inline(always)]
        fn default() -> Pcrb13 {
            Pcrb13(0)
        }
    }
    impl core::fmt::Debug for Pcrb13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrb13")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRB2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrb2(pub u16);
    impl Pcrb2 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrb2 {
        #[inline(always)]
        fn default() -> Pcrb2 {
            Pcrb2(0)
        }
    }
    impl core::fmt::Debug for Pcrb2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrb2")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRB3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrb3(pub u16);
    impl Pcrb3 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrb3 {
        #[inline(always)]
        fn default() -> Pcrb3 {
            Pcrb3(0)
        }
    }
    impl core::fmt::Debug for Pcrb3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrb3")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRB4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrb4(pub u16);
    impl Pcrb4 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrb4 {
        #[inline(always)]
        fn default() -> Pcrb4 {
            Pcrb4(0)
        }
    }
    impl core::fmt::Debug for Pcrb4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrb4")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRB5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrb5(pub u16);
    impl Pcrb5 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrb5 {
        #[inline(always)]
        fn default() -> Pcrb5 {
            Pcrb5(0)
        }
    }
    impl core::fmt::Debug for Pcrb5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrb5")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRB6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrb6(pub u16);
    impl Pcrb6 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrb6 {
        #[inline(always)]
        fn default() -> Pcrb6 {
            Pcrb6(0)
        }
    }
    impl core::fmt::Debug for Pcrb6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrb6")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRB7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrb7(pub u16);
    impl Pcrb7 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrb7 {
        #[inline(always)]
        fn default() -> Pcrb7 {
            Pcrb7(0)
        }
    }
    impl core::fmt::Debug for Pcrb7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrb7")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc0(pub u16);
    impl Pcrc0 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc0 {
        #[inline(always)]
        fn default() -> Pcrc0 {
            Pcrc0(0)
        }
    }
    impl core::fmt::Debug for Pcrc0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc0")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc1(pub u16);
    impl Pcrc1 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc1 {
        #[inline(always)]
        fn default() -> Pcrc1 {
            Pcrc1(0)
        }
    }
    impl core::fmt::Debug for Pcrc1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc1")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc14(pub u16);
    impl Pcrc14 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc14 {
        #[inline(always)]
        fn default() -> Pcrc14 {
            Pcrc14(0)
        }
    }
    impl core::fmt::Debug for Pcrc14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc14")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc15(pub u16);
    impl Pcrc15 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc15 {
        #[inline(always)]
        fn default() -> Pcrc15 {
            Pcrc15(0)
        }
    }
    impl core::fmt::Debug for Pcrc15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc15")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC16."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc16(pub u16);
    impl Pcrc16 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc16 {
        #[inline(always)]
        fn default() -> Pcrc16 {
            Pcrc16(0)
        }
    }
    impl core::fmt::Debug for Pcrc16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc16")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC17."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc17(pub u16);
    impl Pcrc17 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc17 {
        #[inline(always)]
        fn default() -> Pcrc17 {
            Pcrc17(0)
        }
    }
    impl core::fmt::Debug for Pcrc17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc17")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc2(pub u16);
    impl Pcrc2 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc2 {
        #[inline(always)]
        fn default() -> Pcrc2 {
            Pcrc2(0)
        }
    }
    impl core::fmt::Debug for Pcrc2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc2")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc3(pub u16);
    impl Pcrc3 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc3 {
        #[inline(always)]
        fn default() -> Pcrc3 {
            Pcrc3(0)
        }
    }
    impl core::fmt::Debug for Pcrc3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc3")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc4(pub u16);
    impl Pcrc4 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc4 {
        #[inline(always)]
        fn default() -> Pcrc4 {
            Pcrc4(0)
        }
    }
    impl core::fmt::Debug for Pcrc4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc4")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc5(pub u16);
    impl Pcrc5 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc5 {
        #[inline(always)]
        fn default() -> Pcrc5 {
            Pcrc5(0)
        }
    }
    impl core::fmt::Debug for Pcrc5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc5")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc6(pub u16);
    impl Pcrc6 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc6 {
        #[inline(always)]
        fn default() -> Pcrc6 {
            Pcrc6(0)
        }
    }
    impl core::fmt::Debug for Pcrc6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc6")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc7(pub u16);
    impl Pcrc7 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc7 {
        #[inline(always)]
        fn default() -> Pcrc7 {
            Pcrc7(0)
        }
    }
    impl core::fmt::Debug for Pcrc7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc7")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc8(pub u16);
    impl Pcrc8 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc8 {
        #[inline(always)]
        fn default() -> Pcrc8 {
            Pcrc8(0)
        }
    }
    impl core::fmt::Debug for Pcrc8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc8")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRC9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrc9(pub u16);
    impl Pcrc9 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrc9 {
        #[inline(always)]
        fn default() -> Pcrc9 {
            Pcrc9(0)
        }
    }
    impl core::fmt::Debug for Pcrc9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrc9")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRD0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrd0(pub u16);
    impl Pcrd0 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrd0 {
        #[inline(always)]
        fn default() -> Pcrd0 {
            Pcrd0(0)
        }
    }
    impl core::fmt::Debug for Pcrd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrd0")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRD1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrd1(pub u16);
    impl Pcrd1 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrd1 {
        #[inline(always)]
        fn default() -> Pcrd1 {
            Pcrd1(0)
        }
    }
    impl core::fmt::Debug for Pcrd1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrd1")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRD15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrd15(pub u16);
    impl Pcrd15 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrd15 {
        #[inline(always)]
        fn default() -> Pcrd15 {
            Pcrd15(0)
        }
    }
    impl core::fmt::Debug for Pcrd15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrd15")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRD16."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrd16(pub u16);
    impl Pcrd16 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrd16 {
        #[inline(always)]
        fn default() -> Pcrd16 {
            Pcrd16(0)
        }
    }
    impl core::fmt::Debug for Pcrd16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrd16")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRD2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrd2(pub u16);
    impl Pcrd2 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrd2 {
        #[inline(always)]
        fn default() -> Pcrd2 {
            Pcrd2(0)
        }
    }
    impl core::fmt::Debug for Pcrd2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrd2")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRD3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrd3(pub u16);
    impl Pcrd3 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrd3 {
        #[inline(always)]
        fn default() -> Pcrd3 {
            Pcrd3(0)
        }
    }
    impl core::fmt::Debug for Pcrd3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrd3")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRD4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrd4(pub u16);
    impl Pcrd4 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrd4 {
        #[inline(always)]
        fn default() -> Pcrd4 {
            Pcrd4(0)
        }
    }
    impl core::fmt::Debug for Pcrd4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrd4")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRD5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrd5(pub u16);
    impl Pcrd5 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrd5 {
        #[inline(always)]
        fn default() -> Pcrd5 {
            Pcrd5(0)
        }
    }
    impl core::fmt::Debug for Pcrd5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrd5")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRD6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrd6(pub u16);
    impl Pcrd6 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrd6 {
        #[inline(always)]
        fn default() -> Pcrd6 {
            Pcrd6(0)
        }
    }
    impl core::fmt::Debug for Pcrd6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrd6")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRD7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrd7(pub u16);
    impl Pcrd7 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcrd7 {
        #[inline(always)]
        fn default() -> Pcrd7 {
            Pcrd7(0)
        }
    }
    impl core::fmt::Debug for Pcrd7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrd7")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRE0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcre0(pub u16);
    impl Pcre0 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcre0 {
        #[inline(always)]
        fn default() -> Pcre0 {
            Pcre0(0)
        }
    }
    impl core::fmt::Debug for Pcre0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcre0")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRE1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcre1(pub u16);
    impl Pcre1 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcre1 {
        #[inline(always)]
        fn default() -> Pcre1 {
            Pcre1(0)
        }
    }
    impl core::fmt::Debug for Pcre1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcre1")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRE10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcre10(pub u16);
    impl Pcre10 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcre10 {
        #[inline(always)]
        fn default() -> Pcre10 {
            Pcre10(0)
        }
    }
    impl core::fmt::Debug for Pcre10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcre10")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRE11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcre11(pub u16);
    impl Pcre11 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcre11 {
        #[inline(always)]
        fn default() -> Pcre11 {
            Pcre11(0)
        }
    }
    impl core::fmt::Debug for Pcre11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcre11")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRE2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcre2(pub u16);
    impl Pcre2 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcre2 {
        #[inline(always)]
        fn default() -> Pcre2 {
            Pcre2(0)
        }
    }
    impl core::fmt::Debug for Pcre2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcre2")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRE3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcre3(pub u16);
    impl Pcre3 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcre3 {
        #[inline(always)]
        fn default() -> Pcre3 {
            Pcre3(0)
        }
    }
    impl core::fmt::Debug for Pcre3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcre3")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRE4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcre4(pub u16);
    impl Pcre4 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcre4 {
        #[inline(always)]
        fn default() -> Pcre4 {
            Pcre4(0)
        }
    }
    impl core::fmt::Debug for Pcre4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcre4")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRE5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcre5(pub u16);
    impl Pcre5 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcre5 {
        #[inline(always)]
        fn default() -> Pcre5 {
            Pcre5(0)
        }
    }
    impl core::fmt::Debug for Pcre5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcre5")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRE6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcre6(pub u16);
    impl Pcre6 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcre6 {
        #[inline(always)]
        fn default() -> Pcre6 {
            Pcre6(0)
        }
    }
    impl core::fmt::Debug for Pcre6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcre6")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRE7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcre7(pub u16);
    impl Pcre7 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcre7 {
        #[inline(always)]
        fn default() -> Pcre7 {
            Pcre7(0)
        }
    }
    impl core::fmt::Debug for Pcre7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcre7")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRE8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcre8(pub u16);
    impl Pcre8 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcre8 {
        #[inline(always)]
        fn default() -> Pcre8 {
            Pcre8(0)
        }
    }
    impl core::fmt::Debug for Pcre8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcre8")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PCRE9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcre9(pub u16);
    impl Pcre9 {
        #[doc = "desc POUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT."]
        #[inline(always)]
        pub const fn set_pout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn poute(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE."]
        #[inline(always)]
        pub const fn set_poute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc NOD."]
        #[must_use]
        #[inline(always)]
        pub const fn nod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOD."]
        #[inline(always)]
        pub const fn set_nod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn drv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRV."]
        #[inline(always)]
        pub const fn set_drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc LTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lte(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTE."]
        #[inline(always)]
        pub const fn set_lte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc PUU."]
        #[must_use]
        #[inline(always)]
        pub const fn puu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUU."]
        #[inline(always)]
        pub const fn set_puu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc PUD."]
        #[must_use]
        #[inline(always)]
        pub const fn pud(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PUD."]
        #[inline(always)]
        pub const fn set_pud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc PIN."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc INVE."]
        #[must_use]
        #[inline(always)]
        pub const fn inve(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVE."]
        #[inline(always)]
        pub const fn set_inve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc CINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cinsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CINSEL."]
        #[inline(always)]
        pub const fn set_cinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc INTE."]
        #[must_use]
        #[inline(always)]
        pub const fn inte(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTE."]
        #[inline(always)]
        pub const fn set_inte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc LK."]
        #[must_use]
        #[inline(always)]
        pub const fn lk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LK."]
        #[inline(always)]
        pub const fn set_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Pcre9 {
        #[inline(always)]
        fn default() -> Pcre9 {
            Pcre9(0)
        }
    }
    impl core::fmt::Debug for Pcre9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcre9")
                .field("pout", &self.pout())
                .field("poute", &self.poute())
                .field("nod", &self.nod())
                .field("drv", &self.drv())
                .field("lte", &self.lte())
                .field("puu", &self.puu())
                .field("pud", &self.pud())
                .field("pin", &self.pin())
                .field("inve", &self.inve())
                .field("cinsel", &self.cinsel())
                .field("inte", &self.inte())
                .field("lk", &self.lk())
                .finish()
        }
    }
    #[doc = "desc PFSRA0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsra0(pub u16);
    impl Pfsra0 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsra0 {
        #[inline(always)]
        fn default() -> Pfsra0 {
            Pfsra0(0)
        }
    }
    impl core::fmt::Debug for Pfsra0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsra0")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRA1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsra1(pub u16);
    impl Pfsra1 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsra1 {
        #[inline(always)]
        fn default() -> Pfsra1 {
            Pfsra1(0)
        }
    }
    impl core::fmt::Debug for Pfsra1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsra1")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRA10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsra10(pub u16);
    impl Pfsra10 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsra10 {
        #[inline(always)]
        fn default() -> Pfsra10 {
            Pfsra10(0)
        }
    }
    impl core::fmt::Debug for Pfsra10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsra10")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRA11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsra11(pub u16);
    impl Pfsra11 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsra11 {
        #[inline(always)]
        fn default() -> Pfsra11 {
            Pfsra11(0)
        }
    }
    impl core::fmt::Debug for Pfsra11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsra11")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRA12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsra12(pub u16);
    impl Pfsra12 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsra12 {
        #[inline(always)]
        fn default() -> Pfsra12 {
            Pfsra12(0)
        }
    }
    impl core::fmt::Debug for Pfsra12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsra12")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRA13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsra13(pub u16);
    impl Pfsra13 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsra13 {
        #[inline(always)]
        fn default() -> Pfsra13 {
            Pfsra13(0)
        }
    }
    impl core::fmt::Debug for Pfsra13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsra13")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRA2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsra2(pub u16);
    impl Pfsra2 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsra2 {
        #[inline(always)]
        fn default() -> Pfsra2 {
            Pfsra2(0)
        }
    }
    impl core::fmt::Debug for Pfsra2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsra2")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRA3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsra3(pub u16);
    impl Pfsra3 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsra3 {
        #[inline(always)]
        fn default() -> Pfsra3 {
            Pfsra3(0)
        }
    }
    impl core::fmt::Debug for Pfsra3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsra3")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRA4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsra4(pub u16);
    impl Pfsra4 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsra4 {
        #[inline(always)]
        fn default() -> Pfsra4 {
            Pfsra4(0)
        }
    }
    impl core::fmt::Debug for Pfsra4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsra4")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRA5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsra5(pub u16);
    impl Pfsra5 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsra5 {
        #[inline(always)]
        fn default() -> Pfsra5 {
            Pfsra5(0)
        }
    }
    impl core::fmt::Debug for Pfsra5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsra5")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRA6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsra6(pub u16);
    impl Pfsra6 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsra6 {
        #[inline(always)]
        fn default() -> Pfsra6 {
            Pfsra6(0)
        }
    }
    impl core::fmt::Debug for Pfsra6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsra6")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRA7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsra7(pub u16);
    impl Pfsra7 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsra7 {
        #[inline(always)]
        fn default() -> Pfsra7 {
            Pfsra7(0)
        }
    }
    impl core::fmt::Debug for Pfsra7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsra7")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRB0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrb0(pub u16);
    impl Pfsrb0 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrb0 {
        #[inline(always)]
        fn default() -> Pfsrb0 {
            Pfsrb0(0)
        }
    }
    impl core::fmt::Debug for Pfsrb0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrb0")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRB1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrb1(pub u16);
    impl Pfsrb1 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrb1 {
        #[inline(always)]
        fn default() -> Pfsrb1 {
            Pfsrb1(0)
        }
    }
    impl core::fmt::Debug for Pfsrb1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrb1")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRB12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrb12(pub u16);
    impl Pfsrb12 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrb12 {
        #[inline(always)]
        fn default() -> Pfsrb12 {
            Pfsrb12(0)
        }
    }
    impl core::fmt::Debug for Pfsrb12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrb12")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRB13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrb13(pub u16);
    impl Pfsrb13 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrb13 {
        #[inline(always)]
        fn default() -> Pfsrb13 {
            Pfsrb13(0)
        }
    }
    impl core::fmt::Debug for Pfsrb13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrb13")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRB2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrb2(pub u16);
    impl Pfsrb2 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrb2 {
        #[inline(always)]
        fn default() -> Pfsrb2 {
            Pfsrb2(0)
        }
    }
    impl core::fmt::Debug for Pfsrb2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrb2")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRB3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrb3(pub u16);
    impl Pfsrb3 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrb3 {
        #[inline(always)]
        fn default() -> Pfsrb3 {
            Pfsrb3(0)
        }
    }
    impl core::fmt::Debug for Pfsrb3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrb3")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRB4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrb4(pub u16);
    impl Pfsrb4 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrb4 {
        #[inline(always)]
        fn default() -> Pfsrb4 {
            Pfsrb4(0)
        }
    }
    impl core::fmt::Debug for Pfsrb4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrb4")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRB5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrb5(pub u16);
    impl Pfsrb5 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrb5 {
        #[inline(always)]
        fn default() -> Pfsrb5 {
            Pfsrb5(0)
        }
    }
    impl core::fmt::Debug for Pfsrb5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrb5")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRB6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrb6(pub u16);
    impl Pfsrb6 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrb6 {
        #[inline(always)]
        fn default() -> Pfsrb6 {
            Pfsrb6(0)
        }
    }
    impl core::fmt::Debug for Pfsrb6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrb6")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRB7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrb7(pub u16);
    impl Pfsrb7 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrb7 {
        #[inline(always)]
        fn default() -> Pfsrb7 {
            Pfsrb7(0)
        }
    }
    impl core::fmt::Debug for Pfsrb7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrb7")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc0(pub u16);
    impl Pfsrc0 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc0 {
        #[inline(always)]
        fn default() -> Pfsrc0 {
            Pfsrc0(0)
        }
    }
    impl core::fmt::Debug for Pfsrc0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc0")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc1(pub u16);
    impl Pfsrc1 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc1 {
        #[inline(always)]
        fn default() -> Pfsrc1 {
            Pfsrc1(0)
        }
    }
    impl core::fmt::Debug for Pfsrc1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc1")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc14(pub u16);
    impl Pfsrc14 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc14 {
        #[inline(always)]
        fn default() -> Pfsrc14 {
            Pfsrc14(0)
        }
    }
    impl core::fmt::Debug for Pfsrc14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc14")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc15(pub u16);
    impl Pfsrc15 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc15 {
        #[inline(always)]
        fn default() -> Pfsrc15 {
            Pfsrc15(0)
        }
    }
    impl core::fmt::Debug for Pfsrc15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc15")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC16."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc16(pub u16);
    impl Pfsrc16 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc16 {
        #[inline(always)]
        fn default() -> Pfsrc16 {
            Pfsrc16(0)
        }
    }
    impl core::fmt::Debug for Pfsrc16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc16")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC17."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc17(pub u16);
    impl Pfsrc17 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc17 {
        #[inline(always)]
        fn default() -> Pfsrc17 {
            Pfsrc17(0)
        }
    }
    impl core::fmt::Debug for Pfsrc17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc17")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc2(pub u16);
    impl Pfsrc2 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc2 {
        #[inline(always)]
        fn default() -> Pfsrc2 {
            Pfsrc2(0)
        }
    }
    impl core::fmt::Debug for Pfsrc2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc2")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc3(pub u16);
    impl Pfsrc3 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc3 {
        #[inline(always)]
        fn default() -> Pfsrc3 {
            Pfsrc3(0)
        }
    }
    impl core::fmt::Debug for Pfsrc3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc3")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc4(pub u16);
    impl Pfsrc4 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc4 {
        #[inline(always)]
        fn default() -> Pfsrc4 {
            Pfsrc4(0)
        }
    }
    impl core::fmt::Debug for Pfsrc4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc4")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc5(pub u16);
    impl Pfsrc5 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc5 {
        #[inline(always)]
        fn default() -> Pfsrc5 {
            Pfsrc5(0)
        }
    }
    impl core::fmt::Debug for Pfsrc5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc5")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc6(pub u16);
    impl Pfsrc6 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc6 {
        #[inline(always)]
        fn default() -> Pfsrc6 {
            Pfsrc6(0)
        }
    }
    impl core::fmt::Debug for Pfsrc6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc6")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc7(pub u16);
    impl Pfsrc7 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc7 {
        #[inline(always)]
        fn default() -> Pfsrc7 {
            Pfsrc7(0)
        }
    }
    impl core::fmt::Debug for Pfsrc7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc7")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc8(pub u16);
    impl Pfsrc8 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc8 {
        #[inline(always)]
        fn default() -> Pfsrc8 {
            Pfsrc8(0)
        }
    }
    impl core::fmt::Debug for Pfsrc8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc8")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRC9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrc9(pub u16);
    impl Pfsrc9 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrc9 {
        #[inline(always)]
        fn default() -> Pfsrc9 {
            Pfsrc9(0)
        }
    }
    impl core::fmt::Debug for Pfsrc9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrc9")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRD0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrd0(pub u16);
    impl Pfsrd0 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrd0 {
        #[inline(always)]
        fn default() -> Pfsrd0 {
            Pfsrd0(0)
        }
    }
    impl core::fmt::Debug for Pfsrd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrd0")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRD1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrd1(pub u16);
    impl Pfsrd1 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrd1 {
        #[inline(always)]
        fn default() -> Pfsrd1 {
            Pfsrd1(0)
        }
    }
    impl core::fmt::Debug for Pfsrd1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrd1")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRD15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrd15(pub u16);
    impl Pfsrd15 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrd15 {
        #[inline(always)]
        fn default() -> Pfsrd15 {
            Pfsrd15(0)
        }
    }
    impl core::fmt::Debug for Pfsrd15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrd15")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRD16."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrd16(pub u16);
    impl Pfsrd16 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrd16 {
        #[inline(always)]
        fn default() -> Pfsrd16 {
            Pfsrd16(0)
        }
    }
    impl core::fmt::Debug for Pfsrd16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrd16")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRD2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrd2(pub u16);
    impl Pfsrd2 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrd2 {
        #[inline(always)]
        fn default() -> Pfsrd2 {
            Pfsrd2(0)
        }
    }
    impl core::fmt::Debug for Pfsrd2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrd2")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRD3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrd3(pub u16);
    impl Pfsrd3 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrd3 {
        #[inline(always)]
        fn default() -> Pfsrd3 {
            Pfsrd3(0)
        }
    }
    impl core::fmt::Debug for Pfsrd3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrd3")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRD4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrd4(pub u16);
    impl Pfsrd4 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrd4 {
        #[inline(always)]
        fn default() -> Pfsrd4 {
            Pfsrd4(0)
        }
    }
    impl core::fmt::Debug for Pfsrd4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrd4")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRD5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrd5(pub u16);
    impl Pfsrd5 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrd5 {
        #[inline(always)]
        fn default() -> Pfsrd5 {
            Pfsrd5(0)
        }
    }
    impl core::fmt::Debug for Pfsrd5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrd5")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRD6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrd6(pub u16);
    impl Pfsrd6 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrd6 {
        #[inline(always)]
        fn default() -> Pfsrd6 {
            Pfsrd6(0)
        }
    }
    impl core::fmt::Debug for Pfsrd6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrd6")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRD7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsrd7(pub u16);
    impl Pfsrd7 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsrd7 {
        #[inline(always)]
        fn default() -> Pfsrd7 {
            Pfsrd7(0)
        }
    }
    impl core::fmt::Debug for Pfsrd7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsrd7")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRE0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsre0(pub u16);
    impl Pfsre0 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsre0 {
        #[inline(always)]
        fn default() -> Pfsre0 {
            Pfsre0(0)
        }
    }
    impl core::fmt::Debug for Pfsre0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsre0")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRE1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsre1(pub u16);
    impl Pfsre1 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsre1 {
        #[inline(always)]
        fn default() -> Pfsre1 {
            Pfsre1(0)
        }
    }
    impl core::fmt::Debug for Pfsre1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsre1")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRE10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsre10(pub u16);
    impl Pfsre10 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsre10 {
        #[inline(always)]
        fn default() -> Pfsre10 {
            Pfsre10(0)
        }
    }
    impl core::fmt::Debug for Pfsre10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsre10")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRE11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsre11(pub u16);
    impl Pfsre11 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsre11 {
        #[inline(always)]
        fn default() -> Pfsre11 {
            Pfsre11(0)
        }
    }
    impl core::fmt::Debug for Pfsre11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsre11")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRE2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsre2(pub u16);
    impl Pfsre2 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsre2 {
        #[inline(always)]
        fn default() -> Pfsre2 {
            Pfsre2(0)
        }
    }
    impl core::fmt::Debug for Pfsre2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsre2")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRE3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsre3(pub u16);
    impl Pfsre3 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsre3 {
        #[inline(always)]
        fn default() -> Pfsre3 {
            Pfsre3(0)
        }
    }
    impl core::fmt::Debug for Pfsre3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsre3")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRE4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsre4(pub u16);
    impl Pfsre4 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsre4 {
        #[inline(always)]
        fn default() -> Pfsre4 {
            Pfsre4(0)
        }
    }
    impl core::fmt::Debug for Pfsre4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsre4")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRE5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsre5(pub u16);
    impl Pfsre5 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsre5 {
        #[inline(always)]
        fn default() -> Pfsre5 {
            Pfsre5(0)
        }
    }
    impl core::fmt::Debug for Pfsre5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsre5")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRE6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsre6(pub u16);
    impl Pfsre6 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsre6 {
        #[inline(always)]
        fn default() -> Pfsre6 {
            Pfsre6(0)
        }
    }
    impl core::fmt::Debug for Pfsre6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsre6")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRE7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsre7(pub u16);
    impl Pfsre7 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsre7 {
        #[inline(always)]
        fn default() -> Pfsre7 {
            Pfsre7(0)
        }
    }
    impl core::fmt::Debug for Pfsre7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsre7")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRE8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsre8(pub u16);
    impl Pfsre8 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsre8 {
        #[inline(always)]
        fn default() -> Pfsre8 {
            Pfsre8(0)
        }
    }
    impl core::fmt::Debug for Pfsre8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsre8")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PFSRE9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfsre9(pub u16);
    impl Pfsre9 {
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "desc BFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BFE."]
        #[inline(always)]
        pub const fn set_bfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for Pfsre9 {
        #[inline(always)]
        fn default() -> Pfsre9 {
            Pfsre9(0)
        }
    }
    impl core::fmt::Debug for Pfsre9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfsre9")
                .field("fsel", &self.fsel())
                .field("bfe", &self.bfe())
                .finish()
        }
    }
    #[doc = "desc PIDRA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidra(pub u32);
    impl Pidra {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc PIN16."]
        #[must_use]
        #[inline(always)]
        pub const fn pin16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN16."]
        #[inline(always)]
        pub const fn set_pin16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc PIN17."]
        #[must_use]
        #[inline(always)]
        pub const fn pin17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN17."]
        #[inline(always)]
        pub const fn set_pin17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Pidra {
        #[inline(always)]
        fn default() -> Pidra {
            Pidra(0)
        }
    }
    impl core::fmt::Debug for Pidra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidra")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .field("pin16", &self.pin16())
                .field("pin17", &self.pin17())
                .finish()
        }
    }
    #[doc = "desc PIDRB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidrb(pub u32);
    impl Pidrb {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc PIN16."]
        #[must_use]
        #[inline(always)]
        pub const fn pin16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN16."]
        #[inline(always)]
        pub const fn set_pin16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc PIN17."]
        #[must_use]
        #[inline(always)]
        pub const fn pin17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN17."]
        #[inline(always)]
        pub const fn set_pin17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Pidrb {
        #[inline(always)]
        fn default() -> Pidrb {
            Pidrb(0)
        }
    }
    impl core::fmt::Debug for Pidrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidrb")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .field("pin16", &self.pin16())
                .field("pin17", &self.pin17())
                .finish()
        }
    }
    #[doc = "desc PIDRC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidrc(pub u32);
    impl Pidrc {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc PIN16."]
        #[must_use]
        #[inline(always)]
        pub const fn pin16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN16."]
        #[inline(always)]
        pub const fn set_pin16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc PIN17."]
        #[must_use]
        #[inline(always)]
        pub const fn pin17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN17."]
        #[inline(always)]
        pub const fn set_pin17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Pidrc {
        #[inline(always)]
        fn default() -> Pidrc {
            Pidrc(0)
        }
    }
    impl core::fmt::Debug for Pidrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidrc")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .field("pin16", &self.pin16())
                .field("pin17", &self.pin17())
                .finish()
        }
    }
    #[doc = "desc PIDRD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidrd(pub u32);
    impl Pidrd {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc PIN16."]
        #[must_use]
        #[inline(always)]
        pub const fn pin16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN16."]
        #[inline(always)]
        pub const fn set_pin16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc PIN17."]
        #[must_use]
        #[inline(always)]
        pub const fn pin17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN17."]
        #[inline(always)]
        pub const fn set_pin17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Pidrd {
        #[inline(always)]
        fn default() -> Pidrd {
            Pidrd(0)
        }
    }
    impl core::fmt::Debug for Pidrd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidrd")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .field("pin16", &self.pin16())
                .field("pin17", &self.pin17())
                .finish()
        }
    }
    #[doc = "desc PIDRE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidre(pub u32);
    impl Pidre {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc PIN16."]
        #[must_use]
        #[inline(always)]
        pub const fn pin16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN16."]
        #[inline(always)]
        pub const fn set_pin16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc PIN17."]
        #[must_use]
        #[inline(always)]
        pub const fn pin17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN17."]
        #[inline(always)]
        pub const fn set_pin17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Pidre {
        #[inline(always)]
        fn default() -> Pidre {
            Pidre(0)
        }
    }
    impl core::fmt::Debug for Pidre {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidre")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .field("pin16", &self.pin16())
                .field("pin17", &self.pin17())
                .finish()
        }
    }
    #[doc = "desc PINAER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pinaer(pub u32);
    impl Pinaer {
        #[doc = "desc PINAE."]
        #[must_use]
        #[inline(always)]
        pub const fn pinae(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc PINAE."]
        #[inline(always)]
        pub const fn set_pinae(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Pinaer {
        #[inline(always)]
        fn default() -> Pinaer {
            Pinaer(0)
        }
    }
    impl core::fmt::Debug for Pinaer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pinaer")
                .field("pinae", &self.pinae())
                .finish()
        }
    }
    #[doc = "desc PODRA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Podra(pub u32);
    impl Podra {
        #[doc = "desc POUT0."]
        #[must_use]
        #[inline(always)]
        pub const fn pout0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT0."]
        #[inline(always)]
        pub const fn set_pout0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POUT1."]
        #[must_use]
        #[inline(always)]
        pub const fn pout1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT1."]
        #[inline(always)]
        pub const fn set_pout1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POUT2."]
        #[must_use]
        #[inline(always)]
        pub const fn pout2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT2."]
        #[inline(always)]
        pub const fn set_pout2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POUT3."]
        #[must_use]
        #[inline(always)]
        pub const fn pout3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT3."]
        #[inline(always)]
        pub const fn set_pout3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POUT4."]
        #[must_use]
        #[inline(always)]
        pub const fn pout4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT4."]
        #[inline(always)]
        pub const fn set_pout4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POUT5."]
        #[must_use]
        #[inline(always)]
        pub const fn pout5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT5."]
        #[inline(always)]
        pub const fn set_pout5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POUT6."]
        #[must_use]
        #[inline(always)]
        pub const fn pout6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT6."]
        #[inline(always)]
        pub const fn set_pout6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POUT7."]
        #[must_use]
        #[inline(always)]
        pub const fn pout7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT7."]
        #[inline(always)]
        pub const fn set_pout7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POUT8."]
        #[must_use]
        #[inline(always)]
        pub const fn pout8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT8."]
        #[inline(always)]
        pub const fn set_pout8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POUT9."]
        #[must_use]
        #[inline(always)]
        pub const fn pout9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT9."]
        #[inline(always)]
        pub const fn set_pout9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POUT10."]
        #[must_use]
        #[inline(always)]
        pub const fn pout10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT10."]
        #[inline(always)]
        pub const fn set_pout10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POUT11."]
        #[must_use]
        #[inline(always)]
        pub const fn pout11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT11."]
        #[inline(always)]
        pub const fn set_pout11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POUT12."]
        #[must_use]
        #[inline(always)]
        pub const fn pout12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT12."]
        #[inline(always)]
        pub const fn set_pout12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POUT13."]
        #[must_use]
        #[inline(always)]
        pub const fn pout13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT13."]
        #[inline(always)]
        pub const fn set_pout13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POUT14."]
        #[must_use]
        #[inline(always)]
        pub const fn pout14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT14."]
        #[inline(always)]
        pub const fn set_pout14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POUT15."]
        #[must_use]
        #[inline(always)]
        pub const fn pout15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT15."]
        #[inline(always)]
        pub const fn set_pout15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POUT16."]
        #[must_use]
        #[inline(always)]
        pub const fn pout16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT16."]
        #[inline(always)]
        pub const fn set_pout16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POUT17."]
        #[must_use]
        #[inline(always)]
        pub const fn pout17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT17."]
        #[inline(always)]
        pub const fn set_pout17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Podra {
        #[inline(always)]
        fn default() -> Podra {
            Podra(0)
        }
    }
    impl core::fmt::Debug for Podra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Podra")
                .field("pout0", &self.pout0())
                .field("pout1", &self.pout1())
                .field("pout2", &self.pout2())
                .field("pout3", &self.pout3())
                .field("pout4", &self.pout4())
                .field("pout5", &self.pout5())
                .field("pout6", &self.pout6())
                .field("pout7", &self.pout7())
                .field("pout8", &self.pout8())
                .field("pout9", &self.pout9())
                .field("pout10", &self.pout10())
                .field("pout11", &self.pout11())
                .field("pout12", &self.pout12())
                .field("pout13", &self.pout13())
                .field("pout14", &self.pout14())
                .field("pout15", &self.pout15())
                .field("pout16", &self.pout16())
                .field("pout17", &self.pout17())
                .finish()
        }
    }
    #[doc = "desc PODRB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Podrb(pub u32);
    impl Podrb {
        #[doc = "desc POUT0."]
        #[must_use]
        #[inline(always)]
        pub const fn pout0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT0."]
        #[inline(always)]
        pub const fn set_pout0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POUT1."]
        #[must_use]
        #[inline(always)]
        pub const fn pout1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT1."]
        #[inline(always)]
        pub const fn set_pout1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POUT2."]
        #[must_use]
        #[inline(always)]
        pub const fn pout2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT2."]
        #[inline(always)]
        pub const fn set_pout2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POUT3."]
        #[must_use]
        #[inline(always)]
        pub const fn pout3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT3."]
        #[inline(always)]
        pub const fn set_pout3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POUT4."]
        #[must_use]
        #[inline(always)]
        pub const fn pout4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT4."]
        #[inline(always)]
        pub const fn set_pout4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POUT5."]
        #[must_use]
        #[inline(always)]
        pub const fn pout5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT5."]
        #[inline(always)]
        pub const fn set_pout5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POUT6."]
        #[must_use]
        #[inline(always)]
        pub const fn pout6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT6."]
        #[inline(always)]
        pub const fn set_pout6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POUT7."]
        #[must_use]
        #[inline(always)]
        pub const fn pout7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT7."]
        #[inline(always)]
        pub const fn set_pout7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POUT8."]
        #[must_use]
        #[inline(always)]
        pub const fn pout8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT8."]
        #[inline(always)]
        pub const fn set_pout8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POUT9."]
        #[must_use]
        #[inline(always)]
        pub const fn pout9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT9."]
        #[inline(always)]
        pub const fn set_pout9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POUT10."]
        #[must_use]
        #[inline(always)]
        pub const fn pout10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT10."]
        #[inline(always)]
        pub const fn set_pout10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POUT11."]
        #[must_use]
        #[inline(always)]
        pub const fn pout11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT11."]
        #[inline(always)]
        pub const fn set_pout11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POUT12."]
        #[must_use]
        #[inline(always)]
        pub const fn pout12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT12."]
        #[inline(always)]
        pub const fn set_pout12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POUT13."]
        #[must_use]
        #[inline(always)]
        pub const fn pout13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT13."]
        #[inline(always)]
        pub const fn set_pout13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POUT14."]
        #[must_use]
        #[inline(always)]
        pub const fn pout14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT14."]
        #[inline(always)]
        pub const fn set_pout14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POUT15."]
        #[must_use]
        #[inline(always)]
        pub const fn pout15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT15."]
        #[inline(always)]
        pub const fn set_pout15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POUT16."]
        #[must_use]
        #[inline(always)]
        pub const fn pout16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT16."]
        #[inline(always)]
        pub const fn set_pout16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POUT17."]
        #[must_use]
        #[inline(always)]
        pub const fn pout17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT17."]
        #[inline(always)]
        pub const fn set_pout17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Podrb {
        #[inline(always)]
        fn default() -> Podrb {
            Podrb(0)
        }
    }
    impl core::fmt::Debug for Podrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Podrb")
                .field("pout0", &self.pout0())
                .field("pout1", &self.pout1())
                .field("pout2", &self.pout2())
                .field("pout3", &self.pout3())
                .field("pout4", &self.pout4())
                .field("pout5", &self.pout5())
                .field("pout6", &self.pout6())
                .field("pout7", &self.pout7())
                .field("pout8", &self.pout8())
                .field("pout9", &self.pout9())
                .field("pout10", &self.pout10())
                .field("pout11", &self.pout11())
                .field("pout12", &self.pout12())
                .field("pout13", &self.pout13())
                .field("pout14", &self.pout14())
                .field("pout15", &self.pout15())
                .field("pout16", &self.pout16())
                .field("pout17", &self.pout17())
                .finish()
        }
    }
    #[doc = "desc PODRC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Podrc(pub u32);
    impl Podrc {
        #[doc = "desc POUT0."]
        #[must_use]
        #[inline(always)]
        pub const fn pout0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT0."]
        #[inline(always)]
        pub const fn set_pout0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POUT1."]
        #[must_use]
        #[inline(always)]
        pub const fn pout1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT1."]
        #[inline(always)]
        pub const fn set_pout1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POUT2."]
        #[must_use]
        #[inline(always)]
        pub const fn pout2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT2."]
        #[inline(always)]
        pub const fn set_pout2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POUT3."]
        #[must_use]
        #[inline(always)]
        pub const fn pout3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT3."]
        #[inline(always)]
        pub const fn set_pout3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POUT4."]
        #[must_use]
        #[inline(always)]
        pub const fn pout4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT4."]
        #[inline(always)]
        pub const fn set_pout4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POUT5."]
        #[must_use]
        #[inline(always)]
        pub const fn pout5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT5."]
        #[inline(always)]
        pub const fn set_pout5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POUT6."]
        #[must_use]
        #[inline(always)]
        pub const fn pout6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT6."]
        #[inline(always)]
        pub const fn set_pout6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POUT7."]
        #[must_use]
        #[inline(always)]
        pub const fn pout7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT7."]
        #[inline(always)]
        pub const fn set_pout7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POUT8."]
        #[must_use]
        #[inline(always)]
        pub const fn pout8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT8."]
        #[inline(always)]
        pub const fn set_pout8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POUT9."]
        #[must_use]
        #[inline(always)]
        pub const fn pout9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT9."]
        #[inline(always)]
        pub const fn set_pout9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POUT10."]
        #[must_use]
        #[inline(always)]
        pub const fn pout10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT10."]
        #[inline(always)]
        pub const fn set_pout10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POUT11."]
        #[must_use]
        #[inline(always)]
        pub const fn pout11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT11."]
        #[inline(always)]
        pub const fn set_pout11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POUT12."]
        #[must_use]
        #[inline(always)]
        pub const fn pout12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT12."]
        #[inline(always)]
        pub const fn set_pout12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POUT13."]
        #[must_use]
        #[inline(always)]
        pub const fn pout13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT13."]
        #[inline(always)]
        pub const fn set_pout13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POUT14."]
        #[must_use]
        #[inline(always)]
        pub const fn pout14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT14."]
        #[inline(always)]
        pub const fn set_pout14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POUT15."]
        #[must_use]
        #[inline(always)]
        pub const fn pout15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT15."]
        #[inline(always)]
        pub const fn set_pout15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POUT16."]
        #[must_use]
        #[inline(always)]
        pub const fn pout16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT16."]
        #[inline(always)]
        pub const fn set_pout16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POUT17."]
        #[must_use]
        #[inline(always)]
        pub const fn pout17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT17."]
        #[inline(always)]
        pub const fn set_pout17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Podrc {
        #[inline(always)]
        fn default() -> Podrc {
            Podrc(0)
        }
    }
    impl core::fmt::Debug for Podrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Podrc")
                .field("pout0", &self.pout0())
                .field("pout1", &self.pout1())
                .field("pout2", &self.pout2())
                .field("pout3", &self.pout3())
                .field("pout4", &self.pout4())
                .field("pout5", &self.pout5())
                .field("pout6", &self.pout6())
                .field("pout7", &self.pout7())
                .field("pout8", &self.pout8())
                .field("pout9", &self.pout9())
                .field("pout10", &self.pout10())
                .field("pout11", &self.pout11())
                .field("pout12", &self.pout12())
                .field("pout13", &self.pout13())
                .field("pout14", &self.pout14())
                .field("pout15", &self.pout15())
                .field("pout16", &self.pout16())
                .field("pout17", &self.pout17())
                .finish()
        }
    }
    #[doc = "desc PODRD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Podrd(pub u32);
    impl Podrd {
        #[doc = "desc POUT0."]
        #[must_use]
        #[inline(always)]
        pub const fn pout0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT0."]
        #[inline(always)]
        pub const fn set_pout0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POUT1."]
        #[must_use]
        #[inline(always)]
        pub const fn pout1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT1."]
        #[inline(always)]
        pub const fn set_pout1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POUT2."]
        #[must_use]
        #[inline(always)]
        pub const fn pout2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT2."]
        #[inline(always)]
        pub const fn set_pout2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POUT3."]
        #[must_use]
        #[inline(always)]
        pub const fn pout3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT3."]
        #[inline(always)]
        pub const fn set_pout3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POUT4."]
        #[must_use]
        #[inline(always)]
        pub const fn pout4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT4."]
        #[inline(always)]
        pub const fn set_pout4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POUT5."]
        #[must_use]
        #[inline(always)]
        pub const fn pout5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT5."]
        #[inline(always)]
        pub const fn set_pout5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POUT6."]
        #[must_use]
        #[inline(always)]
        pub const fn pout6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT6."]
        #[inline(always)]
        pub const fn set_pout6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POUT7."]
        #[must_use]
        #[inline(always)]
        pub const fn pout7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT7."]
        #[inline(always)]
        pub const fn set_pout7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POUT8."]
        #[must_use]
        #[inline(always)]
        pub const fn pout8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT8."]
        #[inline(always)]
        pub const fn set_pout8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POUT9."]
        #[must_use]
        #[inline(always)]
        pub const fn pout9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT9."]
        #[inline(always)]
        pub const fn set_pout9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POUT10."]
        #[must_use]
        #[inline(always)]
        pub const fn pout10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT10."]
        #[inline(always)]
        pub const fn set_pout10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POUT11."]
        #[must_use]
        #[inline(always)]
        pub const fn pout11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT11."]
        #[inline(always)]
        pub const fn set_pout11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POUT12."]
        #[must_use]
        #[inline(always)]
        pub const fn pout12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT12."]
        #[inline(always)]
        pub const fn set_pout12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POUT13."]
        #[must_use]
        #[inline(always)]
        pub const fn pout13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT13."]
        #[inline(always)]
        pub const fn set_pout13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POUT14."]
        #[must_use]
        #[inline(always)]
        pub const fn pout14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT14."]
        #[inline(always)]
        pub const fn set_pout14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POUT15."]
        #[must_use]
        #[inline(always)]
        pub const fn pout15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT15."]
        #[inline(always)]
        pub const fn set_pout15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POUT16."]
        #[must_use]
        #[inline(always)]
        pub const fn pout16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT16."]
        #[inline(always)]
        pub const fn set_pout16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POUT17."]
        #[must_use]
        #[inline(always)]
        pub const fn pout17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT17."]
        #[inline(always)]
        pub const fn set_pout17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Podrd {
        #[inline(always)]
        fn default() -> Podrd {
            Podrd(0)
        }
    }
    impl core::fmt::Debug for Podrd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Podrd")
                .field("pout0", &self.pout0())
                .field("pout1", &self.pout1())
                .field("pout2", &self.pout2())
                .field("pout3", &self.pout3())
                .field("pout4", &self.pout4())
                .field("pout5", &self.pout5())
                .field("pout6", &self.pout6())
                .field("pout7", &self.pout7())
                .field("pout8", &self.pout8())
                .field("pout9", &self.pout9())
                .field("pout10", &self.pout10())
                .field("pout11", &self.pout11())
                .field("pout12", &self.pout12())
                .field("pout13", &self.pout13())
                .field("pout14", &self.pout14())
                .field("pout15", &self.pout15())
                .field("pout16", &self.pout16())
                .field("pout17", &self.pout17())
                .finish()
        }
    }
    #[doc = "desc PODRE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Podre(pub u32);
    impl Podre {
        #[doc = "desc POUT0."]
        #[must_use]
        #[inline(always)]
        pub const fn pout0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT0."]
        #[inline(always)]
        pub const fn set_pout0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POUT1."]
        #[must_use]
        #[inline(always)]
        pub const fn pout1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT1."]
        #[inline(always)]
        pub const fn set_pout1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POUT2."]
        #[must_use]
        #[inline(always)]
        pub const fn pout2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT2."]
        #[inline(always)]
        pub const fn set_pout2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POUT3."]
        #[must_use]
        #[inline(always)]
        pub const fn pout3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT3."]
        #[inline(always)]
        pub const fn set_pout3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POUT4."]
        #[must_use]
        #[inline(always)]
        pub const fn pout4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT4."]
        #[inline(always)]
        pub const fn set_pout4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POUT5."]
        #[must_use]
        #[inline(always)]
        pub const fn pout5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT5."]
        #[inline(always)]
        pub const fn set_pout5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POUT6."]
        #[must_use]
        #[inline(always)]
        pub const fn pout6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT6."]
        #[inline(always)]
        pub const fn set_pout6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POUT7."]
        #[must_use]
        #[inline(always)]
        pub const fn pout7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT7."]
        #[inline(always)]
        pub const fn set_pout7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POUT8."]
        #[must_use]
        #[inline(always)]
        pub const fn pout8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT8."]
        #[inline(always)]
        pub const fn set_pout8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POUT9."]
        #[must_use]
        #[inline(always)]
        pub const fn pout9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT9."]
        #[inline(always)]
        pub const fn set_pout9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POUT10."]
        #[must_use]
        #[inline(always)]
        pub const fn pout10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT10."]
        #[inline(always)]
        pub const fn set_pout10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POUT11."]
        #[must_use]
        #[inline(always)]
        pub const fn pout11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT11."]
        #[inline(always)]
        pub const fn set_pout11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POUT12."]
        #[must_use]
        #[inline(always)]
        pub const fn pout12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT12."]
        #[inline(always)]
        pub const fn set_pout12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POUT13."]
        #[must_use]
        #[inline(always)]
        pub const fn pout13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT13."]
        #[inline(always)]
        pub const fn set_pout13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POUT14."]
        #[must_use]
        #[inline(always)]
        pub const fn pout14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT14."]
        #[inline(always)]
        pub const fn set_pout14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POUT15."]
        #[must_use]
        #[inline(always)]
        pub const fn pout15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT15."]
        #[inline(always)]
        pub const fn set_pout15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POUT16."]
        #[must_use]
        #[inline(always)]
        pub const fn pout16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT16."]
        #[inline(always)]
        pub const fn set_pout16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POUT17."]
        #[must_use]
        #[inline(always)]
        pub const fn pout17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUT17."]
        #[inline(always)]
        pub const fn set_pout17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Podre {
        #[inline(always)]
        fn default() -> Podre {
            Podre(0)
        }
    }
    impl core::fmt::Debug for Podre {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Podre")
                .field("pout0", &self.pout0())
                .field("pout1", &self.pout1())
                .field("pout2", &self.pout2())
                .field("pout3", &self.pout3())
                .field("pout4", &self.pout4())
                .field("pout5", &self.pout5())
                .field("pout6", &self.pout6())
                .field("pout7", &self.pout7())
                .field("pout8", &self.pout8())
                .field("pout9", &self.pout9())
                .field("pout10", &self.pout10())
                .field("pout11", &self.pout11())
                .field("pout12", &self.pout12())
                .field("pout13", &self.pout13())
                .field("pout14", &self.pout14())
                .field("pout15", &self.pout15())
                .field("pout16", &self.pout16())
                .field("pout17", &self.pout17())
                .finish()
        }
    }
    #[doc = "desc POERA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Poera(pub u32);
    impl Poera {
        #[doc = "desc POUTE0."]
        #[must_use]
        #[inline(always)]
        pub const fn poute0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE0."]
        #[inline(always)]
        pub const fn set_poute0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE1."]
        #[must_use]
        #[inline(always)]
        pub const fn poute1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE1."]
        #[inline(always)]
        pub const fn set_poute1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POUTE2."]
        #[must_use]
        #[inline(always)]
        pub const fn poute2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE2."]
        #[inline(always)]
        pub const fn set_poute2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POUTE3."]
        #[must_use]
        #[inline(always)]
        pub const fn poute3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE3."]
        #[inline(always)]
        pub const fn set_poute3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POUTE4."]
        #[must_use]
        #[inline(always)]
        pub const fn poute4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE4."]
        #[inline(always)]
        pub const fn set_poute4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POUTE5."]
        #[must_use]
        #[inline(always)]
        pub const fn poute5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE5."]
        #[inline(always)]
        pub const fn set_poute5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POUTE6."]
        #[must_use]
        #[inline(always)]
        pub const fn poute6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE6."]
        #[inline(always)]
        pub const fn set_poute6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POUTE7."]
        #[must_use]
        #[inline(always)]
        pub const fn poute7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE7."]
        #[inline(always)]
        pub const fn set_poute7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POUTE8."]
        #[must_use]
        #[inline(always)]
        pub const fn poute8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE8."]
        #[inline(always)]
        pub const fn set_poute8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POUTE9."]
        #[must_use]
        #[inline(always)]
        pub const fn poute9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE9."]
        #[inline(always)]
        pub const fn set_poute9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POUTE10."]
        #[must_use]
        #[inline(always)]
        pub const fn poute10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE10."]
        #[inline(always)]
        pub const fn set_poute10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POUTE11."]
        #[must_use]
        #[inline(always)]
        pub const fn poute11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE11."]
        #[inline(always)]
        pub const fn set_poute11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POUTE12."]
        #[must_use]
        #[inline(always)]
        pub const fn poute12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE12."]
        #[inline(always)]
        pub const fn set_poute12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POUTE13."]
        #[must_use]
        #[inline(always)]
        pub const fn poute13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE13."]
        #[inline(always)]
        pub const fn set_poute13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POUTE14."]
        #[must_use]
        #[inline(always)]
        pub const fn poute14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE14."]
        #[inline(always)]
        pub const fn set_poute14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POUTE15."]
        #[must_use]
        #[inline(always)]
        pub const fn poute15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE15."]
        #[inline(always)]
        pub const fn set_poute15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POUTE16."]
        #[must_use]
        #[inline(always)]
        pub const fn poute16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE16."]
        #[inline(always)]
        pub const fn set_poute16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POUTE17."]
        #[must_use]
        #[inline(always)]
        pub const fn poute17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE17."]
        #[inline(always)]
        pub const fn set_poute17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Poera {
        #[inline(always)]
        fn default() -> Poera {
            Poera(0)
        }
    }
    impl core::fmt::Debug for Poera {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Poera")
                .field("poute0", &self.poute0())
                .field("poute1", &self.poute1())
                .field("poute2", &self.poute2())
                .field("poute3", &self.poute3())
                .field("poute4", &self.poute4())
                .field("poute5", &self.poute5())
                .field("poute6", &self.poute6())
                .field("poute7", &self.poute7())
                .field("poute8", &self.poute8())
                .field("poute9", &self.poute9())
                .field("poute10", &self.poute10())
                .field("poute11", &self.poute11())
                .field("poute12", &self.poute12())
                .field("poute13", &self.poute13())
                .field("poute14", &self.poute14())
                .field("poute15", &self.poute15())
                .field("poute16", &self.poute16())
                .field("poute17", &self.poute17())
                .finish()
        }
    }
    #[doc = "desc POERB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Poerb(pub u32);
    impl Poerb {
        #[doc = "desc POUTE0."]
        #[must_use]
        #[inline(always)]
        pub const fn poute0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE0."]
        #[inline(always)]
        pub const fn set_poute0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE1."]
        #[must_use]
        #[inline(always)]
        pub const fn poute1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE1."]
        #[inline(always)]
        pub const fn set_poute1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POUTE2."]
        #[must_use]
        #[inline(always)]
        pub const fn poute2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE2."]
        #[inline(always)]
        pub const fn set_poute2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POUTE3."]
        #[must_use]
        #[inline(always)]
        pub const fn poute3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE3."]
        #[inline(always)]
        pub const fn set_poute3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POUTE4."]
        #[must_use]
        #[inline(always)]
        pub const fn poute4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE4."]
        #[inline(always)]
        pub const fn set_poute4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POUTE5."]
        #[must_use]
        #[inline(always)]
        pub const fn poute5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE5."]
        #[inline(always)]
        pub const fn set_poute5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POUTE6."]
        #[must_use]
        #[inline(always)]
        pub const fn poute6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE6."]
        #[inline(always)]
        pub const fn set_poute6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POUTE7."]
        #[must_use]
        #[inline(always)]
        pub const fn poute7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE7."]
        #[inline(always)]
        pub const fn set_poute7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POUTE8."]
        #[must_use]
        #[inline(always)]
        pub const fn poute8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE8."]
        #[inline(always)]
        pub const fn set_poute8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POUTE9."]
        #[must_use]
        #[inline(always)]
        pub const fn poute9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE9."]
        #[inline(always)]
        pub const fn set_poute9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POUTE10."]
        #[must_use]
        #[inline(always)]
        pub const fn poute10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE10."]
        #[inline(always)]
        pub const fn set_poute10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POUTE11."]
        #[must_use]
        #[inline(always)]
        pub const fn poute11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE11."]
        #[inline(always)]
        pub const fn set_poute11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POUTE12."]
        #[must_use]
        #[inline(always)]
        pub const fn poute12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE12."]
        #[inline(always)]
        pub const fn set_poute12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POUTE13."]
        #[must_use]
        #[inline(always)]
        pub const fn poute13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE13."]
        #[inline(always)]
        pub const fn set_poute13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POUTE14."]
        #[must_use]
        #[inline(always)]
        pub const fn poute14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE14."]
        #[inline(always)]
        pub const fn set_poute14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POUTE15."]
        #[must_use]
        #[inline(always)]
        pub const fn poute15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE15."]
        #[inline(always)]
        pub const fn set_poute15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POUTE16."]
        #[must_use]
        #[inline(always)]
        pub const fn poute16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE16."]
        #[inline(always)]
        pub const fn set_poute16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POUTE17."]
        #[must_use]
        #[inline(always)]
        pub const fn poute17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE17."]
        #[inline(always)]
        pub const fn set_poute17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Poerb {
        #[inline(always)]
        fn default() -> Poerb {
            Poerb(0)
        }
    }
    impl core::fmt::Debug for Poerb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Poerb")
                .field("poute0", &self.poute0())
                .field("poute1", &self.poute1())
                .field("poute2", &self.poute2())
                .field("poute3", &self.poute3())
                .field("poute4", &self.poute4())
                .field("poute5", &self.poute5())
                .field("poute6", &self.poute6())
                .field("poute7", &self.poute7())
                .field("poute8", &self.poute8())
                .field("poute9", &self.poute9())
                .field("poute10", &self.poute10())
                .field("poute11", &self.poute11())
                .field("poute12", &self.poute12())
                .field("poute13", &self.poute13())
                .field("poute14", &self.poute14())
                .field("poute15", &self.poute15())
                .field("poute16", &self.poute16())
                .field("poute17", &self.poute17())
                .finish()
        }
    }
    #[doc = "desc POERC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Poerc(pub u32);
    impl Poerc {
        #[doc = "desc POUTE0."]
        #[must_use]
        #[inline(always)]
        pub const fn poute0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE0."]
        #[inline(always)]
        pub const fn set_poute0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE1."]
        #[must_use]
        #[inline(always)]
        pub const fn poute1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE1."]
        #[inline(always)]
        pub const fn set_poute1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POUTE2."]
        #[must_use]
        #[inline(always)]
        pub const fn poute2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE2."]
        #[inline(always)]
        pub const fn set_poute2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POUTE3."]
        #[must_use]
        #[inline(always)]
        pub const fn poute3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE3."]
        #[inline(always)]
        pub const fn set_poute3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POUTE4."]
        #[must_use]
        #[inline(always)]
        pub const fn poute4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE4."]
        #[inline(always)]
        pub const fn set_poute4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POUTE5."]
        #[must_use]
        #[inline(always)]
        pub const fn poute5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE5."]
        #[inline(always)]
        pub const fn set_poute5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POUTE6."]
        #[must_use]
        #[inline(always)]
        pub const fn poute6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE6."]
        #[inline(always)]
        pub const fn set_poute6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POUTE7."]
        #[must_use]
        #[inline(always)]
        pub const fn poute7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE7."]
        #[inline(always)]
        pub const fn set_poute7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POUTE8."]
        #[must_use]
        #[inline(always)]
        pub const fn poute8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE8."]
        #[inline(always)]
        pub const fn set_poute8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POUTE9."]
        #[must_use]
        #[inline(always)]
        pub const fn poute9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE9."]
        #[inline(always)]
        pub const fn set_poute9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POUTE10."]
        #[must_use]
        #[inline(always)]
        pub const fn poute10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE10."]
        #[inline(always)]
        pub const fn set_poute10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POUTE11."]
        #[must_use]
        #[inline(always)]
        pub const fn poute11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE11."]
        #[inline(always)]
        pub const fn set_poute11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POUTE12."]
        #[must_use]
        #[inline(always)]
        pub const fn poute12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE12."]
        #[inline(always)]
        pub const fn set_poute12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POUTE13."]
        #[must_use]
        #[inline(always)]
        pub const fn poute13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE13."]
        #[inline(always)]
        pub const fn set_poute13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POUTE14."]
        #[must_use]
        #[inline(always)]
        pub const fn poute14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE14."]
        #[inline(always)]
        pub const fn set_poute14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POUTE15."]
        #[must_use]
        #[inline(always)]
        pub const fn poute15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE15."]
        #[inline(always)]
        pub const fn set_poute15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POUTE16."]
        #[must_use]
        #[inline(always)]
        pub const fn poute16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE16."]
        #[inline(always)]
        pub const fn set_poute16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POUTE17."]
        #[must_use]
        #[inline(always)]
        pub const fn poute17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE17."]
        #[inline(always)]
        pub const fn set_poute17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Poerc {
        #[inline(always)]
        fn default() -> Poerc {
            Poerc(0)
        }
    }
    impl core::fmt::Debug for Poerc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Poerc")
                .field("poute0", &self.poute0())
                .field("poute1", &self.poute1())
                .field("poute2", &self.poute2())
                .field("poute3", &self.poute3())
                .field("poute4", &self.poute4())
                .field("poute5", &self.poute5())
                .field("poute6", &self.poute6())
                .field("poute7", &self.poute7())
                .field("poute8", &self.poute8())
                .field("poute9", &self.poute9())
                .field("poute10", &self.poute10())
                .field("poute11", &self.poute11())
                .field("poute12", &self.poute12())
                .field("poute13", &self.poute13())
                .field("poute14", &self.poute14())
                .field("poute15", &self.poute15())
                .field("poute16", &self.poute16())
                .field("poute17", &self.poute17())
                .finish()
        }
    }
    #[doc = "desc POERD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Poerd(pub u32);
    impl Poerd {
        #[doc = "desc POUTE0."]
        #[must_use]
        #[inline(always)]
        pub const fn poute0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE0."]
        #[inline(always)]
        pub const fn set_poute0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE1."]
        #[must_use]
        #[inline(always)]
        pub const fn poute1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE1."]
        #[inline(always)]
        pub const fn set_poute1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POUTE2."]
        #[must_use]
        #[inline(always)]
        pub const fn poute2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE2."]
        #[inline(always)]
        pub const fn set_poute2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POUTE3."]
        #[must_use]
        #[inline(always)]
        pub const fn poute3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE3."]
        #[inline(always)]
        pub const fn set_poute3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POUTE4."]
        #[must_use]
        #[inline(always)]
        pub const fn poute4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE4."]
        #[inline(always)]
        pub const fn set_poute4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POUTE5."]
        #[must_use]
        #[inline(always)]
        pub const fn poute5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE5."]
        #[inline(always)]
        pub const fn set_poute5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POUTE6."]
        #[must_use]
        #[inline(always)]
        pub const fn poute6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE6."]
        #[inline(always)]
        pub const fn set_poute6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POUTE7."]
        #[must_use]
        #[inline(always)]
        pub const fn poute7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE7."]
        #[inline(always)]
        pub const fn set_poute7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POUTE8."]
        #[must_use]
        #[inline(always)]
        pub const fn poute8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE8."]
        #[inline(always)]
        pub const fn set_poute8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POUTE9."]
        #[must_use]
        #[inline(always)]
        pub const fn poute9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE9."]
        #[inline(always)]
        pub const fn set_poute9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POUTE10."]
        #[must_use]
        #[inline(always)]
        pub const fn poute10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE10."]
        #[inline(always)]
        pub const fn set_poute10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POUTE11."]
        #[must_use]
        #[inline(always)]
        pub const fn poute11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE11."]
        #[inline(always)]
        pub const fn set_poute11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POUTE12."]
        #[must_use]
        #[inline(always)]
        pub const fn poute12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE12."]
        #[inline(always)]
        pub const fn set_poute12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POUTE13."]
        #[must_use]
        #[inline(always)]
        pub const fn poute13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE13."]
        #[inline(always)]
        pub const fn set_poute13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POUTE14."]
        #[must_use]
        #[inline(always)]
        pub const fn poute14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE14."]
        #[inline(always)]
        pub const fn set_poute14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POUTE15."]
        #[must_use]
        #[inline(always)]
        pub const fn poute15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE15."]
        #[inline(always)]
        pub const fn set_poute15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POUTE16."]
        #[must_use]
        #[inline(always)]
        pub const fn poute16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE16."]
        #[inline(always)]
        pub const fn set_poute16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POUTE17."]
        #[must_use]
        #[inline(always)]
        pub const fn poute17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE17."]
        #[inline(always)]
        pub const fn set_poute17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Poerd {
        #[inline(always)]
        fn default() -> Poerd {
            Poerd(0)
        }
    }
    impl core::fmt::Debug for Poerd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Poerd")
                .field("poute0", &self.poute0())
                .field("poute1", &self.poute1())
                .field("poute2", &self.poute2())
                .field("poute3", &self.poute3())
                .field("poute4", &self.poute4())
                .field("poute5", &self.poute5())
                .field("poute6", &self.poute6())
                .field("poute7", &self.poute7())
                .field("poute8", &self.poute8())
                .field("poute9", &self.poute9())
                .field("poute10", &self.poute10())
                .field("poute11", &self.poute11())
                .field("poute12", &self.poute12())
                .field("poute13", &self.poute13())
                .field("poute14", &self.poute14())
                .field("poute15", &self.poute15())
                .field("poute16", &self.poute16())
                .field("poute17", &self.poute17())
                .finish()
        }
    }
    #[doc = "desc POERE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Poere(pub u32);
    impl Poere {
        #[doc = "desc POUTE0."]
        #[must_use]
        #[inline(always)]
        pub const fn poute0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE0."]
        #[inline(always)]
        pub const fn set_poute0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POUTE1."]
        #[must_use]
        #[inline(always)]
        pub const fn poute1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE1."]
        #[inline(always)]
        pub const fn set_poute1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POUTE2."]
        #[must_use]
        #[inline(always)]
        pub const fn poute2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE2."]
        #[inline(always)]
        pub const fn set_poute2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POUTE3."]
        #[must_use]
        #[inline(always)]
        pub const fn poute3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE3."]
        #[inline(always)]
        pub const fn set_poute3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POUTE4."]
        #[must_use]
        #[inline(always)]
        pub const fn poute4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE4."]
        #[inline(always)]
        pub const fn set_poute4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POUTE5."]
        #[must_use]
        #[inline(always)]
        pub const fn poute5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE5."]
        #[inline(always)]
        pub const fn set_poute5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POUTE6."]
        #[must_use]
        #[inline(always)]
        pub const fn poute6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE6."]
        #[inline(always)]
        pub const fn set_poute6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POUTE7."]
        #[must_use]
        #[inline(always)]
        pub const fn poute7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE7."]
        #[inline(always)]
        pub const fn set_poute7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POUTE8."]
        #[must_use]
        #[inline(always)]
        pub const fn poute8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE8."]
        #[inline(always)]
        pub const fn set_poute8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POUTE9."]
        #[must_use]
        #[inline(always)]
        pub const fn poute9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE9."]
        #[inline(always)]
        pub const fn set_poute9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POUTE10."]
        #[must_use]
        #[inline(always)]
        pub const fn poute10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE10."]
        #[inline(always)]
        pub const fn set_poute10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POUTE11."]
        #[must_use]
        #[inline(always)]
        pub const fn poute11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE11."]
        #[inline(always)]
        pub const fn set_poute11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POUTE12."]
        #[must_use]
        #[inline(always)]
        pub const fn poute12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE12."]
        #[inline(always)]
        pub const fn set_poute12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POUTE13."]
        #[must_use]
        #[inline(always)]
        pub const fn poute13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE13."]
        #[inline(always)]
        pub const fn set_poute13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POUTE14."]
        #[must_use]
        #[inline(always)]
        pub const fn poute14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE14."]
        #[inline(always)]
        pub const fn set_poute14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POUTE15."]
        #[must_use]
        #[inline(always)]
        pub const fn poute15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE15."]
        #[inline(always)]
        pub const fn set_poute15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POUTE16."]
        #[must_use]
        #[inline(always)]
        pub const fn poute16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE16."]
        #[inline(always)]
        pub const fn set_poute16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POUTE17."]
        #[must_use]
        #[inline(always)]
        pub const fn poute17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POUTE17."]
        #[inline(always)]
        pub const fn set_poute17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Poere {
        #[inline(always)]
        fn default() -> Poere {
            Poere(0)
        }
    }
    impl core::fmt::Debug for Poere {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Poere")
                .field("poute0", &self.poute0())
                .field("poute1", &self.poute1())
                .field("poute2", &self.poute2())
                .field("poute3", &self.poute3())
                .field("poute4", &self.poute4())
                .field("poute5", &self.poute5())
                .field("poute6", &self.poute6())
                .field("poute7", &self.poute7())
                .field("poute8", &self.poute8())
                .field("poute9", &self.poute9())
                .field("poute10", &self.poute10())
                .field("poute11", &self.poute11())
                .field("poute12", &self.poute12())
                .field("poute13", &self.poute13())
                .field("poute14", &self.poute14())
                .field("poute15", &self.poute15())
                .field("poute16", &self.poute16())
                .field("poute17", &self.poute17())
                .finish()
        }
    }
    #[doc = "desc PORRA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Porra(pub u32);
    impl Porra {
        #[doc = "desc POR0."]
        #[must_use]
        #[inline(always)]
        pub const fn por0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR0."]
        #[inline(always)]
        pub const fn set_por0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POR1."]
        #[must_use]
        #[inline(always)]
        pub const fn por1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR1."]
        #[inline(always)]
        pub const fn set_por1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POR2."]
        #[must_use]
        #[inline(always)]
        pub const fn por2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR2."]
        #[inline(always)]
        pub const fn set_por2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POR3."]
        #[must_use]
        #[inline(always)]
        pub const fn por3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR3."]
        #[inline(always)]
        pub const fn set_por3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POR4."]
        #[must_use]
        #[inline(always)]
        pub const fn por4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR4."]
        #[inline(always)]
        pub const fn set_por4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POR5."]
        #[must_use]
        #[inline(always)]
        pub const fn por5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR5."]
        #[inline(always)]
        pub const fn set_por5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POR6."]
        #[must_use]
        #[inline(always)]
        pub const fn por6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR6."]
        #[inline(always)]
        pub const fn set_por6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POR7."]
        #[must_use]
        #[inline(always)]
        pub const fn por7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR7."]
        #[inline(always)]
        pub const fn set_por7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POR8."]
        #[must_use]
        #[inline(always)]
        pub const fn por8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR8."]
        #[inline(always)]
        pub const fn set_por8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POR9."]
        #[must_use]
        #[inline(always)]
        pub const fn por9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR9."]
        #[inline(always)]
        pub const fn set_por9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POR10."]
        #[must_use]
        #[inline(always)]
        pub const fn por10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR10."]
        #[inline(always)]
        pub const fn set_por10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POR11."]
        #[must_use]
        #[inline(always)]
        pub const fn por11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR11."]
        #[inline(always)]
        pub const fn set_por11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POR12."]
        #[must_use]
        #[inline(always)]
        pub const fn por12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR12."]
        #[inline(always)]
        pub const fn set_por12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POR13."]
        #[must_use]
        #[inline(always)]
        pub const fn por13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR13."]
        #[inline(always)]
        pub const fn set_por13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POR14."]
        #[must_use]
        #[inline(always)]
        pub const fn por14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR14."]
        #[inline(always)]
        pub const fn set_por14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POR15."]
        #[must_use]
        #[inline(always)]
        pub const fn por15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR15."]
        #[inline(always)]
        pub const fn set_por15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POR16."]
        #[must_use]
        #[inline(always)]
        pub const fn por16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR16."]
        #[inline(always)]
        pub const fn set_por16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POR17."]
        #[must_use]
        #[inline(always)]
        pub const fn por17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR17."]
        #[inline(always)]
        pub const fn set_por17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Porra {
        #[inline(always)]
        fn default() -> Porra {
            Porra(0)
        }
    }
    impl core::fmt::Debug for Porra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Porra")
                .field("por0", &self.por0())
                .field("por1", &self.por1())
                .field("por2", &self.por2())
                .field("por3", &self.por3())
                .field("por4", &self.por4())
                .field("por5", &self.por5())
                .field("por6", &self.por6())
                .field("por7", &self.por7())
                .field("por8", &self.por8())
                .field("por9", &self.por9())
                .field("por10", &self.por10())
                .field("por11", &self.por11())
                .field("por12", &self.por12())
                .field("por13", &self.por13())
                .field("por14", &self.por14())
                .field("por15", &self.por15())
                .field("por16", &self.por16())
                .field("por17", &self.por17())
                .finish()
        }
    }
    #[doc = "desc PORRB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Porrb(pub u32);
    impl Porrb {
        #[doc = "desc POR0."]
        #[must_use]
        #[inline(always)]
        pub const fn por0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR0."]
        #[inline(always)]
        pub const fn set_por0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POR1."]
        #[must_use]
        #[inline(always)]
        pub const fn por1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR1."]
        #[inline(always)]
        pub const fn set_por1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POR2."]
        #[must_use]
        #[inline(always)]
        pub const fn por2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR2."]
        #[inline(always)]
        pub const fn set_por2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POR3."]
        #[must_use]
        #[inline(always)]
        pub const fn por3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR3."]
        #[inline(always)]
        pub const fn set_por3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POR4."]
        #[must_use]
        #[inline(always)]
        pub const fn por4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR4."]
        #[inline(always)]
        pub const fn set_por4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POR5."]
        #[must_use]
        #[inline(always)]
        pub const fn por5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR5."]
        #[inline(always)]
        pub const fn set_por5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POR6."]
        #[must_use]
        #[inline(always)]
        pub const fn por6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR6."]
        #[inline(always)]
        pub const fn set_por6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POR7."]
        #[must_use]
        #[inline(always)]
        pub const fn por7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR7."]
        #[inline(always)]
        pub const fn set_por7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POR8."]
        #[must_use]
        #[inline(always)]
        pub const fn por8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR8."]
        #[inline(always)]
        pub const fn set_por8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POR9."]
        #[must_use]
        #[inline(always)]
        pub const fn por9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR9."]
        #[inline(always)]
        pub const fn set_por9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POR10."]
        #[must_use]
        #[inline(always)]
        pub const fn por10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR10."]
        #[inline(always)]
        pub const fn set_por10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POR11."]
        #[must_use]
        #[inline(always)]
        pub const fn por11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR11."]
        #[inline(always)]
        pub const fn set_por11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POR12."]
        #[must_use]
        #[inline(always)]
        pub const fn por12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR12."]
        #[inline(always)]
        pub const fn set_por12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POR13."]
        #[must_use]
        #[inline(always)]
        pub const fn por13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR13."]
        #[inline(always)]
        pub const fn set_por13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POR14."]
        #[must_use]
        #[inline(always)]
        pub const fn por14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR14."]
        #[inline(always)]
        pub const fn set_por14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POR15."]
        #[must_use]
        #[inline(always)]
        pub const fn por15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR15."]
        #[inline(always)]
        pub const fn set_por15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POR16."]
        #[must_use]
        #[inline(always)]
        pub const fn por16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR16."]
        #[inline(always)]
        pub const fn set_por16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POR17."]
        #[must_use]
        #[inline(always)]
        pub const fn por17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR17."]
        #[inline(always)]
        pub const fn set_por17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Porrb {
        #[inline(always)]
        fn default() -> Porrb {
            Porrb(0)
        }
    }
    impl core::fmt::Debug for Porrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Porrb")
                .field("por0", &self.por0())
                .field("por1", &self.por1())
                .field("por2", &self.por2())
                .field("por3", &self.por3())
                .field("por4", &self.por4())
                .field("por5", &self.por5())
                .field("por6", &self.por6())
                .field("por7", &self.por7())
                .field("por8", &self.por8())
                .field("por9", &self.por9())
                .field("por10", &self.por10())
                .field("por11", &self.por11())
                .field("por12", &self.por12())
                .field("por13", &self.por13())
                .field("por14", &self.por14())
                .field("por15", &self.por15())
                .field("por16", &self.por16())
                .field("por17", &self.por17())
                .finish()
        }
    }
    #[doc = "desc PORRC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Porrc(pub u32);
    impl Porrc {
        #[doc = "desc POR0."]
        #[must_use]
        #[inline(always)]
        pub const fn por0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR0."]
        #[inline(always)]
        pub const fn set_por0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POR1."]
        #[must_use]
        #[inline(always)]
        pub const fn por1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR1."]
        #[inline(always)]
        pub const fn set_por1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POR2."]
        #[must_use]
        #[inline(always)]
        pub const fn por2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR2."]
        #[inline(always)]
        pub const fn set_por2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POR3."]
        #[must_use]
        #[inline(always)]
        pub const fn por3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR3."]
        #[inline(always)]
        pub const fn set_por3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POR4."]
        #[must_use]
        #[inline(always)]
        pub const fn por4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR4."]
        #[inline(always)]
        pub const fn set_por4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POR5."]
        #[must_use]
        #[inline(always)]
        pub const fn por5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR5."]
        #[inline(always)]
        pub const fn set_por5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POR6."]
        #[must_use]
        #[inline(always)]
        pub const fn por6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR6."]
        #[inline(always)]
        pub const fn set_por6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POR7."]
        #[must_use]
        #[inline(always)]
        pub const fn por7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR7."]
        #[inline(always)]
        pub const fn set_por7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POR8."]
        #[must_use]
        #[inline(always)]
        pub const fn por8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR8."]
        #[inline(always)]
        pub const fn set_por8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POR9."]
        #[must_use]
        #[inline(always)]
        pub const fn por9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR9."]
        #[inline(always)]
        pub const fn set_por9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POR10."]
        #[must_use]
        #[inline(always)]
        pub const fn por10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR10."]
        #[inline(always)]
        pub const fn set_por10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POR11."]
        #[must_use]
        #[inline(always)]
        pub const fn por11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR11."]
        #[inline(always)]
        pub const fn set_por11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POR12."]
        #[must_use]
        #[inline(always)]
        pub const fn por12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR12."]
        #[inline(always)]
        pub const fn set_por12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POR13."]
        #[must_use]
        #[inline(always)]
        pub const fn por13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR13."]
        #[inline(always)]
        pub const fn set_por13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POR14."]
        #[must_use]
        #[inline(always)]
        pub const fn por14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR14."]
        #[inline(always)]
        pub const fn set_por14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POR15."]
        #[must_use]
        #[inline(always)]
        pub const fn por15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR15."]
        #[inline(always)]
        pub const fn set_por15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POR16."]
        #[must_use]
        #[inline(always)]
        pub const fn por16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR16."]
        #[inline(always)]
        pub const fn set_por16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POR17."]
        #[must_use]
        #[inline(always)]
        pub const fn por17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR17."]
        #[inline(always)]
        pub const fn set_por17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Porrc {
        #[inline(always)]
        fn default() -> Porrc {
            Porrc(0)
        }
    }
    impl core::fmt::Debug for Porrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Porrc")
                .field("por0", &self.por0())
                .field("por1", &self.por1())
                .field("por2", &self.por2())
                .field("por3", &self.por3())
                .field("por4", &self.por4())
                .field("por5", &self.por5())
                .field("por6", &self.por6())
                .field("por7", &self.por7())
                .field("por8", &self.por8())
                .field("por9", &self.por9())
                .field("por10", &self.por10())
                .field("por11", &self.por11())
                .field("por12", &self.por12())
                .field("por13", &self.por13())
                .field("por14", &self.por14())
                .field("por15", &self.por15())
                .field("por16", &self.por16())
                .field("por17", &self.por17())
                .finish()
        }
    }
    #[doc = "desc PORRD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Porrd(pub u32);
    impl Porrd {
        #[doc = "desc POR0."]
        #[must_use]
        #[inline(always)]
        pub const fn por0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR0."]
        #[inline(always)]
        pub const fn set_por0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POR1."]
        #[must_use]
        #[inline(always)]
        pub const fn por1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR1."]
        #[inline(always)]
        pub const fn set_por1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POR2."]
        #[must_use]
        #[inline(always)]
        pub const fn por2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR2."]
        #[inline(always)]
        pub const fn set_por2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POR3."]
        #[must_use]
        #[inline(always)]
        pub const fn por3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR3."]
        #[inline(always)]
        pub const fn set_por3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POR4."]
        #[must_use]
        #[inline(always)]
        pub const fn por4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR4."]
        #[inline(always)]
        pub const fn set_por4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POR5."]
        #[must_use]
        #[inline(always)]
        pub const fn por5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR5."]
        #[inline(always)]
        pub const fn set_por5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POR6."]
        #[must_use]
        #[inline(always)]
        pub const fn por6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR6."]
        #[inline(always)]
        pub const fn set_por6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POR7."]
        #[must_use]
        #[inline(always)]
        pub const fn por7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR7."]
        #[inline(always)]
        pub const fn set_por7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POR8."]
        #[must_use]
        #[inline(always)]
        pub const fn por8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR8."]
        #[inline(always)]
        pub const fn set_por8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POR9."]
        #[must_use]
        #[inline(always)]
        pub const fn por9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR9."]
        #[inline(always)]
        pub const fn set_por9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POR10."]
        #[must_use]
        #[inline(always)]
        pub const fn por10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR10."]
        #[inline(always)]
        pub const fn set_por10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POR11."]
        #[must_use]
        #[inline(always)]
        pub const fn por11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR11."]
        #[inline(always)]
        pub const fn set_por11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POR12."]
        #[must_use]
        #[inline(always)]
        pub const fn por12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR12."]
        #[inline(always)]
        pub const fn set_por12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POR13."]
        #[must_use]
        #[inline(always)]
        pub const fn por13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR13."]
        #[inline(always)]
        pub const fn set_por13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POR14."]
        #[must_use]
        #[inline(always)]
        pub const fn por14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR14."]
        #[inline(always)]
        pub const fn set_por14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POR15."]
        #[must_use]
        #[inline(always)]
        pub const fn por15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR15."]
        #[inline(always)]
        pub const fn set_por15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POR16."]
        #[must_use]
        #[inline(always)]
        pub const fn por16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR16."]
        #[inline(always)]
        pub const fn set_por16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POR17."]
        #[must_use]
        #[inline(always)]
        pub const fn por17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR17."]
        #[inline(always)]
        pub const fn set_por17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Porrd {
        #[inline(always)]
        fn default() -> Porrd {
            Porrd(0)
        }
    }
    impl core::fmt::Debug for Porrd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Porrd")
                .field("por0", &self.por0())
                .field("por1", &self.por1())
                .field("por2", &self.por2())
                .field("por3", &self.por3())
                .field("por4", &self.por4())
                .field("por5", &self.por5())
                .field("por6", &self.por6())
                .field("por7", &self.por7())
                .field("por8", &self.por8())
                .field("por9", &self.por9())
                .field("por10", &self.por10())
                .field("por11", &self.por11())
                .field("por12", &self.por12())
                .field("por13", &self.por13())
                .field("por14", &self.por14())
                .field("por15", &self.por15())
                .field("por16", &self.por16())
                .field("por17", &self.por17())
                .finish()
        }
    }
    #[doc = "desc PORRE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Porre(pub u32);
    impl Porre {
        #[doc = "desc POR0."]
        #[must_use]
        #[inline(always)]
        pub const fn por0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR0."]
        #[inline(always)]
        pub const fn set_por0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POR1."]
        #[must_use]
        #[inline(always)]
        pub const fn por1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR1."]
        #[inline(always)]
        pub const fn set_por1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POR2."]
        #[must_use]
        #[inline(always)]
        pub const fn por2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR2."]
        #[inline(always)]
        pub const fn set_por2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POR3."]
        #[must_use]
        #[inline(always)]
        pub const fn por3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR3."]
        #[inline(always)]
        pub const fn set_por3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POR4."]
        #[must_use]
        #[inline(always)]
        pub const fn por4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR4."]
        #[inline(always)]
        pub const fn set_por4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POR5."]
        #[must_use]
        #[inline(always)]
        pub const fn por5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR5."]
        #[inline(always)]
        pub const fn set_por5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POR6."]
        #[must_use]
        #[inline(always)]
        pub const fn por6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR6."]
        #[inline(always)]
        pub const fn set_por6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POR7."]
        #[must_use]
        #[inline(always)]
        pub const fn por7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR7."]
        #[inline(always)]
        pub const fn set_por7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POR8."]
        #[must_use]
        #[inline(always)]
        pub const fn por8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR8."]
        #[inline(always)]
        pub const fn set_por8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POR9."]
        #[must_use]
        #[inline(always)]
        pub const fn por9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR9."]
        #[inline(always)]
        pub const fn set_por9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POR10."]
        #[must_use]
        #[inline(always)]
        pub const fn por10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR10."]
        #[inline(always)]
        pub const fn set_por10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POR11."]
        #[must_use]
        #[inline(always)]
        pub const fn por11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR11."]
        #[inline(always)]
        pub const fn set_por11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POR12."]
        #[must_use]
        #[inline(always)]
        pub const fn por12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR12."]
        #[inline(always)]
        pub const fn set_por12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POR13."]
        #[must_use]
        #[inline(always)]
        pub const fn por13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR13."]
        #[inline(always)]
        pub const fn set_por13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POR14."]
        #[must_use]
        #[inline(always)]
        pub const fn por14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR14."]
        #[inline(always)]
        pub const fn set_por14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POR15."]
        #[must_use]
        #[inline(always)]
        pub const fn por15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR15."]
        #[inline(always)]
        pub const fn set_por15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POR16."]
        #[must_use]
        #[inline(always)]
        pub const fn por16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR16."]
        #[inline(always)]
        pub const fn set_por16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POR17."]
        #[must_use]
        #[inline(always)]
        pub const fn por17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR17."]
        #[inline(always)]
        pub const fn set_por17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Porre {
        #[inline(always)]
        fn default() -> Porre {
            Porre(0)
        }
    }
    impl core::fmt::Debug for Porre {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Porre")
                .field("por0", &self.por0())
                .field("por1", &self.por1())
                .field("por2", &self.por2())
                .field("por3", &self.por3())
                .field("por4", &self.por4())
                .field("por5", &self.por5())
                .field("por6", &self.por6())
                .field("por7", &self.por7())
                .field("por8", &self.por8())
                .field("por9", &self.por9())
                .field("por10", &self.por10())
                .field("por11", &self.por11())
                .field("por12", &self.por12())
                .field("por13", &self.por13())
                .field("por14", &self.por14())
                .field("por15", &self.por15())
                .field("por16", &self.por16())
                .field("por17", &self.por17())
                .finish()
        }
    }
    #[doc = "desc POSRA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Posra(pub u32);
    impl Posra {
        #[doc = "desc POS0."]
        #[must_use]
        #[inline(always)]
        pub const fn pos0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS0."]
        #[inline(always)]
        pub const fn set_pos0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POS1."]
        #[must_use]
        #[inline(always)]
        pub const fn pos1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS1."]
        #[inline(always)]
        pub const fn set_pos1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POS2."]
        #[must_use]
        #[inline(always)]
        pub const fn pos2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS2."]
        #[inline(always)]
        pub const fn set_pos2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POS3."]
        #[must_use]
        #[inline(always)]
        pub const fn pos3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS3."]
        #[inline(always)]
        pub const fn set_pos3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POS4."]
        #[must_use]
        #[inline(always)]
        pub const fn pos4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS4."]
        #[inline(always)]
        pub const fn set_pos4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POS5."]
        #[must_use]
        #[inline(always)]
        pub const fn pos5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS5."]
        #[inline(always)]
        pub const fn set_pos5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POS6."]
        #[must_use]
        #[inline(always)]
        pub const fn pos6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS6."]
        #[inline(always)]
        pub const fn set_pos6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POS7."]
        #[must_use]
        #[inline(always)]
        pub const fn pos7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS7."]
        #[inline(always)]
        pub const fn set_pos7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POS8."]
        #[must_use]
        #[inline(always)]
        pub const fn pos8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS8."]
        #[inline(always)]
        pub const fn set_pos8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POS9."]
        #[must_use]
        #[inline(always)]
        pub const fn pos9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS9."]
        #[inline(always)]
        pub const fn set_pos9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POS10."]
        #[must_use]
        #[inline(always)]
        pub const fn pos10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS10."]
        #[inline(always)]
        pub const fn set_pos10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POS11."]
        #[must_use]
        #[inline(always)]
        pub const fn pos11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS11."]
        #[inline(always)]
        pub const fn set_pos11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POS12."]
        #[must_use]
        #[inline(always)]
        pub const fn pos12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS12."]
        #[inline(always)]
        pub const fn set_pos12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POS13."]
        #[must_use]
        #[inline(always)]
        pub const fn pos13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS13."]
        #[inline(always)]
        pub const fn set_pos13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POS14."]
        #[must_use]
        #[inline(always)]
        pub const fn pos14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS14."]
        #[inline(always)]
        pub const fn set_pos14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POS15."]
        #[must_use]
        #[inline(always)]
        pub const fn pos15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS15."]
        #[inline(always)]
        pub const fn set_pos15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POS16."]
        #[must_use]
        #[inline(always)]
        pub const fn pos16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS16."]
        #[inline(always)]
        pub const fn set_pos16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POS17."]
        #[must_use]
        #[inline(always)]
        pub const fn pos17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS17."]
        #[inline(always)]
        pub const fn set_pos17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Posra {
        #[inline(always)]
        fn default() -> Posra {
            Posra(0)
        }
    }
    impl core::fmt::Debug for Posra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Posra")
                .field("pos0", &self.pos0())
                .field("pos1", &self.pos1())
                .field("pos2", &self.pos2())
                .field("pos3", &self.pos3())
                .field("pos4", &self.pos4())
                .field("pos5", &self.pos5())
                .field("pos6", &self.pos6())
                .field("pos7", &self.pos7())
                .field("pos8", &self.pos8())
                .field("pos9", &self.pos9())
                .field("pos10", &self.pos10())
                .field("pos11", &self.pos11())
                .field("pos12", &self.pos12())
                .field("pos13", &self.pos13())
                .field("pos14", &self.pos14())
                .field("pos15", &self.pos15())
                .field("pos16", &self.pos16())
                .field("pos17", &self.pos17())
                .finish()
        }
    }
    #[doc = "desc POSRB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Posrb(pub u32);
    impl Posrb {
        #[doc = "desc POS0."]
        #[must_use]
        #[inline(always)]
        pub const fn pos0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS0."]
        #[inline(always)]
        pub const fn set_pos0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POS1."]
        #[must_use]
        #[inline(always)]
        pub const fn pos1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS1."]
        #[inline(always)]
        pub const fn set_pos1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POS2."]
        #[must_use]
        #[inline(always)]
        pub const fn pos2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS2."]
        #[inline(always)]
        pub const fn set_pos2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POS3."]
        #[must_use]
        #[inline(always)]
        pub const fn pos3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS3."]
        #[inline(always)]
        pub const fn set_pos3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POS4."]
        #[must_use]
        #[inline(always)]
        pub const fn pos4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS4."]
        #[inline(always)]
        pub const fn set_pos4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POS5."]
        #[must_use]
        #[inline(always)]
        pub const fn pos5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS5."]
        #[inline(always)]
        pub const fn set_pos5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POS6."]
        #[must_use]
        #[inline(always)]
        pub const fn pos6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS6."]
        #[inline(always)]
        pub const fn set_pos6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POS7."]
        #[must_use]
        #[inline(always)]
        pub const fn pos7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS7."]
        #[inline(always)]
        pub const fn set_pos7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POS8."]
        #[must_use]
        #[inline(always)]
        pub const fn pos8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS8."]
        #[inline(always)]
        pub const fn set_pos8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POS9."]
        #[must_use]
        #[inline(always)]
        pub const fn pos9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS9."]
        #[inline(always)]
        pub const fn set_pos9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POS10."]
        #[must_use]
        #[inline(always)]
        pub const fn pos10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS10."]
        #[inline(always)]
        pub const fn set_pos10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POS11."]
        #[must_use]
        #[inline(always)]
        pub const fn pos11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS11."]
        #[inline(always)]
        pub const fn set_pos11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POS12."]
        #[must_use]
        #[inline(always)]
        pub const fn pos12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS12."]
        #[inline(always)]
        pub const fn set_pos12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POS13."]
        #[must_use]
        #[inline(always)]
        pub const fn pos13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS13."]
        #[inline(always)]
        pub const fn set_pos13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POS14."]
        #[must_use]
        #[inline(always)]
        pub const fn pos14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS14."]
        #[inline(always)]
        pub const fn set_pos14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POS15."]
        #[must_use]
        #[inline(always)]
        pub const fn pos15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS15."]
        #[inline(always)]
        pub const fn set_pos15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POS16."]
        #[must_use]
        #[inline(always)]
        pub const fn pos16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS16."]
        #[inline(always)]
        pub const fn set_pos16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POS17."]
        #[must_use]
        #[inline(always)]
        pub const fn pos17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS17."]
        #[inline(always)]
        pub const fn set_pos17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Posrb {
        #[inline(always)]
        fn default() -> Posrb {
            Posrb(0)
        }
    }
    impl core::fmt::Debug for Posrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Posrb")
                .field("pos0", &self.pos0())
                .field("pos1", &self.pos1())
                .field("pos2", &self.pos2())
                .field("pos3", &self.pos3())
                .field("pos4", &self.pos4())
                .field("pos5", &self.pos5())
                .field("pos6", &self.pos6())
                .field("pos7", &self.pos7())
                .field("pos8", &self.pos8())
                .field("pos9", &self.pos9())
                .field("pos10", &self.pos10())
                .field("pos11", &self.pos11())
                .field("pos12", &self.pos12())
                .field("pos13", &self.pos13())
                .field("pos14", &self.pos14())
                .field("pos15", &self.pos15())
                .field("pos16", &self.pos16())
                .field("pos17", &self.pos17())
                .finish()
        }
    }
    #[doc = "desc POSRC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Posrc(pub u32);
    impl Posrc {
        #[doc = "desc POS0."]
        #[must_use]
        #[inline(always)]
        pub const fn pos0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS0."]
        #[inline(always)]
        pub const fn set_pos0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POS1."]
        #[must_use]
        #[inline(always)]
        pub const fn pos1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS1."]
        #[inline(always)]
        pub const fn set_pos1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POS2."]
        #[must_use]
        #[inline(always)]
        pub const fn pos2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS2."]
        #[inline(always)]
        pub const fn set_pos2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POS3."]
        #[must_use]
        #[inline(always)]
        pub const fn pos3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS3."]
        #[inline(always)]
        pub const fn set_pos3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POS4."]
        #[must_use]
        #[inline(always)]
        pub const fn pos4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS4."]
        #[inline(always)]
        pub const fn set_pos4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POS5."]
        #[must_use]
        #[inline(always)]
        pub const fn pos5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS5."]
        #[inline(always)]
        pub const fn set_pos5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POS6."]
        #[must_use]
        #[inline(always)]
        pub const fn pos6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS6."]
        #[inline(always)]
        pub const fn set_pos6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POS7."]
        #[must_use]
        #[inline(always)]
        pub const fn pos7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS7."]
        #[inline(always)]
        pub const fn set_pos7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POS8."]
        #[must_use]
        #[inline(always)]
        pub const fn pos8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS8."]
        #[inline(always)]
        pub const fn set_pos8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POS9."]
        #[must_use]
        #[inline(always)]
        pub const fn pos9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS9."]
        #[inline(always)]
        pub const fn set_pos9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POS10."]
        #[must_use]
        #[inline(always)]
        pub const fn pos10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS10."]
        #[inline(always)]
        pub const fn set_pos10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POS11."]
        #[must_use]
        #[inline(always)]
        pub const fn pos11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS11."]
        #[inline(always)]
        pub const fn set_pos11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POS12."]
        #[must_use]
        #[inline(always)]
        pub const fn pos12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS12."]
        #[inline(always)]
        pub const fn set_pos12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POS13."]
        #[must_use]
        #[inline(always)]
        pub const fn pos13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS13."]
        #[inline(always)]
        pub const fn set_pos13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POS14."]
        #[must_use]
        #[inline(always)]
        pub const fn pos14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS14."]
        #[inline(always)]
        pub const fn set_pos14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POS15."]
        #[must_use]
        #[inline(always)]
        pub const fn pos15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS15."]
        #[inline(always)]
        pub const fn set_pos15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POS16."]
        #[must_use]
        #[inline(always)]
        pub const fn pos16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS16."]
        #[inline(always)]
        pub const fn set_pos16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POS17."]
        #[must_use]
        #[inline(always)]
        pub const fn pos17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS17."]
        #[inline(always)]
        pub const fn set_pos17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Posrc {
        #[inline(always)]
        fn default() -> Posrc {
            Posrc(0)
        }
    }
    impl core::fmt::Debug for Posrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Posrc")
                .field("pos0", &self.pos0())
                .field("pos1", &self.pos1())
                .field("pos2", &self.pos2())
                .field("pos3", &self.pos3())
                .field("pos4", &self.pos4())
                .field("pos5", &self.pos5())
                .field("pos6", &self.pos6())
                .field("pos7", &self.pos7())
                .field("pos8", &self.pos8())
                .field("pos9", &self.pos9())
                .field("pos10", &self.pos10())
                .field("pos11", &self.pos11())
                .field("pos12", &self.pos12())
                .field("pos13", &self.pos13())
                .field("pos14", &self.pos14())
                .field("pos15", &self.pos15())
                .field("pos16", &self.pos16())
                .field("pos17", &self.pos17())
                .finish()
        }
    }
    #[doc = "desc POSRD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Posrd(pub u32);
    impl Posrd {
        #[doc = "desc POS0."]
        #[must_use]
        #[inline(always)]
        pub const fn pos0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS0."]
        #[inline(always)]
        pub const fn set_pos0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POS1."]
        #[must_use]
        #[inline(always)]
        pub const fn pos1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS1."]
        #[inline(always)]
        pub const fn set_pos1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POS2."]
        #[must_use]
        #[inline(always)]
        pub const fn pos2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS2."]
        #[inline(always)]
        pub const fn set_pos2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POS3."]
        #[must_use]
        #[inline(always)]
        pub const fn pos3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS3."]
        #[inline(always)]
        pub const fn set_pos3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POS4."]
        #[must_use]
        #[inline(always)]
        pub const fn pos4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS4."]
        #[inline(always)]
        pub const fn set_pos4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POS5."]
        #[must_use]
        #[inline(always)]
        pub const fn pos5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS5."]
        #[inline(always)]
        pub const fn set_pos5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POS6."]
        #[must_use]
        #[inline(always)]
        pub const fn pos6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS6."]
        #[inline(always)]
        pub const fn set_pos6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POS7."]
        #[must_use]
        #[inline(always)]
        pub const fn pos7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS7."]
        #[inline(always)]
        pub const fn set_pos7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POS8."]
        #[must_use]
        #[inline(always)]
        pub const fn pos8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS8."]
        #[inline(always)]
        pub const fn set_pos8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POS9."]
        #[must_use]
        #[inline(always)]
        pub const fn pos9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS9."]
        #[inline(always)]
        pub const fn set_pos9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POS10."]
        #[must_use]
        #[inline(always)]
        pub const fn pos10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS10."]
        #[inline(always)]
        pub const fn set_pos10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POS11."]
        #[must_use]
        #[inline(always)]
        pub const fn pos11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS11."]
        #[inline(always)]
        pub const fn set_pos11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POS12."]
        #[must_use]
        #[inline(always)]
        pub const fn pos12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS12."]
        #[inline(always)]
        pub const fn set_pos12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POS13."]
        #[must_use]
        #[inline(always)]
        pub const fn pos13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS13."]
        #[inline(always)]
        pub const fn set_pos13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POS14."]
        #[must_use]
        #[inline(always)]
        pub const fn pos14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS14."]
        #[inline(always)]
        pub const fn set_pos14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POS15."]
        #[must_use]
        #[inline(always)]
        pub const fn pos15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS15."]
        #[inline(always)]
        pub const fn set_pos15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POS16."]
        #[must_use]
        #[inline(always)]
        pub const fn pos16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS16."]
        #[inline(always)]
        pub const fn set_pos16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POS17."]
        #[must_use]
        #[inline(always)]
        pub const fn pos17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS17."]
        #[inline(always)]
        pub const fn set_pos17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Posrd {
        #[inline(always)]
        fn default() -> Posrd {
            Posrd(0)
        }
    }
    impl core::fmt::Debug for Posrd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Posrd")
                .field("pos0", &self.pos0())
                .field("pos1", &self.pos1())
                .field("pos2", &self.pos2())
                .field("pos3", &self.pos3())
                .field("pos4", &self.pos4())
                .field("pos5", &self.pos5())
                .field("pos6", &self.pos6())
                .field("pos7", &self.pos7())
                .field("pos8", &self.pos8())
                .field("pos9", &self.pos9())
                .field("pos10", &self.pos10())
                .field("pos11", &self.pos11())
                .field("pos12", &self.pos12())
                .field("pos13", &self.pos13())
                .field("pos14", &self.pos14())
                .field("pos15", &self.pos15())
                .field("pos16", &self.pos16())
                .field("pos17", &self.pos17())
                .finish()
        }
    }
    #[doc = "desc POSRE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Posre(pub u32);
    impl Posre {
        #[doc = "desc POS0."]
        #[must_use]
        #[inline(always)]
        pub const fn pos0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS0."]
        #[inline(always)]
        pub const fn set_pos0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POS1."]
        #[must_use]
        #[inline(always)]
        pub const fn pos1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS1."]
        #[inline(always)]
        pub const fn set_pos1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POS2."]
        #[must_use]
        #[inline(always)]
        pub const fn pos2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS2."]
        #[inline(always)]
        pub const fn set_pos2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POS3."]
        #[must_use]
        #[inline(always)]
        pub const fn pos3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS3."]
        #[inline(always)]
        pub const fn set_pos3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POS4."]
        #[must_use]
        #[inline(always)]
        pub const fn pos4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS4."]
        #[inline(always)]
        pub const fn set_pos4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POS5."]
        #[must_use]
        #[inline(always)]
        pub const fn pos5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS5."]
        #[inline(always)]
        pub const fn set_pos5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POS6."]
        #[must_use]
        #[inline(always)]
        pub const fn pos6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS6."]
        #[inline(always)]
        pub const fn set_pos6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POS7."]
        #[must_use]
        #[inline(always)]
        pub const fn pos7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS7."]
        #[inline(always)]
        pub const fn set_pos7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POS8."]
        #[must_use]
        #[inline(always)]
        pub const fn pos8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS8."]
        #[inline(always)]
        pub const fn set_pos8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POS9."]
        #[must_use]
        #[inline(always)]
        pub const fn pos9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS9."]
        #[inline(always)]
        pub const fn set_pos9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POS10."]
        #[must_use]
        #[inline(always)]
        pub const fn pos10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS10."]
        #[inline(always)]
        pub const fn set_pos10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POS11."]
        #[must_use]
        #[inline(always)]
        pub const fn pos11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS11."]
        #[inline(always)]
        pub const fn set_pos11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POS12."]
        #[must_use]
        #[inline(always)]
        pub const fn pos12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS12."]
        #[inline(always)]
        pub const fn set_pos12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POS13."]
        #[must_use]
        #[inline(always)]
        pub const fn pos13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS13."]
        #[inline(always)]
        pub const fn set_pos13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POS14."]
        #[must_use]
        #[inline(always)]
        pub const fn pos14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS14."]
        #[inline(always)]
        pub const fn set_pos14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POS15."]
        #[must_use]
        #[inline(always)]
        pub const fn pos15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS15."]
        #[inline(always)]
        pub const fn set_pos15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POS16."]
        #[must_use]
        #[inline(always)]
        pub const fn pos16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS16."]
        #[inline(always)]
        pub const fn set_pos16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POS17."]
        #[must_use]
        #[inline(always)]
        pub const fn pos17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POS17."]
        #[inline(always)]
        pub const fn set_pos17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Posre {
        #[inline(always)]
        fn default() -> Posre {
            Posre(0)
        }
    }
    impl core::fmt::Debug for Posre {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Posre")
                .field("pos0", &self.pos0())
                .field("pos1", &self.pos1())
                .field("pos2", &self.pos2())
                .field("pos3", &self.pos3())
                .field("pos4", &self.pos4())
                .field("pos5", &self.pos5())
                .field("pos6", &self.pos6())
                .field("pos7", &self.pos7())
                .field("pos8", &self.pos8())
                .field("pos9", &self.pos9())
                .field("pos10", &self.pos10())
                .field("pos11", &self.pos11())
                .field("pos12", &self.pos12())
                .field("pos13", &self.pos13())
                .field("pos14", &self.pos14())
                .field("pos15", &self.pos15())
                .field("pos16", &self.pos16())
                .field("pos17", &self.pos17())
                .finish()
        }
    }
    #[doc = "desc POTRA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Potra(pub u32);
    impl Potra {
        #[doc = "desc POT0."]
        #[must_use]
        #[inline(always)]
        pub const fn pot0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT0."]
        #[inline(always)]
        pub const fn set_pot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POT1."]
        #[must_use]
        #[inline(always)]
        pub const fn pot1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT1."]
        #[inline(always)]
        pub const fn set_pot1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POT2."]
        #[must_use]
        #[inline(always)]
        pub const fn pot2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT2."]
        #[inline(always)]
        pub const fn set_pot2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POT3."]
        #[must_use]
        #[inline(always)]
        pub const fn pot3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT3."]
        #[inline(always)]
        pub const fn set_pot3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POT4."]
        #[must_use]
        #[inline(always)]
        pub const fn pot4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT4."]
        #[inline(always)]
        pub const fn set_pot4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POT5."]
        #[must_use]
        #[inline(always)]
        pub const fn pot5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT5."]
        #[inline(always)]
        pub const fn set_pot5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POT6."]
        #[must_use]
        #[inline(always)]
        pub const fn pot6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT6."]
        #[inline(always)]
        pub const fn set_pot6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POT7."]
        #[must_use]
        #[inline(always)]
        pub const fn pot7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT7."]
        #[inline(always)]
        pub const fn set_pot7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POT8."]
        #[must_use]
        #[inline(always)]
        pub const fn pot8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT8."]
        #[inline(always)]
        pub const fn set_pot8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POT9."]
        #[must_use]
        #[inline(always)]
        pub const fn pot9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT9."]
        #[inline(always)]
        pub const fn set_pot9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POT10."]
        #[must_use]
        #[inline(always)]
        pub const fn pot10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT10."]
        #[inline(always)]
        pub const fn set_pot10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POT11."]
        #[must_use]
        #[inline(always)]
        pub const fn pot11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT11."]
        #[inline(always)]
        pub const fn set_pot11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POT12."]
        #[must_use]
        #[inline(always)]
        pub const fn pot12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT12."]
        #[inline(always)]
        pub const fn set_pot12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POT13."]
        #[must_use]
        #[inline(always)]
        pub const fn pot13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT13."]
        #[inline(always)]
        pub const fn set_pot13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POT14."]
        #[must_use]
        #[inline(always)]
        pub const fn pot14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT14."]
        #[inline(always)]
        pub const fn set_pot14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POT15."]
        #[must_use]
        #[inline(always)]
        pub const fn pot15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT15."]
        #[inline(always)]
        pub const fn set_pot15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POT16."]
        #[must_use]
        #[inline(always)]
        pub const fn pot16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT16."]
        #[inline(always)]
        pub const fn set_pot16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POT17."]
        #[must_use]
        #[inline(always)]
        pub const fn pot17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT17."]
        #[inline(always)]
        pub const fn set_pot17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Potra {
        #[inline(always)]
        fn default() -> Potra {
            Potra(0)
        }
    }
    impl core::fmt::Debug for Potra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Potra")
                .field("pot0", &self.pot0())
                .field("pot1", &self.pot1())
                .field("pot2", &self.pot2())
                .field("pot3", &self.pot3())
                .field("pot4", &self.pot4())
                .field("pot5", &self.pot5())
                .field("pot6", &self.pot6())
                .field("pot7", &self.pot7())
                .field("pot8", &self.pot8())
                .field("pot9", &self.pot9())
                .field("pot10", &self.pot10())
                .field("pot11", &self.pot11())
                .field("pot12", &self.pot12())
                .field("pot13", &self.pot13())
                .field("pot14", &self.pot14())
                .field("pot15", &self.pot15())
                .field("pot16", &self.pot16())
                .field("pot17", &self.pot17())
                .finish()
        }
    }
    #[doc = "desc POTRB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Potrb(pub u32);
    impl Potrb {
        #[doc = "desc POT0."]
        #[must_use]
        #[inline(always)]
        pub const fn pot0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT0."]
        #[inline(always)]
        pub const fn set_pot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POT1."]
        #[must_use]
        #[inline(always)]
        pub const fn pot1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT1."]
        #[inline(always)]
        pub const fn set_pot1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POT2."]
        #[must_use]
        #[inline(always)]
        pub const fn pot2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT2."]
        #[inline(always)]
        pub const fn set_pot2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POT3."]
        #[must_use]
        #[inline(always)]
        pub const fn pot3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT3."]
        #[inline(always)]
        pub const fn set_pot3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POT4."]
        #[must_use]
        #[inline(always)]
        pub const fn pot4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT4."]
        #[inline(always)]
        pub const fn set_pot4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POT5."]
        #[must_use]
        #[inline(always)]
        pub const fn pot5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT5."]
        #[inline(always)]
        pub const fn set_pot5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POT6."]
        #[must_use]
        #[inline(always)]
        pub const fn pot6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT6."]
        #[inline(always)]
        pub const fn set_pot6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POT7."]
        #[must_use]
        #[inline(always)]
        pub const fn pot7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT7."]
        #[inline(always)]
        pub const fn set_pot7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POT8."]
        #[must_use]
        #[inline(always)]
        pub const fn pot8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT8."]
        #[inline(always)]
        pub const fn set_pot8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POT9."]
        #[must_use]
        #[inline(always)]
        pub const fn pot9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT9."]
        #[inline(always)]
        pub const fn set_pot9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POT10."]
        #[must_use]
        #[inline(always)]
        pub const fn pot10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT10."]
        #[inline(always)]
        pub const fn set_pot10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POT11."]
        #[must_use]
        #[inline(always)]
        pub const fn pot11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT11."]
        #[inline(always)]
        pub const fn set_pot11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POT12."]
        #[must_use]
        #[inline(always)]
        pub const fn pot12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT12."]
        #[inline(always)]
        pub const fn set_pot12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POT13."]
        #[must_use]
        #[inline(always)]
        pub const fn pot13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT13."]
        #[inline(always)]
        pub const fn set_pot13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POT14."]
        #[must_use]
        #[inline(always)]
        pub const fn pot14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT14."]
        #[inline(always)]
        pub const fn set_pot14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POT15."]
        #[must_use]
        #[inline(always)]
        pub const fn pot15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT15."]
        #[inline(always)]
        pub const fn set_pot15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POT16."]
        #[must_use]
        #[inline(always)]
        pub const fn pot16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT16."]
        #[inline(always)]
        pub const fn set_pot16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POT17."]
        #[must_use]
        #[inline(always)]
        pub const fn pot17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT17."]
        #[inline(always)]
        pub const fn set_pot17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Potrb {
        #[inline(always)]
        fn default() -> Potrb {
            Potrb(0)
        }
    }
    impl core::fmt::Debug for Potrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Potrb")
                .field("pot0", &self.pot0())
                .field("pot1", &self.pot1())
                .field("pot2", &self.pot2())
                .field("pot3", &self.pot3())
                .field("pot4", &self.pot4())
                .field("pot5", &self.pot5())
                .field("pot6", &self.pot6())
                .field("pot7", &self.pot7())
                .field("pot8", &self.pot8())
                .field("pot9", &self.pot9())
                .field("pot10", &self.pot10())
                .field("pot11", &self.pot11())
                .field("pot12", &self.pot12())
                .field("pot13", &self.pot13())
                .field("pot14", &self.pot14())
                .field("pot15", &self.pot15())
                .field("pot16", &self.pot16())
                .field("pot17", &self.pot17())
                .finish()
        }
    }
    #[doc = "desc POTRC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Potrc(pub u32);
    impl Potrc {
        #[doc = "desc POT0."]
        #[must_use]
        #[inline(always)]
        pub const fn pot0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT0."]
        #[inline(always)]
        pub const fn set_pot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POT1."]
        #[must_use]
        #[inline(always)]
        pub const fn pot1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT1."]
        #[inline(always)]
        pub const fn set_pot1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POT2."]
        #[must_use]
        #[inline(always)]
        pub const fn pot2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT2."]
        #[inline(always)]
        pub const fn set_pot2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POT3."]
        #[must_use]
        #[inline(always)]
        pub const fn pot3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT3."]
        #[inline(always)]
        pub const fn set_pot3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POT4."]
        #[must_use]
        #[inline(always)]
        pub const fn pot4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT4."]
        #[inline(always)]
        pub const fn set_pot4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POT5."]
        #[must_use]
        #[inline(always)]
        pub const fn pot5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT5."]
        #[inline(always)]
        pub const fn set_pot5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POT6."]
        #[must_use]
        #[inline(always)]
        pub const fn pot6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT6."]
        #[inline(always)]
        pub const fn set_pot6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POT7."]
        #[must_use]
        #[inline(always)]
        pub const fn pot7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT7."]
        #[inline(always)]
        pub const fn set_pot7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POT8."]
        #[must_use]
        #[inline(always)]
        pub const fn pot8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT8."]
        #[inline(always)]
        pub const fn set_pot8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POT9."]
        #[must_use]
        #[inline(always)]
        pub const fn pot9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT9."]
        #[inline(always)]
        pub const fn set_pot9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POT10."]
        #[must_use]
        #[inline(always)]
        pub const fn pot10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT10."]
        #[inline(always)]
        pub const fn set_pot10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POT11."]
        #[must_use]
        #[inline(always)]
        pub const fn pot11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT11."]
        #[inline(always)]
        pub const fn set_pot11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POT12."]
        #[must_use]
        #[inline(always)]
        pub const fn pot12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT12."]
        #[inline(always)]
        pub const fn set_pot12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POT13."]
        #[must_use]
        #[inline(always)]
        pub const fn pot13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT13."]
        #[inline(always)]
        pub const fn set_pot13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POT14."]
        #[must_use]
        #[inline(always)]
        pub const fn pot14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT14."]
        #[inline(always)]
        pub const fn set_pot14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POT15."]
        #[must_use]
        #[inline(always)]
        pub const fn pot15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT15."]
        #[inline(always)]
        pub const fn set_pot15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POT16."]
        #[must_use]
        #[inline(always)]
        pub const fn pot16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT16."]
        #[inline(always)]
        pub const fn set_pot16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POT17."]
        #[must_use]
        #[inline(always)]
        pub const fn pot17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT17."]
        #[inline(always)]
        pub const fn set_pot17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Potrc {
        #[inline(always)]
        fn default() -> Potrc {
            Potrc(0)
        }
    }
    impl core::fmt::Debug for Potrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Potrc")
                .field("pot0", &self.pot0())
                .field("pot1", &self.pot1())
                .field("pot2", &self.pot2())
                .field("pot3", &self.pot3())
                .field("pot4", &self.pot4())
                .field("pot5", &self.pot5())
                .field("pot6", &self.pot6())
                .field("pot7", &self.pot7())
                .field("pot8", &self.pot8())
                .field("pot9", &self.pot9())
                .field("pot10", &self.pot10())
                .field("pot11", &self.pot11())
                .field("pot12", &self.pot12())
                .field("pot13", &self.pot13())
                .field("pot14", &self.pot14())
                .field("pot15", &self.pot15())
                .field("pot16", &self.pot16())
                .field("pot17", &self.pot17())
                .finish()
        }
    }
    #[doc = "desc POTRD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Potrd(pub u32);
    impl Potrd {
        #[doc = "desc POT0."]
        #[must_use]
        #[inline(always)]
        pub const fn pot0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT0."]
        #[inline(always)]
        pub const fn set_pot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POT1."]
        #[must_use]
        #[inline(always)]
        pub const fn pot1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT1."]
        #[inline(always)]
        pub const fn set_pot1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POT2."]
        #[must_use]
        #[inline(always)]
        pub const fn pot2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT2."]
        #[inline(always)]
        pub const fn set_pot2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POT3."]
        #[must_use]
        #[inline(always)]
        pub const fn pot3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT3."]
        #[inline(always)]
        pub const fn set_pot3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POT4."]
        #[must_use]
        #[inline(always)]
        pub const fn pot4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT4."]
        #[inline(always)]
        pub const fn set_pot4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POT5."]
        #[must_use]
        #[inline(always)]
        pub const fn pot5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT5."]
        #[inline(always)]
        pub const fn set_pot5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POT6."]
        #[must_use]
        #[inline(always)]
        pub const fn pot6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT6."]
        #[inline(always)]
        pub const fn set_pot6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POT7."]
        #[must_use]
        #[inline(always)]
        pub const fn pot7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT7."]
        #[inline(always)]
        pub const fn set_pot7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POT8."]
        #[must_use]
        #[inline(always)]
        pub const fn pot8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT8."]
        #[inline(always)]
        pub const fn set_pot8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POT9."]
        #[must_use]
        #[inline(always)]
        pub const fn pot9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT9."]
        #[inline(always)]
        pub const fn set_pot9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POT10."]
        #[must_use]
        #[inline(always)]
        pub const fn pot10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT10."]
        #[inline(always)]
        pub const fn set_pot10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POT11."]
        #[must_use]
        #[inline(always)]
        pub const fn pot11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT11."]
        #[inline(always)]
        pub const fn set_pot11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POT12."]
        #[must_use]
        #[inline(always)]
        pub const fn pot12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT12."]
        #[inline(always)]
        pub const fn set_pot12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POT13."]
        #[must_use]
        #[inline(always)]
        pub const fn pot13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT13."]
        #[inline(always)]
        pub const fn set_pot13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POT14."]
        #[must_use]
        #[inline(always)]
        pub const fn pot14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT14."]
        #[inline(always)]
        pub const fn set_pot14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POT15."]
        #[must_use]
        #[inline(always)]
        pub const fn pot15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT15."]
        #[inline(always)]
        pub const fn set_pot15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POT16."]
        #[must_use]
        #[inline(always)]
        pub const fn pot16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT16."]
        #[inline(always)]
        pub const fn set_pot16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POT17."]
        #[must_use]
        #[inline(always)]
        pub const fn pot17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT17."]
        #[inline(always)]
        pub const fn set_pot17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Potrd {
        #[inline(always)]
        fn default() -> Potrd {
            Potrd(0)
        }
    }
    impl core::fmt::Debug for Potrd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Potrd")
                .field("pot0", &self.pot0())
                .field("pot1", &self.pot1())
                .field("pot2", &self.pot2())
                .field("pot3", &self.pot3())
                .field("pot4", &self.pot4())
                .field("pot5", &self.pot5())
                .field("pot6", &self.pot6())
                .field("pot7", &self.pot7())
                .field("pot8", &self.pot8())
                .field("pot9", &self.pot9())
                .field("pot10", &self.pot10())
                .field("pot11", &self.pot11())
                .field("pot12", &self.pot12())
                .field("pot13", &self.pot13())
                .field("pot14", &self.pot14())
                .field("pot15", &self.pot15())
                .field("pot16", &self.pot16())
                .field("pot17", &self.pot17())
                .finish()
        }
    }
    #[doc = "desc POTRE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Potre(pub u32);
    impl Potre {
        #[doc = "desc POT0."]
        #[must_use]
        #[inline(always)]
        pub const fn pot0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT0."]
        #[inline(always)]
        pub const fn set_pot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POT1."]
        #[must_use]
        #[inline(always)]
        pub const fn pot1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT1."]
        #[inline(always)]
        pub const fn set_pot1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc POT2."]
        #[must_use]
        #[inline(always)]
        pub const fn pot2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT2."]
        #[inline(always)]
        pub const fn set_pot2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc POT3."]
        #[must_use]
        #[inline(always)]
        pub const fn pot3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT3."]
        #[inline(always)]
        pub const fn set_pot3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POT4."]
        #[must_use]
        #[inline(always)]
        pub const fn pot4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT4."]
        #[inline(always)]
        pub const fn set_pot4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc POT5."]
        #[must_use]
        #[inline(always)]
        pub const fn pot5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT5."]
        #[inline(always)]
        pub const fn set_pot5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POT6."]
        #[must_use]
        #[inline(always)]
        pub const fn pot6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT6."]
        #[inline(always)]
        pub const fn set_pot6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc POT7."]
        #[must_use]
        #[inline(always)]
        pub const fn pot7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT7."]
        #[inline(always)]
        pub const fn set_pot7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc POT8."]
        #[must_use]
        #[inline(always)]
        pub const fn pot8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT8."]
        #[inline(always)]
        pub const fn set_pot8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc POT9."]
        #[must_use]
        #[inline(always)]
        pub const fn pot9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT9."]
        #[inline(always)]
        pub const fn set_pot9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc POT10."]
        #[must_use]
        #[inline(always)]
        pub const fn pot10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT10."]
        #[inline(always)]
        pub const fn set_pot10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc POT11."]
        #[must_use]
        #[inline(always)]
        pub const fn pot11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT11."]
        #[inline(always)]
        pub const fn set_pot11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc POT12."]
        #[must_use]
        #[inline(always)]
        pub const fn pot12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT12."]
        #[inline(always)]
        pub const fn set_pot12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc POT13."]
        #[must_use]
        #[inline(always)]
        pub const fn pot13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT13."]
        #[inline(always)]
        pub const fn set_pot13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc POT14."]
        #[must_use]
        #[inline(always)]
        pub const fn pot14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT14."]
        #[inline(always)]
        pub const fn set_pot14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc POT15."]
        #[must_use]
        #[inline(always)]
        pub const fn pot15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT15."]
        #[inline(always)]
        pub const fn set_pot15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc POT16."]
        #[must_use]
        #[inline(always)]
        pub const fn pot16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT16."]
        #[inline(always)]
        pub const fn set_pot16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc POT17."]
        #[must_use]
        #[inline(always)]
        pub const fn pot17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc POT17."]
        #[inline(always)]
        pub const fn set_pot17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Potre {
        #[inline(always)]
        fn default() -> Potre {
            Potre(0)
        }
    }
    impl core::fmt::Debug for Potre {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Potre")
                .field("pot0", &self.pot0())
                .field("pot1", &self.pot1())
                .field("pot2", &self.pot2())
                .field("pot3", &self.pot3())
                .field("pot4", &self.pot4())
                .field("pot5", &self.pot5())
                .field("pot6", &self.pot6())
                .field("pot7", &self.pot7())
                .field("pot8", &self.pot8())
                .field("pot9", &self.pot9())
                .field("pot10", &self.pot10())
                .field("pot11", &self.pot11())
                .field("pot12", &self.pot12())
                .field("pot13", &self.pot13())
                .field("pot14", &self.pot14())
                .field("pot15", &self.pot15())
                .field("pot16", &self.pot16())
                .field("pot17", &self.pot17())
                .finish()
        }
    }
    #[doc = "desc PSPCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pspcr(pub u16);
    impl Pspcr {
        #[doc = "desc SPFE."]
        #[must_use]
        #[inline(always)]
        pub const fn spfe(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc SPFE."]
        #[inline(always)]
        pub const fn set_spfe(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
    }
    impl Default for Pspcr {
        #[inline(always)]
        fn default() -> Pspcr {
            Pspcr(0)
        }
    }
    impl core::fmt::Debug for Pspcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pspcr").field("spfe", &self.spfe()).finish()
        }
    }
    #[doc = "desc PWPR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwpr(pub u16);
    impl Pwpr {
        #[doc = "desc WE."]
        #[must_use]
        #[inline(always)]
        pub const fn we(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc WE."]
        #[inline(always)]
        pub const fn set_we(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc WP."]
        #[must_use]
        #[inline(always)]
        pub const fn wp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc WP."]
        #[inline(always)]
        pub const fn set_wp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for Pwpr {
        #[inline(always)]
        fn default() -> Pwpr {
            Pwpr(0)
        }
    }
    impl core::fmt::Debug for Pwpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwpr")
                .field("we", &self.we())
                .field("wp", &self.wp())
                .finish()
        }
    }
}
