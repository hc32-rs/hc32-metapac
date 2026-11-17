include!("../metadata_0010.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F115H8TH",
    family: "HC32F115",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
