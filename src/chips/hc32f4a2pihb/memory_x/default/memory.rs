use crate::mem_layout::{Access, MemoryRegion, MemoryRegionKind, MemoryRole::*};

pub static MEMORY: &[MemoryRegion] = &[
    MemoryRegion {
        name: "FLASH",
        kind: MemoryRegionKind::Flash,
        role: Application,
        address: 0x0,
        size: 2097152,
        access: Some(Access {
            read: true,
            write: true,
            execute: true,
        }),
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        role: Ram,
        address: 0x1ffe0000,
        size: 524288,
        access: Some(Access {
            read: true,
            write: true,
            execute: true,
        }),
    },
];
