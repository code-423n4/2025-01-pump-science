import { Clock, ProgramTestContext } from "solana-bankrun";
import assert from "assert";

export const advanceBySlots = async (context: any, slots: BigInt) => {
  const currentClock = await context.banksClient.getClock();
  const slot = currentClock.slot + slots;
  context.setClock(
    new Clock(
      slot,
      currentClock.epochStartTimestamp,
      currentClock.epoch,
      currentClock.leaderScheduleEpoch,
      currentClock.unixTimestamp
    )
  );
};

export const expectError = (
  expectedError: string,
  message: string
): [() => void, (e: any) => void] => {
  return [
    () => assert.fail(message),
    (e) => {
      assert(e.error != undefined, `problem retrieving program error: ${e}`);
      assert(
        e.error.errorCode != undefined,
        "problem retrieving program error code"
      );
      assert.equal(
        e.error.errorCode.code,
        expectedError,
        `the program threw for a reason that we didn't expect. error : ${e}`
      );
    },
  ];
};
