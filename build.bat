@echo off
cls
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
copy "target\i686-linux-android\release\liblib_test.so" "builds\x86\libadd.so"
copy "target\aarch64-linux-android\release\liblib_test.so" "builds\arm64-v8a\libadd.so"
copy "target\x86_64-linux-android\release\liblib_test.so" "builds\x86_64\libadd.so"
copy "target\armv7-linux-androideabi\release\liblib_test.so" "builds\armeabi-v7a\libadd.so"