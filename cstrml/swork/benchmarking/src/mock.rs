// Copyright (C) 2019-2021 Crust Network Technologies Ltd.
// This file is part of Crust.

use crate::*;

pub use frame_support::{
    parameter_types, assert_ok,
    weights::{Weight, constants::RocksDbWeight},
    traits::{OnInitialize, OnFinalize, Get, TestRandomness, WithdrawReasons}
};
pub use sp_core::{crypto::{AccountId32, Ss58Codec}, H256};
use sp_runtime::{
    testing::Header, ModuleId, DispatchError,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};
pub use market::{Replica, FileInfo};
use swork::Works;
use balances::{AccountData, NegativeImbalance};
pub use std::{cell::RefCell, collections::HashMap, borrow::Borrow, iter::FromIterator, collections::btree_map::BTreeMap};
use primitives::{traits::BenefitInterface, EraIndex};

pub type AccountId = AccountId32;
pub type Balance = u64;

pub type BalanceOf<T> =
<<T as swork::Config>::Currency as Currency<<T as system::Config>::AccountId>>::Balance;
pub type NegativeImbalanceOf<T> = <<T as swork::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

thread_local! {
    static EXISTENTIAL_DEPOSIT: RefCell<u64> = RefCell::new(0);
}

pub struct ExistentialDeposit;
impl Get<u64> for ExistentialDeposit {
    fn get() -> u64 {
        EXISTENTIAL_DEPOSIT.with(|v| *v.borrow())
    }
}

parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(4 * 1024 * 1024);
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = BlockWeights;
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type DbWeight = RocksDbWeight;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

impl balances::Config for Test {
    type Balance = Balance;
    type DustRemoval = ();
    type Event = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
}

parameter_types! {
    /// Unit is pico
    pub const MarketModuleId: ModuleId = ModuleId(*b"crmarket");
    pub const FileDuration: BlockNumber = 1000;
    pub const LiquidityDuration: BlockNumber = 1000;
    pub const FileReplica: u32 = 4;
    pub const InitFileByteFee: Balance = 1000; // Need align with FileDuration and FileBaseReplica
    pub const InitFileKeysCountFee: Balance = 1000;
    pub const StorageReferenceRatio: (u128, u128) = (1, 2);
    pub const StorageIncreaseRatio: Perbill = Perbill::from_percent(1);
    pub const StorageDecreaseRatio: Perbill = Perbill::from_percent(1);
    pub const StakingRatio: Perbill = Perbill::from_percent(72);
    pub const StorageRatio: Perbill = Perbill::from_percent(18);
    pub const MaximumFileSize: u64 = 137_438_953_472; // 128G = 128 * 1024 * 1024 * 1024
    pub const RenewRewardRatio: Perbill = Perbill::from_percent(5);
}

impl market::Config for Test {
    type ModuleId = MarketModuleId;
    type Currency = balances::Module<Self>;
    type SworkerInterface = Swork;
    type BenefitInterface = TestBenefitInterface;
    type Event = ();
    /// File duration.
    type FileDuration = FileDuration;
    type LiquidityDuration = LiquidityDuration;
    type FileReplica = FileReplica;
    type InitFileByteFee = InitFileByteFee;
    type InitFileKeysCountFee = InitFileKeysCountFee;
    type StorageReferenceRatio = StorageReferenceRatio;
    type StorageIncreaseRatio = StorageIncreaseRatio;
    type StorageDecreaseRatio = StorageDecreaseRatio;
    type StakingRatio = StakingRatio;
    type StorageRatio = StorageRatio;
    type MaximumFileSize = MaximumFileSize;
    type WeightInfo = market::weight::WeightInfo<Test>;
    type RenewRewardRatio = RenewRewardRatio;
}

pub struct TestWorksInterface;

impl Works<AccountId> for TestWorksInterface {
    fn report_works(_: BTreeMap<AccountId, u128>, _: u128) -> Weight { 0 }
}

pub struct TestBenefitInterface;

impl<AID> BenefitInterface<AID, BalanceOf<Test>, NegativeImbalanceOf<Test>> for TestBenefitInterface {
    fn update_era_benefit(_: EraIndex, _: BalanceOf<Test>) -> BalanceOf<Test> {
        Zero::zero()
    }

    fn update_reward(_: &AID, _: BalanceOf<Test>) { }

    fn maybe_reduce_fee(_: &AID, _: BalanceOf<Test>, _: WithdrawReasons) -> Result<NegativeImbalance<Test>, DispatchError> {
        Ok(NegativeImbalance::new(0))
    }

    fn maybe_free_count(_: &AID) -> bool {
        return true;
    }

    fn get_collateral_and_reward(_: &AID) -> (BalanceOf<Test>, BalanceOf<Test>) {
        (Zero::zero(), Zero::zero())
    }

    fn get_market_funds_ratio(_: &AID) -> Perbill {
        Perbill::zero()
    }
}

parameter_types! {
    pub const PunishmentSlots: u32 = 1;
    pub const MaxGroupSize: u32 = 100;
}

impl swork::Config for Test {
    type Currency = balances::Module<Self>;
    type Event = ();
    type PunishmentSlots = PunishmentSlots;
    type Works = TestWorksInterface;
    type MarketInterface = Market;
    type MaxGroupSize = MaxGroupSize;
    type BenefitInterface = TestBenefitInterface;
    type WeightInfo = swork::weight::WeightInfo<Test>;
}

impl crate::Config for Test {}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: balances::{Module, Call, Storage, Config<T>, Event<T>},
		Swork: swork::{Module, Call, Storage, Event<T>, Config<T>},
		Market: market::{Module, Call, Storage, Event<T>, Config},
	}
);

pub struct ExtBuilder { }

impl Default for ExtBuilder {
    fn default() -> Self {
        Self { }
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let t = system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        let mut ext: sp_io::TestExternalities = t.into();
        ext.execute_with(|| {
            assert_ok!(Market::set_enable_market(Origin::root(), true));
            assert_ok!(Market::set_base_fee(Origin::root(), 1000));
        });

        ext
    }
}
