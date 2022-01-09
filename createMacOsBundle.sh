#!/bin/bash

APP_NAME=gtk_min

mv ./target/release/bundle/osx/ktail.app/Contents/MacOS/ktail ./target/release/bundle/osx/ktail.app/Contents/MacOS/ktail-bin
chmod +x target/release/bundle/osx/ktail.app/Contents/MacOS/ktail-bin

cp ./assets/MacOS/ktail ./target/release/bundle/osx/ktail.app/Contents/MacOS/
chmod +x target/release/bundle/osx/ktail.app/Contents/MacOS/ktail

cp ./assets/MacOS/gdk-pixbuf-query-loaders ./target/release/bundle/osx/ktail.app/Contents/MacOS/
chmod +x ./target/release/bundle/osx/ktail.app/Contents/MacOS/gdk-pixbuf-query-loaders

cp -R ./assets/MacOS/Resources ./target/release/bundle/osx/ktail.app/Contents/
cp -R ./assets/icons ./target/release/bundle/osx/ktail.app/Contents/Resources
cp -R ./assets/themes/Mint-Y-Grey ./target/release/bundle/osx/ktail.app/Contents/Resources/themes
cp -R ./assets/MacOS/lib ./target/release/bundle/osx/ktail.app/Contents/MacOS/

cd ./target/release/bundle/osx/
hdiutil create "$APP_NAME.dmg" -volname "$APP_NAME Installer" -fs HFS+ -srcfolder "ktail.app"