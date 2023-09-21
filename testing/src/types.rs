use alloy_sol_types::sol;


// MarketTypes.PublishStorageDealsParams
sol! {
    struct Cid {
        bytes data;
    }

    struct DealLabel {
        bytes data;
        bool isString;
    }

    struct FilAddress {
        bytes data;
    }

    struct BigInt {
        bytes val;
        bool neg;
    }

    struct WithdrawBalanceParams {
        FilAddress provider_or_client;
        BigInt tokenAmount;
    }
}

sol! {
    type ChainEpoch is int64;

    struct DealProposal {
        Cid piece_cid;
        uint64 piece_size;
        bool verified_deal;
        FilAddress client;
        FilAddress provider;
        DealLabel label;
        ChainEpoch start_epoch;
        ChainEpoch end_epoch;
        BigInt storage_price_per_epoch;
        BigInt provider_collateral;
        BigInt client_collateral;
    }

    struct ClientDealProposal {
        DealProposal proposal;
        bytes client_signature;
    }

    struct PublishStorageDealsParams {
        ClientDealProposal[] deals;
    }
}
