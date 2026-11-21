include!("../metadata_0009.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F072PATA",
    family: "HC32F072",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
