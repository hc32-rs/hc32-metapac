include!("../metadata_0027.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32L176KATA",
                family: "HC32L17x",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };