import assert from "assert";
import { KeyringPair } from "@polkadot/keyring/types";
import { ExtrinsicHelper } from "../scaffolding/extrinsicHelpers";
import { createAndFundKeypair } from "../scaffolding/helpers";
import { ApiTypes, SubmittableExtrinsic } from "@polkadot/api/types";

describe("Utility Batch Filtering", function () {
    let sender: KeyringPair;
    let recipient: KeyringPair;

    before(async function () {
        sender = await createAndFundKeypair();
        recipient = await createAndFundKeypair();
    });

    it("should successfully execute ✅ batch with allowed calls", async function () {
        // good batch: with only allowed calls
        const goodBatch: SubmittableExtrinsic<ApiTypes>[] = [];
        goodBatch.push(ExtrinsicHelper.api.tx.balances.transfer(recipient.address, 1000))
        goodBatch.push(ExtrinsicHelper.api.tx.system.remark("Hello From Batch"))
        goodBatch.push(ExtrinsicHelper.api.tx.msa.create())
        const batch = ExtrinsicHelper.executeUtilityBatchAll(sender, goodBatch);
        const [event, eventMap] = await batch.fundAndSend();
        assert.notEqual(event, undefined, "should return an event");
        assert.notEqual(eventMap, undefined, "should return an eventMap");
    });

    it("should fail to execute ❌ batch with disallowed calls", async function () {
        // bad batch: with a mix of allowed and disallowed calls
        const badBatch: SubmittableExtrinsic<ApiTypes>[] = [];
        badBatch.push(ExtrinsicHelper.api.tx.balances.transfer(recipient.address, 1000))
        badBatch.push(ExtrinsicHelper.api.tx.system.remark("Hello From Batch"))
        badBatch.push(ExtrinsicHelper.api.tx.handles.retireHandle())
        badBatch.push(ExtrinsicHelper.api.tx.msa.retireMsa())
        const batchAll = ExtrinsicHelper.executeUtilityBatchAll(sender, badBatch);
        let error: any;
        try {
            await batchAll.fundAndSend();
        } catch (err) {
            error = err;
            assert.notEqual(error, undefined, " batchAll should return an error");
        }
        assert.notEqual(error, undefined, " batchAll should return an error");

        const batch = ExtrinsicHelper.executeUtilityBatch(sender, badBatch);
        try {
            await batch.fundAndSend();
        }
        catch (err) {
            error = err;
            assert.notEqual(error, undefined, "batch should return an error");
        }
        assert.notEqual(error, undefined, "batch should return an error");

        const forceBatch = ExtrinsicHelper.executeUtilityForceBatch(sender, badBatch);
        try {
            await forceBatch.fundAndSend();
        }
        catch (err) {
            error = err;
            assert.notEqual(error, undefined, "forceBatch should return an error");
        }
        assert.notEqual(error, undefined, "forceBatch should return an error");
    });
    
    it("should fail to execute ❌ batch  with `Pays::No` calls", async function () {
        // bad batch: with frequency related Pays::No call
        const badBatch: SubmittableExtrinsic<ApiTypes>[] = [];
        badBatch.push(ExtrinsicHelper.api.tx.msa.retireMsa())
        const batch = ExtrinsicHelper.executeUtilityBatchAll(sender, badBatch);
        let error: any;
        try {
            await batch.fundAndSend();
        } catch (err) {
            error = err;
            assert.notEqual(error, undefined, "should return an error");
        }
        assert.notEqual(error, undefined, "should return an error");    
    });

    it("should fail to execute ❌ batch with `Pays::Yes` `create_provider`call blocked by Frequency", async function () {
        // bad batch: with frequency related Pays::Yes call
        const badBatch: SubmittableExtrinsic<ApiTypes>[] = [];
        badBatch.push(ExtrinsicHelper.api.tx.msa.createProvider("I am a ba(tch)d provider"))
        const batch = ExtrinsicHelper.executeUtilityBatchAll(sender, badBatch);
        let error: any;
        try {
            await batch.fundAndSend();
        } catch (err) {
            error = err;
            assert.notEqual(error, undefined, "should return an error");
        }
        assert.notEqual(error, undefined, "should return an error");
    });

    it("should fail to execute ❌ batch with `Pays::Yes` `create_schema` call blocked by Frequency", async function () {
        // bad batch: with frequency related Pays::Yes call
        const badBatch: SubmittableExtrinsic<ApiTypes>[] = [];
        badBatch.push(ExtrinsicHelper.api.tx.msa.createProvider("I am a ba(tch)d provider"))
        const batch = ExtrinsicHelper.executeUtilityBatchAll(sender, badBatch);
        let error: any;
        try {
            await batch.fundAndSend();
        } catch (err) {
            error = err;
            assert.notEqual(error, undefined, "should return an error");
        }
        assert.notEqual(error, undefined, "should return an error");
    });

    it("should fail to execute ❌ batch with nested batch", async function () {
        // batch with nested batch
        const nestedBatch: SubmittableExtrinsic<ApiTypes>[] = [];
        const innerBatch: SubmittableExtrinsic<ApiTypes>[] = [];
        innerBatch.push(ExtrinsicHelper.api.tx.balances.transfer(recipient.address, 1000))
        innerBatch.push(ExtrinsicHelper.api.tx.system.remark("Hello From Batch"))
        nestedBatch.push(ExtrinsicHelper.api.tx.utility.batch(innerBatch))
        const batch = ExtrinsicHelper.executeUtilityBatchAll(sender, nestedBatch);
        let error: any;
        try {
            await batch.fundAndSend();
        } catch (err) {
            error = err;
            assert.notEqual(error, undefined, "should return an error");
        }
        assert.notEqual(error, undefined, "should return an error");
    });
});
