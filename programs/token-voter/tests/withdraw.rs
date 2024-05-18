use program_test::token_voter_test::TokenVoterTest;
use program_test::tools::*;
use solana_program_test::*;
use solana_sdk::transport::TransportError;
use token_voter::error::TokenVoterError;

mod program_test;

#[tokio::test]
async fn test_withdraw_with_token_extensions() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new_token_extensions().await;

    let realm_cookie = token_voter_test.governance.with_realm_token_extension().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();
    let first_mint_cookie = token_voter_test.mints.first().unwrap();

    let voter_cookie = token_voter_test
        .with_voter(&registrar_cookie, first_user_cookie)
        .await?;

    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let _voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    let token_owner_record_cookie = token_voter_test
        .governance
        .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie).await?;
    let amount_deposited = 10_u64;
    token_voter_test
        .deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token_2022::id(),
            0,
            amount_deposited,
        )
        .await?;


    token_voter_test.bench.advance_clock().await;

    token_voter_test
        .withdraw_deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token_2022::id(),
            0,
            amount_deposited,
        )
        .await?;
    
    // Assert
    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

    assert_eq!(voter_data.registrar, registrar_cookie.address);
    // println!("{:?}", voter_data);
    assert_eq!(voter_data.deposits.first().unwrap().amount_deposited_native, 0);
    assert_eq!(voter_data.deposits.len(), 1);

    Ok(())
}


#[tokio::test]
async fn test_withdraw() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();
    let first_mint_cookie = token_voter_test.mints.first().unwrap();

    let voter_cookie = token_voter_test
        .with_voter(&registrar_cookie, first_user_cookie)
        .await?;

    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let _voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    let token_owner_record_cookie = token_voter_test
        .governance
        .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie).await?;
    let amount_deposited = 10_u64;
    token_voter_test
        .deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token::id(),
            0,
            amount_deposited,
        )
        .await?;


    token_voter_test.bench.advance_clock().await;

    token_voter_test
        .withdraw_deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token::id(),
            0,
            amount_deposited,
        )
        .await?;
    
    // Assert
    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

    assert_eq!(voter_data.registrar, registrar_cookie.address);
    println!("{:?}", voter_data);
    assert_eq!(voter_data.deposits.first().unwrap().amount_deposited_native, 0);
    assert_eq!(voter_data.deposits.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_withdraw_fail_to_withdraw_in_same_slot() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();
    let first_mint_cookie = token_voter_test.mints.first().unwrap();

    let voter_cookie = token_voter_test
        .with_voter(&registrar_cookie, first_user_cookie)
        .await?;

    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let _voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    let token_owner_record_cookie = token_voter_test
        .governance
        .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie).await?;
    let amount_deposited = 10_u64;
    token_voter_test
        .deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token::id(),
            0,
            amount_deposited,
        )
        .await?;

    let err = token_voter_test
        .withdraw_deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token::id(),
            0,
            amount_deposited,
        )
                .await
                .err()
                .unwrap();
    
    // Assert
    assert_token_voter_err(err, TokenVoterError::CannotWithdraw);

    Ok(())
}

// #[tokio::test]
// async fn test_configure_voter_weights_multi_deposit() -> Result<(), TransportError> {
//     // Arrange
//     let mut token_voter_test = TokenVoterTest::start_new().await;

//     let realm_cookie = token_voter_test.governance.with_realm().await?;

//     let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
//     let governance_program_cookie = token_voter_test.with_governance_program(None).await;
    
//     let mut users_iter = token_voter_test.users.iter();
//     let first_user_cookie = users_iter.next().unwrap();

//     let mut mint_iter = token_voter_test.mints.iter();
//     let first_mint_cookie = mint_iter.next().unwrap();
//     let second_mint_cookie = mint_iter.next().unwrap();



//     let voter_cookie = token_voter_test
//         .with_voter(&registrar_cookie, &first_user_cookie)
//         .await?;

//     let max_voter_weight_record_cookie = token_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     let _voting_mint_config = token_voter_test
//         .configure_mint_config(
//             &registrar_cookie,
//             &governance_program_cookie,
//             &max_voter_weight_record_cookie,
//             &first_mint_cookie,
//             0, // no digit shift
//         )
//         .await?;

