use arma_file_formats::real_virtuality::paa::Mipmap;

use crate::bridge::PaaMipmapCxx;

impl From<Mipmap> for PaaMipmapCxx {
    fn from(mm: Mipmap) -> Self {
        Self {
            width: mm.width,
            height: mm.height,
            data: mm.data,
        }
    }
}
