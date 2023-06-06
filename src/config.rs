use crate::util::{ugly_schematic_find_addr, DefaultPanic};
use iroha_crypto::prelude::{PrivateKey, PublicKey};
use iroha_data_model::prelude::{LengthLimits, MetadataLimits, PeerId, TransactionLimits};
use iroha_primitives::addr::SocketAddr;
use schematic::{Config, ConfigEnum};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::num::NonZeroU64;
use std::path::PathBuf;

#[derive(Config, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Iroha {
    // FIXME: should not be `Option`
    public_key: Option<PublicKey>,
    // FIXME: should not be `Option`
    private_key: Option<PrivateKey>,

    // deprecated
    #[setting(default = false)]
    #[serde(skip_serializing)]
    disable_panic_terminal_colors: bool,

    #[setting(nested)]
    kura: Kura,
    #[setting(nested)]
    sumeragi: Sumeragi,
    #[setting(nested)]
    torii: Torii,
    #[setting(nested)]
    queue: Queue,
    #[setting(nested)]
    logger: Logger,
    #[setting(nested)]
    genesis: Genesis,
    #[setting(nested)]
    wsv: Wsv,
    #[setting(nested)]
    network: Network,
    #[setting(nested)]
    telemetry: Telemetry,
}

#[derive(Config, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Kura {
    #[setting(default)]
    init_mode: KuraInitMode,
    #[setting(default = PathBuf::from("./storage"))]
    block_store_path: PathBuf,
    #[setting(default = default_blocks_per_storage)]
    blocks_per_storage: DefaultPanic<NonZeroU64>,
    #[setting(default = 100)]
    actor_channel_capacity: u32,
    #[setting(default = false)]
    debug_output_new_blocks: bool,
}

const fn default_blocks_per_storage(_: &()) -> Option<DefaultPanic<NonZeroU64>> {
    const VALUE: NonZeroU64 = {
        match NonZeroU64::new(1000) {
            Some(x) => x,
            None => unreachable!(),
        }
    };
    Some(DefaultPanic(VALUE))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KuraInitMode {
    #[default]
    Strict,
    Fast,
}

#[derive(Config, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Sumeragi {
    #[setting(default = 2000)]
    block_time_ms: u64,
    #[setting(default)]
    trusted_peers: HashSet<PeerId>,
    #[setting(default = 4000)]
    commit_time_limit_ms: u64,
    #[setting(default = default_max_txs_in_block)]
    max_transactions_in_block: u32,
    #[setting(default = 100)]
    actor_channel_capacity: u32,
    #[setting(default = 500)]
    gossip_batch_size: u32,
    #[setting(default = 1000)]
    gossip_period_ms: u64,
    #[setting(default = false)]
    debug_force_soft_fork: bool,
}

const fn default_max_txs_in_block(_: &()) -> Option<u32> {
    const VALUE: u32 = 2_u32.pow(9);
    Some(VALUE)
}

fn default_addr_p2p(_: &()) -> Option<DefaultPanic<SocketAddr>> {
    ugly_schematic_find_addr(1337)
}

fn default_addr_api(_: &()) -> Option<DefaultPanic<SocketAddr>> {
    ugly_schematic_find_addr(8080)
}

fn default_addr_telemetry(_: &()) -> Option<DefaultPanic<SocketAddr>> {
    ugly_schematic_find_addr(8180)
}

const fn default_max_transaction_size(_: &()) -> Option<u32> {
    const VALUE: u32 = 2_u32.pow(15);
    Some(VALUE)
}

const fn default_max_content_len(_: &()) -> Option<u32> {
    const VALUE: u32 = 2_u32.pow(12) * 4000;
    Some(VALUE)
}

#[derive(Config, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Torii {
    #[setting(default = default_addr_p2p)]
    addr_p2p: DefaultPanic<SocketAddr>,
    #[setting(default = default_addr_api)]
    addr_api: DefaultPanic<SocketAddr>,
    #[setting(default = default_addr_telemetry)]
    addr_telemetry: DefaultPanic<SocketAddr>,
    #[setting(default = default_max_transaction_size)]
    max_transaction_size: u32,
    #[setting(default = default_max_content_len)]
    max_content_len: u32,
}

