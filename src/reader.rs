use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;
use std::{fs, path::Path};

use crate::{
    file_metadata::read_file_metadata, magic::ensure_header_footer_magic,
    row_group::read_row_groups,
};

/// Read a parquet file into [`DataFrame`].
///
/// This function verifies if the magic number is correct,
/// reads the file metadata, then parses all row groups into the [`DataFrame`].
#[allow(unused_variables)]
pub fn read_parquet(file_path: impl AsRef<Path>) -> Result<DataFrame> {
    let file_data = fs::read(file_path)?;
    let data = Bytes::from(file_data);
    ensure_header_footer_magic(data.clone())?;
    let metadata = read_file_metadata(data.clone())?;
    read_row_groups(data, &metadata)
}
