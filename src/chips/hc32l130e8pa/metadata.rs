include!("../metadata_0001.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32L130E8PA",
                family: "HC32L13x",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };