use crate::args;

#[test]
fn test_command_setup() {
    args::setup().debug_assert();
}
