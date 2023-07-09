use {
    f::*,
    std::{
        rc::Rc,
        result::Result,
    },
};

fn main() -> Result<(),String> {
    let system = Rc::new(f::System::open()?);
    let _frame_window = system.create_frame(
        Rect {
            o: Vec2 { x: 10i32,y: 10i32, },
            s: Vec2 { x: 800i32,y: 600i32, },
        },
        "Hello, World!",
    )?;
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
