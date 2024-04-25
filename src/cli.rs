use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("target").required(true).args(&["executable", "attach_pid", "attach_name"]),
))]
pub struct Cli {
    pub executable: Option<String>,
    pub args: Option<Vec<String>>,

    /// Tells the debugger to attach to a process with the given pid.
    #[arg(short = 'p', long)]
    pub attach_pid: Option<u64>,

    /// Tells the debugger to attach to a process with the given name.
    #[arg(short = 'n', long)]
    pub attach_name: Option<String>,

    /// Do not automatically parse any '.lldbinit' files.
    #[arg(short = 'x', long)]
    pub no_lldbinit: bool,
}
