use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="rust find utility", about = "A simple find utility implemented in Rust language")]
struct RustFindOpt{
    #[structopt(short, long, default_value="/home")]
    entrance_dir: String,

    #[structopt(short, long, required=true)]
    patterns: Vec<String>,

    #[structopt(short, long)]
    debug: bool,
}

fn main() {
    let args = RustFindOpt::from_args();
    println!("args = {:?}", args);

    find(&args.entrance_dir)
}

fn find(dir: &str) {

}
