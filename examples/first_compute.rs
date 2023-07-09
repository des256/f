use {
    f::*,
    std::{
        rc::Rc,
        result::Result,
    },
};

fn main() -> Result<(),String> {

    // dimensions
    let size = Vec2 { x: 800i32,y: 600i32, };
    
    // open system
    let system = Rc::new(f::System::open()?);

    // create frame window
    let frame = system.create_frame(Rect { o: Vec2 { x: 10i32,y: 10i32, },s: size, },"Compute Shader",)?;

    // create GPU(s)
    let vulkan = system.create_vulkan_gpu()?;
    //let opengl = Rc::new(system.create_opengl_gpu()?);

    // create surface for the window and GPU
    let r = Rect { o: Vec2{ x: 0i32,y: 0i32, },s: size, };
    let vulkan_surface = vulkan.create_surface(&frame,r)?;
    
    // get number of frames in the swapchain of the surface
    let count = vulkan_surface.get_swapchain_count();
        
    // create a command buffer for each swapchain frame
    let mut command_buffers: Vec<T::CommandBuffer> = Vec::new();
    for _ in 0..count {
        command_buffers.push(gpu.create_command_buffer()?);
    }
        
    // load and create vertex shader
    let mut f = File::open(format!("assets/triangle-vs.{}",ext)).expect("Unable to open vertex shader");
    let mut code = Vec::<u8>::new();
    f.read_to_end(&mut code).expect("Unable to read vertex shader");
    let vertex_shader = Rc::new(gpu.create_vertex_shader(&triangle_vs::ast())?);
        
    
    let mut close_clicked = false;
    while !close_clicked {
        system.wait();
        system.flush().into_iter().for_each(|(_,event)| {
            dprintln!("event {}",event);
            if let Event::Close = event {
                close_clicked = true;
            }
        });
    }
    Ok(())
}
