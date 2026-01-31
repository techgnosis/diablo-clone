#! /usr/bin/bash

set -euox pipefail

podman run \
-it \
--rm \
-v ~/.claude.json:/home/james/.claude.json:Z \
-v ~/.claude:/home/james/.claude:Z \
-v ~/.gitconfig:/home/james/.gitconfig:Z \
-v ~/.ssh:/home/james/.ssh:ro,Z \
-v .:/workspace:Z \
--userns=keep-id \
rust-claude-beads:1 bash
