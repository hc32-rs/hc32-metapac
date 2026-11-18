include!("../metadata_0022.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32L073JATA",
    family: "HC32L07x",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
