#! /usr/bin/env bash

set euo -pipefail

podman build -t rust-claude-beads:1 .
