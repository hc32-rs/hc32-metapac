include!("../metadata_0017.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F451FEUB",
    family: "HC32F451",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
