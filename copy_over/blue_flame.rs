pub fn load(filepath: &str, remove_shapes: bool,
/*Game engine shit*/ renderer: &mut crate::Renderer, objects: &mut crate::ObjectStorage, window: &crate::Window)
{
    let mut flameobjects: Vec<(common::Flameobject, common::FlameobjectSettings)> = Vec::new();
    blue_flame_common::db::flameobjects::load(&mut flameobjects, filepath, remove_shapes, renderer, objects, window);
}