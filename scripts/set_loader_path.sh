for lib in ./lib/mac/arm/*.dylib; do 
  echo "Patching install name for $lib..."
  install_name_tool -id "@loader_path/$(basename "$lib")" "$lib"
  for dep in $(otool -L "$lib" | awk 'NR>1 {print $1}'); do
    if [[ "$dep" == /usr/lib/* || "$dep" == /System/* || "$dep" == "@loader_path/"* ]]; then
      continue
    fi
    new_dep="@loader_path/$(basename "$dep")"
    echo "Changing dependency: $dep -> $new_dep"
    install_name_tool -change "$dep" "$new_dep" "$lib"
  done
  echo "Code signing $lib..."
  codesign --force --sign "Developer ID Application: Brandon Sexton (AR7B22KF2Q)" "$lib"
done

for lib in ./lib/mac/x86/*.dylib; do 
  echo "Patching install name for $lib..."
  install_name_tool -id "@loader_path/$(basename "$lib")" "$lib"
  for dep in $(otool -L "$lib" | awk 'NR>1 {print $1}'); do
    if [[ "$dep" == /usr/lib/* || "$dep" == /System/* || "$dep" == "@loader_path/"* ]]; then
      continue
    fi
    new_dep="@loader_path/$(basename "$dep")"
    echo "Changing dependency: $dep -> $new_dep"
    install_name_tool -change "$dep" "$new_dep" "$lib"
  done
  echo "Code signing $lib..."
  codesign --force --sign "Developer ID Application: Brandon Sexton (AR7B22KF2Q)" "$lib"
done

find lib/linux/x86 -name '*.so*' -exec patchelf --set-rpath '$ORIGIN' {} +
find lib/linux/arm -name '*.so*' -exec patchelf --set-rpath '$ORIGIN' {} +