//     let _second_voting_mint_config = token_voter_test
//         .configure_mint_config(
//             &registrar_cookie,
//             &governance_program_cookie,
//             &max_voter_weight_record_cookie,
//             &second_mint_cookie,
//             0, // no digit shift
//         )
//         .await?;

//     let token_owner_record_cookie = token_voter_test
//         .governance
//         .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie).await?;

//     let amount_deposited = 3_u64;
//     token_voter_test
//         .deposit_entry(
//             &registrar_cookie,
//             &voter_cookie,
//             &first_user_cookie,
//             &token_owner_record_cookie,
//             &first_mint_cookie,
//             &spl_token::id(),
//             0,
//             amount_deposited,
//         )
//         .await?;

//     token_voter_test.bench.advance_clock().await;

//     token_voter_test
//         .deposit_entry(
//             &registrar_cookie,
//             &voter_cookie,
//             &first_user_cookie,
//             &token_owner_record_cookie,
//             &first_mint_cookie,
//             &spl_token::id(),
//             0,
//             amount_deposited,
//         )
//         .await?;
//     token_voter_test.bench.advance_clock().await;

//     token_voter_test
//         .deposit_entry(
//             &registrar_cookie,
//             &voter_cookie,
//             &first_user_cookie,
//             &token_owner_record_cookie,
//             &first_mint_cookie,
//             &spl_token::id(),
//             0,
//             amount_deposited,
//         )
//         .await?;

//     // Assert
//     let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

//     assert_eq!(voter_data.registrar, registrar_cookie.address);
//     let mut deposit_entries = voter_data.deposits.iter();
//     let first_deposit_entry = deposit_entries.next().unwrap();
//     assert_eq!(first_deposit_entry.amount_deposited_native, amount_deposited * 3);
//     assert_eq!(voter_data.deposits.len(), 1);

//     let registrar = token_voter_test
//         .get_registrar_account(&registrar_cookie.address)
//         .await;

//     assert_eq!(registrar.voting_mint_configs.len(), 2);
//     assert_eq!(
//         registrar.voting_mint_configs.first().unwrap().mint,
//         first_mint_cookie.address
//     );

//     let max_voter_weight_record = token_voter_test
//         .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
//         .await;

//     // supply is 100 * 2
//     assert_eq!(max_voter_weight_record.max_voter_weight, 200);

//     assert_eq!(max_voter_weight_record.max_voter_weight_expiry, None);
//     assert_eq!(max_voter_weight_record.realm, realm_cookie.address);
//     assert_eq!(
//         max_voter_weight_record.governing_token_mint,
//         realm_cookie.account.community_mint
//     );

//     Ok(())
// }

// #[tokio::test]
// async fn test_configure_voter_weights_multi_token() -> Result<(), TransportError> {
//     // Arrange
//     let mut token_voter_test = TokenVoterTest::start_new().await;

//     let realm_cookie = token_voter_test.governance.with_realm().await?;

//     let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
//     let governance_program_cookie = token_voter_test.with_governance_program(None).await;
    
//     let mut users_iter = token_voter_test.users.iter();
//     let first_user_cookie = users_iter.next().unwrap();

//     let mut mint_iter = token_voter_test.mints.iter();
//     let first_mint_cookie = mint_iter.next().unwrap();
//     let second_mint_cookie = mint_iter.next().unwrap();



//     let voter_cookie = token_voter_test
//         .with_voter(&registrar_cookie, &first_user_cookie)
//         .await?;

//     let max_voter_weight_record_cookie = token_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     let _voting_mint_config = token_voter_test
//         .configure_mint_config(
//             &registrar_cookie,
//             &governance_program_cookie,
//             &max_voter_weight_record_cookie,
//             &first_mint_cookie,
//             0, // no digit shift
//         )
//         .await?;

//     let _second_voting_mint_config = token_voter_test
//         .configure_mint_config(
//             &registrar_cookie,
//             &governance_program_cookie,
//             &max_voter_weight_record_cookie,
//             &second_mint_cookie,
//             0, // no digit shift
//         )
//         .await?;

//     let token_owner_record_cookie = token_voter_test
//         .governance
//         .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie).await?;

