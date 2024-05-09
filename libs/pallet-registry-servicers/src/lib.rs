#![cfg_attr(not(feature = "std"), no_std)]

pub use nagara_council_bigbrothers as ngr_bbcm;
pub use pallet::*;

pub type AccountTypeOf<T> = <T as frame_system::Config>::AccountId;
pub type AttesterId = sp_core::ed25519::Public;
pub type PeerId = sp_core::ed25519::Public;
pub type BalanceCurrencyTypeOf<T> =
    <<T as Config>::Currency as frame_support::traits::Currency<AccountTypeOf<T>>>::Balance;
pub type BalanceInspectTypeOf<T> = <<T as Config>::Currency as frame_support::traits::fungible::Inspect<AccountTypeOf<T>>>::Balance;
pub type Guid = [u8; 16];
pub type UniqueMap<K, V> = sp_std::collections::btree_map::BTreeMap<K, V>;

pub const PALLET_IDENTIFICATION: frame_support::PalletId = frame_support::PalletId(*b"ngr/svrg");

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // region: Pallet Declaration

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + ngr_bbcm::Config {
        /// Runtime Event registrar
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Overarching hold reason.
        type RuntimeHoldReason: From<HoldReason>;
        /// Currency for pallet operations
        type Currency: frame_support::traits::Currency<Self::AccountId>
            + frame_support::traits::fungible::Mutate<Self::AccountId>
            + frame_support::traits::fungible::Inspect<Self::AccountId>
            + frame_support::traits::fungible::MutateHold<
                Self::AccountId,
                Reason = <Self as pallet::Config>::RuntimeHoldReason,
            > + frame_support::traits::fungible::InspectHold<
                Self::AccountId,
                Reason = <Self as pallet::Config>::RuntimeHoldReason,
            >;
        /// Attester binding deposit amount, once per attester
        #[pallet::constant]
        type BindingDepositAmount: sp_core::Get<BalanceInspectTypeOf<Self>>;
        /// Servicer registry fee (this to prevent cheap reputation reset) paid
        /// only once per existence
        #[pallet::constant]
        type RegistrationFeeAmount: sp_core::Get<BalanceCurrencyTypeOf<Self>>;
        /// Maximum mediator for servicers, mediator is a role that can be
        /// filled by smart contracts to mediate services between the
        /// chain and dApps
        #[pallet::constant]
        type MaxMediators: sp_core::Get<u32>;
    }

    // endregion

    // region: Storage

    #[pallet::storage]
    #[pallet::getter(fn attesters)]
    pub(super) type Attesters<T: Config> = StorageMap<
        _,
        frame_support::Blake2_128Concat,
        AttesterId, // should always be ed25519
        RemoteAttestationDevice<T::AccountId>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn mediators)]
    pub(super) type Mediators<T: Config> =
        StorageValue<_, frame_support::BoundedBTreeSet<T::AccountId, T::MaxMediators>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn servicers)]
    pub(super) type Servicers<T: Config> =
        StorageMap<_, frame_support::Blake2_128Concat, T::AccountId, ServicerInformation>;

    // endregion

    // region: Genesis

    // endregion

    // region: Custom, Event, and Errors type

    /// Remote Attestation Device Supply arguments
    #[derive(Clone, Eq, PartialEq)]
    #[derive(codec::Decode, codec::Encode, codec::MaxEncodedLen)]
    #[derive(sp_core::RuntimeDebug, scale_info::TypeInfo)]
    pub struct RemoteAttestationDeviceSupplyArgs {
        pub id: AttesterId,
        pub guid: Guid,
        pub serial_number: u32,
    }

    /// Remote Attestation Device
    #[derive(Clone, Default, Eq, PartialEq)]
    #[derive(codec::Decode, codec::Encode, codec::MaxEncodedLen)]
    #[derive(sp_core::RuntimeDebug, scale_info::TypeInfo)]
    pub struct RemoteAttestationDevice<AccountId> {
        pub big_brother: AccountId,
        pub serial_number: u32,
        pub guid: Guid,                // this is important for windows drivers
        pub binder: Option<AccountId>, // None means unbinded
    }

    impl<AccountId> RemoteAttestationDevice<AccountId> {
        pub fn is_binded(&self) -> bool {
            self.binder.is_some()
        }
    }

    /// Servicer Information (Cooperatives)
    #[derive(Clone)]
    #[derive(codec::Decode, codec::Encode)]
    #[derive(sp_core::RuntimeDebug, scale_info::TypeInfo)]
    pub struct ServicerInformation {
        pub rep_positive: u32,
        pub rep_negative: u32,
        pub bindings: UniqueMap<AttesterId, PeerId>,
    }

    impl ServicerInformation {
        pub fn get_peer_id(&self, attester_id: &AttesterId) -> Option<PeerId> {
            self.bindings.get(attester_id).copied()
        }

        pub fn get_total_reputation(&self) -> i64 {
            (self.rep_positive as i64).saturating_sub(self.rep_negative as i64)
        }

        fn try_add_binding<T: Config>(
            &mut self,
            attester_id: AttesterId,
            peer_id: PeerId,
        ) -> Result<(), sp_runtime::DispatchError> {
            if self.bindings.contains_key(&attester_id) {
                return Err(<Error<T>>::AttesterAlreadyBinded.into());
            }

            self.bindings.insert(attester_id, peer_id);

            Ok(())
        }

        fn increase_reputation(&mut self) {
            self.rep_positive = self.rep_positive.saturating_add(1);
        }

        fn decrease_reputation(&mut self) {
            self.rep_negative = self.rep_negative.saturating_sub(1);
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Fatal error, chain storage compromised
        FatalError,
        /// Restricted call (sudo or big brothers only)
        RestrictedCall,
        /// Attester already binded
        AttesterAlreadyBinded,
        /// Attester doesn't exist
        AttesterDoesntExist,
        /// Attester already supplied
        AttesterAlreadySupplied,
        /// Attester is binded to no one
        AttesterIsUnbinded,
        /// Servicer cannot pay registration fee
        ServicerCannotPayRegistrationFee,
        /// Mediator already registered
        MediatorAlreadyRegistered,
        /// Mediator is not registered
        MediatorNotFound,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Big Brother's unbinded attestation device supplied
        BigBrotherAttesterSupplied { id: AttesterId, bb: T::AccountId },
        /// Big Brother's unbinded attestation device recalled
        BigBrotherAttesterRecalled { id: AttesterId, bb: T::AccountId },
        /// Servicer's reputation increased
        ServicerReputationIncreased {
            by: T::AccountId,
            on: AttesterId,
            who: T::AccountId,
        },
        /// Servicer's reputation decreased
        ServicerReputationDecreased {
            by: T::AccountId,
            on: AttesterId,
            who: T::AccountId,
        },
        /// Attester binded
        AttesterBinded {
            to: T::AccountId,
            which: AttesterId,
            peer_id: PeerId,
        },
        /// Attester unbinded (this shouldn't happen in normal condition)
        AttesterUnbinded {
            by: Option<T::AccountId>,
            from: T::AccountId,
        },
        /// Mediator added
        MediatorAdded { who: T::AccountId, by: T::AccountId },
        /// Mediator added
        MediatorRemoved { who: T::AccountId, by: T::AccountId },
        /// Servicer registration fee paid
        ServicerRegistrationFeePaid {
            who: T::AccountId,
            amount: BalanceCurrencyTypeOf<T>,
        },
        /// Servicer's balance held for binding
        ServicerBalanceHeldForBinding {
            who: T::AccountId,
            amount: BalanceInspectTypeOf<T>,
        },
        /// Servicer's balance held released
        ServicerBalanceHeldForBindingReleased {
            who: T::AccountId,
            amount: BalanceInspectTypeOf<T>,
        },
    }

    #[pallet::composite_enum]
    pub enum HoldReason {
        /// Servicer registration, this is once per attester id
        #[codec(index = 0)]
        Binding,
    }

    // endregion

    // region: Helper methods

    impl<T: Config> Pallet<T> {
        fn ensure_and_get_signed_mediator(
            origin: OriginFor<T>,
        ) -> Result<T::AccountId, sp_runtime::DispatchError> {
            let mediator = ensure_signed(origin)?;

            if !<Mediators<T>>::get().contains(&mediator) {
                return Err(<Error<T>>::MediatorNotFound.into());
            }

            Ok(mediator)
        }

        fn try_supply_new_attester(
            big_brother: T::AccountId,
            supply_args: RemoteAttestationDeviceSupplyArgs,
        ) -> Result<Event<T>, sp_runtime::DispatchError> {
            let RemoteAttestationDeviceSupplyArgs {
                id,
                guid,
                serial_number,
            } = supply_args;

            if <Attesters<T>>::contains_key(id) {
                return Err(<Error<T>>::AttesterAlreadySupplied.into());
            }

            let new_attester = RemoteAttestationDevice {
                big_brother: big_brother.clone(),
                binder: None,
                guid,
                serial_number,
            };
            let event = Event::BigBrotherAttesterSupplied {
                id,
                bb: big_brother,
            };
            <Attesters<T>>::insert(id, new_attester);

            Ok(event)
        }

        fn try_recall_attester(
            caller: T::AccountId,
            attester_id: AttesterId,
        ) -> Result<Event<T>, sp_runtime::DispatchError> {
            if !<Attesters<T>>::contains_key(attester_id) {
                return Err(<Error<T>>::AttesterDoesntExist.into());
            }

            let attester = Self::attesters(attester_id).unwrap();

            if attester.is_binded() {
                return Err(<Error<T>>::AttesterAlreadyBinded.into());
            }

            if !caller.eq(&attester.big_brother) {
                return Err(<Error<T>>::RestrictedCall.into());
            }

            <Attesters<T>>::remove(attester_id);
            let event = Event::BigBrotherAttesterRecalled {
                bb: caller,
                id: attester_id,
            };

            Ok(event)
        }

        fn try_hold_balance(who: &T::AccountId) -> Result<Event<T>, sp_runtime::DispatchError> {
            let amount = T::BindingDepositAmount::get();
            let runtime_hold_reason = HoldReason::Binding.into();
            <<T as Config>::Currency as frame_support::traits::fungible::MutateHold<
                T::AccountId,
            >>::hold(&runtime_hold_reason, who, amount)?;

            Ok(Event::ServicerBalanceHeldForBinding {
                who: who.clone(),
                amount,
            })
        }

        fn try_unhold_balance(who: &T::AccountId) -> Result<Event<T>, sp_runtime::DispatchError> {
            let amount = T::BindingDepositAmount::get();
            let runtime_hold_reason = HoldReason::Binding.into();
            let precision = frame_support::traits::tokens::Precision::BestEffort;
            let actual_released =
                <<T as Config>::Currency as frame_support::traits::fungible::MutateHold<
                    T::AccountId,
                >>::release(&runtime_hold_reason, who, amount, precision)?;

            Ok(Event::ServicerBalanceHeldForBindingReleased {
                who: who.clone(),
                amount: actual_released,
            })
        }

        fn try_insert_attester_into_servicer(
            who: &T::AccountId,
            peer_id: PeerId,
            attester_id: AttesterId,
        ) -> Result<sp_std::vec::Vec<Event<T>>, sp_runtime::DispatchError> {
            let mut events = sp_std::vec![];

            if !<Servicers<T>>::contains_key(who) {
                let amount = T::RegistrationFeeAmount::get();
                let withdraw_reason = frame_support::traits::tokens::WithdrawReasons::FEE;
                let _ = <<T as Config>::Currency as frame_support::traits::Currency<
                    T::AccountId,
                >>::withdraw(
                    who,
                    amount,
                    withdraw_reason,
                    frame_support::traits::tokens::ExistenceRequirement::KeepAlive,
                )
                .map_err(|_| <Error<T>>::ServicerCannotPayRegistrationFee)?;
                events.push(Event::ServicerRegistrationFeePaid {
                    who: who.clone(),
                    amount,
                });
            }

            <Servicers<T>>::try_mutate(who, |mutable_servicer| {
                let mutable_servicer = mutable_servicer.get_or_insert(ServicerInformation {
                    rep_positive: 0,
                    rep_negative: 0,
                    bindings: Default::default(),
                });
                mutable_servicer.try_add_binding::<T>(attester_id, peer_id)?;

                Result::<(), sp_runtime::DispatchError>::Ok(())
            })?;

            Ok(events)
        }

        fn try_bind_attester(
            binder: T::AccountId,
            peer_id: PeerId,
            attester_id: AttesterId,
        ) -> Result<sp_std::vec::Vec<Event<T>>, sp_runtime::DispatchError> {
            if !<Attesters<T>>::contains_key(attester_id) {
                return Err(<Error<T>>::AttesterDoesntExist.into());
            }

            let attester = Self::attesters(attester_id).unwrap();

            if attester.is_binded() {
                return Err(<Error<T>>::AttesterAlreadyBinded.into());
            }

            let mut events = sp_std::vec![];

            <Attesters<T>>::try_mutate(attester_id, |mutable_attester| {
                events.push(Self::try_hold_balance(&binder)?);
                let result = Self::try_insert_attester_into_servicer(&binder, peer_id, attester_id);

                match result {
                    | Err(err) => {
                        let _ = Self::try_unhold_balance(&binder)?; // suppress event
                        return Err(err);
                    },
                    | Ok(servicer_add_events) => {
                        for event in servicer_add_events {
                            events.push(event)
                        }
                    },
                }

                let mutable_attester = mutable_attester.as_mut().unwrap();
                mutable_attester.binder = Some(binder.clone());

                Result::<(), sp_runtime::DispatchError>::Ok(())
            })?;

            events.push(Event::AttesterBinded {
                to: binder,
                which: attester_id,
                peer_id,
            });

            Ok(events)
        }
    }

    // endregion

    // region: Extrinsics

    #[pallet::call]
    /// Dispatchable functions.
    impl<T: Config> Pallet<T> {
        /// Big Brother: Supply an Attester
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn bb_attester_supply(
            origin: OriginFor<T>,
            supply_args: RemoteAttestationDeviceSupplyArgs,
        ) -> DispatchResultWithPostInfo {
            let big_brother = ngr_bbcm::Pallet::<T>::ensure_and_get_council_member(origin)?;
            let event = Self::try_supply_new_attester(big_brother, supply_args)?;
            Self::deposit_event(event);

            Ok(Pays::Yes.into())
        }

        /// Big Brother: Add a mediator
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn bb_mediator_add(
            origin: OriginFor<T>,
            who: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let by = ngr_bbcm::Pallet::<T>::ensure_and_get_council_member(origin)?;
            ngr_bbcm::Pallet::<T>::ensure_account_has_verified_legality(&who)?;
            ensure!(
                !<Mediators<T>>::get().contains(&who),
                <Error<T>>::MediatorAlreadyRegistered,
            );
            let _ = <Mediators<T>>::mutate(|mediators_mut| mediators_mut.try_insert(who.clone()));
            Self::deposit_event(Event::MediatorAdded {
                who,
                by,
            });

            Ok(Pays::Yes.into())
        }

        /// Big Brother: Remove a mediator
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn bb_mediator_remove(
            origin: OriginFor<T>,
            who: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let by = ngr_bbcm::Pallet::<T>::ensure_and_get_council_member(origin)?;
            ensure!(
                <Mediators<T>>::get().contains(&who),
                <Error<T>>::MediatorNotFound,
            );
            let _ = <Mediators<T>>::mutate(|mediators_mut| mediators_mut.remove(&who));
            Self::deposit_event(Event::MediatorRemoved {
                who,
                by,
            });

            Ok(Pays::Yes.into())
        }

        /// Big Brother: Recall an attester
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn bb_attester_recall(
            origin: OriginFor<T>,
            attester_id: AttesterId,
        ) -> DispatchResultWithPostInfo {
            let big_brother = ngr_bbcm::Pallet::<T>::ensure_and_get_council_member(origin)?;
            let event = Self::try_recall_attester(big_brother, attester_id)?;
            Self::deposit_event(event);

            Ok(Pays::Yes.into())
        }

        /// Servicer: Bind attester
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn sv_attester_bind(
            origin: OriginFor<T>,
            peer_id: PeerId,
            attester_id: AttesterId,
        ) -> DispatchResultWithPostInfo {
            let binder = ensure_signed(origin)?;
            let events = Self::try_bind_attester(binder, peer_id, attester_id)?;

            for event in events {
                Self::deposit_event(event);
            }

            Ok(Pays::Yes.into())
        }

        /// Mediators: Increase servicer reputation
        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn md_rep_increase(origin: OriginFor<T>, on: AttesterId) -> DispatchResultWithPostInfo {
            let by = Self::ensure_and_get_signed_mediator(origin)?;
            ensure!(
                <Attesters<T>>::contains_key(on),
                <Error<T>>::AttesterDoesntExist,
            );
            let attester = <Attesters<T>>::get(on).unwrap();
            ensure!(attester.binder.is_some(), <Error<T>>::AttesterIsUnbinded);
            let who = attester.binder.unwrap();
            <Servicers<T>>::mutate(&who, |servicers_mut| {
                servicers_mut.as_mut().unwrap().increase_reputation()
            });
            Self::deposit_event(Event::ServicerReputationIncreased {
                who,
                on,
                by,
            });

            Ok(Pays::No.into())
        }

        /// Mediators: decrease servicer reputation
        #[pallet::call_index(6)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn md_rep_decrease(origin: OriginFor<T>, on: AttesterId) -> DispatchResultWithPostInfo {
            let by = Self::ensure_and_get_signed_mediator(origin)?;
            ensure!(
                <Attesters<T>>::contains_key(on),
                <Error<T>>::AttesterDoesntExist,
            );
            let attester = <Attesters<T>>::get(on).unwrap();
            ensure!(attester.binder.is_some(), <Error<T>>::AttesterIsUnbinded);
            let who = attester.binder.unwrap();
            <Servicers<T>>::mutate(&who, |servicers_mut| {
                servicers_mut.as_mut().unwrap().decrease_reputation()
            });
            Self::deposit_event(Event::ServicerReputationDecreased {
                who,
                on,
                by,
            });

            Ok(Pays::No.into())
        }
    }

    // endregion
}
