impl pallet_transaction_payment::Config for crate::Runtime {
    type FeeMultiplierUpdate = pallet_transaction_payment::ConstFeeMultiplier<crate::FeeMultiplier>;
    type LengthToFee = NormalizedLengthToFee<crate::Balance>;
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<crate::Balances, ()>;
    type OperationalFeeMultiplier = crate::ConstU8<{ crate::constants::OPERATIONAL_FEE_MULTIPLIER }>;
    type RuntimeEvent = crate::RuntimeEvent;
    type WeightToFee = NormalizedWeightToFee<crate::Balance>;
}

pub struct NormalizedWeightToFee<T>(sp_std::marker::PhantomData<T>);

impl<T> frame_support::weights::WeightToFee for NormalizedWeightToFee<T>
where
    T: sp_arithmetic::traits::BaseArithmetic
        + core::convert::From<u32>
        + core::marker::Copy
        + sp_arithmetic::traits::Unsigned,
{
    type Balance = T;

    fn weight_to_fee(weight: &frame_support::weights::Weight) -> Self::Balance {
        let mut normalized_ref_time = (weight
            .ref_time()
            .saturating_div(crate::constants::REF_TIME_GAS_FEE_DIVIDER))
        .into();

        if normalized_ref_time == 0 {
            normalized_ref_time = crate::constants::MIN_GAS_FEE;
        }

        <Self::Balance as sp_arithmetic::traits::SaturatedConversion>::saturated_from(normalized_ref_time)
    }
}

pub struct NormalizedLengthToFee<T>(sp_std::marker::PhantomData<T>);

impl<T> frame_support::weights::WeightToFee for NormalizedLengthToFee<T>
where
    T: sp_arithmetic::traits::BaseArithmetic
        + core::convert::From<u32>
        + core::marker::Copy
        + sp_arithmetic::traits::Unsigned,
{
    type Balance = T;

    fn weight_to_fee(weight: &frame_support::weights::Weight) -> Self::Balance {
        let saturated_weight =
            sp_arithmetic::traits::SaturatedConversion::saturated_into(weight.proof_size());
        let length_fee = crate::constants::get_fee(1, saturated_weight);

        <Self::Balance as sp_arithmetic::traits::SaturatedConversion>::saturated_from(length_fee)
    }
}
