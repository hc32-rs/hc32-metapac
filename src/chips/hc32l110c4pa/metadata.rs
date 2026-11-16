include!("../metadata_0024.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32L110C4PA",
                family: "HC32L110",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };