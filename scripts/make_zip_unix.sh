#!/bin/bash
set -euxo pipefail

echo $MODE
echo $ZIP_NAME

TMP=$(mktemp -d)
mkdir $TMP/$ZIP_NAME
cp target/$MODE/rip_graphical $TMP/$ZIP_NAME/rip_graphic
cp target/$MODE/rip_ansi_terminal $TMP/$ZIP_NAME/rip_terminal
if [ -f target/$MODE/rip_graphical_opengl ]; then
  cp target/$MODE/rip_graphical_opengl $TMP/$ZIP_NAME/rip_graphic_opengl
fi
pushd $TMP
zip $ZIP_NAME.zip $ZIP_NAME/rip_graphic $ZIP_NAME/rip_terminal
popd
mv $TMP/$ZIP_NAME.zip .
