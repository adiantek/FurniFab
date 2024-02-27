import json
import sys
import os
from pathlib import Path
from datetime import datetime

JSON_FORMAT = """
{{
  "version": "{version}",
  "notes": "{notes}",
  "pub_date": "{date}",
  "platforms": {{
    "darwin-x86_64": {{
      "signature": "{darwin_x64_sig}",
      "url": "https://adiantek.github.io/FurniFab/{release_type}/FurniFab_x64.app.tar.gz"
    }},
    "darwin-aarch64": {{
      "signature": "{darwin_aarch64_sig}",
      "url": "https://adiantek.github.io/FurniFab/{release_type}/FurniFab_aarch64.app.tar.gz"
    }},
    "linux-x86_64": {{
      "signature": "{linux_x64_sig}",
      "url": "https://adiantek.github.io/FurniFab/{release_type}/furni-fab_amd64.AppImage.tar.gz"
    }},
    "windows-x86_64": {{
      "signature": "{windows_x64_sig}",
      "url": "https://adiantek.github.io/FurniFab/{release_type}/FurniFab_x64_en-US.msi.zip"
    }}
  }}
}}
"""


def prepare_file(path: str, version: str):
    path = Path(path)
    return path.stem.replace('_' + version, '') + path.suffix


def create_version(number: str):
    config = json.load(open(os.path.join('src-tauri', 'tauri.conf.json'), 'r'))
    return config['package']['version'] + '+' + number


def release(release_type: str, version: str):
    print(f'Releasing {release_type} {version}')

    date = datetime.now().replace(microsecond=0).isoformat() + 'Z'

    params = {'version': version, 'notes': 'There are no notes available', 'date': date, 'release_type': release_type,
              'darwin_x64_sig': '', 'linux_x64_sig': '', 'windows_x64_sig': '', 'darwin_aarch64_sig': ''}

    # Handle the macOS bundle
    bundle_path = os.path.join('src-tauri', 'target', 'release', 'bundle', 'macos')
    if os.path.exists(bundle_path):
        for file in os.listdir(bundle_path):
            if file.endswith('.sig'):
                params['darwin_x64_sig'] = open(os.path.join(bundle_path, file), 'r').read()
            elif file.endswith('.tar.gz'):
                path = prepare_file(file, version)
                path = path.replace('.app.tar.gz', '_x64.app.tar.gz')
                os.replace(os.path.join(bundle_path, file), os.path.join('release', path))
        bundle_path = os.path.join('src-tauri', 'target', 'release', 'bundle', 'dmg')
        for file in os.listdir(bundle_path):
            if file.endswith('.dmg'):
                os.replace(os.path.join(bundle_path, file), os.path.join('release', prepare_file(file, version)))

    # Handle the macOS aarch64 bundle
    bundle_path = os.path.join('src-tauri', 'target', 'aarch64-apple-darwin', 'release', 'bundle', 'macos')
    if os.path.exists(bundle_path):
        for file in os.listdir(bundle_path):
            if file.endswith('.sig'):
                params['darwin_aarch64_sig'] = open(os.path.join(bundle_path, file), 'r').read()
            elif file.endswith('.tar.gz'):
                path = prepare_file(file, version)
                path = path.replace('.app.tar.gz', '_aarch64.app.tar.gz')
                os.replace(os.path.join(bundle_path, file), os.path.join('release', path))
        bundle_path = os.path.join('src-tauri', 'target', 'aarch64-apple-darwin', 'release', 'bundle', 'dmg')
        for file in os.listdir(bundle_path):
            if file.endswith('.dmg'):
                os.replace(os.path.join(bundle_path, file), os.path.join('release', prepare_file(file, version)))

    # Handle the linux bundle
    bundle_path = os.path.join('src-tauri', 'target', 'release', 'bundle', 'appimage')
    if os.path.exists(bundle_path):
        for file in os.listdir(bundle_path):
            if file.endswith('.sig'):
                params['linux_x64_sig'] = open(os.path.join(bundle_path, file), 'r').read()
            elif file.endswith('.tar.gz'):
                os.replace(os.path.join(bundle_path, file), os.path.join('release', prepare_file(file, version)))
            elif file.endswith('.AppImage'):
                os.replace(os.path.join(bundle_path, file), os.path.join('release', prepare_file(file, version)))
        bundle_path = os.path.join('src-tauri', 'target', 'release', 'bundle', 'deb')
        for file in os.listdir(bundle_path):
            if file.endswith('.deb'):
                os.replace(os.path.join(bundle_path, file), os.path.join('release', prepare_file(file, version)))

    # Handle the windows bundle
    bundle_path = os.path.join('src-tauri', 'target', 'release', 'bundle', 'msi')
    if os.path.exists(bundle_path):
        for file in os.listdir(bundle_path):
            if file.endswith('.sig'):
                params['windows_x64_sig'] = open(os.path.join(bundle_path, file), 'r').read()
            elif file.endswith('.zip') or file.endswith('.msi'):
                os.replace(os.path.join(bundle_path, file),
                           os.path.join('release', prepare_file(file, version.replace('+', '.'))))

    with open(os.path.join('release', 'version.json'), 'w') as f:
        f.write(JSON_FORMAT.format(**params))


if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: python release.py <type> <version>")
        sys.exit(1)

    release_type = sys.argv[1]
    version = sys.argv[2]

    os.mkdir(os.path.join('.', 'release'))

    if release_type == 'release':
        release('release', version)
    elif release_type == 'snapshot':
        release('snapshot', create_version(version))
