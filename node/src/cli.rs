// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use crate::chain_spec;
use sc_cli;
use std::path::PathBuf;
use structopt::StructOpt;

/// Sub-commands supported by the collator.
#[derive(Debug, StructOpt)]
pub enum Subcommand {
	/// Export the genesis state of the parachain.
	#[structopt(name = "export-genesis-state")]
	ExportGenesisState(ExportGenesisStateCommand),

	/// Export the genesis wasm of the parachain.
	#[structopt(name = "export-genesis-wasm")]
	ExportGenesisWasm(ExportGenesisWasmCommand),

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

	/// Remove the whole chain.
	PurgeChain(cumulus_client_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// The custom benchmark subcommmand benchmarking runtime pallets.
	#[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}



// benchmark --chain=local --steps=100 --repeat=200 --pallet=kylin-oracle --extrinsic=* --execution=wasm --wasm-execution=compiled --heap-pages=4096 --output=pallets/example/weights.rs
/// Command for exporting the genesis state of the parachain
#[derive(Debug, StructOpt)]
pub struct ExportGenesisStateCommand {
	/// Output file name or stdout if unspecified.
	#[structopt(parse(from_os_str))]
	pub output: Option<PathBuf>,

	/// Id of the parachain this state is for.
	///
	/// Default: 100
	#[structopt(long)]
	pub parachain_id: Option<u32>,

	/// Write output in binary. Default is to write in hex.
	#[structopt(short, long)]
	pub raw: bool,

	/// The name of the chain for that the genesis state should be exported.
	#[structopt(long)]
	pub chain: Option<String>,
}

/// Command for exporting the genesis wasm file.
#[derive(Debug, StructOpt)]
pub struct ExportGenesisWasmCommand {
	/// Output file name or stdout if unspecified.
	#[structopt(parse(from_os_str))]
	pub output: Option<PathBuf>,

	/// Write output in binary. Default is to write in hex.
	#[structopt(short, long)]
	pub raw: bool,

	/// The name of the chain for that the genesis wasm file should be exported.
	#[structopt(long)]
	pub chain: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(settings = &[
structopt::clap::AppSettings::GlobalVersion,
structopt::clap::AppSettings::ArgsNegateSubcommands,
structopt::clap::AppSettings::SubcommandsNegateReqs,
])]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: cumulus_client_cli::RunCmd,

	/// Relaychain arguments
	#[structopt(raw = true)]
	pub relaychain_args: Vec<String>,
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
		let extension = chain_spec::Extensions::try_get(&*para_config.chain_spec);
		let chain_id = extension.map(|e| e.relay_chain.clone());
		let base_path = para_config
			.base_path
			.as_ref()
			.map(|x| x.path().join("polkadot"));
		Self {
			base_path,
			chain_id,
			base: polkadot_cli::RunCmd::from_iter(relay_chain_args),
		}
	}
}
