use crate::encoding::traits::{Encoding, EncodingMetrics};
use protobuf::Message;
use std::time::Instant;

pub struct ProtocolBufEncoding;

impl ProtocolBufEncoding {
	pub fn new() -> Self {
		Self {}
	}
}

impl<T: Message> Encoding<T> for ProtocolBufEncoding {
	fn encode(&self, data: &T) -> Vec<u8> {
		data.write_to_bytes().unwrap()
	}

	fn decode(&self, data: &[u8]) -> T {
		let mut decoded_data = T::new();
		decoded_data.merge_from_bytes(data).unwrap();
		decoded_data
	}

	fn get_metrics(&self, data: &T, input_size: usize) -> EncodingMetrics {
		let start_encode = Instant::now();
		let encoded = self.encode(data);
		let encoding_time = start_encode.elapsed().as_secs_f64();

		let encoded_size = encoded.len();
		let compression_ratio = (input_size as f64) / (encoded_size as f64);

		let start_decode = Instant::now();
		<ProtocolBufEncoding as Encoding<T>>::decode(self, &encoded);
		let decoding_time = start_decode.elapsed().as_secs_f64();

		EncodingMetrics { encoded_size, decoding_time, compression_ratio, encoding_time }
	}
}
