from datetime import datetime
from json import load, dump
from os import environ, remove
from os.path import join
from pathlib import Path

OUTPUT = 'output'

TARGET = {
    'darwin-aarch64': 'FurniFab_aarch64.app.tar.gz',
    'windows-aarch64': 'FurniFab_arm64-setup.nsis.zip',
    'linux-aarch64': 'TODO',
    'darwin-x86_64': 'FurniFab_x64.app.tar.gz',
    'windows-x86_64': 'FurniFab_x64_en-US.msi.zip',
    'linux-x86_64': 'furni-fab_amd64.AppImage.tar.gz',
}


def load_env(name: str) -> str:
    """Load the environment variable with the given name or exit."""
    value = environ.get(name)
    if value is None:
        print(f'Missing environment variable: {name}')
        exit(1)
    return value


def make_version(number: str, separator: str) -> str:
    """Create a version string from the given number."""
    config = load(open(join('src-tauri', 'tauri.conf.json'), 'r'))
    return f"{config['package']['version']}{separator}{number}"


def make_url(channel: str, filename: str) -> str:
    """Create the URL from the given channel and filename."""
    return f'https://adiantek.github.io/FurniFab/{channel}/{filename}'


def deploy(channel: str, version: str, notes: str):
    """Deploy the project."""
    platforms = {}
    config = {
        'version': version, 'notes': notes,
        'pub_date': f'{datetime.now().replace(microsecond=0).isoformat()}Z',
        'platforms': platforms
    }

    for (target, filename) in TARGET.items():
        path = join(OUTPUT, target + '.sig')
        if Path(path).exists():
            platforms[target] = {'signature': open(path, 'r').read(), 'url': make_url(channel, filename)}
            remove(path)

    dump(config, open(join(OUTPUT, 'version.json'), 'w'))


if __name__ == '__main__':
    env_channel = load_env('BUILD_TYPE')
    env_version = load_env('VERSION_TAG')
    env_notes = load_env('RELEASE_NOTES')

    if env_channel == 'snapshot':
        env_version = make_version(env_version, '+')

    deploy(env_channel, env_version, env_notes)
