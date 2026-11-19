include!("../metadata_0031.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32M441JATI",
    family: "HC32M441",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
