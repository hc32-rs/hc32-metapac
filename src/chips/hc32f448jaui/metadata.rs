include!("../metadata_0002.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F448JAUI",
    family: "HC32F448",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
