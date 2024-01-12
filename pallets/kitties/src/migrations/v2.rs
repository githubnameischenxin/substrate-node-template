


use frame_support::{
    pallet_prelude::*,
    storage::StoragePrefixedMap,
    traits::GetStorageVersion,
    weights::Weight,
    migration::storage_key_iter,
    Blake2_128Concat,
};


use crate::{Config, Pallet, Kitties, KittyId, Kitty};

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct OldKitty {
	pub dna: [u8; 16],
	pub name: [u8; 4],
}

pub fn migrate<T: Config>() -> Weight {
    let on_chain_version = Pallet::<T>::on_chain_storage_version();
    let current_version = Pallet::<T>::current_storage_version();

    log::info!(target: "migrate", "on_chain_version={:?}", on_chain_version);
    log::info!(target: "migrate", "current_version={:?}", current_version);

    if on_chain_version != 1 {
        return Weight::zero();
    }
    if current_version != 2 {
        return Weight::zero();
    }

    let module = Kitties::<T>::module_prefix();
    let item = Kitties::<T>::storage_prefix();

    for (index, kitty) in storage_key_iter::<KittyId, OldKitty, Blake2_128Concat>(module, item).drain() {
        let new_kitty = Kitty {
            dna: kitty.dna,
            name: *b"kitty_v1",
        };

        Kitties::<T>::insert(index, &new_kitty);
    }
    Weight::zero()
}