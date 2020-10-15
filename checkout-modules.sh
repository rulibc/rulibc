git submodule update --init
cd deps/rust
git submodule update --progress --init -- "library/backtrace"
git submodule update --progress --init -- "library/stdarch"
echo done
