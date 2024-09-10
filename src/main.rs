use clap::{Arg, ArgMatches, Command};
use mdbook::errors::Error;
use mdbook::preprocess::CmdPreprocessor;
use mdbook::preprocess::Preprocessor;

use std::io;
use std::process;

use mdbook_cmdrun::CmdRun;

fn main() {
    let matches = make_app().get_matches();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(sub_args);
    } else if let Some(sub_args) = matches.subcommand_matches("cmdrun") {
        let cmd = sub_args.get_many::<String>("cmd");
        let correct_exit_code : Option<i32> = if sub_args.get_flag("strict") {
            Some(&0)
        } else {
            sub_args.get_one("expect-return-code")
        };
        println!("{:?} {:?}", correct_exit_code, cmd)
        //CmdRun::run_cmdrun(sub_args, ".", false);
    } else if let Err(e) = handle_preprocessing() {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn make_cmdrun_parser() -> Command {
    Command::new("cmdrun")
        .about("test run a command before putting it in a book")
        .arg(
            Arg::new("expect-return-code")
            .help("require the specific return code N")
            .long("expect-return-code")
            .conflicts_with("strict")
//            .conflicts_with("exit-code-short")
            .num_args(1)
            .value_name("N")
        ).arg(
            Arg::new("strict")
            .help("require command to return the successful exit code 0")
            .long("strict")
            .conflicts_with("expect-return-code")
//            .conflicts_with("exit-code-short")
            .action(clap::ArgAction::SetTrue)
//        ).arg(
//            Arg::new("exit-code-short")
//            .help("require the specific exit code N")
//            .conflicts_with("expect-return-code")
//            .conflicts_with("strict")
//            .value_name("-N")
//            .allow_negative_numbers(true)
//            .value_parser(..=0)
        ).arg(
            Arg::new("cmd")
            .help("command whose output will be injected into book")
            .num_args(1..)
            .trailing_var_arg(true)
        )
}

fn make_app() -> Command {
    Command::new("mdbook-cmdrun")
        .about("mdbook preprocessor to run arbitrary commands and replace the stdout of these commands inside the markdown file.")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        ).subcommand(
            make_cmdrun_parser()
        )
}

fn handle_preprocessing() -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The mdbook-cmdrun preprocessor was built against version \
             {} of mdbook, but we're being called from version {}",
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = CmdRun.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = CmdRun.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
