use anyhow::Result;
use bytes::{Buf, Bytes};

use crate::{
    format::{ColumnMetaData, CompressionCodec, Encoding, PageHeader, PageType},
    thrift::read_thrift_metadata,
};

#[derive(Debug)]
pub enum Page {
    /// A data page stores the actual data for a single column. It contains 4 pieces of information.
    ///
    /// ```text
    /// PageHeader
    /// repetition_levels (optional)
    /// definition_levels (optional)
    /// encoded_values
    /// ```
    ///
    /// There are two assumptions at the moment that make the implementation simpler:
    /// - no nested data types support: `repetition_levels` **must not** be present.
    /// - all columns can contain nulls: `definition_levels` **must always** be present.
    ///
    /// The `definition_levels` value includes the length itself in the bytes. This is for compatible with the spec.
    /// And its data is encoded using RLE bit-packed encoding.
    ///
    /// [data page file format]: https://parquet.apache.org/docs/file-format/data-pages/
    DataPage {
        page_header: PageHeader,
        definition_levels: Bytes,
        encoded_values: Bytes,
    },
    /// A dictionary page contains the actual values for a column chunk, the actual indexes are store
    /// in the data page itself.
    ///
    /// [dictionary page for column chunk]: https://parquet.apache.org/docs/file-format/data-pages/columnchunks/
    /// [dictionary encoding]: https://parquet.apache.org/docs/file-format/data-pages/encodings/#DICTIONARY
    DictionaryPage {
        page_header: PageHeader,
        encoded_values: Bytes,
    },
}

impl Page {
    pub fn num_values(&self) -> usize {
        match self {
            Page::DataPage { page_header, .. } => {
                page_header.data_page_header.as_ref().unwrap().num_values as usize
            }
            Page::DictionaryPage { page_header, .. } => {
                page_header
                    .dictionary_page_header
                    .as_ref()
                    .unwrap()
                    .num_values as usize
            }
        }
    }

    pub fn encoding(&self) -> Encoding {
        match self {
            Page::DataPage { page_header, .. } => {
                page_header.data_page_header.as_ref().unwrap().encoding
            }
            Page::DictionaryPage { page_header, .. } => {
                page_header
                    .dictionary_page_header
                    .as_ref()
                    .unwrap()
                    .encoding
            }
        }
    }

    pub fn encoded_values(&self) -> Bytes {
        match self {
            Page::DataPage { encoded_values, .. } => encoded_values.clone(),
            Page::DictionaryPage { encoded_values, .. } => encoded_values.clone(),
        }
    }

    pub fn definition_levels(&self) -> Option<Bytes> {
        match self {
            Page::DataPage {
                definition_levels, ..
            } => Some(definition_levels.clone()),
            Page::DictionaryPage { .. } => None,
        }
    }

    pub fn is_dictionary(&self) -> bool {
        matches!(self, Page::DictionaryPage { .. })
    }
}

/// Read a page data into [`Page`].
///
/// This function receives `data` at page boundary,  and `codec` (to handle compression),
/// and returns a [`Page`] and remaining bytes.
#[allow(unused_variables)]
pub fn read_page(data: Bytes, codec: CompressionCodec) -> Result<(Page, Bytes)> {
    let (page_header, mut remaining) = read_thrift_metadata::<PageHeader>(data)?;
    let mut page_data = remaining.split_to(page_header.compressed_page_size as usize);

    let page = match page_header.type_ {
        PageType::DATA_PAGE => {
            let length_def = page_data.clone().get_u32_le() as usize;
            let definition_levels = page_data.split_to(length_def + 4);

            Page::DataPage {
                page_header,
                definition_levels,
                encoded_values: page_data,
            }
        }
        PageType::DICTIONARY_PAGE => {
            todo!("read page has not implemented dictionary page")
        }
        page_type => unimplemented!("read_page: unsupported {:?}", page_type),
    };
    Ok((page, remaining))
}

/// Pages for a column chunk
#[derive(Debug)]
pub struct Pages {
    pub data_pages: Vec<Page>,
    pub dictionary_page: Option<Page>,
}

/// Read all [`Page`] for a given column chunk.
///
/// All pages for a given column chunk are written back to back.
#[allow(unused_variables)]
pub fn read_pages(data: Bytes, column_metadata: &ColumnMetaData) -> Result<Pages> {
    let pages_start = column_metadata.data_page_offset as usize;
    let compressed_size = column_metadata.total_compressed_size as usize;
    let mut page_data = data.slice(pages_start..pages_start + compressed_size);
    let mut data_pages = vec![];

    while !page_data.is_empty() {
        let (page, remaining) = read_page(page_data, column_metadata.codec)?;
        data_pages.push(page);
        page_data = remaining;
    }

    Ok(Pages {
        data_pages,
        dictionary_page: None,
    })
}
