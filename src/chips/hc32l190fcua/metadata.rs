include!("../metadata_0028.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32L190FCUA",
                family: "HC32L19x",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };