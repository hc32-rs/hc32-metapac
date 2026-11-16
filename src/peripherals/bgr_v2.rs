#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]




# [doc = "desc BGR."]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Bgr { ptr : * mut u8 } unsafe impl Send for Bgr { } unsafe impl Sync for Bgr { } impl Bgr { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [doc = "desc CR."]
# [inline (always)]
pub const fn cr (self) -> crate :: common :: Reg < regs :: Cr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0usize) as _) } } } pub mod regs { # [doc = "desc CR."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Cr (pub u32) ; impl Cr { # [doc = "desc BGR_EN."]
# [must_use]
# [inline (always)]
pub const fn bgr_en (& self) -> bool { let val = (self . 0 >> 0usize) & 0x01 ; val != 0 } # [doc = "desc BGR_EN."]
# [inline (always)]
pub const fn set_bgr_en (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize) ; } # [doc = "desc TS_EN."]
# [must_use]
# [inline (always)]
pub const fn ts_en (& self) -> bool { let val = (self . 0 >> 1usize) & 0x01 ; val != 0 } # [doc = "desc TS_EN."]
# [inline (always)]
pub const fn set_ts_en (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize) ; } } impl Default for Cr { # [inline (always)]
fn default () -> Cr { Cr (0) } } impl core :: fmt :: Debug for Cr { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("Cr") . field ("bgr_en" , & self . bgr_en ()) . field ("ts_en" , & self . ts_en ()) . finish () } } }