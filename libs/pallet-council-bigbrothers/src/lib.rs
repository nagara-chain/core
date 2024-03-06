#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub type AccountTypeOf<T> = <T as frame_system::Config>::AccountId;
pub type BalanceTypeOf<T> = <<T as Config>::Currency as frame_support::traits::fungible::Inspect<
    AccountTypeOf<T>,
>>::Balance;

pub const PALLET_IDENTIFICATION: frame_support::PalletId = frame_support::PalletId(*b"ngr/bbcm");

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
    pub trait Config: frame_system::Config + pallet_identity::Config {
        /// Runtime Event registrar
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Overarching hold reason.
        type RuntimeHoldReason: From<HoldReason>;
        /// Currency for pallet operations
        type Currency: frame_support::traits::fungible::Mutate<Self::AccountId>
            + frame_support::traits::fungible::Inspect<Self::AccountId>
            + frame_support::traits::fungible::MutateHold<
                Self::AccountId,
                Reason = Self::RuntimeHoldReason,
            > + frame_support::traits::fungible::InspectHold<
                Self::AccountId,
                Reason = Self::RuntimeHoldReason,
            >;
        /// Council Membership deposit amount
        #[pallet::constant]
        type RegistrationDepositAmount: sp_core::Get<BalanceTypeOf<Self>>;
        /// Maximum members (Big Brothers)
        #[pallet::constant]
        type MaxMembers: sp_core::Get<u32>;
        /// Chain's Burn Address
        #[pallet::constant]
        type BurnAddress: sp_core::Get<Self::AccountId>;
        /// Initial Weight to Fee Divider
        #[pallet::constant]
        type InitialWeightToFeeDivider: sp_core::Get<u64>;
        /// Initial Weight to Fee Multiplier
        #[pallet::constant]
        type InitialWeightToFeeMultiplier: sp_core::Get<u64>;
        /// Initial Minimum Transaction Fee
        #[pallet::constant]
        type InitialMinimumTransactionFee: sp_core::Get<BalanceTypeOf<Self>>;
    }

    // endregion

    // region: Storage

    #[pallet::storage]
    #[pallet::getter(fn elder)]
    pub(super) type Elder<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn members)]
    pub(super) type Members<T: Config> =
        StorageValue<_, sp_runtime::BoundedBTreeSet<T::AccountId, T::MaxMembers>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn tx_fee_info)]
    pub(super) type TxFeeInfo<T: Config> =
        StorageValue<_, TransactionFeeInfo<BalanceTypeOf<T>>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn current_proposal)]
    pub(super) type CurrentProposal<T: Config> = StorageValue<
        _,
        TransactionFeeChangeProposal<T::AccountId, BalanceTypeOf<T>, BlockNumberFor<T>>,
        OptionQuery,
    >;

    // endregion

    // region: Genesis

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub elder: Option<T::AccountId>,
        pub big_brothers: sp_std::vec::Vec<T::AccountId>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            // Transaction Fee Info
            let initial_weight_to_fee_divider = T::InitialWeightToFeeDivider::get();
            let initial_weight_to_fee_multiplier = T::InitialWeightToFeeMultiplier::get();
            let initial_minimum_transaction_fee = T::InitialMinimumTransactionFee::get();
            let transaction_fee_info = TransactionFeeInfo {
                minimum_transaction_fee: initial_minimum_transaction_fee,
                weight_to_fee_divider: initial_weight_to_fee_divider,
                weight_to_fee_multiplier: initial_weight_to_fee_multiplier,
            };
            <TxFeeInfo<T>>::set(transaction_fee_info);

            // Elder
            <Elder<T>>::set(self.elder.clone());

            assert!(
                <Members<T>>::get().is_empty(),
                "Cannot reinitialize BigBrothers!"
            );
            assert!(
                self.big_brothers.len() <= (T::MaxMembers::get() as usize),
                "Initial BigBrothers exceeds Runtime Config!"
            );
            let mut bounded_members = sp_runtime::BoundedBTreeSet::new();

            for big_brother in &self.big_brothers {
                let _ = bounded_members.try_insert(big_brother.clone());
            }

            // Members
            <Members<T>>::put(bounded_members);
        }
    }

    // endregion

    // region: Custom, Event, and Errors type

    #[derive(Clone)]
    #[derive(codec::Decode, codec::Encode, codec::MaxEncodedLen)]
    #[derive(scale_info::TypeInfo)]
    pub struct TransactionFeeChangeProposal<AccountId: core::cmp::Ord, BalanceType, BlockNumber> {
        pub initiator: AccountId,
        pub initiated_at: BlockNumber,
        pub new_parameters: TransactionFeeInfo<BalanceType>,
        pub required_vote_count: u32,
        pub approvers: sp_std::collections::btree_set::BTreeSet<AccountId>,
    }

    /// Transaction Fee Information
    #[derive(Clone, Copy, Default, Eq, PartialEq)]
    #[derive(codec::Decode, codec::Encode, codec::MaxEncodedLen)]
    #[derive(sp_core::RuntimeDebug, scale_info::TypeInfo)]
    pub struct TransactionFeeInfo<BalanceType> {
        pub weight_to_fee_divider: u64,
        pub weight_to_fee_multiplier: u64,
        pub minimum_transaction_fee: BalanceType,
    }

    #[pallet::composite_enum]
    pub enum HoldReason {
        /// Held/Reserved for Council Membership
        #[codec(index = 0)]
        CouncilMembership,
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Fatal error, chain storage compromised
        FatalError,
        /// Elder is undefined
        UndefinedElder,
        /// Restricted call, only for Elder Brother (Sudo)
        SudoOrElderOnly,
        /// Council member only
        CouncilMemberOnly,
        /// Council membership full
        CouncilMembershipFull,
        /// Account has no verified legality
        AccountIsNotVerifiedLegally,
        /// Account has no legal name
        AccountHasNoLegalName,
        /// Account already a member
        AccountAlreadyAMember,
        /// Account is not a member
        AccountIsNotAMember,
        /// Account already an Elder
        AccountAlreadyAnElder,
        /// No proposal exists
        NoProposalExists,
        /// Incorrect proposal
        IncorrectProposal,
        /// Vote already counted
        VoteAlreadyCounted,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Big Brother Registered/Added
        BigBrotherAdded {
            who: T::AccountId,
            by: Option<T::AccountId>,
            hold: BalanceTypeOf<T>,
        },
        /// Big Brother Unregistered/Removed
        BigBrotherRemoved {
            who: T::AccountId,
            by: Option<T::AccountId>,
            release: BalanceTypeOf<T>,
        },
        /// Elder ascended
        ElderAscended { who: T::AccountId },
        /// Elder descended
        ElderDescended { who: T::AccountId },
        /// Token circulation increased
        CirculationIncreased {
            increase: BalanceTypeOf<T>,
            by: Option<T::AccountId>,
        },
        /// Token circulation decreased
        CirculationDecreased {
            decrease: BalanceTypeOf<T>,
            by: Option<T::AccountId>,
        },
        /// Transaction Fee parameters changed
        TxFeeParametersChange {
            old: TransactionFeeInfo<BalanceTypeOf<T>>,
            new: TransactionFeeInfo<BalanceTypeOf<T>>,
        },
        /// Transaction fee parameters proposal rejected
        TxFeeParametersRejected {
            rejected: TransactionFeeInfo<BalanceTypeOf<T>>,
            by: T::AccountId,
        },
        /// New transaction fee parameters proposal
        TxFeeParametersChangeProposed {
            proposal: TransactionFeeInfo<BalanceTypeOf<T>>,
            by: T::AccountId,
        },
        /// Transaction fee parameters proposal vote count
        TxFeeParametersChangeVoted {
            by: T::AccountId,
            remaining_count: u32,
        },
    }

    // endregion

    // region: Helper methods

    impl<T: Config> Pallet<T> {
        /// ensure origin is Elder or Root
        pub fn ensure_elder_or_root(origin: OriginFor<T>) -> Result<(), sp_runtime::DispatchError> {
            Self::ensure_and_get_elder_or_root(origin)?;

            Ok(())
        }

        /// ensure origin is Council Member or Root
        pub fn ensure_council_member_or_root(
            origin: OriginFor<T>,
        ) -> Result<(), sp_runtime::DispatchError> {
            let maybe_signed_caller = ensure_signed_or_root(origin.clone())?;

            if maybe_signed_caller.is_none() {
                return Ok(());
            }

            Self::ensure_and_get_council_member(origin)?;

            Ok(())
        }

        /// ensure an account has legal name filled
        pub fn ensure_account_has_legal_field(
            account: &T::AccountId,
        ) -> Result<(), sp_runtime::DispatchError> {
            let minimum_identity_field = pallet_identity::IdentityField::Legal as u64;
            ensure!(
                pallet_identity::Pallet::<T>::has_identity(&account, minimum_identity_field),
                <Error<T>>::AccountHasNoLegalName,
            );

            Ok(())
        }

        /// ensure has verified legality
        pub fn ensure_account_has_verified_legality(
            account: &T::AccountId,
        ) -> Result<(), sp_runtime::DispatchError> {
            Self::ensure_account_has_legal_field(account)?;
            let account_registration = pallet_identity::Pallet::<T>::identity(&account).unwrap();
            let account_judgements = &account_registration.judgements;

            if account_judgements.is_empty() {
                return Err(<Error<T>>::AccountIsNotVerifiedLegally.into());
            }

            for (_, judgement) in account_judgements {
                let has_bad_judgement = !matches!(
                    judgement,
                    pallet_identity::Judgement::Reasonable | pallet_identity::Judgement::KnownGood
                );

                if has_bad_judgement {
                    return Err(<Error<T>>::AccountIsNotVerifiedLegally.into());
                }
            }

            Ok(())
        }

        /// ensure origin is Elder or Root and return it's account
        pub fn ensure_and_get_elder_or_root(
            origin: OriginFor<T>,
        ) -> Result<Option<T::AccountId>, sp_runtime::DispatchError> {
            let maybe_signed_caller = ensure_signed_or_root(origin)?;

            if maybe_signed_caller.is_none() {
                return Ok(None);
            }

            let signed_caller = maybe_signed_caller.unwrap();

            match Self::elder() {
                | None => Err(<Error<T>>::UndefinedElder.into()),
                | Some(elder) => {
                    if elder.eq(&signed_caller) {
                        Ok(Some(signed_caller))
                    } else {
                        Err(<Error<T>>::SudoOrElderOnly.into())
                    }
                },
            }
        }

        /// ensure origin is Council Member and return it's account
        pub fn ensure_and_get_council_member(
            origin: OriginFor<T>,
        ) -> Result<T::AccountId, sp_runtime::DispatchError> {
            let signed_caller = ensure_signed(origin)?;

            if let Some(elder) = Self::elder() {
                if elder.eq(&signed_caller) {
                    return Ok(signed_caller); // elder is also a member
                }
            }

            if <Members<T>>::get().contains(&signed_caller) {
                Ok(signed_caller)
            } else {
                Err(<Error<T>>::CouncilMemberOnly.into())
            }
        }

        /// (private) try reserve one's balance for membership registration
        fn try_hold_balance_for_membership(
            who: &T::AccountId,
        ) -> Result<(), sp_runtime::DispatchError> {
            let amount = T::RegistrationDepositAmount::get();
            let runtime_hold_reason = HoldReason::CouncilMembership.into();
            <<T as Config>::Currency as frame_support::traits::fungible::MutateHold<
                T::AccountId,
            >>::hold(&runtime_hold_reason, who, amount)?;

            Ok(())
        }

        /// (private) try unreserve one's balance for membership revocation
        fn try_unhold_balance_of_membership(
            who: &T::AccountId,
        ) -> Result<BalanceTypeOf<T>, sp_runtime::DispatchError> {
            let runtime_hold_reason = HoldReason::CouncilMembership.into();
            let amount =
                <<T as Config>::Currency as frame_support::traits::fungible::InspectHold<
                    T::AccountId,
                >>::balance_on_hold(&runtime_hold_reason, who);
            let released_balance =
                <<T as Config>::Currency as frame_support::traits::fungible::MutateHold<
                    T::AccountId,
                >>::release(
                    &runtime_hold_reason,
                    who,
                    amount,
                    frame_support::traits::tokens::Precision::BestEffort,
                )?;

            Ok(released_balance)
        }

        /// (private) mint into default
        fn try_mint_into(
            dest: &T::AccountId,
            amount: BalanceTypeOf<T>,
        ) -> Result<BalanceTypeOf<T>, sp_runtime::DispatchError> {
            <<T as Config>::Currency as frame_support::traits::fungible::Mutate<
                T::AccountId,
            >>::mint_into(dest, amount)
        }

        /// (private) burn from default
        fn try_burn_from_default(
            amount: BalanceTypeOf<T>,
        ) -> Result<BalanceTypeOf<T>, sp_runtime::DispatchError> {
            let who = T::BurnAddress::get();
            let precision = frame_support::traits::tokens::Precision::BestEffort;
            let fortitude = frame_support::traits::tokens::Fortitude::Force;

            <<T as Config>::Currency as frame_support::traits::fungible::Mutate<T::AccountId>>::burn_from(&who, amount, precision, fortitude)
        }

        /// (private) burn all from default
        fn try_burn_all_from_default() -> Result<BalanceTypeOf<T>, sp_runtime::DispatchError> {
            let who = T::BurnAddress::get();
            let precision = frame_support::traits::tokens::Precision::BestEffort;
            let fortitude = frame_support::traits::tokens::Fortitude::Force;
            let amount = <<T as Config>::Currency as frame_support::traits::fungible::Inspect<
                T::AccountId,
            >>::balance(&who);

            <<T as Config>::Currency as frame_support::traits::fungible::Mutate<T::AccountId>>::burn_from(&who, amount, precision, fortitude)
        }

        /// (private) create proposal for tx fee parameter changes
        fn propose_tx_fee_change(
            proposer: T::AccountId,
            new_multiplier: Option<u64>,
            new_divider: Option<u64>,
            new_minimum_fee: Option<BalanceTypeOf<T>>,
        ) -> Result<sp_std::vec::Vec<Event<T>>, sp_runtime::DispatchError> {
            if new_multiplier.is_none() && new_divider.is_none() && new_minimum_fee.is_none() {
                return Err(<Error<T>>::IncorrectProposal.into());
            }

            let mut events = sp_std::vec![];

            if let Some(old_proposal) = <CurrentProposal<T>>::get() {
                events.push(Event::TxFeeParametersRejected {
                    rejected: old_proposal.new_parameters,
                    by: proposer.clone(),
                });
            }

            let TransactionFeeInfo {
                minimum_transaction_fee,
                weight_to_fee_divider,
                weight_to_fee_multiplier,
            } = <TxFeeInfo<T>>::get();
            let new_multiplier = match new_multiplier {
                | None => weight_to_fee_multiplier,
                | Some(value) => value,
            };
            let new_divider = match new_divider {
                | None => weight_to_fee_divider,
                | Some(value) => value,
            };
            let new_minimum_fee = match new_minimum_fee {
                | None => minimum_transaction_fee,
                | Some(value) => value,
            };
            let new_proposal = TransactionFeeChangeProposal {
                approvers: Default::default(),
                initiated_at: <frame_system::Pallet<T>>::block_number(),
                initiator: proposer.clone(),
                required_vote_count: <Members<T>>::get().len() as u32 + 1, // elder also a vote
                new_parameters: TransactionFeeInfo {
                    minimum_transaction_fee: new_minimum_fee,
                    weight_to_fee_divider: new_divider,
                    weight_to_fee_multiplier: new_multiplier,
                },
            };
            events.push(Event::TxFeeParametersChangeProposed {
                proposal: new_proposal.new_parameters.clone(),
                by: proposer,
            });
            <CurrentProposal<T>>::set(Some(new_proposal));

            Ok(events)
        }

        /// (private) try vote proposal
        fn vote_to_proposal(
            votee: T::AccountId,
            is_approving: bool,
        ) -> Result<sp_std::vec::Vec<Event<T>>, sp_runtime::DispatchError> {
            if <CurrentProposal<T>>::get().is_none() {
                return Err(<Error<T>>::NoProposalExists.into());
            }

            let mut events = sp_std::vec![];

            if !is_approving {
                let rejected = <CurrentProposal<T>>::get().take().unwrap().new_parameters;
                <CurrentProposal<T>>::set(None);
                events.push(Event::TxFeeParametersRejected {
                    by: votee,
                    rejected,
                });

                return Ok(events);
            }

            let (remaining_count, parameters) =
                <CurrentProposal<T>>::try_mutate(|current_proposal| {
                    let current_proposal_inner = current_proposal.as_mut().unwrap();

                    if current_proposal_inner.approvers.contains(&votee) {
                        return Err(<Error<T>>::VoteAlreadyCounted);
                    }

                    current_proposal_inner.approvers.insert(votee.clone());
                    let current_vote_count = current_proposal_inner.approvers.len() as u32;
                    let required_vote = current_proposal_inner.required_vote_count;
                    let remaining_count = required_vote.saturating_sub(current_vote_count);
                    events.push(Event::TxFeeParametersChangeVoted {
                        by: votee,
                        remaining_count,
                    });

                    Ok((remaining_count, current_proposal_inner.new_parameters))
                })?;

            if remaining_count == 0 {
                <CurrentProposal<T>>::set(None);
                let multiplier = parameters.weight_to_fee_multiplier;
                let divider = parameters.weight_to_fee_divider;
                let minimum_fee = parameters.minimum_transaction_fee;
                let (old, new) = Self::set_tx_fee_info(multiplier, divider, minimum_fee);
                events.push(Event::TxFeeParametersChange {
                    old,
                    new,
                });
            }

            Ok(events)
        }

        /// (private) set transaction fee parameters
        fn set_tx_fee_info(
            multiplier: u64,
            divider: u64,
            minimum_fee: BalanceTypeOf<T>,
        ) -> (
            TransactionFeeInfo<BalanceTypeOf<T>>,
            TransactionFeeInfo<BalanceTypeOf<T>>,
        ) {
            let old = Self::tx_fee_info();
            let new = TransactionFeeInfo {
                minimum_transaction_fee: minimum_fee,
                weight_to_fee_multiplier: multiplier,
                weight_to_fee_divider: divider,
            };

            <TxFeeInfo<T>>::set(new);

            (old, new)
        }
    }

    // endregion

    // region: Extrinsics

    #[pallet::call]
    /// Dispatchable functions.
    impl<T: Config> Pallet<T> {
        /// Sudo: Replace Elder
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn su_elder_replace(
            origin: OriginFor<T>,
            new_elder: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            let previous_elder = <Elder<T>>::try_mutate(|inner| {
                if let Some(previous_elder) = inner.replace(new_elder.clone()) {
                    if previous_elder.eq(&new_elder) {
                        Err(<Error<T>>::AccountAlreadyAnElder)
                    } else {
                        Ok(Some(previous_elder))
                    }
                } else {
                    Ok(None)
                }
            })?;

            if let Some(descended_elder) = previous_elder {
                Self::deposit_event(Event::ElderDescended {
                    who: descended_elder,
                });
            }

            Self::deposit_event(Event::ElderAscended {
                who: new_elder,
            });

            Ok(Pays::No.into())
        }

        /// Sudo or Elder: Add Council Member
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn se_membership_add(
            origin: OriginFor<T>,
            new_council_member: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let by = Self::ensure_and_get_elder_or_root(origin)?;
            Self::ensure_account_has_verified_legality(&new_council_member)?;
            ensure!(
                !<Members<T>>::get().contains(&new_council_member),
                <Error<T>>::AccountAlreadyAMember
            );
            ensure!(
                <Members<T>>::get().len() <= (T::MaxMembers::get() as usize),
                <Error<T>>::CouncilMembershipFull
            );

            if let Some(elder) = <Elder<T>>::get() {
                ensure!(
                    new_council_member != elder,
                    <Error<T>>::AccountAlreadyAMember,
                );
            }

            <Members<T>>::try_mutate(|inner| {
                Self::try_hold_balance_for_membership(&new_council_member)?;
                let _ = inner.try_insert(new_council_member.clone());

                Result::<(), sp_runtime::DispatchError>::Ok(())
            })?;

            Self::deposit_event(Event::BigBrotherAdded {
                who: new_council_member,
                by,
                hold: T::RegistrationDepositAmount::get(),
            });

            Ok(Pays::Yes.into())
        }

        /// Sudo or Elder: Remove Council Member
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn se_membership_remove(
            origin: OriginFor<T>,
            council_member: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let by = Self::ensure_and_get_elder_or_root(origin)?;
            ensure!(
                <Members<T>>::get().contains(&council_member),
                <Error<T>>::AccountIsNotAMember
            );

            let release = <Members<T>>::try_mutate(|inner| {
                let released_balance = Self::try_unhold_balance_of_membership(&council_member)?;
                inner.remove(&council_member);

                Result::<BalanceTypeOf<T>, sp_runtime::DispatchError>::Ok(released_balance)
            })?;

            Self::deposit_event(Event::BigBrotherRemoved {
                who: council_member,
                by,
                release,
            });

            Ok(Pays::Yes.into())
        }

        /// Sudo or Elder: Mint token
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn se_currency_mint_into(
            origin: OriginFor<T>,
            dest: T::AccountId,
            amount: BalanceTypeOf<T>,
        ) -> DispatchResultWithPostInfo {
            let by = Self::ensure_and_get_elder_or_root(origin)?;
            let increase = Self::try_mint_into(&dest, amount)?;

            Self::deposit_event(Event::CirculationIncreased {
                increase,
                by,
            });

            Ok(Pays::No.into())
        }

        /// Sudo or Elder: Burn token
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn se_currency_burn(
            origin: OriginFor<T>,
            amount: BalanceTypeOf<T>,
        ) -> DispatchResultWithPostInfo {
            let by = Self::ensure_and_get_elder_or_root(origin)?;
            let decrease = Self::try_burn_from_default(amount)?;

            Self::deposit_event(Event::CirculationDecreased {
                decrease,
                by,
            });

            Ok(Pays::No.into())
        }

        /// Sudo or Elder: Burn all token
        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn se_currency_burn_all(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let by = Self::ensure_and_get_elder_or_root(origin)?;
            let decrease = Self::try_burn_all_from_default()?;

            Self::deposit_event(Event::CirculationDecreased {
                decrease,
                by,
            });

            Ok(Pays::No.into())
        }

        /// Council Member: Propose new transaction fee parameters, replace with
        /// rejection if a proposal already in place
        #[pallet::call_index(6)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn cm_proposal_new(
            origin: OriginFor<T>,
            new_multiplier: Option<u64>,
            new_divider: Option<u64>,
            new_minimum_fee: Option<BalanceTypeOf<T>>,
        ) -> DispatchResultWithPostInfo {
            let proposer = Self::ensure_and_get_council_member(origin)?;
            let events = Self::propose_tx_fee_change(
                proposer,
                new_multiplier,
                new_divider,
                new_minimum_fee,
            )?;

            for event in events {
                Self::deposit_event(event);
            }

            Ok(Pays::Yes.into())
        }

        /// Council Member: Vote to a proposal if exists, this will immediately
        /// execute change if the proposal met the required vote count, or
        /// immediately rejects it
        #[pallet::call_index(7)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn cm_proposal_vote(
            origin: OriginFor<T>,
            is_approving: bool,
        ) -> DispatchResultWithPostInfo {
            let votee = Self::ensure_and_get_council_member(origin)?;
            let events = Self::vote_to_proposal(votee, is_approving)?;

            for event in events {
                Self::deposit_event(event);
            }

            Ok(Pays::Yes.into())
        }
    }

    // endregion
}
