include!("../metadata_0020.rs");
pub static METADATA: Metadata = Metadata {
    name: "HC32F467PGTI",
    family: "HC32F467",
    memory: crate::memory_select::MEMORY,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
