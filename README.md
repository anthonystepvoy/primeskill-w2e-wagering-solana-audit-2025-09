# PrimeSkill — Win-2-Earn Wagering (Solana/Anchor) — Pre-Audit

**Date:** 2025-09-13  
**Scope:** 11 Rust files (~880 LOC)  
**Method:** Manual/static review (Anchor). No fuzzing in pre-audit.

## Summary
No unauthenticated drains. Medium risks: (1) constrain SPL/ATA program IDs, (2) guard `u16` spawn math, (3) pre-settlement guard to block re-exec. Lows: duplicate winners, decimals assertion, O(n^2) scan, unused bump.

## Submission Contents
- **Pre-audit report (Markdown):** [`docs/Wager_PreAudit_Vokter_2025-09-13.md`](docs/Wager_PreAudit_Vokter_2025-09-13.md)
- **Patch diff:** [`patches/audit_patches.diff`](patches/audit_patches.diff)
- **Test skeletons (Anchor program-test):** [`tests/tests_skeleton.rs`](tests/tests_skeleton.rs)

## How to apply patches
```bash
# From your program root:
git apply --3way patches/audit_patches.diff
# or
patch -p1 < patches/audit_patches.diff
```

## Findings (abridged)

* **F-01 (MED):** Constrain Token/ATA program IDs → `#[account(address = ...)]`
* **F-02 (MED):** `u16` decrement underflow guard
* **F-03 (MED):** `u16` increment overflow → `checked_add(10)` or cap
* **F-04 (MED):** Settlement re-exec guard → `status != Completed` (+ set on refund)
* **F-05 (LOW):** Unique winners (`HashSet<Pubkey>`)
* **F-06 (LOW):** Assert/record `mint.decimals`
* **F-07 (LOW):** Remove or wire `vault_token_bump`
* **F-08 (LOW):** O(n^2) scan → pre-index recipients

## Additional Recommendations

* Enforce `session_id` length (≤10 stored / ≤32B seed) or adjust storage.
* Define payout dust policy; add tests.
* Optional: close/settle flow to reclaim rent after completion.

## Timeline & Deliverables (if selected for full audit)

4–5 calendar days: PDF report with PoCs, remediation PRs, tests, walkthrough call.

## Test Harness Instructions

Tests target Anchor program-test; wire into your workspace`s `Cargo.toml` and run with `anchor test`.

## License

MIT (see `LICENSE`).

—
*Pre-audit only; a formal audit is recommended before mainnet.*
