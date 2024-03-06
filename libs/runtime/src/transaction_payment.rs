impl pallet_transaction_payment::Config for crate::Runtime {
    type FeeMultiplierUpdate = pallet_transaction_payment::ConstFeeMultiplier<crate::FeeMultiplier>;
    type LengthToFee = NormalizedLengthToFee<crate::Balance>;
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<crate::Balances, ()>;
    type OperationalFeeMultiplier =
        crate::ConstU8<{ crate::constants::OPERATIONAL_FEE_MULTIPLIER }>;
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
        let nagara_council_bigbrothers::TransactionFeeInfo::<u128> {
            weight_to_fee_divider,
            weight_to_fee_multiplier,
            minimum_transaction_fee,
        } = crate::BigBrotherCouncil::tx_fee_info();

        let mut normalized_ref_time = (weight
            .ref_time()
            .saturating_mul(weight_to_fee_multiplier)
            .saturating_div(weight_to_fee_divider))
        .into();

        if normalized_ref_time < minimum_transaction_fee {
            normalized_ref_time = minimum_transaction_fee;
        }

        <Self::Balance as sp_arithmetic::traits::SaturatedConversion>::saturated_from(
            normalized_ref_time,
        )
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
            sp_arithmetic::traits::SaturatedConversion::saturated_into(weight.ref_time());
        let length_fee = crate::constants::get_fee(1, saturated_weight);

        <Self::Balance as sp_arithmetic::traits::SaturatedConversion>::saturated_from(length_fee)
    }
}
