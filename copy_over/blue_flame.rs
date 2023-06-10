pub fn load(file_path: &str, remove_shapes: bool,
/*Game engine shit*/ renderer: &mut crate::Renderer, gameengine_objects: &mut crate::ObjectStorage, window: &crate::Window)
{
    let mut objects: Vec<(common::Objects, common::ObjectSettings)> = Vec::new();
    common::db::objects::load(&mut objects, file_path, remove_shapes, renderer, gameengine_objects, window);
}