use std::env;

pub fn load(filepath: &str, remove_shapes: bool,
/*Game engine shit*/ renderer: &mut crate::Renderer, objects: &mut crate::ObjectStorage, window: &crate::Window)
{
    let project_dir = env::current_dir().unwrap().display().to_string();
    let mut flameobjects: Vec<(blue_flame_common::Flameobject, blue_flame_common::FlameobjectSettings)> = Vec::new();
    
    blue_flame_common::db::flameobjects::load(&mut flameobjects, &project_dir, filepath, remove_shapes, renderer, objects, window);
}