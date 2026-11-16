#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]




# [doc = "desc EFM2."]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Efm { ptr : * mut u8 } unsafe impl Send for Efm { } unsafe impl Sync for Efm { } impl Efm { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [doc = "desc PN0."]
# [inline (always)]
pub const fn pn0 (self) -> crate :: common :: Reg < u32 , crate :: common :: R > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0usize) as _) } } # [doc = "desc PN1."]
# [inline (always)]
pub const fn pn1 (self) -> crate :: common :: Reg < u32 , crate :: common :: R > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x04usize) as _) } } # [doc = "desc PN2."]
# [inline (always)]
pub const fn pn2 (self) -> crate :: common :: Reg < u32 , crate :: common :: R > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x08usize) as _) } } # [doc = "desc PN3."]
# [inline (always)]
pub const fn pn3 (self) -> crate :: common :: Reg < u32 , crate :: common :: R > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0cusize) as _) } } }