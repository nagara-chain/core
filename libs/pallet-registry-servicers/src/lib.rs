#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type BalanceType<T> =
    <<T as Config>::Currency as frame_support::traits::Currency<AccountIdOf<T>>>::Balance;
pub type Guid = [u8; 16];
pub type NegativeImbalanceType<T> =
    <<T as Config>::Currency as frame_support::traits::Currency<AccountIdOf<T>>>::NegativeImbalance;
pub type UniqueCollection<T> = sp_std::collections::btree_set::BTreeSet<T>;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // ** Pallet Declaration **

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + nagara_council_bigbrothers::Config {
        /// Runtime Event registrar
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Currency for taxing configured fee for registration
        type Currency: frame_support::traits::Currency<Self::AccountId>;
        /// Binding fee
        type BindingFee: sp_core::Get<BalanceType<Self>>;
    }

    // ** Storage **

    #[pallet::storage]
    #[pallet::getter(fn big_brothers)]
    pub(super) type BigBrothers<T: Config> =
        StorageMap<_, frame_support::Blake2_128Concat, T::AccountId, ()>;

    #[pallet::storage]
    #[pallet::getter(fn rad)]
    pub(super) type RemoteAttestationDevices<T: Config> = StorageMap<
        _,
        frame_support::Blake2_128Concat,
        T::AccountId,
        RemoteAttestationDevice<T::AccountId>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn servicer)]
    pub(super) type Servicers<T: Config> =
        StorageMap<_, frame_support::Blake2_128Concat, T::AccountId, ServicerInfo<T::AccountId>>;

    // ** Custom, Event, and Errors type **

    /// Remote Attestation Device - Storage
    #[derive(Clone, Default, Eq, PartialEq)]
    #[derive(codec::Decode, codec::Encode, codec::MaxEncodedLen)]
    #[derive(sp_core::RuntimeDebug, scale_info::TypeInfo)]
    pub struct RemoteAttestationDevice<AccountId> {
        pub id: AccountId,
        pub big_brother: AccountId,
        pub serial_number: u32,
        pub guid: crate::Guid,
        pub owner: Option<AccountId>,
        pub web2_url: Option<frame_support::BoundedVec<u8, sp_core::ConstU32<256>>>,
        pub web3_url: Option<frame_support::BoundedVec<u8, sp_core::ConstU32<256>>>,
    }

    /// Cooperative Node - Info
    #[derive(Clone, Default, Eq, PartialEq)]
    #[derive(codec::Decode, codec::Encode, codec::MaxEncodedLen)]
    #[derive(sp_core::RuntimeDebug, scale_info::TypeInfo)]
    pub struct ServicerInfo<AccountId> {
        pub reputation: i32,
        pub rads: UniqueCollection<AccountId>,
    }

    /// Remote Attestation Device - Supply Arguments
    #[derive(Clone, Default, Eq, PartialEq)]
    #[derive(codec::Decode, codec::Encode, codec::MaxEncodedLen)]
    #[derive(sp_core::RuntimeDebug, scale_info::TypeInfo)]
    pub struct RemoteAttestationDeviceSupplyArgs<AccountId> {
        pub id: AccountId,
        pub serial_number: u32,
        pub guid: crate::Guid,
    }

    /// Remote Attestation Device - Binding Arguments
    #[derive(Clone, Default, Eq, PartialEq)]
    #[derive(codec::Decode, codec::Encode, codec::MaxEncodedLen)]
    #[derive(sp_core::RuntimeDebug, scale_info::TypeInfo)]
    pub struct RemoteAttestationDeviceBindArgs<AccountId> {
        pub rad_id: AccountId,
        pub web2_url: Option<frame_support::BoundedVec<u8, sp_core::ConstU32<256>>>,
        pub web3_url: Option<frame_support::BoundedVec<u8, sp_core::ConstU32<256>>>,
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Fatal error, chain storage compromised
        FatalError,
        /// Restricted call, only for Ancient Brother (Sudo)
        RestrictedCall,
        /// RAD already binded
        DeviceAlreadyBinded,
        /// RAD doesn't exist
        DeviceDoesntExist,
        /// RAD rebind rejected
        DeviceForceRebindRejected,
        /// RAD device is unbinded
        DeviceUnbinded,
        /// Eligible account for binding must have legal name
        AccountDoesntHaveLegalName,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Remote Attestation Device - Batch Supplied
        RemoteAttestationDeviceBatchSupplied {
            big_brother: T::AccountId,
            count: u64,
        },
        /// Remote Attestation Device - Batch Recalled
        RemoteAttestationDeviceBatchRecalled {
            big_brother: T::AccountId,
            count: u64,
        },
        /// Remote Attestation Device - Binded
        RemoteAttestationDeviceBinded {
            rad_id: T::AccountId,
            owner: T::AccountId,
        },
        /// Remote Attestation Device - Rebinded Forcefully
        RemoteAttestationDeviceRebindedForcefully {
            rad_id: T::AccountId,
            from: T::AccountId,
            to: T::AccountId,
            by: T::AccountId,
        },
        /// Servicer reputation increased
        ServicerReputationIncreased {
            who: T::AccountId,
            rad_id: T::AccountId,
        },
        /// Servicer reputation decreased
        ServicerReputationDecreased {
            who: T::AccountId,
            rad_id: T::AccountId,
        },
    }

    // ** Internal methods **

    impl<T: Config> Pallet<T> {
        /// ensure origin to be a sudo or Ancient Brother
        pub(crate) fn ensure_big_brother(
            origin: OriginFor<T>,
        ) -> Result<T::AccountId, sp_runtime::DispatchError> {
            let caller = ensure_signed(origin)?;

            if <BigBrothers<T>>::contains_key(&caller) {
                Ok(caller)
            } else {
                Err(<Error<T>>::RestrictedCall.into())
            }
        }

        /// ensure origin is Ancient Brother or System
        pub(crate) fn ensure_big_brother_or_root(
            origin: OriginFor<T>,
        ) -> Result<Option<T::AccountId>, sp_runtime::DispatchError> {
            let maybe_non_root = ensure_signed_or_root(origin)?;

            match maybe_non_root {
                | None => Ok(None),
                | Some(caller) => {
                    if <BigBrothers<T>>::contains_key(&caller) {
                        Ok(Some(caller))
                    } else {
                        Err(<Error<T>>::RestrictedCall.into())
                    }
                },
            }
        }
    }

    // ** Extrinsics **

    #[pallet::call]
    /// Dispatchable functions.
    impl<T: Config> Pallet<T> {
        /// Big Brother - Add Unbinded RAD
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(500_000_000_000, 3471))] // TODO: this is an estimation, please benchmark
        pub fn bb_rad_supply(
            origin: OriginFor<T>,
            devices: sp_std::vec::Vec<RemoteAttestationDeviceSupplyArgs<T::AccountId>>,
        ) -> DispatchResultWithPostInfo {
            let big_brother = Self::ensure_big_brother(origin)?;
            let mut count = 0u64;
            devices.into_iter().for_each(|new_device| {
                let rad = RemoteAttestationDevice {
                    id: new_device.id,
                    serial_number: new_device.serial_number,
                    guid: new_device.guid,
                    big_brother: big_brother.clone(),
                    owner: None,
                    web2_url: None,
                    web3_url: None,
                };

                if rad.id != big_brother && !<RemoteAttestationDevices<T>>::contains_key(&rad.id) {
                    <RemoteAttestationDevices<T>>::insert(rad.id.clone(), rad);
                    count = count.saturating_add(1);
                }
            });
            Self::deposit_event(Event::<T>::RemoteAttestationDeviceBatchSupplied {
                big_brother,
                count,
            });

            Ok(Pays::No.into())
        }

        /// Big Brother - Recalls Unbinded RAD
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(500_000_000_000, 3471))] // TODO: this is an estimation, please benchmark
        pub fn bb_rad_recall(
            origin: OriginFor<T>,
            devices: sp_std::vec::Vec<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            let big_brother = Self::ensure_big_brother(origin)?;
            let mut count = 0u64;
            devices.into_iter().for_each(|device_id| {
                <RemoteAttestationDevices<T>>::mutate_exists(&device_id, |maybe_exist| {
                    if let Some(maybe_unbounded) = maybe_exist {
                        if maybe_unbounded.owner.is_none() {
                            *maybe_exist = None;
                            count = count.saturating_add(1);
                        }
                    }
                });
            });
            Self::deposit_event(Event::<T>::RemoteAttestationDeviceBatchRecalled {
                big_brother,
                count,
            });

            Ok(Pays::No.into())
        }

        /// Big Brother - Recalls Unbinded RAD
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(200_000_000, 3471))] // TODO: this is an estimation, please benchmark
        pub fn bb_rad_force_rebind(
            origin: OriginFor<T>,
            rad_id: T::AccountId,
            to: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let big_brother = Self::ensure_big_brother(origin)?;

            ensure!(
                <RemoteAttestationDevices<T>>::contains_key(&rad_id),
                <Error<T>>::DeviceDoesntExist
            );

            let from = <RemoteAttestationDevices<T>>::mutate(&rad_id, |rad| {
                let rad = rad.as_mut().unwrap();

                if rad.owner.is_none() {
                    return Err(<Error<T>>::DeviceForceRebindRejected);
                }

                let from = rad.owner.clone().unwrap();

                if from == to {
                    return Err(<Error<T>>::DeviceForceRebindRejected);
                }

                rad.owner = Some(to.clone());

                Ok(from)
            })?;

            <Servicers<T>>::mutate(&from, |servicer_info| {
                if servicer_info.is_none() {
                    return Err(<Error<T>>::FatalError);
                }

                let servicer_info = servicer_info.as_mut().unwrap();
                servicer_info.rads.remove(&rad_id);

                Ok(())
            })?;

            <Servicers<T>>::mutate(&to, |servicer_info| {
                let servicer_info = servicer_info.get_or_insert(ServicerInfo::<T::AccountId> {
                    rads: Default::default(),
                    reputation: 0,
                });
                servicer_info.rads.insert(rad_id.clone());
            });

            Self::deposit_event(Event::<T>::RemoteAttestationDeviceRebindedForcefully {
                rad_id,
                from,
                to,
                by: big_brother,
            });

            Ok(Pays::No.into())
        }

        /// System or Ancient Brother - Increase Reputation
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(100_000_000, 3471))] // TODO: this is an estimation, please benchmark
        pub fn sys_rep_increase(
            origin: OriginFor<T>,
            rad_id: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let _ = Self::ensure_big_brother_or_root(origin)?;

            ensure!(
                <RemoteAttestationDevices<T>>::contains_key(&rad_id),
                <Error<T>>::DeviceDoesntExist
            );

            let who = {
                let rad_info = <RemoteAttestationDevices<T>>::get(&rad_id).unwrap();
                ensure!(rad_info.owner.is_some(), <Error<T>>::DeviceUnbinded);

                rad_info.owner.unwrap()
            };

            <Servicers<T>>::mutate(&who, |servicer_info| {
                if servicer_info.is_none() {
                    return Err(<Error<T>>::FatalError);
                }

                let servicer_info = servicer_info.as_mut().unwrap();
                servicer_info.reputation = servicer_info.reputation.saturating_add(1);

                Ok(())
            })?;

            Self::deposit_event(Event::<T>::ServicerReputationIncreased {
                who,
                rad_id,
            });

            Ok(Pays::No.into())
        }

        /// System or Ancient Brother - Decrease Reputation
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(100_000_000, 3471))] // TODO: this is an estimation, please benchmark
        pub fn sys_rep_decrease(
            origin: OriginFor<T>,
            rad_id: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let _ = Self::ensure_big_brother_or_root(origin)?;

            ensure!(
                <RemoteAttestationDevices<T>>::contains_key(&rad_id),
                <Error<T>>::DeviceDoesntExist
            );

            let who = {
                let rad_info = <RemoteAttestationDevices<T>>::get(&rad_id).unwrap();
                ensure!(rad_info.owner.is_some(), <Error<T>>::DeviceUnbinded);

                rad_info.owner.unwrap()
            };

            <Servicers<T>>::mutate(&who, |servicer_info| {
                if servicer_info.is_none() {
                    return Err(<Error<T>>::FatalError);
                }

                let servicer_info = servicer_info.as_mut().unwrap();
                servicer_info.reputation = servicer_info.reputation.saturating_sub(1);

                Ok(())
            })?;

            Self::deposit_event(Event::<T>::ServicerReputationDecreased {
                who,
                rad_id,
            });

            Ok(Pays::No.into())
        }

        /// User - Bind RAD
        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(100_000_000, 3471))] // TODO: this is an estimation, please benchmark
        pub fn user_bind(
            origin: OriginFor<T>,
            bind_args: RemoteAttestationDeviceBindArgs<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;
            nagara_council_bigbrothers::Pallet::<T>::ensure_account_has_legal_field(&owner)?;

            let RemoteAttestationDeviceBindArgs {
                rad_id,
                web2_url,
                web3_url,
            } = bind_args;

            ensure!(
                <RemoteAttestationDevices<T>>::contains_key(&rad_id),
                <Error<T>>::DeviceDoesntExist
            );

            <RemoteAttestationDevices<T>>::mutate(&rad_id, |rad| {
                let rad = rad.as_mut().unwrap();

                if rad.owner.is_some() {
                    return Err(<Error<T>>::DeviceAlreadyBinded);
                }

                rad.owner = Some(owner.clone());
                rad.web2_url = web2_url;
                rad.web3_url = web3_url;

                Ok(())
            })?;

            <Servicers<T>>::mutate(&owner, |servicer_info| {
                let servicer_info = servicer_info.get_or_insert(ServicerInfo::<T::AccountId> {
                    rads: Default::default(),
                    reputation: 0,
                });
                servicer_info.rads.insert(rad_id.clone());
            });

            Self::deposit_event(Event::<T>::RemoteAttestationDeviceBinded {
                owner,
                rad_id,
            });

            Ok(Pays::Yes.into())
        }
    }
}