//     let amount_deposited = 10_u64;
//     token_voter_test
//         .deposit_entry(
//             &registrar_cookie,
//             &voter_cookie,
//             &first_user_cookie,
//             &token_owner_record_cookie,
//             &first_mint_cookie,
//             &spl_token::id(),
//             0,
//             amount_deposited,
//         )
//         .await?;

//     token_voter_test
//         .deposit_entry(
//             &registrar_cookie,
//             &voter_cookie,
//             &first_user_cookie,
//             &token_owner_record_cookie,
//             &second_mint_cookie,
//             &spl_token::id(),
//             1,
//             amount_deposited,
//         )
//         .await?;

//     // Assert
//     let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

//     assert_eq!(voter_data.registrar, registrar_cookie.address);
    
//     let mut deposit_entries = voter_data.deposits.iter();
//     let first_deposit_entry = deposit_entries.next().unwrap();
//     let second_deposit_entry = deposit_entries.next().unwrap();
//     assert_eq!(first_deposit_entry.amount_deposited_native, amount_deposited);
//     assert_eq!(second_deposit_entry.amount_deposited_native, amount_deposited);
//     assert_eq!(voter_data.deposits.len(), 2);

//     let first_vault_balance = token_voter_test.vault_balance(&voter_cookie, &first_mint_cookie).await;
//     assert_eq!(first_vault_balance, amount_deposited);
//     let second_vault_balance = token_voter_test.vault_balance(&voter_cookie, &second_mint_cookie).await;
//     assert_eq!(second_vault_balance, amount_deposited);


//     let registrar = token_voter_test
//         .get_registrar_account(&registrar_cookie.address)
//         .await;

//     assert_eq!(registrar.voting_mint_configs.len(), 2);
//     assert_eq!(
//         registrar.voting_mint_configs.first().unwrap().mint,
//         first_mint_cookie.address
//     );

//     let max_voter_weight_record = token_voter_test
//         .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
//         .await;

//     // supply is 100 * 2
//     assert_eq!(max_voter_weight_record.max_voter_weight, 200);

//     assert_eq!(max_voter_weight_record.max_voter_weight_expiry, None);
//     assert_eq!(max_voter_weight_record.realm, realm_cookie.address);
//     assert_eq!(
//         max_voter_weight_record.governing_token_mint,
//         realm_cookie.account.community_mint
//     );

//     Ok(())
// }

// #[tokio::test]
// async fn test_configure_voter_weights_invalid_deposit_entry_index() -> Result<(), TransportError> {
//     // Arrange
//     let mut token_voter_test = TokenVoterTest::start_new().await;

//     let realm_cookie = token_voter_test.governance.with_realm().await?;

//     let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
//     let governance_program_cookie = token_voter_test.with_governance_program(None).await;
    
//     let mut users_iter = token_voter_test.users.iter();
//     let first_user_cookie = users_iter.next().unwrap();

//     let mut mint_iter = token_voter_test.mints.iter();
//     let first_mint_cookie = mint_iter.next().unwrap();
//     let second_mint_cookie = mint_iter.next().unwrap();



//     let voter_cookie = token_voter_test
//         .with_voter(&registrar_cookie, &first_user_cookie)
//         .await?;

//     let max_voter_weight_record_cookie = token_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     let _voting_mint_config = token_voter_test
//         .configure_mint_config(
//             &registrar_cookie,
//             &governance_program_cookie,
//             &max_voter_weight_record_cookie,
//             &first_mint_cookie,
//             0, // no digit shift
//         )
//         .await?;

//     let _second_voting_mint_config = token_voter_test
//         .configure_mint_config(
//             &registrar_cookie,
//             &governance_program_cookie,
//             &max_voter_weight_record_cookie,
//             &second_mint_cookie,
//             0, // no digit shift
//         )
//         .await?;

//     let token_owner_record_cookie = token_voter_test
//         .governance
//         .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie).await?;

//     let amount_deposited = 5_u64;
//     token_voter_test
//         .deposit_entry(
//             &registrar_cookie,
//             &voter_cookie,
//             &first_user_cookie,
//             &token_owner_record_cookie,
//             &first_mint_cookie,
//             &spl_token::id(),
//             0,
//             amount_deposited,
//         )
//         .await?;

