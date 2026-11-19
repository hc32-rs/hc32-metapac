include!("../metadata_0004.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32A4A0SITI",
    family: "HC32A4A0",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
