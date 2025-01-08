# Show all scripts
default:
  just -l

# Generate coverage/lcov.info
coverage:
  cargo tarpaulin --engine ptrace -o lcov --output-dir coverage --all --all-targets --all-features

# For getting ptrace as html on macos
docker-coverage:
  docker image pull pkgxdev/pkgx
  docker run \
    --name semverator \
    --rm \
    --volume .:/volume \
    --security-opt seccomp=unconfined \
    --platform linux/amd64 \
    xd009642/tarpaulin \
    cargo tarpaulin --engine llvm -o html --all --all-targets --all-features --output-dir /volume/coverage