//     let err = token_voter_test
//         .deposit_entry(
//             &registrar_cookie,
//             &voter_cookie,
//             &first_user_cookie,
//             &token_owner_record_cookie,
//             &first_mint_cookie,
//             &spl_token::id(),
//             2,
//             amount_deposited,
//         )
//         .await
//         .err()
//         .unwrap();

//     // Deposit entry entered out of bounds
//     assert_token_voter_err(err, TokenVoterError::OutOfBoundsDepositEntryIndex);

//     Ok(())
// }


// #[tokio::test]
// async fn test_configure_voter_weights_insufficient_funds() -> Result<(), TransportError> {
//     // Arrange
//     let mut token_voter_test = TokenVoterTest::start_new().await;

//     let realm_cookie = token_voter_test.governance.with_realm().await?;

//     let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
//     let governance_program_cookie = token_voter_test.with_governance_program(None).await;
    
//     let mut users_iter = token_voter_test.users.iter();
//     let first_user_cookie = users_iter.next().unwrap();

//     let mut mint_iter = token_voter_test.mints.iter();
//     let first_mint_cookie = mint_iter.next().unwrap();



//     let voter_cookie = token_voter_test
//         .with_voter(&registrar_cookie, &first_user_cookie)
//         .await?;

//     let max_voter_weight_record_cookie = token_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     let _voting_mint_config = token_voter_test
//         .configure_mint_config(
//             &registrar_cookie,
//             &governance_program_cookie,
//             &max_voter_weight_record_cookie,
//             &first_mint_cookie,
//             0, // no digit shift
//         )
//         .await?;

//     let token_owner_record_cookie = token_voter_test
//         .governance
//         .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie).await?;

//     let amount_deposited = 20;

//     let err = token_voter_test
//         .deposit_entry(
//             &registrar_cookie,
//             &voter_cookie,
//             &first_user_cookie,
//             &token_owner_record_cookie,
//             &first_mint_cookie,
//             &spl_token::id(),
//             0,
//             amount_deposited,
//         )
//         .await
//         .err()
//         .unwrap();

//     // Insufficient funds it throws Custom(1) error
//     assert_ix_err(err, InstructionError::Custom(1));

//     Ok(())
// }

// #[tokio::test]
// async fn test_configure_voter_weights_with_invalid_registrar_error() -> Result<(), TransportError> {
//     // Arrange
//     let mut token_voter_test = TokenVoterTest::start_new().await;

//     let realm_cookie = token_voter_test.governance.with_realm().await?;

//     let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;

//     let second_realm_cookie = token_voter_test.governance.with_realm().await?;

//     let second_registrar_cookie = token_voter_test.with_registrar(&second_realm_cookie).await?;
//     let governance_program_cookie = token_voter_test.with_governance_program(None).await;
    
//     let mut users_iter = token_voter_test.users.iter();
//     let first_user_cookie = users_iter.next().unwrap();

//     let mut mint_iter = token_voter_test.mints.iter();
//     let first_mint_cookie = mint_iter.next().unwrap();



//     let voter_cookie = token_voter_test
//         .with_voter(&registrar_cookie, &first_user_cookie)
//         .await?;

//     let max_voter_weight_record_cookie = token_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     let _voting_mint_config = token_voter_test
//         .configure_mint_config(
//             &registrar_cookie,
//             &governance_program_cookie,
//             &max_voter_weight_record_cookie,
//             &first_mint_cookie,
//             0, // no digit shift
//         )
//         .await?;

//     let token_owner_record_cookie = token_voter_test
//         .governance
//         .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie).await?;

//     let amount_deposited = 20;

//     let err = token_voter_test
//         .deposit_entry(
//             &second_registrar_cookie,
//             &voter_cookie,
//             &first_user_cookie,
//             &token_owner_record_cookie,
//             &first_mint_cookie,
//             &spl_token::id(),
//             0,
//             amount_deposited,
//         )
//         .await
//         .err()
//         .unwrap();

//     // Assert

//     // PDA doesn't match and hence the error is ConstraintSeeds
//     assert_anchor_err(err, ErrorCode::ConstraintSeeds);

//     Ok(())
// }