# Show all scripts
default:
  just -l

# Generate coverage/lcov.info
coverage:
  cargo tarpaulin --engine llvm -o lcov --output-dir coverage --coveralls $COVERALLS_TOKEN