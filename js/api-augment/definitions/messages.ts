export default {
  rpc: {
    getBySchema: {
      description: "Get messages by schemaId paginated",
      params: [
        {
          name: "schema_id",
          type: "SchemaId",
        },
        {
          name: "pagination",
          type: "BlockPaginationRequest",
        },
      ],
      type: "BlockPaginationResponseMessage",
    },
  },
  types: {
    BlockPaginationRequest: {
      from_block: "BlockNumber", // inclusive
      from_index: "u32", // starts from 0
      to_block: "BlockNumber", // exclusive
      page_size: "u32",
    },
    IPFSPayload: {
      cid: "Vec<u8>", // An IPFS content address (https://proto.school/anatomy-of-a-cid)
      payload_length: "u32",
    },
    MessageResponse: {
      payload: "Vec<u8>", //  Serialized data in a user-defined schema format
      signer_msa_id: "MessageSourceId", //  Message source account id of the MSA that signed the transaction.
      original_msa_id: "MessageSourceId", //  Message source account id (the original source)
      index: "u16", // index in block to get total order
      block_number: "BlockNumber",
    },
    BlockPaginationResponseMessage: {
      content: "Vec<MessageResponse>",
      has_next: "bool",
      next_block: "Option<BlockNumber>",
      next_index: "Option<u32>",
    },
  },
};
