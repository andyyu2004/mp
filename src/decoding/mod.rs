mod binary_decoder;
mod decodable;
mod decoder;

use crate::{Encoding, ProtocolError};
pub use binary_decoder::BinaryDecoder;
pub use decodable::Decodable;
pub use decoder::Decoder;

pub fn get_decoder(encoding: Encoding) -> Box<dyn Decoder<Error = ProtocolError>> {
    match encoding {
        Encoding::Binary => box BinaryDecoder,
        Encoding::Json => todo!(),
    }
}