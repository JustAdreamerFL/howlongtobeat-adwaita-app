project('howlongtobeat',
  version: '0.1.0',
  meson_version: '>= 0.59.0',
)

dependency('gtk4', version: '>= 4.10')
dependency('libadwaita-1', version: '>= 1.5')

cargo = find_program('cargo', required: true)

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
datadir = prefix / get_option('datadir')

meson.add_install_script('build-aux/meson_post_install.py')

subdir('data')
subdir('src')
