## Error Codes

| Code | Variant | Meaning |
|---|---|---|
| 1 | `NotAuthorized` | Caller is not a registered guardian or admin |
| 2 | `DuplicateVote` | Guardian already voted on this task |
| 3 | `TaskNotVerified` | Task is not yet resolved; cannot start reward stream |
| 4 | `StreamAlreadyActive` | A reward stream for this task already exists |
| 5 | `DripsCallFailed` | Cross-contract call to Drips protocol reverted |
| 6 | `Locked` | Re-entrancy guard is active |
| 7 | `AlreadyInitialized` | Contract has already been initialized |
| 8 | `NotInitialized` | Contract has not been initialized |
| 9 | `InsufficientLockedBalance` | Guardian's locked balance does not exceed the threshold |
| 10 | `WeightOverflow` | Adding vote weight would overflow u64 |
| 11 | `StillGuardian` | Cannot unlock tokens while still registered as a guardian |
| 12 | `NotGuardian` | Address is not a registered guardian |
| 13 | `ZeroWeightVote` | Guardian's reputation score is zero |
| 14 | `NoReputationScore` | Guardian has no reputation score assigned |
| 15 | `ContractPaused` | Contract is paused; all state-changing calls are blocked |
| 16 | `EscrowUnavailable` | Cross-contract call to vault/escrow reverted |
| 17 | `TaskCancelled` | Task has been cancelled and cannot be processed |
| 18 | `InvalidAddress` | Invalid address provided |
| 19 | `InvalidAmount` | Invalid amount provided |
| 20 | `InvalidConfig` | Invalid configuration |
| 21 | `InvalidRange` | Value is outside valid range |
| 22 | `BatchTooLarge` | Batch operation is too large |
| 23 | `TaskNotFound` | Task not found |
| 24 | `TaskAlreadyArchived` | Task has already been archived |
| 25 | `TaskNotStale` | Task is not stale enough to be pruned |
| 26 | `SnapshotNotFound` | Snapshot not found |
| 27 | `WithdrawalTimelockActive` | Withdrawal timelock is still active |
| 28 | `TaskNotTerminal` | Task is not in terminal state |
| 29 | `InsufficientReputation` | Insufficient reputation score |
| 30 | `NotUpgradeSigner` | Caller is not authorized as a multi-sig upgrade signer |
| 31 | `UpgradeThresholdNotMet` | Not enough upgrade approvals collected yet |
| 32 | `NoPendingUpgrade` | No pending upgrade proposal to act on |
| 33 | `AlreadyApproved` | Signer has already approved this upgrade proposal |
| 34 | `InvalidUpgradeConfig` | Invalid multi-sig upgrade configuration (threshold > signers or zero) |
| 35 | `LastAdminRemovalBlocked` | Cannot revoke the last remaining Admin role holder (would cause lockout) |
| 36 | `DuplicateGuardian` | Attempted to add a guardian that is already registered |
| 37 | `InvalidVersion` | Storage version mismatch during pre-flight checks |


---

## Emergency Halt (Circuit Breaker)

The contract has a two-track emergency halt system that allows an admin to immediately freeze all state-changing operations if a vulnerability is discovered, without requiring a contract migration.

### Manual pause / unpause

```rust
// Immediately block all state-changing entry points
client.pause(&admin);

// Restore normal operation
client.unpause(&admin);

// Or toggle the current state
client.toggle_pause(&admin);

// Check current state
let frozen: bool = client.is_paused();
```

Both `pause` and `unpause` require `admin.require_auth()`. No other address can call them.

When paused, any call to `register_task`, `vote`, `add_guardian`, `set_reputation`, `set_weight_threshold`, or `start_reward_stream` returns `Err(ContractError::ContractPaused)` immediately.

### Automatic circuit breaker

Off-chain monitors can report observed failures via `record_failure`. After **50 cumulative failures** the contract pauses itself automatically and emits a `cb_trip` event.

```rust
// Called by off-chain monitor after observing a failed invocation
client.record_failure();
```

To resume after investigation:

```rust
// Resets the failure counter and unpauses
client.reset_circuit_breaker(&admin);
```

### Emergency halt procedure

1. **Detect** — Either trigger `pause` manually, or wait for `record_failure` to trip the breaker at >50 failures.
2. **Verify** — Call `is_paused()` on-chain to confirm the contract is frozen.
3. **Investigate** — Audit storage state and transaction history off-chain.
4. **Remediate** — Deploy a patched WASM via `upgrade_contract` if needed.
5. **Resume** — Call `reset_circuit_breaker` (resets counter + unpauses) or `unpause` if the failure counter was not the trigger.

> **Security note:** Only the Multi-Sig admin key can call `pause`, `unpause`, and `reset_circuit_breaker`. The `record_failure` entry point is permissionless so that any observer can report failures, but it only increments a counter — it cannot directly manipulate task or guardian state.

---

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for dev environment setup, build/test/lint instructions, branch and PR conventions, and how to find good first issues.

---

## License

Apache-2.0



