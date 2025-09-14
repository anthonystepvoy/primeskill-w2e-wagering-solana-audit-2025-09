# Preliminary Security Review — Wagering Program (Anchor/Solana)
Date: September 13, 2025
Auditor: Vokter Solana Security (Pre-audit Assistant)

## Scope & Methodology
- Codebase reviewed: 11 Rust files (Anchor 0.30.x), ~880 non-blank LOC.
- Focus areas: account invariants, CPI safety (SPL/ATA), signer & seeds, payouts accounting, state-machine transitions, DoS vectors, and compute usage.
- Methods: targeted static analysis, constraint review, seed/signature verification, and edge-case reasoning. No dynamic fuzzing performed in this pre-audit pass.

## Summary
No unauthenticated direct drain identified. Medium-risk issues include unchecked SPL/ATA program IDs, u16 spawn accounting under/overflow that can panic under overflow-checks, and missing pre-settlement guards which allow re-execution. Several low-risk footguns and performance issues were also found. Patches and test skeletons are provided.

## Findings Overview
| ID  | Severity | Title                                              |
|-----|----------|----------------------------------------------------|
| F-01| MED      | SPL Token & ATA program IDs not constrained        |
| F-02| MED      | Spawn underflow on kill (u16) can panic (DoS)      |
| F-03| MED      | Spawn increment (u16) may overflow (DoS)           |
| F-04| MED      | Settlement can be re-invoked (missing guard)       |
| F-05| LOW      | Duplicate winners not prevented                    |
| F-06| LOW      | Token decimals not asserted                        |
| F-07| LOW      | Unused `vault_token_bump` field                    |
| F-08| LOW      | O(n^2) scan in pay-spawn earnings distribution     |

## Selected Details & Remediation Notes
### F-01 — SPL/ATA program IDs not constrained (MED)
All instructions accept `token_program` / `associated_token_program` without constraining them to canonical program IDs. This permits arbitrary CPI targets (best-practice violation, potential DoS).
**Fix:** add `#[account(address = anchor_spl::token::ID)]` and `#[account(address = anchor_spl::associated_token::ID)]` to the respective Program accounts.

### F-02/F-03 — Spawn accounting under/overflow (MED)
Direct `-= 1` and `+= 10u16` on `u16` can panic under `overflow-checks = true` (present in release profile).
**Fix:** guard zero before decrement; use `checked_add`/cap for increments; return typed errors.

### F-04 — Re-settlement guard (MED)
Distribution/refund handlers set `Completed` at the end, but do not pre-check.
**Fix:** add `require!(status != Completed, ...)` at entry; set `Completed` on refund as well.

### F-05 — Duplicate winners (LOW)
Membership check only; no uniqueness across `remaining_accounts`.
**Fix:** enforce uniqueness via a `HashSet<Pubkey>` to prevent double counting.

### F-06 — Token decimals not asserted (LOW)
Assumes mint decimals; never checked.
**Fix:** assert or record `mint.decimals` to ensure economic correctness.

### F-07 — Unused `vault_token_bump` (LOW)
Defined but never set/used.
**Fix:** remove or wire & validate consistently.

### F-08 — O(n^2) scan (LOW)
Recipient lookup uses `.position()` per player.
**Fix:** pre-index `remaining_accounts` into a map: `Pubkey -> token account`.

## Additional Recommendations (beyond current findings)
- Enforce `session_id` max length (≤10 stored / ≤32B per seed), or adjust storage; document policy.
- Define payout remainder (“dust”) policy; add tests to prove determinism.
- Prefer canonical ATA constraints for recipients; reduce reliance on `remaining_accounts` ordering.
- Add property tests/fuzz for transitions: join → in-progress → complete, double calls, randomized account lists.
- Optional: add a close/settle flow to reclaim rent post-completion.

## Timeline & Deliverables (Full Audit)
- Turnaround: 4–5 calendar days for ~880 LOC.
- Deliverables: (1) PDF audit with severity & PoCs, (2) remediation PR/patches, (3) tests (Anchor), (4) walkthrough call.

*Pre-audit assistant only; seek a formal audit before mainnet.*
