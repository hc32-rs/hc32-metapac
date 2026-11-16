include!("../metadata_0006.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32F002D4PZ",
                family: "HC32F002",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };