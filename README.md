# iroha_config

A pilot project to experiment with configuration approaches for Iroha 2.

## Notes

`schematic` downsides:

- For fields without defaults (i.e. for required fields) we have to put `Option<..>`, because `schematic` uses `.unwrap_or_default()` on the resolution stage. Therefore, it is not possible to avoid fallback at all. Not good.  
  - Since you cannot avoid fallback, you have interesting situations even when you have default. E.g. you have a `NonZeroU64` field **with** a default value. But you cannot use `NonZeroU64` as a configuration field **completely**, because it doesn't implement `Default`! So you have to workaround it e.g. by making a newtype.
  - If your default is a little complicated expression, like `2.pow(9)`, you cannot use it directly as `setting(default = 2.pow(9))`. Instead, you have to put `setting(default = default_value)` & `fn default_value(_: &()) -> Option<u32> { Some(2.pow(9)) }`. This is ugly and redundant.
- No support for multiple env vars for a single parameter
- It seems that `ConfigEnum` behavior differs from documentation - it requires `serde` traits in order to work properly
- Automatically applies `serde(rename_all = "camelCase")`. It is a weird default. In order to change it, we have to put `serde(rename_all = "snake_case")` back, and in order to do it, we have to put `Serialize, Deserialize`. This is extra verbosity out of nowhere.

Differences:

- Renamed `torii` fields:
  - ~~`p2p_addr`~~ -> `addr_p2p`
  - ~~`api_url`~~ -> `addr_api`
  - ~~`telemetry_url`~~ -> `addr_telemetry`
- ~~`logger.max_log_level`~~ -> `logger.log_level`. I also removed `SyncLevel` wrap around it, which was used for runtime value replacement.
- ~~`wsv.warm_runtime_config`~~ -> `wsv.wasm_runtime`

## TODO

- [ ] Samples: different valid and invalid configuration files
- [ ] Samples: bash script with ENV vars
- [ ] Samples: addrs fallback