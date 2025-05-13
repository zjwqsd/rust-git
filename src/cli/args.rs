use clap::{Parser, Subcommand};
use crate::commands::{
    init::git_init, add::git_add,commit::git_commit,rm::git_rm,//checkout::git_checkout,
    branch::git_branch,merge::git_merge,branch::git_branch_delete,
    status::git_status
};

#[derive(Parser)]
#[command(name = "rust-git")]
#[command(about = "一个用Rust实现的简易Git工具", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(default_value = ".")]
        path: String,
    },
    Add {
        file: String,
    },
    Commit {
        #[arg(short, long)]
        message: String,
    },
    Branch {
        /// 删除分支
        #[arg(short = 'd', long = "delete")]
        delete: bool,

        /// 分支名
        name: Option<String>,
    },
    Checkout {
        #[arg(short = 'b', long = "create", help = "创建新分支")]
        create: bool,

        #[arg(help = "要切换的分支名称")]
        branch: String,
    },
    Merge {
        branch: String,
    },
    Rm {
        /// 是否递归删除目录
        #[arg(short = 'r', long = "recursive")]
        recursive: bool,

        file: String,
    },
    Status,
}

impl Cli {
    pub fn execute(&self) {
        match &self.command {
            Commands::Init { path } => git_init(path),
            Commands::Add { file } => git_add(file),
            Commands::Commit { message } => git_commit(message),
            // Commands::Branch { name } => {
            //     let _ = git_branch(name.as_deref());
            // },
            Commands::Branch { delete, name } => {
                if *delete {
                    if let Some(name) = name {
                        git_branch_delete(name);
                    } else {
                        eprintln!("请指定要删除的分支名");
                    }
                } else {
                    // git_branch(name.as_deref());
                    if let Err(e) = git_branch(name.as_deref()) {
                        eprintln!("创建分支失败: {}", e);
                    }
                }
            }
            Commands::Checkout { create, branch } => {
                crate::commands::checkout::git_checkout(branch, *create);
            }
            Commands::Merge { branch } => git_merge(branch),
            Commands::Rm { file, recursive } => git_rm(file, *recursive),
            Commands::Status => git_status(),
        }
    }
}
