//! Frequency CLI library.
use std::{path::PathBuf, str::FromStr};

use crate::{ExportMetadataCmd, ExportRuntimeVersionCmd};

/// Sub-commands supported by the collator.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Export metadata.
	ExportMetadata(ExportMetadataCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// Remove the whole chain.
	PurgeChain(cumulus_client_cli::PurgeChainCmd),

	/// Export the genesis state of the parachain.
	ExportGenesisState(cumulus_client_cli::ExportGenesisStateCommand),

	/// Export the genesis wasm of the parachain.
	ExportGenesisWasm(cumulus_client_cli::ExportGenesisWasmCommand),

	/// Sub-commands concerned with benchmarking.
	/// The pallet benchmarking moved to the `pallet` sub-command.
	#[clap(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Try some testing command against a specified runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Get current runtime spec version.
	ExportRuntimeVersion(ExportRuntimeVersionCmd),
}

/// Block authoring scheme to be used by the dev service.
#[derive(Debug, Copy, Clone)]
pub enum SealingMode {
	/// Author a block immediately upon receiving a transaction into the transaction pool
	Instant,
	/// Author a block upon receiving an RPC command
	Manual,
	/// Author blocks at a regular interval specified in seconds
	Interval,
}

impl FromStr for SealingMode {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"instant" => Ok(Self::Instant),
			"manual" => Ok(Self::Manual),
			"interval" => Ok(Self::Interval),
			_ => Err(String::from("")),
		}
	}
}
#[derive(Debug, clap::Parser)]
#[clap(
	propagate_version = true,
	args_conflicts_with_subcommands = true,
	subcommand_negates_reqs = true
)]
pub struct Cli {
	#[clap(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[clap(flatten)]
	pub run: cumulus_client_cli::RunCmd,

	/// Disable automatic hardware benchmarks.
	///
	/// By default these benchmarks are automatically ran at startup and measure
	/// the CPU speed, the memory bandwidth and the disk speed.
	///
	/// The results are then printed out in the logs, and also sent as part of
	/// telemetry, if telemetry is enabled.
	#[clap(long)]
	pub no_hardware_benchmarks: bool,

	/// Relay chain arguments
	#[clap(raw = true)]
	pub relay_chain_args: Vec<String>,

	/// Instant block sealing
	/// Blocks are triggered to be formed each time a transaction hits the validated transaction pool
	/// Empty blocks can also be formed using the `engine_createBlock` RPC
	#[cfg(feature = "frequency-no-relay")]
	#[clap(long, default_value = "instant")]
	pub sealing: SealingMode,

	/// Interval in seconds for interval sealing.
	#[cfg(feature = "frequency-no-relay")]
	#[clap(long, default_value = "120")]
	pub sealing_interval: u16,

	/// Whether to create empty blocks in manual and interval sealing modes.
	#[cfg(feature = "frequency-no-relay")]
	#[clap(long)]
	pub create_empty_blocks: bool,
}

#[derive(Debug)]
pub struct RelayChainCli {
	/// The actual relay chain cli object.
	pub base: polkadot_cli::RunCmd,

	/// Optional chain id that should be passed to the relay chain.
	pub chain_id: Option<String>,

	/// The base path that should be used by the relay chain.
	pub base_path: Option<PathBuf>,
}

impl RelayChainCli {
	/// Parse the relay chain CLI parameters using the para chain `Configuration`.
	pub fn new<'a>(
		para_config: &sc_service::Configuration,
		relay_chain_args: impl Iterator<Item = &'a String>,
	) -> Self {
		let extension =
			frequency_service::chain_spec::Extensions::try_get(&*para_config.chain_spec);
		let chain_id = extension.map(|e| e.relay_chain.clone());
		let base_path = para_config.base_path.as_ref().map(|x| x.path().join("polkadot"));
		Self { base_path, chain_id, base: clap::Parser::parse_from(relay_chain_args) }
	}
}
