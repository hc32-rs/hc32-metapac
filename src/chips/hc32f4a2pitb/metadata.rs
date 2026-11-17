include!("../metadata_0020.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F4A2PITB",
    family: "HC32F4A2",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
