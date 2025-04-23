// src/operations/mod.rs
pub mod clone;
pub mod executor;
pub mod hook;
pub mod pull;
pub mod push;
pub mod run;
pub mod status;

use crate::types::{HookSubCommand, RepositoryOperation};

// Export all operations
pub use clone::CloneOperation;
pub use hook::HookOperation;
pub use pull::PullOperation;
pub use push::PushOperation;
pub use run::RunOperation;
pub use status::StatusOperation;

// Factory function to create operation instances
pub fn create_operation(
    operation_type: &crate::types::Operation,
    args: &crate::args::Args,
) -> Box<dyn RepositoryOperation> {
    match operation_type {
        crate::types::Operation::Clone => Box::new(clone::CloneOperation::new(
            args.target_dir.as_ref().unwrap().clone(),
            args.reverse,
        )),
        crate::types::Operation::Pull => Box::new(pull::PullOperation::new()),
        crate::types::Operation::Push => Box::new(push::PushOperation::new()),
        crate::types::Operation::Status => Box::new(status::StatusOperation::new()),
        crate::types::Operation::Run { command } => Box::new(run::RunOperation::new(command.clone())),
        crate::types::Operation::Hook { subcommand } => Box::new(hook::HookOperation::new(subcommand.clone())),
    }
}