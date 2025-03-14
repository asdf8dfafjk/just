use super::*;

/// Main entry point into just binary.
pub fn run() -> Result<(), i32> {
  #[cfg(windows)]
  ansi_term::enable_ansi_support().ok();

  env_logger::Builder::from_env(
    env_logger::Env::new()
      .filter("JUST_LOG")
      .write_style("JUST_LOG_STYLE"),
  )
  .init();

  let app = Config::app();

  info!("Parsing command line arguments…");
  let matches = app.get_matches();

  let loader = Loader::new();

  let config = Config::from_matches(&matches).map_err(Error::from);

  let (color, verbosity) = config
    .as_ref()
    .map(|config| (config.color, config.verbosity))
    .unwrap_or((Color::auto(), Verbosity::default()));

  config
    .and_then(|config| config.run(&loader))
    .map_err(|error| {
      if !verbosity.quiet() && !error.suppress_message() {
        eprintln!("{}", error.color_display(color.stderr()));
      }
      error.code().unwrap_or(EXIT_FAILURE)
    })
}
