include!("../metadata_0018.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F460KETA",
    family: "HC32F460",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
