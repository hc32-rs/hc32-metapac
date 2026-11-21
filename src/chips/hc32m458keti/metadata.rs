include!("../metadata_0032.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32M458KETI",
    family: "HC32M458",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
