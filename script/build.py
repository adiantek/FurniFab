import json
import sys
import os
import subprocess

LINK = "https://adiantek.github.io/FurniFab/release/version.json"


def create_version(number: str):
    config = json.load(open(os.path.join('src-tauri', 'tauri.conf.json'), 'r'))
    return config['package']['version'] + '+' + number


def build(release_type, version, target):
    args = ["cargo", "tauri", "build", "--ci", "-b", "msi,dmg,deb,appimage,updater", "-c"]

    config = {"package": {"version": version}}
    if release_type == 'release':
        config['tauri'] = {"updater": {"endpoints": [LINK]}}
    args.append(json.dumps(config))

    if target is not None:
        args.append('--target')
        args.append(target)
    subprocess.call(args)


if __name__ == '__main__':
    if len(sys.argv) < 3:
        print("Usage: python release.py <type> <version>")
        sys.exit(1)

    release_type = sys.argv[1]
    version = sys.argv[2]
    target = sys.argv[3] if len(sys.argv) == 4 else None

    if release_type == 'release':
        build(release_type, version, target)
    elif release_type == 'snapshot':
        build(release_type, create_version(version), target)
