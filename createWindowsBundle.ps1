$appName = "gtk_min.exe"

$root = "D:\a\_temp\msys64\mingw64"
Get-ChildItem -Recurse $root
$libSrc = "$root\bin"
$libs = "libjpeg-8.dll", "libtiff-5.dll", "libzstd.dll", "libwebp-7.dll", "libLerc.dll", "libjbig-0.dll", "libdeflate.dll", "libxml2-2.dll", "libpcre2-8-0.dll", "liblzma-5.dll", "libgtksourceview-5-0.dll", "libbrotlicommon.dll", "libbrotlidec.dll", "libbz2-1.dll", "libcairo-2.dll", "libcairo-gobject-2.dll", "libcairo-script-interpreter-2.dll", "libdatrie-1.dll", "libepoxy-0.dll", "libexpat-1.dll", "libffi-7.dll", "libfontconfig-1.dll", "libfreetype-6.dll", "libfribidi-0.dll", "libgcc_s_seh-1.dll", "libgdk_pixbuf-2.0-0.dll", "libgio-2.0-0.dll", "libglib-2.0-0.dll", "libgmodule-2.0-0.dll", "libgobject-2.0-0.dll", "libgraphene-1.0-0.dll", "libgraphite2.dll", "libgtk-4-1.dll", "libharfbuzz-0.dll", "libiconv-2.dll", "libintl-8.dll", "liblzo2-2.dll", "libpango-1.0-0.dll", "libpangocairo-1.0-0.dll", "libpangoft2-1.0-0.dll", "libpangowin32-1.0-0.dll", "libpcre-1.dll", "libpixman-1-0.dll", "libpng16-16.dll", "libstdc++-6.dll", "libthai-0.dll", "libvulkan-1.dll", "libwinpthread-1.dll", "zlib1.dll"
mkdir bundle

copy "target\release\$appName" .\bundle
foreach($lib in $libs)
{
    copy $libSrc\$lib .\bundle
}


Compress-Archive -Path bundle -DestinationPath Windows_x64.zip
