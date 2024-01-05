mod chainspec;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
    sp_core::crypto::set_default_ss58_version(ss58_registry::Ss58AddressFormatRegistry::NagaraAccount.into());

    command::run()
}
