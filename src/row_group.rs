use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::{
    column::read_column,
    format::{FileMetaData, RowGroup},
};

/// Read a row group into [`DataFrame`].
///
/// A row group contains multiple column chunks.
/// This function reads all the column chunks into a single [`DataFrame`].
#[allow(unused_variables)]
pub fn read_row_group(data: Bytes, row_group: &RowGroup) -> Result<DataFrame> {
    let mut columns = vec![];

    for column in &row_group.columns {
        let col = read_column(data.clone(), &column)?;
        columns.push(col);
    }

    Ok(DataFrame::new_infer_height(columns)?)
}

/// Read row groups into [`DataFrame`].
///
/// A file contains multiple row groups.
/// This function reads all the row groups, and
/// concatenate all the returned [`DataFrame`]s into a single [`DataFrame`].
#[allow(unused_variables)]
pub fn read_row_groups(data: Bytes, file_metadata: &FileMetaData) -> Result<DataFrame> {
    let mut frames = vec![];
    for row in &file_metadata.row_groups {
        let df = read_row_group(data.clone(), &row)?;
        frames.push(df.lazy());
    }

    Ok(concat(
        frames,
        UnionArgs {
            strict: true,
            ..Default::default()
        },
    )?
    .collect()?)
}
