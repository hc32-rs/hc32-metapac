include!("../metadata_0016.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F431JATB",
    family: "HC32F431",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
