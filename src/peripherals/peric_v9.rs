#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]




# [doc = "desc PERIC."]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Peric { ptr : * mut u8 } unsafe impl Send for Peric { } unsafe impl Sync for Peric { } impl Peric { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [doc = "desc TMR_SYNENR."]
# [inline (always)]
pub const fn tmr_synenr (self) -> crate :: common :: Reg < regs :: TmrSynenr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0usize) as _) } } } pub mod regs { # [doc = "desc TMR_SYNENR."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct TmrSynenr (pub u32) ; impl TmrSynenr { # [doc = "desc TMRA."]
# [must_use]
# [inline (always)]
pub const fn tmra (& self) -> bool { let val = (self . 0 >> 0usize) & 0x01 ; val != 0 } # [doc = "desc TMRA."]
# [inline (always)]
pub const fn set_tmra (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize) ; } # [doc = "desc TMR4."]
# [must_use]
# [inline (always)]
pub const fn tmr4 (& self) -> bool { let val = (self . 0 >> 1usize) & 0x01 ; val != 0 } # [doc = "desc TMR4."]
# [inline (always)]
pub const fn set_tmr4 (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize) ; } } impl Default for TmrSynenr { # [inline (always)]
fn default () -> TmrSynenr { TmrSynenr (0) } } impl core :: fmt :: Debug for TmrSynenr { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("TmrSynenr") . field ("tmra" , & self . tmra ()) . field ("tmr4" , & self . tmr4 ()) . finish () } } }