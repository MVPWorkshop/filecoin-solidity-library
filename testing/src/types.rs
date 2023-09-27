use alloy_sol_types::{sol, SolCall};
use alloy_sol_types::SolStruct;

sol! {
    struct FilAddress {
        bytes data;
    }

    type FilActorId is uint64;
    type Amount is uint256;
}


// CommonTypes.FilAddress
// CommonTypes.FilActorId
// sol! {
//     struct Cid {
//         bytes data;
//     }
// }

// sol! {
//     struct DealLabel {
//         bytes data;
//         bool isString;
//     }
// }

// sol! {
//     struct FilAddress {
//         bytes data;
//     }
// }

// sol! {
//     struct BigInt {
//         bytes val;
//         bool neg;
//     }
// }

// sol! {
//     struct WithdrawBalanceParams {
//         FilAddress provider_or_client;
//         BigInt tokenAmount;
//     }
// }

// sol! {
//     type ChainEpoch is int64;

//     struct DealProposal {
//         Cid piece_cid;
//         uint64 piece_size;
//         bool verified_deal;
//         FilAddress client;
//         FilAddress provider;
//         DealLabel label;
//         ChainEpoch start_epoch;
//         ChainEpoch end_epoch;
//         BigInt storage_price_per_epoch;
//         BigInt provider_collateral;
//         BigInt client_collateral;
//     }
// }

// sol! {
//     struct ClientDealProposal {
//         DealProposal proposal;
//         bytes client_signature;
//     }
// }

// sol! {
//     struct PublishStorageDealsParams {
//         ClientDealProposal[] deals;
//     }
// }