#[derive(Config, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlockSync {
    #[setting(default = 10000)]
    gossip_period_ms: u64,
    #[setting(default = 4)]
    block_batch_size: u32,
    #[setting(default = 100)]
    actor_channel_capacity: u32,
}

#[derive(Config, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Queue {
    #[setting(default = default_max_txs_in_queue)]
    max_transactions_in_queue: u32,
    #[setting(default = default_max_txs_in_signature_buffer)]
    max_transactions_in_signature_buffer: u32,
    #[setting(default = default_tx_ttl)]
    transaction_time_to_live_ms: u64,
    #[setting(default = 1000)]
    future_threshold_ms: u64,
}

const fn default_max_txs_in_queue(_: &()) -> Option<u32> {
    const VALUE: u32 = 2_u32.pow(16);
    Some(VALUE)
}

const fn default_max_txs_in_signature_buffer(_: &()) -> Option<u32> {
    const VALUE: u32 = 2_u32.pow(16);
    Some(VALUE)
}

const fn default_tx_ttl(_: &()) -> Option<u64> {
    const VALUE: u64 = 24 * 60 * 60 * 1000;
    Some(VALUE)
}

#[derive(Config, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Logger {
    #[setting(default, env = "LOG_LEVEL")]
    log_level: LogLevel,
    #[setting(default = 1000)]
    telemetry_capacity: u32,
    #[setting(default = false)]
    compact_mode: bool,
    // #[setting(default = None)]
    log_file_path: Option<PathBuf>,
    #[setting(default = true)]
    terminal_colors: bool,
    // #[setting(default = None)]
    tokio_console_addr: Option<SocketAddr>,
}

#[derive(Default, ConfigEnum, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    #[default]
    INFO,
    WARN,
    ERROR,
}

#[derive(Config, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Genesis {
    // FIXME: no way to enforce setting
    pub account_public_key: Option<PublicKey>,
    pub account_private_key: Option<PrivateKey>,
}

#[derive(Config, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Wsv {
    #[setting(default = default_metadata_limits)]
    pub asset_metadata_limits: DefaultPanic<MetadataLimits>,
    #[setting(default = default_metadata_limits)]
    pub asset_definition_metadata_limits: DefaultPanic<MetadataLimits>,
    #[setting(default = default_metadata_limits)]
    pub account_metadata_limits: DefaultPanic<MetadataLimits>,
    #[setting(default = default_metadata_limits)]
    pub domain_metadata_limits: DefaultPanic<MetadataLimits>,
    #[setting(default = default_ident_len_limits)]
    pub ident_length_limits: DefaultPanic<LengthLimits>,
    #[setting(default = default_tx_limits)]
    pub transaction_limits: DefaultPanic<TransactionLimits>,
    #[setting(nested)]
    pub wasm_runtime: Wasm,
}

const fn default_metadata_limits(_: &()) -> Option<DefaultPanic<MetadataLimits>> {
    const VALUE: MetadataLimits = MetadataLimits::new(2_u32.pow(20), 2_u32.pow(12));
    Some(DefaultPanic(VALUE))
}

const fn default_ident_len_limits(_: &()) -> Option<DefaultPanic<LengthLimits>> {
    const VALUE: LengthLimits = LengthLimits::new(1, 2_u32.pow(7));
    Some(DefaultPanic(VALUE))
}

const fn default_tx_limits(_: &()) -> Option<DefaultPanic<TransactionLimits>> {
    Some(DefaultPanic(
        iroha_data_model::transaction::DEFAULT_TRANSACTION_LIMITS,
    ))
}

#[derive(Config, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Wasm {
    #[setting(default = 23_000_000)]
    fuel_limit: u64,
    // 500 MiB
    #[setting(default = default_wasm_max_memory)]
    max_memory: u32,
}

const fn default_wasm_max_memory(_: &()) -> Option<u32> {
    const VALUE: u32 = 500 * 2_u32.pow(20);
    Some(VALUE)
}

#[derive(Config, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Network {
    #[setting(default = 100)]
    actor_channel_capacity: u32,
}

#[derive(Config, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Telemetry {
    #[config(serde_as_str)]
    name: Option<String>,
    #[config(serde_as_str)]
    url: Option<url::Url>,
    #[setting(default = 1)]
    min_retry_period: u64,
    #[setting(default = 4)]
    max_retry_delay_exponent: u8,
    #[config(serde_as_str)]
    file: Option<PathBuf>,
}
