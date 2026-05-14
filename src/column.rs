use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::{
    decoder::decode_page,
    format::{ColumnChunk, ColumnMetaData},
    page::read_pages,
};

/// Convert a vector of [`Scalar`] to a [`Column`].
#[allow(unused)]
fn column_from_scalars(scalars: Vec<Scalar>, column_metadata: &ColumnMetaData) -> Result<Column> {
    let values: Vec<AnyValue<'_>> = scalars
        .into_iter()
        .map(|scalar| scalar.into_value())
        .collect();

    let column_name = column_metadata.path_in_schema.join(".");
    let series = Series::from_any_values(column_name.into(), &values, true)?;
    let column = Column::from(series);

    Ok(column)
}

/// Read [`Column`] from a parquet file based on [`ColumnChunk`]'s metadata.
///
/// A column chunk contains multiple pages, this function extract all the pages,
/// decodes them and returns the correct [`Column`] for a chunk.
#[allow(unused_variables)]
pub fn read_column(data: Bytes, column_chunk: &ColumnChunk) -> Result<Column> {
    let metadata = column_chunk
        .meta_data
        .as_ref()
        .expect("read_column: expected metadata");
    let rows = metadata.num_values;
    let p_type = metadata.type_;

    let pages = read_pages(data, metadata)?;
    let mut values = Vec::with_capacity(metadata.num_values as usize);

    for page in pages.data_pages {
        let page_values = decode_page(&page, p_type, page.num_values())?;
        values.extend(page_values);
    }
    column_from_scalars(values, metadata)
}
