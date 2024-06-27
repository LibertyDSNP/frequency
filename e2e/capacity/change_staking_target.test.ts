import "@frequency-chain/api-augment";
import { KeyringPair } from "@polkadot/keyring/types";
import { u64, } from "@polkadot/types";
import assert from "assert";
import { ExtrinsicHelper, } from "../scaffolding/extrinsicHelpers";
import {
  devAccounts, createKeys, createMsaAndProvider,
  stakeToProvider,
  CENTS, DOLLARS, createAndFundKeypair, createProviderKeysAndId
}
  from "../scaffolding/helpers";
import { firstValueFrom } from "rxjs";
import { MessageSourceId} from "@frequency-chain/api-augment/interfaces";

describe("change_staking_target tests", () => {
  const tokenMinStake: bigint = 1n * CENTS;
  const capacityMin: bigint = tokenMinStake / 50n;

  it("happy path succeeds", async () => {
      const providerBalance = 2n * DOLLARS;
      const stakeKeys = createKeys("staker");
      const oldProvider = await createMsaAndProvider(stakeKeys, "Provider1", providerBalance);
      const [_unused, newProvider] = await createProviderKeysAndId();

      await assert.doesNotReject(stakeToProvider(stakeKeys, oldProvider, tokenMinStake*3n));

      const call = ExtrinsicHelper.changeStakingTarget(stakeKeys, oldProvider, newProvider, tokenMinStake);
      const [events] = await call.signAndSend();
      assert.notEqual(events, undefined);
  });

  // not intended to be exhaustive, just check one error case
  it("fails if 'to' is not a Provider", async () => {
    const providerBalance = 2n * DOLLARS;
    const stakeKeys = createKeys("staker");
    const oldProvider = await createMsaAndProvider(stakeKeys, "Provider1", providerBalance);
    await assert.doesNotReject(stakeToProvider(stakeKeys, oldProvider, tokenMinStake*3n));
    const notAProvider = 3;
    const call = ExtrinsicHelper.changeStakingTarget(stakeKeys, oldProvider, notAProvider, tokenMinStake);
    await assert.rejects(call.signAndSend(), {name: "InvalidTarget"})
  });
});
