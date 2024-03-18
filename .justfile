# Show all scripts
default:
  just -l

# Generate coverage/lcov.info
coverage:
  cargo tarpaulin --engine ptrace -o lcov --output-dir coverage --coveralls $COVERALLS_TOKEN