#![allow(dead_code)] // Shut up.

use crate::bytes::{ReadBytes, ReadString, WriteBytes, WriteString};
use crate::types::Version;
use std::io::{self, Seek, SeekFrom};

pub const VERSION: Version = 800;

pub struct Script {
    /// The asset name present in GML and the editor.
    pub name: String,

    /// The full source code for the script.
    pub source: String,
}

impl Script {
    pub fn serialize<W>(&self, writer: &mut W) -> io::Result<usize>
    where
        W: io::Write,
    {
        let mut result = writer.write_pas_string(&self.name)?;
        result += writer.write_u32_le(VERSION as u32)?;
        result += writer.write_pas_string(&self.source)?;

        Ok(result)
    }

    pub fn deserialize<B>(bin: B, strict: bool) -> io::Result<Script>
    where
        B: AsRef<[u8]>,
    {
        let mut reader = io::Cursor::new(bin.as_ref());
        let name = reader.read_pas_string()?;

        if strict {
            let version = reader.read_u32_le()?;
            assert_eq!(version, VERSION);
        } else {
            reader.seek(SeekFrom::Current(4))?;
        }

        let source = reader.read_pas_string()?;

        Ok(Script { name, source })
    }
}
