# Wagering Program — Preliminary Security Review (Pre-Audit)

**Date:** 2025-09-13  
**Scope:** 11 Rust files (~880 LOC)  
**Method:** Static/manual review (Anchor); no fuzzing in pre-audit

## Summary
No unauthenticated drains. Medium risks: (1) constrain SPL/ATA program IDs, (2) guard u16 spawn math, (3) add pre-settlement guard.
Low risks: duplicate winners, decimals assertion, O(n^2) scan, unused bump.

## Findings
- **F-01 (MED)**: SPL/ATA program IDs not constrained → add `#[account(address = ...)]`
- **F-02 (MED)**: Spawn underflow on kill (`u16`) → require `> 0` then subtract
- **F-03 (MED)**: Spawn increment overflow → `checked_add(10)` or cap with error
- **F-04 (MED)**: Re-settlement → check `status != Completed` at entry; set on refund
- **F-05 (LOW)**: Duplicate winners → enforce uniqueness (`HashSet<Pubkey>`)
- **F-06 (LOW)**: Token decimals not asserted → assert/record `mint.decimals`
- **F-07 (LOW)**: Unused `vault_token_bump` → remove or wire & validate
- **F-08 (LOW)**: O(n^2) scan → pre-index recipients

## Patches & Tests
- See `audit_patches.diff` and `tests_skeleton.rs` (negative tests and property test placeholders).

## Additional Recommendations
- Enforce `session_id` length (≤10 stored / ≤32B seed), or adjust storage and document.
- Define payout “dust” policy; add tests.
- Optional: close/settle flow to reclaim rent after completion.

## Timeline & Deliverables
4–5 calendar days: PDF report, remediation PRs, tests, walkthrough call.

---
*Pre-audit only; a formal audit is recommended before mainnet.*
