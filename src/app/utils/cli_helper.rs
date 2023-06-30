use std::io::Write;

use clap::Parser;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    /// bind a host, ip only no domain allowed
    #[arg(short, long, default_value = "127.0.0.1:8000")]
    pub bind: String,

    /// log file path
    #[arg(long, default_value = "./")]
    pub log_path: String,

    /// logger level: debug | info | error
    #[arg(short, long, default_value = "debug")]
    pub log_level: String,
}

pub fn handle_start_args() -> CliArgs {
    print_credits();
    let args = CliArgs::parse();
    args
}

fn print_credits() {
    // no help! banner for copyright
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let line1 = "        ";
    let line2 = " /$$$$$$$            /$$           /$$                               /$$$$$$$$ /$$                     /$$      ";
    let line3 = "| $$__  $$          |__/          | $$                              | $$_____/| $$                    | $$      ";
    let line4 = r"| $$  \ $$  /$$$$$$  /$$ /$$$$$$$ | $$$$$$$   /$$$$$$  /$$  /$$  /$$| $$      | $$  /$$$$$$   /$$$$$$$| $$$$$$$ ";
    let line5 = "| $$$$$$$/ |____  $$| $$| $$__  $$| $$__  $$ /$$__  $$| $$ | $$ | $$| $$$$$   | $$ /$$__  $$ /$$_____/| $$__  $$";
    let line6 = r"| $$__  $$  /$$$$$$$| $$| $$  \ $$| $$  \ $$| $$  \ $$| $$ | $$ | $$| $$__/   | $$| $$$$$$$$|  $$$$$$ | $$  \ $$";
    let line7 = r"| $$  \ $$ /$$__  $$| $$| $$  | $$| $$  | $$| $$  | $$| $$ | $$ | $$| $$      | $$| $$_____/ \____  $$| $$  | $$";
    let line8 = "| $$  | $$ | $$$$$$$| $$| $$  | $$| $$$$$$$/|  $$$$$$/|  $$$$$/$$$$/| $$      | $$|  $$$$$$$ /$$$$$$$/| $$  | $$";
    let line9 = r"|__/  |__/ \_______/|__/|__/  |__/|_______/  \______/  \_____/\___/ |__/      |__/ \_______/|_______/ |__/  |__/";
    let line10 = "                                       佛祖保佑 0 error 0 warning";
    _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Black)));
    _ = writeln!(&mut stdout, "{}", line1);
    _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
    _ = writeln!(&mut stdout, "{}", line2);
    _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
    _ = writeln!(&mut stdout, "{}", line3);
    _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
    _ = writeln!(&mut stdout, "{}", line4);
    _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
    _ = writeln!(&mut stdout, "{}", line5);
    _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
    _ = writeln!(&mut stdout, "{}", line6);
    _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
    _ = writeln!(&mut stdout, "{}", line7);
    _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
    _ = writeln!(&mut stdout, "{}", line8);
    _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
    _ = writeln!(&mut stdout, "{}", line9);
    _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    _ = writeln!(&mut stdout, "{}", line10);
    _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_credits() {
        print_credits();
    }

    #[test]
    fn test_handle_args() {
        handle_start_args();
    }
}
