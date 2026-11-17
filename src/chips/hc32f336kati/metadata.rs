include!("../metadata_0014.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F336KATI",
    family: "HC32F336",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
