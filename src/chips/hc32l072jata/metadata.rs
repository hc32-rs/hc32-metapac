include!("../metadata_0023.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32L072JATA",
                family: "HC32L07x",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };