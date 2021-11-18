#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    where_clause { where
		T::AccountId: AsRef<[u8]>,
	}
    
    write_data_onchain {
        let a in 1 .. 100;
        let feed_name = "test";
        let feed_name_vec = feed_name.as_bytes().to_vec();
        let data = b"{'test':'test'}".to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), feed_name_vec, data.clone())
    verify {
        let key = DataId::<T>::get();
        let data_request = Pallet::<T>::data_requests(key);
        assert_eq!(data_request.payload, data);
    }
}
