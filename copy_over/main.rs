use blue_engine::{header::{Engine, Renderer, ObjectStorage, /*ObjectSettings,*/ WindowDescriptor, PowerPreference}, Window};
use blue_engine::{primitive_shapes::{triangle, square}};
use blue_flame_common;
mod blue_flame;


fn main()
{    
    let mut engine = Engine::new_config(
        WindowDescriptor
        {
            width               : 1280,
            height              : 720,
            title               : "{project_name}",
            decorations         : true,
            resizable           : true,
            power_preference    : PowerPreference::HighPerformance,
            backends            : blue_engine::Backends::VULKAN,
        }).unwrap();


    
    blue_flame::load(
        "blue_flame/Scene 0",
        false,
        &mut engine.renderer,
        &mut engine.objects,
        &engine.window
        );
    

    println!("----------Start of update_loop----------");

    engine.update_loop(move
    |
        renderer,
        window,
        objects,
        _,
        _,
        plugins
    |
    {



        
    }).unwrap();
}