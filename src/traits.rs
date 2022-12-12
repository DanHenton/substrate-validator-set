use frame_support::inherent::Vec;

pub trait ValidatorSetStorageProvider<AccountId> {
	fn validators() -> Vec<AccountId>;

	fn approved_validators() -> Vec<AccountId>;

	fn offline_validators() -> Vec<AccountId>;
}
