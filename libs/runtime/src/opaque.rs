use sp_std::vec::Vec; // needed by `impl_opaque_keys` macro

pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type BlockId = sp_runtime::generic::BlockId<Block>;
pub type Header = sp_runtime::generic::Header<crate::BlockNumber, sp_runtime::traits::BlakeTwo256>;
pub type UncheckedExtrinsic = sp_runtime::OpaqueExtrinsic;

sp_runtime::impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: crate::Aura,
        pub grandpa: crate::Grandpa,
    }
}
