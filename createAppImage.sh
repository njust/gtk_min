#!/bin/sh
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

cp "target/release/$APP_NAME" "$BIN_DIR/bin"
cp -r /opt/gtk/lib/x86_64-linux-gnu/* $LIB_DIR

ls -l $LIB_DIR
du -h -k $LIB_DIR

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
