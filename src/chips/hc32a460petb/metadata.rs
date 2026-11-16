include!("../metadata_0003.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32A460PETB",
                family: "HC32A460",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };