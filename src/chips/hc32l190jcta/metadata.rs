include!("../metadata_0027.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32L190JCTA",
    family: "HC32L19x",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
