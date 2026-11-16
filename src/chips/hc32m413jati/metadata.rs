include!("../metadata_0016.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32M413JATI",
                family: "HC32M413",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };