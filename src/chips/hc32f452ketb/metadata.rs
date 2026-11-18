include!("../metadata_0003.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F452KETB",
    family: "HC32F452",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
