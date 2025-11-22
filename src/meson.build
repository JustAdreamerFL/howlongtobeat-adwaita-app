cargo_options = ['--manifest-path', meson.project_source_root() / 'Cargo.toml']
cargo_options += ['--target-dir', meson.project_build_root() / 'target']

if get_option('profile') == 'default'
  cargo_options += ['--release']
  rust_target = 'release'
else
  rust_target = 'debug'
endif

cargo_env = environment()
cargo_env.set('CARGO_HOME', meson.project_build_root() / 'cargo-home')

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: 'howlongtobeat',
  console: true,
  env: cargo_env,
  command: [
    cargo,
    'build',
    cargo_options,
  ],
)

custom_target(
  'cargo-install',
  depends: cargo_build,
  build_by_default: false,
  build_always_stale: true,
  install: true,
  install_dir: bindir,
  console: true,
  output: 'howlongtobeat-install',
  command: [
    'sh', '-c',
    'cp ' + meson.project_build_root() / 'target' / rust_target / 'howlongtobeat' + ' "$MESON_INSTALL_DESTDIR_PREFIX"/' + bindir,
  ],
)
