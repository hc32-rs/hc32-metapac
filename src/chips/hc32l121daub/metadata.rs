include!("../metadata_0025.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32L121DAUB",
                family: "HC32L12x",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };