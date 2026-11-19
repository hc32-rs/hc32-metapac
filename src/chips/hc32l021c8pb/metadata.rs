include!("../metadata_0024.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32L021C8PB",
    family: "HC32L021",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
