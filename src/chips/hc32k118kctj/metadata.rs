include!("../metadata_0020.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32K118KCTJ",
    family: "HC32K118",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
