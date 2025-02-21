use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Config file mainly consisting of paths to extracted files.
#[derive(Serialize, Deserialize, Clone)]
pub struct RomConfig {
    /// Byte value to append between ROM sections
    pub padding_value: u8,

    /// Path to header YAML
    pub header: PathBuf,
    /// Path to header logo PNG
    pub header_logo: PathBuf,

    /// Path to ARM9 binary
    pub arm9_bin: PathBuf,
    /// Path to ARM9 YAML
    pub arm9_config: PathBuf,

    /// Path to ARM7 binary
    pub arm7_bin: PathBuf,
    /// Path to ARM7 YAML
    pub arm7_config: PathBuf,

    /// Path to ITCM files
    pub itcm: RomConfigAutoload,
    /// Path to DTCM files
    pub dtcm: RomConfigAutoload,
    /// Path to unknown autoloads
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub unknown_autoloads: Vec<RomConfigAutoload>,

    /// Path to ARM9 overlays YAML
    pub arm9_overlays: Option<PathBuf>,
    /// Path to ARM7 overlays YAML
    pub arm7_overlays: Option<PathBuf>,

    /// Path to banner YAML
    pub banner: PathBuf,

    /// Path to asset files directory
    pub files_dir: PathBuf,
    /// Path to path order file
    pub path_order: PathBuf,
}

/// Path to autoload files
#[derive(Serialize, Deserialize, Clone)]
pub struct RomConfigAutoload {
    /// Path to binary
    pub bin: PathBuf,
    /// Path to YAML
    pub config: PathBuf,
}
