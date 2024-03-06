#![cfg_attr(not(feature = "std"), no_std)]

pub use nagara_council_bigbrothers as ngr_bbcm;
pub use nagara_registry_servicers as ngr_svrg;
pub use pallet::*;

pub type AccountTypeOf<T> = <T as frame_system::Config>::AccountId;
pub type AttesterId = sp_core::ed25519::Public;
pub type BalanceCurrencyTypeOf<T> =
    <<T as Config>::Currency as frame_support::traits::Currency<AccountTypeOf<T>>>::Balance;
pub type BlockNumber<T> = frame_system::pallet_prelude::BlockNumberFor<T>;
pub type FileHash = [u8; 32];
pub type UniqueMap<K, V> = sp_std::collections::btree_map::BTreeMap<K, V>;

pub const PALLET_IDENTIFICATION: frame_support::PalletId = frame_support::PalletId(*b"ngr/pdaf");

pub trait FeeFromBytes {
    /// The type that is returned as result from calculation.
    type Balance: sp_arithmetic::traits::BaseArithmetic
        + From<u64>
        + Copy
        + sp_arithmetic::traits::Unsigned;

    /// Calculates the fee from the byte size.
    fn bytes_to_fee(size: u64) -> Self::Balance;
}

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
    pub trait Config: frame_system::Config + ngr_bbcm::Config + ngr_svrg::Config {
        /// Runtime Event registrar
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Currency for pallet operations
        type Currency: frame_support::traits::Currency<Self::AccountId>
            + frame_support::traits::fungible::Mutate<Self::AccountId>
            + frame_support::traits::fungible::Inspect<Self::AccountId>;
        /// Upload fee per byte
        type UploadFeePerByte: FeeFromBytes<Balance = BalanceCurrencyTypeOf<Self>>;
        /// Download fee per byte
        type DownloadFeePerByte: FeeFromBytes<Balance = BalanceCurrencyTypeOf<Self>>;
        /// Storage fee per byte per period
        type StorageFeePerBytePerPeriod: FeeFromBytes<Balance = BalanceCurrencyTypeOf<Self>>;
        /// Storage period
        #[pallet::constant]
        type StoragePeriod: sp_core::Get<BlockNumber<Self>>;
        /// Servicer portion of upload fee
        #[pallet::constant]
        type ServicerUploadFeeDistribution: sp_core::Get<sp_runtime::Percent>;
        /// Big brother download fee distribution
        #[pallet::constant]
        type BigBrotherDownloadFeeDistribution: sp_core::Get<sp_runtime::Percent>;
        /// Initial owner + big brother transfer fee distribution
        #[pallet::constant]
        type RoyaltyFeeDistribution: sp_core::Get<sp_runtime::Percent>;
    }

    // endregion

    // region: Storage

    #[pallet::storage]
    #[pallet::getter(fn files)]
    pub(super) type Files<T: Config> = StorageMap<
        _,
        frame_support::Blake2_128Concat,
        T::AccountId,
        FileInformation<T::AccountId, BalanceCurrencyTypeOf<T>>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn hashes)]
    pub(super) type Hashes<T: Config> =
        StorageMap<_, frame_support::Blake2_128Concat, FileHash, T::AccountId>;

    // endregion

    // region: Genesis

    // endregion

    // region: Custom, Event, and Errors type

    /// File Information
    #[derive(Clone, Default, Eq, PartialEq)]
    #[derive(codec::Decode, codec::Encode, codec::MaxEncodedLen)]
    #[derive(sp_core::RuntimeDebug, scale_info::TypeInfo)]
    pub struct FileInformation<AccountId, TransferFee> {
        pub hash: FileHash,
        pub uploader: AccountId,
        pub big_brother: AccountId,
        pub servicer: AccountId,
        pub owner: AccountId,
        pub transfer_fee: TransferFee,
        pub size: u64,
        pub free_for_all: bool,
    }

    #[derive(Clone, Default, Eq, PartialEq)]
    #[derive(codec::Decode, codec::Encode, codec::MaxEncodedLen)]
    #[derive(sp_core::RuntimeDebug, scale_info::TypeInfo)]
    pub struct FileInformationArgs<AccountId, TransferFee> {
        pub hash: FileHash,
        pub uploader: AccountId,
        pub big_brother: AccountId,
        pub servicer: AccountId,
        pub transfer_fee: TransferFee,
        pub size: u64,
        pub free_for_all: bool,
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Fatal error, chain storage compromised
        FatalError,
        /// File not found
        FileNotFound,
        /// File already exist
        FileAlreadyExist,
        /// Ownership transfer fee must greater than zero
        OwnershipTransferFeeMustNotZero,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// File succesfully uploaded
        FileUploaded { file: AccountTypeOf<T>, length: u64 },
        /// Upload fee paid
        UploadFeePaid {
            by: AccountTypeOf<T>,
            amount: BalanceCurrencyTypeOf<T>,
            file: AccountTypeOf<T>,
        },
        /// Upload fee distributed
        UploadFeeDistributed {
            to: AccountTypeOf<T>,
            amount: BalanceCurrencyTypeOf<T>,
            file: AccountTypeOf<T>,
        },
        /// Download fee paid
        DownloadFeePaid {
            by: AccountTypeOf<T>,
            amount: BalanceCurrencyTypeOf<T>,
            file: AccountTypeOf<T>,
        },
        /// Download fee distributed
        DownloadFeeDistributed {
            to: AccountTypeOf<T>,
            amount: BalanceCurrencyTypeOf<T>,
            file: AccountTypeOf<T>,
        },
        /// Ownership transfer fee paid
        OwnershipTransferFeePaid {
            by: AccountTypeOf<T>,
            amount: BalanceCurrencyTypeOf<T>,
            file: AccountTypeOf<T>,
        },
        /// Ownership transfer fee distributed
        OwnershipTransferFeeDistributed {
            to: AccountTypeOf<T>,
            amount: BalanceCurrencyTypeOf<T>,
            file: AccountTypeOf<T>,
        },
        /// File ownership transferred
        FileOwnershipTransferred {
            from: AccountTypeOf<T>,
            to: AccountTypeOf<T>,
            file: AccountTypeOf<T>,
        },
        /// Insufficient amount to keep file existence, file will be deleted
        InsufficientAmountForKeepingFile { file: AccountTypeOf<T> },
        StorageFeePaid {
            file: AccountTypeOf<T>,
            amount: BalanceCurrencyTypeOf<T>,
        },
        /// Storage fee distributed
        StorageFeeDistributed {
            file: AccountTypeOf<T>,
            to: AccountTypeOf<T>,
            amount: BalanceCurrencyTypeOf<T>,
        },
    }

    // endregion

    // region: Helper methods

    impl<T: Config> Pallet<T> {
        fn delete_file(file: &T::AccountId) -> Result<(), sp_runtime::DispatchError> {
            if !<Files<T>>::contains_key(file) {
                return Err(<Error<T>>::FileNotFound.into());
            }

            let FileInformation {
                hash, ..
            } = Self::files(file).unwrap();
            <Files<T>>::remove(file);
            <Hashes<T>>::remove(&hash);

            Self::deposit_event(Event::InsufficientAmountForKeepingFile {
                file: file.clone(),
            });

            Ok(())
        }

        fn distribute_storage_fee(file: &T::AccountId) -> Result<(), sp_runtime::DispatchError> {
            if !<Files<T>>::contains_key(file) {
                return Err(<Error<T>>::FileNotFound.into());
            }

            let FileInformation {
                big_brother,
                servicer,
                size,
                ..
            } = Self::files(file).unwrap();
            let withdraw_reason = frame_support::traits::tokens::WithdrawReasons::FEE;
            let total_fee = T::StorageFeePerBytePerPeriod::bytes_to_fee(size);
            let divider =
                <BalanceCurrencyTypeOf<T> as sp_runtime::traits::SaturatedConversion>::saturated_from(2u32);
            let half_fee =
                sp_runtime::traits::CheckedDiv::checked_div(&total_fee, &divider).unwrap();
            let maybe_success = <<T as Config>::Currency as frame_support::traits::Currency<
                T::AccountId,
            >>::withdraw(
                file,
                total_fee,
                withdraw_reason,
                frame_support::traits::tokens::ExistenceRequirement::KeepAlive,
            );

            if maybe_success.is_err() {
                Self::delete_file(file)?;

                return Ok(());
            }

            Self::deposit_event(Event::StorageFeePaid {
                file: file.clone(),
                amount: total_fee,
            });

            let _ = <<T as Config>::Currency as frame_support::traits::Currency<T::AccountId>>::deposit_creating(&big_brother, half_fee);
            Self::deposit_event(Event::StorageFeeDistributed {
                file: file.clone(),
                to: big_brother,
                amount: half_fee,
            });
            let _ = <<T as Config>::Currency as frame_support::traits::Currency<T::AccountId>>::deposit_creating(&servicer, half_fee);
            Self::deposit_event(Event::StorageFeeDistributed {
                file: file.clone(),
                to: servicer,
                amount: half_fee,
            });

            Ok(())
        }

        fn ownership_transfer(
            file: &T::AccountId,
            beneficiary: T::AccountId,
        ) -> Result<(), sp_runtime::DispatchError> {
            if !<Files<T>>::contains_key(file) {
                return Err(<Error<T>>::FileNotFound.into());
            }

            let FileInformation {
                uploader,
                transfer_fee,
                big_brother,
                owner,
                ..
            } = Self::files(file).unwrap();

            let withdraw_reason = frame_support::traits::tokens::WithdrawReasons::FEE;
            let divider =
                <BalanceCurrencyTypeOf<T> as sp_runtime::traits::SaturatedConversion>::saturated_from(2u32);
            let royalty_part = T::RoyaltyFeeDistribution::get();
            let royalty_part_amount = royalty_part.mul_floor(transfer_fee);
            let owner_part_amount = transfer_fee - royalty_part_amount;
            let half_royalty =
                sp_runtime::traits::CheckedDiv::checked_div(&royalty_part_amount, &divider)
                    .unwrap();

            <Files<T>>::try_mutate(&file, |mutable_file| {
                let _ = <<T as Config>::Currency as frame_support::traits::Currency<
                    T::AccountId,
                >>::withdraw(
                    &beneficiary,
                    transfer_fee,
                    withdraw_reason,
                    frame_support::traits::tokens::ExistenceRequirement::KeepAlive,
                )?;

                Self::deposit_event(Event::OwnershipTransferFeePaid {
                    by: beneficiary.clone(),
                    file: file.clone(),
                    amount: transfer_fee,
                });
                let _ = <<T as Config>::Currency as frame_support::traits::Currency<
                    T::AccountId,
                >>::deposit_creating(&uploader, half_royalty);
                Self::deposit_event(Event::OwnershipTransferFeeDistributed {
                    file: file.clone(),
                    to: uploader,
                    amount: half_royalty,
                });
                let _ = <<T as Config>::Currency as frame_support::traits::Currency<
                    T::AccountId,
                >>::deposit_creating(&big_brother, half_royalty);
                Self::deposit_event(Event::OwnershipTransferFeeDistributed {
                    file: file.clone(),
                    to: big_brother,
                    amount: half_royalty,
                });
                let _ = <<T as Config>::Currency as frame_support::traits::Currency<
                    T::AccountId,
                >>::deposit_creating(&owner, owner_part_amount);
                Self::deposit_event(Event::OwnershipTransferFeeDistributed {
                    file: file.clone(),
                    to: owner.clone(),
                    amount: owner_part_amount,
                });
                let mutable_file = mutable_file.as_mut().unwrap();
                mutable_file.owner = beneficiary.clone();
                Self::deposit_event(Event::FileOwnershipTransferred {
                    file: file.clone(),
                    to: beneficiary,
                    from: owner,
                });

                Result::<(), sp_runtime::DispatchError>::Ok(())
            })?;

            Ok(())
        }

        fn upload_file(
            file: T::AccountId,
            args: FileInformationArgs<T::AccountId, BalanceCurrencyTypeOf<T>>,
        ) -> Result<(), sp_runtime::DispatchError> {
            if <Files<T>>::contains_key(&file) {
                return Err(<Error<T>>::FileAlreadyExist.into());
            }

            if <Hashes<T>>::contains_key(&args.hash) {
                return Err(<Error<T>>::FileAlreadyExist.into());
            }

            let zero = <BalanceCurrencyTypeOf<T> as sp_runtime::traits::SaturatedConversion>::saturated_from(0u32);

            if args.transfer_fee == zero {
                return Err(<Error<T>>::OwnershipTransferFeeMustNotZero.into());
            }

            let file_info = FileInformation {
                hash: args.hash.clone(),
                uploader: args.uploader.clone(),
                big_brother: args.big_brother.clone(),
                servicer: args.servicer.clone(),
                owner: args.uploader.clone(),
                transfer_fee: args.transfer_fee,
                size: args.size,
                free_for_all: args.free_for_all,
            };

            let withdraw_reason = frame_support::traits::tokens::WithdrawReasons::FEE;
            let total_fee = T::UploadFeePerByte::bytes_to_fee(args.size);
            let servicer_part = T::ServicerUploadFeeDistribution::get();
            let servicer_part_amount = servicer_part.mul_floor(total_fee);
            let bb_part_amount = total_fee - servicer_part_amount;
            let _ = <<T as Config>::Currency as frame_support::traits::Currency<T::AccountId>>::withdraw(
                &args.uploader,
                total_fee,
                withdraw_reason,
                frame_support::traits::tokens::ExistenceRequirement::KeepAlive,
            )?;
            Self::deposit_event(Event::UploadFeePaid {
                by: args.uploader.clone(),
                file: file.clone(),
                amount: total_fee,
            });

            <Files<T>>::insert(file.clone(), file_info);
            <Hashes<T>>::insert(args.hash, file.clone());

            Self::deposit_event(Event::FileUploaded {
                file: file.clone(),
                length: args.size,
            });

            let _ = <<T as Config>::Currency as frame_support::traits::Currency<T::AccountId>>::deposit_creating(&args.big_brother, bb_part_amount);
            Self::deposit_event(Event::UploadFeeDistributed {
                file: file.clone(),
                to: args.big_brother,
                amount: bb_part_amount,
            });
            let _ = <<T as Config>::Currency as frame_support::traits::Currency<T::AccountId>>::deposit_creating(&args.servicer, servicer_part_amount);
            Self::deposit_event(Event::UploadFeeDistributed {
                file,
                to: args.servicer,
                amount: servicer_part_amount,
            });

            Ok(())
        }

        fn download_file(
            file: &T::AccountId,
            downloader: &T::AccountId,
        ) -> Result<(), sp_runtime::DispatchError> {
            if !<Files<T>>::contains_key(file) {
                return Err(<Error<T>>::FileNotFound.into());
            }

            let FileInformation {
                big_brother,
                servicer,
                size,
                owner,
                free_for_all,
                ..
            } = Self::files(file).unwrap();

            if free_for_all {
                return Ok(());
            }

            let withdraw_reason = frame_support::traits::tokens::WithdrawReasons::FEE;
            let divider =
                <BalanceCurrencyTypeOf<T> as sp_runtime::traits::SaturatedConversion>::saturated_from(2u32);
            let total_fee = T::DownloadFeePerByte::bytes_to_fee(size);
            let bb_part = T::BigBrotherDownloadFeeDistribution::get();
            let bb_part_amount = bb_part.mul_floor(total_fee);
            let owner_part_amount = total_fee - bb_part_amount;
            let half_bb_fee =
                sp_runtime::traits::CheckedDiv::checked_div(&bb_part_amount, &divider).unwrap();
            let _ = <<T as Config>::Currency as frame_support::traits::Currency<
                T::AccountId,
            >>::withdraw(
                downloader,
                total_fee,
                withdraw_reason,
                frame_support::traits::tokens::ExistenceRequirement::KeepAlive,
            )?;
            Self::deposit_event(Event::DownloadFeePaid {
                by: downloader.clone(),
                file: file.clone(),
                amount: total_fee,
            });
            let _ = <<T as Config>::Currency as frame_support::traits::Currency<T::AccountId>>::deposit_creating(&owner, owner_part_amount);
            Self::deposit_event(Event::DownloadFeeDistributed {
                file: file.clone(),
                to: owner,
                amount: owner_part_amount,
            });
            let _ = <<T as Config>::Currency as frame_support::traits::Currency<T::AccountId>>::deposit_creating(&big_brother, half_bb_fee);
            Self::deposit_event(Event::DownloadFeeDistributed {
                file: file.clone(),
                to: big_brother,
                amount: half_bb_fee,
            });
            let _ = <<T as Config>::Currency as frame_support::traits::Currency<T::AccountId>>::deposit_creating(&servicer, half_bb_fee);
            Self::deposit_event(Event::DownloadFeeDistributed {
                file: file.clone(),
                to: servicer,
                amount: half_bb_fee,
            });

            Ok(())
        }
    }

    // endregion

    // region: Extrinsics

    #[pallet::call]
    /// Dispatchable functions.
    impl<T: Config> Pallet<T> {
        /// Servicer: upload a file
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn servicer_upload(
            origin: OriginFor<T>,
            file: T::AccountId,
            args: FileInformationArgs<T::AccountId, BalanceCurrencyTypeOf<T>>,
        ) -> DispatchResultWithPostInfo {
            ensure_signed_or_root(origin)?;
            Self::upload_file(file, args)?;

            Ok(Pays::No.into())
        }

        /// Servicer: download a file
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn servicer_download(
            origin: OriginFor<T>,
            file: T::AccountId,
            downloader: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_signed_or_root(origin)?;
            Self::download_file(&file, &downloader)?;

            Ok(Pays::No.into())
        }

        /// Servicer: take storage fee
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn servicer_take_storage_fee(
            origin: OriginFor<T>,
            file: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_signed_or_root(origin)?;
            Self::distribute_storage_fee(&file)?;

            Ok(Pays::No.into())
        }

        /// Any: buy file
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(0, 8192))] // TODO: please benchmark
        pub fn any_buy_file(
            origin: OriginFor<T>,
            file: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let beneficiary = ensure_signed(origin)?;
            Self::ownership_transfer(&file, beneficiary)?;

            Ok(Pays::No.into())
        }
    }

    // endregion
}
