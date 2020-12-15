mod binary_decoder;
mod decode;
mod decoder;

use crate::{Encoding, ProtocolError};
pub use binary_decoder::BinaryDecoder;
pub use decode::Decode;
pub use decoder::Decoder;

pub fn get_decoder(encoding: Encoding) -> Box<dyn Decoder<Error = ProtocolError>> {
    match encoding {
        Encoding::Binary => Box::new(BinaryDecoder),
        Encoding::Json => todo!(),
    }
}
