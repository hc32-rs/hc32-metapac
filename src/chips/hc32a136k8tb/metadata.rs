include!("../metadata_0001.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32A136K8TB",
    family: "HC32A136",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
