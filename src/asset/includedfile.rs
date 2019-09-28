use crate::asset::{assert_ver, Asset, AssetDataError, ReadPascalString, /* WritePascalString */};
use crate::GameVersion;

use minio::{ReadPrimitives, /* WritePrimitives */};
use std::io::{self, Seek, SeekFrom};

pub const VERSION: u32 = 800;

pub struct IncludedFile {
    /// The name of the included file.
    pub file_name: String,
    
    /// The path of the source file (from the developer's PC).
    pub source_path: String,
    
    /// The length of the source file.
    pub source_length: usize,

    /// Whether the file is embedded.
    pub stored_in_gmk: bool,
    
    /// Contains the embedded data, if it is embedded.
    pub embedded_data: Option<Box<[u8]>>,
    
    /// The export settings used for the file on load.
    pub export_settings: ExportSetting,

    /// Overwrite file if it exists (while exporting).
    pub overwrite_file: bool,

    /// Whether to free memory after exporting.
    /// Why is this an option.
    pub free_memory: bool,

    /// Whether to delete the exported external file at the end.
    pub remove_at_end: bool,
}

pub enum ExportSetting {
    NoExport,
    TempFolder,
    GameFolder,
    CustomFolder(String),
}

impl Asset for IncludedFile {
    fn deserialize<B>(bytes: B, strict: bool, _version: GameVersion) -> Result<Self, AssetDataError>
    where
        B: AsRef<[u8]>,
        Self: Sized,
    {
        let mut reader = io::Cursor::new(bytes.as_ref());

        if strict {
            let version = reader.read_u32_le()?;
            assert_ver(version, VERSION)?;
        } else {
            reader.seek(SeekFrom::Current(4))?;
        }

        let file_name = reader.read_pas_string()?;
        let source_path = reader.read_pas_string()?;

        let data_exists = reader.read_u32_le()? != 0;
        let source_length = reader.read_u32_le()? as usize;
        let stored_in_gmk = reader.read_u32_le()? != 0;

        let embedded_data = if stored_in_gmk && data_exists {
            let len = reader.read_u32_le()? as usize;
            let pos = reader.position() as usize;
            reader.seek(SeekFrom::Current(len as i64))?;
            Some(reader.get_ref()
                .get(pos..pos+len)
                .unwrap_or_else(|| unreachable!())
                .to_vec()
                .into_boxed_slice()
            )
        } else {
            None
        };

        let export_flag = reader.read_u32_le()?;
        let custom_folder_path = reader.read_pas_string()?; // always present
        let export_settings = match export_flag {
            0 => ExportSetting::NoExport,
            1 => ExportSetting::TempFolder,
            2 => ExportSetting::GameFolder,
            3 | _ => ExportSetting::CustomFolder(custom_folder_path),
        };

        let overwrite_file = reader.read_u32_le()? != 0;
        let free_memory = reader.read_u32_le()? != 0;
        let remove_at_end = reader.read_u32_le()? != 0;

        Ok(IncludedFile {
            file_name,
            source_path,
            source_length,
            stored_in_gmk,
            embedded_data,
            export_settings,
            overwrite_file,
            free_memory,
            remove_at_end,
        })
    }

    fn serialize<W>(&self, _writer: &mut W) -> io::Result<usize>
    where
        W: io::Write,
    {
        Ok(0)
    }
}
