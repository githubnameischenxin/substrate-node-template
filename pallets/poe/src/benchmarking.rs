use crate::*;

use frame_benchmarking::v1::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;
use sp_std::*;

fn assert_last_event<T: Config>(event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(event.into());
}

benchmarks! {
	create_claim {
		let d in 0 .. T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0u8; d as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller.clone()), claim.clone())
	verify {
		assert_eq!(Proofs::<T>::get(&claim).is_some(), true);
		assert_last_event::<T>(Event::ClaimCreated(caller, claim).into())
	}

	revoke_claim {
		let d in 0 .. T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0u8; d as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		let _ = Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone());
	}: _(RawOrigin::Signed(caller.clone()), claim.clone())
	verify {
		assert_last_event::<T>(Event::ClaimRevoked(caller, claim).into())
	}

	transfer_claim {
		let d in 0 .. T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0u8; d as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		let target: T::AccountId = account("target", 0, 0);
		let _ = Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone());
	}: _(RawOrigin::Signed(caller.clone()), claim.clone(), target.clone())
	verify {
		assert_last_event::<T>(Event::ClaimTransferred(caller, claim, target).into())
	}

	impl_benchmark_test_suite!(PoeModule, crate::mock::new_test_ext(), crate::mock::Test);
}
