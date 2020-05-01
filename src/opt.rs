use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Opt {
    Load { nth: u32 },
    SolveFc { nth: Option<u32> },
    Solve { nth: Option<u32> },
}
