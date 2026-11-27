use core::fmt;

/// Runtime distribution markers encoded into the packed version build nibble.
pub const RUNTIME_TYPE_BETHESDA: u16 = 0;
pub const RUNTIME_TYPE_VR: u16 = 1;
pub const RUNTIME_TYPE_GOG: u16 = 1; // VR and GOG share the same build marker in SKSE.
pub const RUNTIME_TYPE_EPIC: u16 = 2;

/// Packed Skyrim runtime version used by both SKSE and REL::Version-like helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Version(u32);

impl Version {
    pub const fn new(major: u16, minor: u16, patch: u16, build: u16) -> Self {
        Self(
            ((major as u32 & 0xFF) << 24)
                | ((minor as u32 & 0xFF) << 16)
                | ((patch as u32 & 0xFFF) << 4)
                | (build as u32 & 0xF),
        )
    }

    pub const fn from_packed(packed: u32) -> Self {
        Self(packed)
    }

    pub const fn pack(&self) -> u32 {
        self.0
    }

    pub const fn major(&self) -> u16 {
        (self.0 >> 24) as u16
    }

    pub const fn minor(&self) -> u16 {
        ((self.0 >> 16) & 0xFF) as u16
    }

    pub const fn patch(&self) -> u16 {
        ((self.0 >> 4) & 0xFFF) as u16
    }

    pub const fn build(&self) -> u16 {
        (self.0 & 0xF) as u16
    }

    pub fn save_folder(&self) -> &'static str {
        // `minor == 4` identifies Skyrim VR. VR and GOG share the same build flag,
        // so the runtime minor is the reliable discriminator here.
        if self.minor() == 4 {
            return "Skyrim VR";
        }

        // Non-VR runtimes use the build nibble to distinguish Steam/Bethesda,
        // GOG, and Epic builds in the same way SKSE does.
        match self.build() {
            RUNTIME_TYPE_EPIC => "Skyrim Special Edition EPIC",
            RUNTIME_TYPE_GOG => "Skyrim Special Edition GOG",
            _ => "Skyrim Special Edition", // Steam / Bethesda.net
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major(),
            self.minor(),
            self.patch(),
            self.build()
        )
    }
}

// Selected runtime constants mirrored from SKSE `Version.h`.
pub const RUNTIME_SSE_1_5_39: Version = Version::new(1, 5, 39, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_5_97: Version = Version::new(1, 5, 97, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_318: Version = Version::new(1, 6, 318, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_353: Version = Version::new(1, 6, 353, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_629: Version = Version::new(1, 6, 629, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_640: Version = Version::new(1, 6, 640, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_659_GOG: Version = Version::new(1, 6, 659, RUNTIME_TYPE_GOG);
pub const RUNTIME_SSE_1_6_678_EPIC: Version = Version::new(1, 6, 678, RUNTIME_TYPE_EPIC);
pub const RUNTIME_SSE_1_6_1130: Version = Version::new(1, 6, 1130, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_1170: Version = Version::new(1, 6, 1170, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_1179_GOG: Version = Version::new(1, 6, 1179, RUNTIME_TYPE_GOG);
pub const RUNTIME_SSE_1_7_99: Version = Version::new(1, 7, 99, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_7_104: Version = Version::new(1, 7, 104, RUNTIME_TYPE_BETHESDA);

pub const RUNTIME_VR_1_4_15: Version = Version::new(1, 4, 15, RUNTIME_TYPE_VR);

pub const RUNTIME_LATEST_SE: Version = RUNTIME_SSE_1_5_97;
pub const RUNTIME_LATEST_AE: Version = RUNTIME_SSE_1_7_104;
pub const RUNTIME_LATEST_VR: Version = RUNTIME_VR_1_4_15;
