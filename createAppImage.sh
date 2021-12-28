#!/bin/bash
TARGET_DIR=target/AppDir
APP_NAME=gtk_min

USR_DIR="$TARGET_DIR/usr"
BIN_DIR="$USR_DIR/bin"
LIB_DIR="$USR_DIR/lib"
SHARE_DIR="$USR_DIR/share"

rm -rf "$TARGET_DIR"

mkdir "$TARGET_DIR"
mkdir "$USR_DIR"
mkdir "$BIN_DIR"
mkdir "$LIB_DIR"
mkdir "$SHARE_DIR"

LIBS=("libgtk-4.so.1.500.0" "libgtk-4.so.1" "libgtk-4.so" "libgio-2.0.so.0.7100.0" "libgio-2.0.so.0" "libgio-2.0.so" "libglib-2.0.so.0.7100.0" "libglib-2.0.so.0" "libglib-2.0.so"
"libgtksourceview-5.so.0.0.0" "libgtksourceview-5.so.0" "libgtksourceview-5.so" "libgobject-2.0.so.0.7100.0" "libgobject-2.0.so.0" "libgobject-2.0.so" "libpango-1*"
"libgraphene-1.0.so.0.1000.7" "libgraphene-1.0.so.0" "libgraphene-1.0.so" "libpangoft2*"
"libpangocairo*" "libpangoxft*" "libgmodule-2.0.so.0.7100.0"
"libgmodule-2.0.so.0" "libgmodule-2.0.so" "libgthread-2.0.so.0.7100.0" "libgthread-2.0.so.0" "libgthread-2.0.so"
)

for FILE in "${LIBS[@]}"
do
  cp -r "/opt/gtk/lib/x86_64-linux-gnu/$FILE" $LIB_DIR
done

cp "target/release/$APP_NAME" "$BIN_DIR/bin"

APP_RUN_SCRIPT="$TARGET_DIR/AppRun"
echo '#!/bin/sh
HERE=$(dirname $(readlink -f "${0}"))
export LD_LIBRARY_PATH="${HERE}"/usr/lib
export XDG_DATA_DIRS="${HERE}"/usr/share
"${HERE}"/usr/bin/bin $@
' > "$APP_RUN_SCRIPT"
chmod +x "$APP_RUN_SCRIPT"

echo "
[Desktop Entry]
Name=$APP_NAME
Exec=bin
Icon=icon
Type=Application
Categories=Utility;
X-AppImage-Version=0.1.0
" > "$TARGET_DIR/$APP_NAME.desktop"

touch "$TARGET_DIR/icon.png"

appimagetool "$TARGET_DIR" $APP_NAME
