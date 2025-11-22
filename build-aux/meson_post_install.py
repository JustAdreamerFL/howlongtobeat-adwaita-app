#!/usr/bin/env python3

import os
import subprocess
from pathlib import Path

prefix = os.environ.get('MESON_INSTALL_PREFIX', '/usr/local')
datadir = Path(prefix) / 'share'

# Compile GSettings schemas
print('Compiling GSettings schemas...')
schemas_dir = datadir / 'glib-2.0' / 'schemas'
if schemas_dir.exists():
    subprocess.call(['glib-compile-schemas', str(schemas_dir)])

# Update icon cache
print('Updating icon cache...')
icons_dir = datadir / 'icons' / 'hicolor'
if icons_dir.exists():
    subprocess.call(['gtk4-update-icon-cache', '-q', '-t', '-f', str(icons_dir)])

# Update desktop database
print('Updating desktop database...')
applications_dir = datadir / 'applications'
if applications_dir.exists():
    subprocess.call(['update-desktop-database', '-q', str(applications_dir)])

print('Post-install script completed.')
