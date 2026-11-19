include!("../metadata_0013.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F334F8UI",
    family: "HC32F334",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
