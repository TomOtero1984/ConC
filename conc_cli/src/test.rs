use conc_cli::logger::{info, warn, error, debug};

fn main() {
    info("Starting ConC decoder");
    debug("Parsed 42 ConC words");
    warn("Missing tone metadata on line 3");
    error("Could not decode symbol ΏΖ");
}
