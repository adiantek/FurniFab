from json import load, dumps
from os import environ, mkdir, rmdir, replace
from os.path import join
from pathlib import Path
from subprocess import call

OUTPUT = 'output'

ARM_MACOS = 'aarch64-apple-darwin'
ARM_WINDOWS = 'aarch64-pc-windows-msvc'
ARM_LINUX = 'aarch64-unknown-linux-gnu'
X64_MACOS = 'x86_64-apple-darwin'
X64_WINDOWS = 'x86_64-pc-windows-msvc'
X64_LINUX = 'x86_64-unknown-linux-gnu'

DMG = 'dmg'
MACOS = 'macos'
NSIS = 'nsis'
MSI = 'msi'
DEB = 'deb'
APP_IMAGE = 'appimage'

BUNDLER = {
    ARM_MACOS: DMG, ARM_WINDOWS: NSIS, ARM_LINUX: f'{DEB},{APP_IMAGE}',
    X64_MACOS: DMG, X64_WINDOWS: MSI, X64_LINUX: f'{DEB},{APP_IMAGE}',
}
INSTALLER = {
    ARM_MACOS: [join(DMG, 'FurniFab_{}_aarch64.dmg')],
    ARM_WINDOWS: [join(NSIS, 'FurniFab_{}_arm64-setup.exe')],
    ARM_LINUX: [join(DEB, 'furni-fab_{}_arm64.deb'), join(APP_IMAGE, 'TODO: tauri:5781')],
    X64_MACOS: [join(DMG, 'FurniFab_{}_x64.dmg')],
    X64_WINDOWS: [join(MSI, 'FurniFab_{}_x64_en-US.msi')],
    X64_LINUX: [join(DEB, 'furni-fab_{}_amd64.deb'), join(APP_IMAGE, 'furni-fab_{}_amd64.AppImage')],
}
UPDATER = {
    ARM_MACOS: join(MACOS, 'FurniFab.app.tar.gz'),
    ARM_WINDOWS: join(NSIS, 'FurniFab_{}_arm64-setup.nsis.zip'),
    ARM_LINUX: join(APP_IMAGE, 'TODO: tauri:5781'),
    X64_MACOS: join(MACOS, 'FurniFab.app.tar.gz'),
    X64_WINDOWS: join(MSI, 'FurniFab_{}_x64_en-US.msi.zip'),
    X64_LINUX: join(APP_IMAGE, 'furni-fab_{}_amd64.AppImage.tar.gz'),
}
SIGNATURE = {
    ARM_MACOS: 'darwin-aarch64', ARM_WINDOWS: 'windows-aarch64', ARM_LINUX: 'linux-aarch64',
    X64_MACOS: 'darwin-x86_64', X64_WINDOWS: 'windows-x86_64', X64_LINUX: 'linux-x86_64',
}


def load_env(name: str) -> str:
    """Load the environment variable with the given name or exit."""
    value = environ.get(name)
    if value is None:
        print(f'Missing environment variable: {name}')
        exit(1)
    return value


def assert_file(file: str):
    """Check if the file exists or exit."""
    if not Path(file).exists():
        print(f'File not found: {file}')
        exit(1)


def make_versioned_name(file: str, version: str) -> str:
    """Create a versioned filename from the given file and version."""
    return file.format(version) if '{}' in file else file


def make_version(number: str) -> str:
    """Create a version string from the given number."""
    config = load(open(join('src-tauri', 'tauri.conf.json'), 'r'))
    return f"{config['package']['version']}+{number}"


def make_filename(path: str, version: str) -> str:
    """Remove the version from the filename."""
    path = Path(path)
    return path.stem.replace(f'_{version}', '') + path.suffix


def build(channel: str, version: str, target: str):
    """Build the project."""
    args = ['cargo', 'tauri', 'build', '--ci', '-b', f'{BUNDLER[target]},updater', '--target', target, '-c']

    config = {'package': {'version': version}}
    if channel == 'release':
        config['tauri'] = {'updater': {'endpoints': ['https://adiantek.github.io/FurniFab/release/version.json']}}
    args.append(dumps(config))

    exit_code = call(args)
    if exit_code != 0:
        print(f'Build failed with exit code {exit_code}')
        exit(exit_code)


def deploy(version: str, target: str):
    """Deploy the project."""
    if Path(OUTPUT).exists():
        rmdir(OUTPUT)
    mkdir(OUTPUT)

    base_path = join('src-tauri', 'target', target, 'release', 'bundle')
    version = version.replace('+', '.') if target == X64_WINDOWS else version

    for file in INSTALLER[target]:
        source_path = join(base_path, make_versioned_name(file, version))
        assert_file(source_path)
        replace(source_path, join(OUTPUT, make_filename(source_path, version)))

    updater = make_versioned_name(UPDATER[target], version)
    updater_path = join(base_path, updater)

    if target == ARM_MACOS:
        updater = updater.replace('.app', '_aarch64.app')
    elif target == X64_MACOS:
        updater = updater.replace('.app', '_x64.app')

    assert_file(updater_path)
    replace(updater_path, join(OUTPUT, make_filename(updater, version)))

    signature_path = updater_path + '.sig'

    assert_file(signature_path)
    replace(signature_path, join(OUTPUT, SIGNATURE[target] + '.sig'))


if __name__ == '__main__':
    env_channel = load_env('BUILD_TYPE')
    env_version = load_env('VERSION_TAG')
    env_target = load_env('TARGET_TRIPLE')

    if env_target not in BUNDLER.keys():
        print(f'Unsupported target: {env_target}')
        exit(1)

    if env_channel == 'snapshot':
        env_version = make_version(env_version)

    build(env_channel, env_version, env_target)
    deploy(env_version, env_target)
