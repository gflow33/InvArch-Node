use crate::{
    Balance, Balances, BlockNumber, CommonId, Event, ExistentialDeposit, Runtime, DAYS, UNIT,
};
use frame_support::{parameter_types, PalletId};

parameter_types! {
    pub const BlocksPerEra: BlockNumber = DAYS;
    pub const RegisterDeposit: Balance = 1000 * UNIT;
    pub const MaxStakersPerCore: u32 = 1000;
    pub const MinimumStakingAmount: Balance = 10 * UNIT;
    pub const MaxEraStakeValues: u32 = 5;
    pub const MaxUnlockingChunks: u32 = 5;
    pub const UnbondingPeriod: u32 = 7;
    pub const OcifStakingPot: PalletId = PalletId(*b"tkr/ocif");
    pub const RewardRatio: (u32, u32) = (60, 40);
    pub const StakeThresholdForActiveCore: Balance = 5000 * UNIT;
    pub const MaxNameLength: u32 = 20;
    pub const MaxDescriptionLength: u32 = 300;
    pub const MaxImageUrlLength: u32 = 100;
}

impl pallet_ocif_staking::Config for Runtime {
    type Currency = Balances;
    type BlocksPerEra = BlocksPerEra;
    type CoreId = CommonId;
    type RegisterDeposit = RegisterDeposit;
    type Event = Event;
    type MaxStakersPerCore = MaxStakersPerCore;
    type ExistentialDeposit = ExistentialDeposit;
    type PotId = OcifStakingPot;
    type MaxUnlocking = MaxUnlockingChunks;
    type UnbondingPeriod = UnbondingPeriod;
    type MinimumStakingAmount = MinimumStakingAmount;
    type MaxEraStakeValues = MaxEraStakeValues;
    type RewardRatio = RewardRatio;
    type StakeThresholdForActiveCore = StakeThresholdForActiveCore;
    type MaxNameLength = MaxNameLength;
    type MaxDescriptionLength = MaxDescriptionLength;
    type MaxImageUrlLength = MaxImageUrlLength;
}
