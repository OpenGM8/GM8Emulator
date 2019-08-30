use crate::asset::{assert_ver, Asset, AssetDataError};
use crate::byteio::{ReadBytes, ReadString, WriteBytes, WriteString};
use crate::GameVersion;
use std::io::{self, Seek, SeekFrom};

pub const VERSION: u32 = 800;

pub struct Script {
    /// The asset name present in GML and the editor.
    pub name: String,

    /// The GML code in the script.
    pub source: String,
}

impl Asset for Script {
    fn deserialize<B>(from: B, strict: bool, _version: GameVersion) -> Result<Self, AssetDataError>
    where
        B: AsRef<[u8]>,
        Self: Sized,
    {
        let mut reader = io::Cursor::new(from.as_ref());
        let name = reader.read_pas_string()?;

        if strict {
            let version = reader.read_u32_le()?;
            assert_ver(version, VERSION)?;
        } else {
            reader.seek(SeekFrom::Current(4))?;
        }

        let source = reader.read_pas_string()?;
        Ok(Script { name, source })
    }

    fn serialize<W>(&self, to: &mut W) -> io::Result<usize>
    where
        W: io::Write,
    {
        let mut result = to.write_pas_string(&self.name)?;
        result += to.write_u32_le(VERSION as u32)?;
        result += to.write_pas_string(&self.source)?;

        Ok(result)
    }
}
