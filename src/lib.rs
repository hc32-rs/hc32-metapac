#![no_std]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(non_camel_case_types)]
#![doc(html_no_source)]

pub mod common;
pub mod mem_layout;

#[cfg(any(feature = "pac", feature = "metadata"))]
mod memory_select {
    include!(env!("HC32_METAPAC_MEMORY_PATH"));
}

#[cfg(feature = "pac")]
include!(env!("HC32_METAPAC_PAC_PATH"));

#[cfg(feature = "metadata")]
pub mod metadata {
    include!("metadata.rs");
    include!(env!("HC32_METAPAC_METADATA_PATH"));
}
