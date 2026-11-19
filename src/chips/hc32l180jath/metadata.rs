include!("../metadata_0028.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32L180JATH",
    family: "HC32L18x",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
