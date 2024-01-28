#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub type UniqueCollection<T> = sp_std::collections::btree_set::BTreeSet<T>;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// Servicer Info
    #[derive(Clone, Default, Eq, PartialEq)]
    #[derive(codec::Decode, codec::Encode, RuntimeDebug, TypeInfo)]
    pub struct ServicerInfo<AccountId, BlockNumber> {
        pub owner: AccountId,
        pub timestamp: BlockNumber,
        pub base_url: frame_support::BoundedVec<u8, sp_core::ConstU32<256>>,
    }

    /// Account Info
    #[derive(Clone, Default, Eq, PartialEq)]
    #[derive(codec::Decode, codec::Encode, RuntimeDebug, TypeInfo)]
    pub struct AccountServicerInfo {
        pub reputations: i32,
        pub servicers: UniqueCollection<sp_core::OpaquePeerId>,
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn account_servicers)]
    pub(super) type AccountServicers<T: Config> =
        StorageMap<_, frame_support::Blake2_128Concat, T::AccountId, AccountServicerInfo>;

    #[pallet::storage]
    #[pallet::getter(fn servicers)]
    pub(super) type Servicers<T: Config> = StorageMap<
        _,
        frame_support::Blake2_128Concat,
        sp_core::OpaquePeerId,
        ServicerInfo<T::AccountId, BlockNumberFor<T>>,
    >;

    #[pallet::error]
    pub enum Error<T> {
        /// Account has no servicer.
        AccountHasNoServicer,
        /// No registered servicer found.
        ServicerNotFound,
        /// Servicer already registered.
        ServicerAlreadyRegistered,
        /// Servicer base url is too long.
        ServicerBaseUrlTooLong,
        /// Ancient Brother (sudo) authorization call only.
        AncientBrotherOnly,
        /// Account is not registered for judgement
        AccountIsNotForJudgement,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Servicer registered.
        ServicerRegistered {
            account_id: T::AccountId,
            peer_id: sp_core::OpaquePeerId,
        },
        /// Servicer removed.
        ServicerRemoved {
            account_id: T::AccountId,
            peer_id: sp_core::OpaquePeerId,
        },
        /// Owner service reputation increased.
        ServiceReputationIncreased { account_id: T::AccountId },
        /// Owner service reputation decreased.
        ServiceReputationDecreased { account_id: T::AccountId },
    }

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_identity::Config + pallet_sudo::Config {
        /// Runtime Event registrar
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    impl<T: Config> Pallet<T> {
        /// ensure origin to be a sudo or Ancient Brother
        pub(crate) fn ensure_ancient_brother(origin: OriginFor<T>) -> DispatchResult {
            let caller = ensure_signed_or_root(origin)?;
            let sudo_account = pallet_sudo::Pallet::<T>::key();

            if sudo_account.is_none() {
                return Ok(());
            }

            let sudo_account = sudo_account.unwrap();

            match caller {
                | None => Err(Error::<T>::AncientBrotherOnly.into()),
                | Some(account) => {
                    if account == sudo_account {
                        Ok(())
                    } else {
                        Err(Error::<T>::AncientBrotherOnly.into())
                    }
                },
            }
        }
    }

    #[pallet::call]
    /// Dispatchable functions.
    impl<T: Config> Pallet<T> {
        /// Register account's servicer, account must already registered in
        /// Identity Pallet
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(4_000_000, 3471) + T::DbWeight::get().writes(2) + T::DbWeight::get().reads(2))] // TODO: this is an estimation, please benchmark
        pub fn register_servicer(
            origin: OriginFor<T>,
            owner: T::AccountId,
            peer_id: sp_core::OpaquePeerId,
            base_url: sp_std::vec::Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            Self::ensure_ancient_brother(origin)?;
            let block = <frame_system::Pallet<T>>::block_number();
            let minimum_identity_field = pallet_identity::IdentityField::Legal as u64;
            ensure!(
                !Servicers::<T>::contains_key(&peer_id),
                Error::<T>::ServicerAlreadyRegistered,
            );
            ensure!(
                pallet_identity::Pallet::<T>::has_identity(&owner, minimum_identity_field),
                Error::<T>::AccountIsNotForJudgement,
            );

            let cloned_peer_id = peer_id.clone();

            AccountServicers::<T>::try_mutate(&owner, |servicers| {
                let account_servicer_info = servicers.get_or_insert(AccountServicerInfo::default());

                if account_servicer_info.servicers.contains(&peer_id) {
                    return Err(Error::<T>::ServicerAlreadyRegistered);
                }
                account_servicer_info.servicers.insert(cloned_peer_id);

                Ok(())
            })?;

            let base_url = frame_support::BoundedVec::try_from(base_url)
                .map_err(|_| Error::<T>::ServicerBaseUrlTooLong)?;
            let servicer_info = ServicerInfo {
                owner: owner.clone(),
                timestamp: block,
                base_url,
            };
            Servicers::<T>::insert(peer_id.clone(), servicer_info);

            Self::deposit_event(Event::<T>::ServicerRegistered {
                account_id: owner,
                peer_id,
            });

            Ok(Pays::Yes.into())
        }

        /// Remove account's servicer
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(4_000_000, 3471) + T::DbWeight::get().writes(2) + T::DbWeight::get().reads(2))] // TODO: this is an estimation, please benchmark
        pub fn remove_servicer(
            origin: OriginFor<T>,
            peer_id: sp_core::OpaquePeerId,
        ) -> DispatchResultWithPostInfo {
            Self::ensure_ancient_brother(origin)?;
            ensure!(
                Servicers::<T>::contains_key(&peer_id),
                Error::<T>::ServicerNotFound
            );
            let servicer_info = Servicers::<T>::take(&peer_id).unwrap();
            let servicer_owner = servicer_info.owner;
            AccountServicers::<T>::mutate(&servicer_owner, |servicers| {
                if let Some(account_servicer_info) = servicers {
                    account_servicer_info.servicers.remove(&peer_id);
                }
            });

            Self::deposit_event(Event::<T>::ServicerRemoved {
                account_id: servicer_owner,
                peer_id,
            });

            Ok(Pays::Yes.into())
        }

        /// Increase reputation
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(4_000_000, 3471) + T::DbWeight::get().writes(2) + T::DbWeight::get().reads(2))] // TODO: this is an estimation, please benchmark
        pub fn increase_reputation(
            origin: OriginFor<T>,
            peer_id: sp_core::OpaquePeerId,
        ) -> DispatchResultWithPostInfo {
            Self::ensure_ancient_brother(origin)?;
            ensure!(
                Servicers::<T>::contains_key(&peer_id),
                Error::<T>::ServicerNotFound
            );
            let servicer_info = Servicers::<T>::get(&peer_id).unwrap();
            let owner_account = servicer_info.owner;
            AccountServicers::<T>::mutate(&owner_account, |servicers| {
                let account_servicer_info = servicers.as_mut().unwrap();
                account_servicer_info.reputations = account_servicer_info.reputations.saturating_add(1);
            });

            Self::deposit_event(Event::<T>::ServiceReputationIncreased {
                account_id: owner_account,
            });

            Ok(Pays::Yes.into())
        }

        /// Decrease reputation
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(4_000_000, 3471) + T::DbWeight::get().writes(2) + T::DbWeight::get().reads(2))] // TODO: this is an estimation, please benchmark
        pub fn decrease_reputation(
            origin: OriginFor<T>,
            peer_id: sp_core::OpaquePeerId,
        ) -> DispatchResultWithPostInfo {
            Self::ensure_ancient_brother(origin)?;
            ensure!(
                Servicers::<T>::contains_key(&peer_id),
                Error::<T>::ServicerNotFound
            );
            let servicer_info = Servicers::<T>::get(&peer_id).unwrap();
            let owner_account = servicer_info.owner;
            AccountServicers::<T>::mutate(&owner_account, |servicers| {
                let account_servicer_info = servicers.as_mut().unwrap();
                account_servicer_info.reputations = account_servicer_info.reputations.saturating_sub(1);
            });

            Self::deposit_event(Event::<T>::ServiceReputationDecreased {
                account_id: owner_account,
            });

            Ok(Pays::Yes.into())
        }
    }
}
