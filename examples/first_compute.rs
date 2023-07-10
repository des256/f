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
    let frame = Rc::new(system.create_frame(Rect { o: Vec2 { x: 10i32,y: 10i32, },s: size, },"Compute Shader",)?);

    // create GPU(s)
    let gpu = system.create_vulkan_gpu()?;

    // create surface for the window and GPU
    let r = Rect { o: Vec2{ x: 0i32,y: 0i32, },s: size, };
    let surface = gpu.create_surface(&frame,r)?;
    
    // create a command buffer for each swapchain frame
    let count = surface.get_swapchain_count();
    let mut command_buffers: Vec<vulkan::CommandBuffer> = Vec::new();
    for _ in 0..count {
        command_buffers.push(gpu.create_command_buffer()?);
    }
        
    // load and create compute shader
    let mut f = File::open(format!("assets/first-cs.{}",ext)).expect("unable to open compute shader");
    let mut code = Vec::<u8>::new();
    f.read_to_end(&mut code).expect("unable to read compute shader");
    let compute_shader = Rc::new(gpu.create_compute_shader(code)?);
        
    
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
