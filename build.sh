#!/bin/sh

set +e
make dctags

echo

./dctags -D 255 --verbose=yes -o tags.test Test/rust-sample.rs

echo
cat tags.test
