include!("../metadata_0012.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F196KCTA",
    family: "HC32F19x",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
