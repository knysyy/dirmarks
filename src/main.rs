use structopt::StructOpt;

use bookmark_directory::command::Opt;

fn main() {
    let opt: Opt = Opt::from_args();
    opt.run();
}
