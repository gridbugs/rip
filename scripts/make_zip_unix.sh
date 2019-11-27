#!/bin/bash
set -euxo pipefail

echo $MODE

TMP=$(mktemp -d)
mkdir $TMP/rip
cp target/$MODE/rip_graphical $TMP/rip/rip_graphic
cp target/$MODE/rip_ansi_terminal $TMP/rip/rip_terminal
pushd $TMP
zip rip.zip rip/rip_graphic rip/rip_terminal
popd
mv $TMP/rip.zip .
