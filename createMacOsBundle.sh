#!/bin/sh

APP_NAME="GtkMin.app"
BIN_NAME="gtk_min"

echo "test\n"

mv "./target/release/bundle/osx/$APP_NAME/Contents/MacOS/$BIN_NAME" "./target/release/bundle/osx/$APP_NAME/Contents/MacOS/$BIN_NAME"-bin
chmod +x "./target/release/bundle/osx/$APP_NAME/Contents/MacOS/$BIN_NAME"-bin

echo '#!/bin/sh
MAC_OS_DIR=$(cd "$(dirname "$0")"; pwd)
ROOT=`dirname "$MAC_OS_DIR"`
LIB_DIR="$MAC_OS_DIR"/lib
RESOURCE_DIR="$ROOT"/Resources

export DYLD_LIBRARY_PATH="$LIB_DIR"
export GTK_PATH="$LIB_DIR"

export GTK_DATA_PREFIX="$RESOURCE_DIR"
export XDG_DATA_DIRS="$RESOURCE_DIR"
export GDK_PIXBUF_MODULE_FILE="$LIB_DIR/gdk-pixbuf-2.0/loaders.cache"
export GDK_PIXBUF_MODULEDIR="$LIB_DIR/gdk-pixbuf-2.0/loaders"
export PANGO_LIBDIR="$LIB_DIR"
export GTK_THEME="Mint-Y-Grey"


"$MAC_OS_DIR/gdk-pixbuf-query-loaders" --update-cache
$EXEC "$MAC_OS_DIR/gtk_min-bin"
' > "./target/release/bundle/osx/$APP_NAME/Contents/MacOS/$BIN_NAME"
chmod +x "./target/release/bundle/osx/$APP_NAME/Contents/MacOS/$BIN_NAME"


cd ./target/release/bundle/osx/
hdiutil create "$BIN_NAME".dmg -volname "$BIN_NAME Installer" -fs HFS+ -srcfolder $APP_NAME