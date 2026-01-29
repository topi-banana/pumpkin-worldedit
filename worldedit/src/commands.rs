use std::sync::Arc;

use pumpkin::plugin::Context;
use pumpkin_util::permission::{Permission, PermissionDefault};

use crate::storage::WorldEditDataStorage;

mod pos1;
mod pos2;
mod select;
mod wand;

mod replace;
mod set;

mod copy;
mod paste;

mod redo;
mod undo;

pub async fn register_command(context: &Context, storage: &Arc<WorldEditDataStorage>) {
    context
        .register_command(pos1::init_command_tree(storage), "worldedit:selection.pos")
        .await;
    context
        .register_command(pos2::init_command_tree(storage), "worldedit:selection.pos")
        .await;
    context
        .register_command(
            select::init_command_tree(storage),
            "worldedit:selection.pos",
        )
        .await;
    context
        .register_command(wand::init_command_tree(storage), "worldedit:wand")
        .await;

    context
        .register_command(
            replace::init_command_tree(storage),
            "worldedit:region.replace",
        )
        .await;
    context
        .register_command(set::init_command_tree(storage), "worldedit:region.set")
        .await;

    context
        .register_command(copy::init_command_tree(storage), "worldedit:clipboard.copy")
        .await;
    context
        .register_command(
            paste::init_command_tree(storage),
            "worldedit:clipboard.paste",
        )
        .await;

    context
        .register_command(redo::init_command_tree(storage), "worldedit:history.redo")
        .await;
    context
        .register_command(undo::init_command_tree(storage), "worldedit:history.undo")
        .await;
}

pub async fn register_permission(context: &Context) {
    let permissions = [
        Permission::new("worldedit:selection.pos", "", PermissionDefault::Allow),
        Permission::new("worldedit:wand", "", PermissionDefault::Allow),
        Permission::new("worldedit:region.replace", "", PermissionDefault::Allow),
        Permission::new("worldedit:region.set", "", PermissionDefault::Allow),
        Permission::new("worldedit:clipboard.copy", "", PermissionDefault::Allow),
        Permission::new("worldedit:history.redo", "", PermissionDefault::Allow),
        Permission::new("worldedit:history.undo", "", PermissionDefault::Allow),
    ];
    for permission in permissions {
        context.register_permission(permission).await.unwrap();
    }
}
