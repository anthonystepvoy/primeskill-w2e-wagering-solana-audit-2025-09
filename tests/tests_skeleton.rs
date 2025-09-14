// [F-02] Spawn underflow is prevented
#[tokio::test]
async fn test_f02_spawn_underflow_prevented() {
    // arrange: create session, add victim with 0 spawns, set status InProgress
    // act: call record_kill with victim who has 0 spawns
    // assert: tx fails with WagerError::PlayerHasNoSpawns
}

// [F-01] Token program must be canonical
#[tokio::test]
async fn test_f01_token_program_id_constrained() {
    // arrange: craft ctx with wrong token_program pubkey
    // act: call join_user
    // assert: tx fails with WagerError::InvalidTokenProgram
}

// [F-04] Double settlement rejected
#[tokio::test]
async fn test_f04_double_settlement_rejected() {
    // arrange: normal win distribution once
    // act: call distribute_all_winnings twice
    // assert: second call fails with WagerError::AlreadySettled
}

// [F-03] Spawn overflow rejected
#[tokio::test]
async fn test_f03_spawn_overflow_rejected() {
    // arrange: set player_spawns near u16::MAX
    // act: call pay_to_spawn to add 10
    // assert: tx fails with WagerError::SpawnOverflow
}

// [F-05] Duplicate winners not accepted
#[tokio::test]
async fn test_f05_duplicate_winners_rejected() {
    // arrange: winning team with N players; build remaining_accounts where the same winner appears twice
    // act: call distribute_all_winnings
    // assert: tx fails with WagerError::DuplicateWinner
}
