include!("../metadata_0021.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32K118FCUJ",
    family: "HC32K118",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
