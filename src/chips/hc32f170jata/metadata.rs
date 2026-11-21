include!("../metadata_0011.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F170JATA",
    family: "HC32F17x",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
