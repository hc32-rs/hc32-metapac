include!("../metadata_0012.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F175HETI",
    family: "HC32F175",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
