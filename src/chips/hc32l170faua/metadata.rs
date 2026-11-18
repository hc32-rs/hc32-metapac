include!("../metadata_0026.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32L170FAUA",
    family: "HC32L17x",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
