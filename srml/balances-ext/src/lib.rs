#![cfg_attr(not(feature = "std"), no_std)]

use srml_support::{StorageValue, dispatch::{Result, Dispatchable}, decl_module, decl_storage, decl_event, ensure};
use system::ensure_signed;
use sr_primitives::traits::{As, StaticLookup};

pub trait Trait: srml_balances::Trait + sudo::Trait {

}

decl_storage! {
    trait Store for Module<T: Trait> as BalancesExtModule {
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        pub fn transfer(
            origin,
            dest: <T::Lookup as StaticLookup>::Source,
            #[compact] value: T::Balance
        ) {
            // 必须是sudo账户才能执行transfer
            let sender = ensure_signed(origin)?;
            ensure!(sender == <sudo::Module<T>>::key(), "Sender must be the Sudo key to transfer");
            let new_origin = system::RawOrigin::Signed(sender).into();

            // 每次转账额度不能大于100万
            let value_u64 = <T::Balance as As<u64>>::as_(value);
            ensure!(value_u64 < 1000000, "The amount of transfer is too large");

            // 调用原生模块的方法
            <srml_balances::Module<T>>::transfer(new_origin, dest, value);

            // 如果原生模块的方法不是pub的，必须使用如下方法进行方法
            // srml_balances::Call::<T>::transfer(dest, value).dispatch(new_origin);
        }

    }
}

