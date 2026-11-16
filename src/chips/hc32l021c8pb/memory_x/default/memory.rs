use crate::mem_layout::{Access, MemoryRegion, MemoryRegionKind, MemoryRole::*};

pub static MEMORY: &[MemoryRegion] = &[
    MemoryRegion {
        name: "FLASH",
        kind: MemoryRegionKind::Flash,
        role: Application,
        address: 0x0,
        size: 65536,
        access: Some(
            Access {
                read: true,
                write: true,
                execute: true,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        role: Ram,
        address: 0x20000000,
        size: 6144,
        access: Some(
            Access {
                read: true,
                write: true,
                execute: true,
            },
        ),
    },
];
