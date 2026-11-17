include!("../metadata_0029.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32M441JCTI",
    family: "HC32M441",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
