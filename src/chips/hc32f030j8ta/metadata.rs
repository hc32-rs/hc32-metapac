include!("../metadata_0008.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F030J8TA",
    family: "HC32F030",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
