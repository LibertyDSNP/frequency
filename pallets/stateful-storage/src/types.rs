use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_core::bounded::BoundedVec;
use sp_std::{cmp::*, collections::btree_map::BTreeMap, fmt::Debug, prelude::*};

/// Defines the actions that can be applied to an Itemized storage
#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, PartialOrd, Ord)]
pub enum ItemAction {
	Add { data: Vec<u8> },
	Remove { index: u16 },
}

/// This header is used to specify how long an item is inside the buffer and inserted into buffer
/// before every item
#[derive(Encode, Decode, PartialEq, MaxEncodedLen, Debug)]
pub struct ItemHeader {
	/// The length of this item, not including the size of this header.
	pub payload_len: u16,
}

#[derive(Debug, PartialEq)]
pub enum PageError {
	ErrorParsing(&'static str),
	InvalidAction(&'static str),
	ArithmeticOverflow,
	PageSizeOverflow,
}

/// A page of data
#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, PartialEq, Debug, Default)]
#[scale_info(skip_type_params(PageDataSize))]
#[codec(mel_bound(PageDataSize: MaxEncodedLen))]
pub struct Page<PageDataSize: Get<u32>> {
	pub data: BoundedVec<u8, PageDataSize>,
}

/// an internal struct which contains the parsed items in a page
#[derive(Debug, PartialEq)]
pub struct ParsedItemPage<'a> {
	/// page current size
	pub page_size: usize,
	/// a map of item index to a slice of blob (header included)
	pub items: BTreeMap<u16, &'a [u8]>,
}

impl<PageDataSize: Get<u32>> Page<PageDataSize> {
	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
}

impl<PageDataSize: Get<u32>> From<BoundedVec<u8, PageDataSize>> for Page<PageDataSize> {
	fn from(bounded: BoundedVec<u8, PageDataSize>) -> Self {
		Self { data: bounded }
	}
}

impl<PageDataSize: Get<u32>> TryFrom<Vec<u8>> for Page<PageDataSize> {
	type Error = ();

	fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
		let bounded: BoundedVec<u8, PageDataSize> = BoundedVec::try_from(data).map_err(|_| ())?;
		Ok(Page::from(bounded))
	}
}

impl<PageDataSize: Get<u32>> Page<PageDataSize> {
	/// applies all actions to specified page and returns the updated page
	pub fn apply_item_actions(&self, actions: &[ItemAction]) -> Result<Self, PageError> {
		let mut parsed = self.parse_as_itemized()?;

		let mut updated_page_buffer = Vec::with_capacity(parsed.page_size);
		let mut add_buffer = Vec::new();

		for action in actions {
			match action {
				ItemAction::Remove { index } => {
					ensure!(
						parsed.items.contains_key(&index),
						PageError::InvalidAction("item index is invalid")
					);
					parsed.items.remove(&index);
				},
				ItemAction::Add { data } => {
					let header = ItemHeader {
						payload_len: data
							.len()
							.try_into()
							.map_err(|_| PageError::InvalidAction("invalid payload size"))?,
					};
					add_buffer.extend_from_slice(&header.encode()[..]);
					add_buffer.extend_from_slice(&data[..]);
				},
			}
		}

		// since BTreemap is sorted by key, all items will be kept in their old order
		for (_, slice) in parsed.items.iter() {
			updated_page_buffer.extend_from_slice(slice);
		}
		updated_page_buffer.append(&mut add_buffer);

		Page::<PageDataSize>::try_from(updated_page_buffer).map_err(|_| PageError::PageSizeOverflow)
	}

	/// Parses all the items inside an ItemPage
	pub fn parse_as_itemized(&self) -> Result<ParsedItemPage, PageError> {
		let mut count = 0u16;
		let mut items = BTreeMap::new();
		let mut offset = 0;
		while offset < self.data.len() {
			ensure!(
				offset + ItemHeader::max_encoded_len() <= self.data.len(),
				PageError::ErrorParsing("wrong header size")
			);
			let header = <ItemHeader>::decode(&mut &self.data[offset..])
				.map_err(|_| PageError::ErrorParsing("decoding header"))?;
			let item_total_length = ItemHeader::max_encoded_len() + header.payload_len as usize;
			ensure!(
				offset + item_total_length <= self.data.len(),
				PageError::ErrorParsing("wrong payload size")
			);

			items.insert(count, &self.data[offset..(offset + item_total_length)]);
			offset += item_total_length;
			count = count.checked_add(1).ok_or(PageError::ArithmeticOverflow)?;
		}

		Ok(ParsedItemPage { page_size: self.data.len(), items })
	}
}
