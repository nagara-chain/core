impl nagara_pda_files::Config for crate::Runtime {
    type BigBrotherDownloadFeeDistribution = crate::BigBrotherDownloadFeeDistribution;
    type Currency = crate::Balances;
    type MinDownloadFeePerByte = MinDownloadFeePerByte<crate::Balance>;
    type RoyaltyFeeDistribution = crate::RoyaltyFeeDistribution;
    type RuntimeEvent = crate::RuntimeEvent;
    type ServicerUploadFeeDistribution = crate::ServicerUploadFeeDistribution;
    type StorageFeePerBytePerPeriod = StorageFeePerBytePerPeriod<crate::Balance>;
    type StoragePeriod = crate::ConstU32<{ crate::constants::STORAGE_PERIOD }>;
    type UploadFeePerByte = UploadFeePerByte<crate::Balance>;
}

pub struct MinDownloadFeePerByte<T>(sp_std::marker::PhantomData<T>);
pub struct StorageFeePerBytePerPeriod<T>(sp_std::marker::PhantomData<T>);
pub struct UploadFeePerByte<T>(sp_std::marker::PhantomData<T>);

impl<T> nagara_pda_files::FeeFromBytes for MinDownloadFeePerByte<T>
where
    T: sp_arithmetic::traits::BaseArithmetic
        + core::convert::From<u64>
        + core::convert::From<u32>
        + core::marker::Copy
        + sp_arithmetic::traits::Unsigned,
{
    type Balance = T;

    fn bytes_to_fee(size: u64) -> Self::Balance {
        let size = <u32 as sp_arithmetic::traits::SaturatedConversion>::saturated_from(size);
        let fee = crate::constants::get_fee_divided(
            1,
            size,
            crate::constants::DOWNLOAD_FEE_DIVIDER,
            crate::constants::DOWNLOAD_FEE_MINIMUM,
        );

        <Self::Balance as sp_arithmetic::traits::SaturatedConversion>::saturated_from(fee)
    }
}

impl<T> nagara_pda_files::FeeFromBytes for StorageFeePerBytePerPeriod<T>
where
    T: sp_arithmetic::traits::BaseArithmetic
        + core::convert::From<u64>
        + core::convert::From<u32>
        + core::marker::Copy
        + sp_arithmetic::traits::Unsigned,
{
    type Balance = T;

    fn bytes_to_fee(size: u64) -> Self::Balance {
        let size = <u32 as sp_arithmetic::traits::SaturatedConversion>::saturated_from(size);
        let fee = crate::constants::get_fee_divided(
            1,
            size,
            crate::constants::STORAGE_FEE_DIVIDER,
            crate::constants::STORAGE_FEE_MINIMUM,
        );

        <Self::Balance as sp_arithmetic::traits::SaturatedConversion>::saturated_from(fee)
    }
}

impl<T> nagara_pda_files::FeeFromBytes for UploadFeePerByte<T>
where
    T: sp_arithmetic::traits::BaseArithmetic
        + core::convert::From<u64>
        + core::convert::From<u32>
        + core::marker::Copy
        + sp_arithmetic::traits::Unsigned,
{
    type Balance = T;

    fn bytes_to_fee(size: u64) -> Self::Balance {
        let size = <u32 as sp_arithmetic::traits::SaturatedConversion>::saturated_from(size);
        let fee = crate::constants::get_fee_divided(
            1,
            size,
            crate::constants::UPLOAD_FEE_DIVIDER,
            crate::constants::UPLOAD_FEE_MINIMUM,
        );

        <Self::Balance as sp_arithmetic::traits::SaturatedConversion>::saturated_from(fee)
    }
}
