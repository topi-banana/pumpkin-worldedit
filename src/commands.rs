use pumpkin::plugin::Context;
use pumpkin_util::permission::{Permission, PermissionDefault};

mod pos1;
mod pos2;

mod replace;
mod set;

pub async fn register_commmand(context: &Context) {
    context
        .register_command(pos1::init_command_tree(), "worldedit:selection.pos")
        .await;
    context
        .register_command(pos2::init_command_tree(), "worldedit:selection.pos")
        .await;
    context
        .register_command(set::init_command_tree(), "worldedit:region.set")
        .await;
    context
        .register_command(replace::init_command_tree(), "worldedit:region.replace")
        .await;
}

pub async fn register_permission(context: &Context) {
    context
        .register_permission(Permission::new(
            "worldedit:selection.pos",
            "",
            PermissionDefault::Allow,
        ))
        .await
        .unwrap();
    context
        .register_permission(Permission::new(
            "worldedit:region.set",
            "",
            PermissionDefault::Allow,
        ))
        .await
        .unwrap();
    context
        .register_permission(Permission::new(
            "worldedit:region.replace",
            "",
            PermissionDefault::Allow,
        ))
        .await
        .unwrap();
}
