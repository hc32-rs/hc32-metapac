// Always-available memory geometry. Exposed per chip as `MEMORY_LAYOUT` at the
// crate root. HC32 regions are named FLASH / SRAM after the XHSC manuals;
// independent retention RAM banks follow the SDK linker names RAMB / RET_RAM.

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct MemoryLayout {
    pub regions: &'static [MemoryRegion],
}

impl MemoryLayout {
    pub const fn find(&self, name: &str) -> Option<&MemoryRegion> {
        let mut i = 0;
        while i < self.regions.len() {
            if str_eq(self.regions[i].name, name) {
                return Some(&self.regions[i]);
            }
            i += 1;
        }
        None
    }

    pub const fn find_by_role(&self, role: MemoryRole) -> Option<&MemoryRegion> {
        let mut i = 0;
        while i < self.regions.len() {
            if self.regions[i].role as u8 == role as u8 {
                return Some(&self.regions[i]);
            }
            i += 1;
        }
        None
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MemoryRegion {
    pub name: &'static str,
    pub kind: MemoryRegionKind,
    pub role: MemoryRole,
    pub address: u32,
    pub size: u32,
    pub access: Option<Access>,
}

impl MemoryRegion {
    pub const fn end(&self) -> u32 {
        self.address + self.size
    }

    pub const fn contains(&self, addr: u32) -> bool {
        addr >= self.address && addr < self.end()
    }

    pub const fn readable(&self) -> bool {
        match &self.access {
            Some(a) => a.read,
            None => true,
        }
    }

    pub const fn writable(&self) -> bool {
        match &self.access {
            Some(a) => a.write,
            None => true,
        }
    }

    pub const fn executable(&self) -> bool {
        match &self.access {
            Some(a) => a.execute,
            None => true,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
#[non_exhaustive]
pub enum MemoryRole {
    /// User application flash (`FLASH`).
    Application,
    /// System / factory bootloader flash (`SYS_*`).
    System,
    /// Option-byte region (`OPT` / EEPROM).
    OptionBytes,
    /// General-purpose SRAM (`SRAM`).
    Ram,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Access {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MemoryRegionKind {
    Flash,
    Ram,
}

const fn str_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}
