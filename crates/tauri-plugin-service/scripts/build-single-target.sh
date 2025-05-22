#/usr/bin/env bash

TARGET=$1

pnpm run build:js && pnpm run build:single-target:cargo $TARGET && pnpm run build:gradle