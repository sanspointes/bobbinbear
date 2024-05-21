use bevy_spts_uid::Uid;
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum ChangesetError {
    #[error("Changeset tried to find entity with uid '{0}' but none was found in world.")]
    UidMismatch(Uid),
}
