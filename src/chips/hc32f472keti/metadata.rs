include!("../metadata_0021.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F472KETI",
    family: "HC32F472",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
