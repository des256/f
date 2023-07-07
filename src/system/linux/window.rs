use {
    crate::*,
    std::{
        rc::Rc,
        os::raw::c_void,
    },
};

pub const KEY_UP: u8 = 111;
pub const KEY_DOWN: u8 = 116;
pub const KEY_LEFT: u8 = 113;
pub const KEY_RIGHT: u8 = 114;

#[derive(Debug)]
pub struct Window {
    pub system: Rc<System>,
#[doc(hidden)]
    pub(crate) xcb_window: sys::xcb_window_t,
}

impl Window {

    // create basic window, decorations are handled in the public create_frame and create_popup
    fn new_common(system: &Rc<System>,r: Rect<i32>,_absolute: bool) -> Result<Window,String> {

        // create window
        let xcb_window = unsafe { sys::xcb_generate_id(system.xcb_connection) };
        let values = [
            sys::XCB_EVENT_MASK_EXPOSURE
            | sys::XCB_EVENT_MASK_KEY_PRESS
            | sys::XCB_EVENT_MASK_KEY_RELEASE
            | sys::XCB_EVENT_MASK_BUTTON_PRESS
            | sys::XCB_EVENT_MASK_BUTTON_RELEASE
            | sys::XCB_EVENT_MASK_POINTER_MOTION
            | sys::XCB_EVENT_MASK_STRUCTURE_NOTIFY,
            sys::XCB_COPY_FROM_PARENT,
        ];
        unsafe {
            sys::xcb_create_window(
                system.xcb_connection,
                (*system.xcb_screen).root_depth,
                xcb_window as u32,
                (*system.xcb_screen).root,
                r.o.x as i16,
                r.o.y as i16,
                r.s.x as u16,
                r.s.y as u16,
                0,
                sys::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
                (*system.xcb_screen).root_visual,
                sys::XCB_CW_EVENT_MASK | sys::XCB_CW_COLORMAP,
                &values as *const u32 as *const c_void
            );
            sys::xcb_map_window(system.xcb_connection,xcb_window as u32);
            sys::xcb_flush(system.xcb_connection);
        }

        Ok(Window {
            system: Rc::clone(system),
            xcb_window,
        })
    }
    
    /// Create application frame window (with frame and title bar).
    pub fn new_frame(system: &Rc<System>,r: Rect<i32>,title: &str) -> Result<Window,String> {
        let window = Self::new_common(system,r,false)?;
        let protocol_set = [system.wm_delete_window];
        let protocol_set_void = protocol_set.as_ptr() as *const std::os::raw::c_void;
        unsafe { sys::xcb_change_property(
            system.xcb_connection,
            sys::XCB_PROP_MODE_REPLACE as u8,
            window.xcb_window as u32,
            system.wm_protocols,
            sys::XCB_ATOM_ATOM,
            32,
            1,
            protocol_set_void
        ) };
        unsafe { sys::xcb_change_property(
            system.xcb_connection,
            sys::XCB_PROP_MODE_REPLACE as u8,
            window.xcb_window as u32,
            sys::XCB_ATOM_WM_NAME,
            sys::XCB_ATOM_STRING,
            8,
            title.len() as u32,
            title.as_bytes().as_ptr() as *const std::os::raw::c_void
        ) };
        unsafe { sys::xcb_flush(system.xcb_connection) };
        Ok(window)
    }
    
    /// Create standalone popup window (no frame or title bar).
    pub fn new_popup(system: &Rc<System>,r: Rect<i32>) -> Result<Window,String> {
        let window = Self::new_common(system,r,true)?;
        let net_state = [system.wm_net_state_above];
        unsafe { sys::xcb_change_property(
            system.xcb_connection,
            sys::XCB_PROP_MODE_REPLACE as u8,
            window.xcb_window as u32,
            system.wm_net_state,
            sys::XCB_ATOM_ATOM,
            32,
            1,
            net_state.as_ptr() as *const std::os::raw::c_void
        ) };
        let hints = [2u32,0,0,0,0];
        unsafe { sys::xcb_change_property(
            system.xcb_connection,
            sys::XCB_PROP_MODE_REPLACE as u8,
            window.xcb_window as u32,
            system.wm_motif_hints,
            sys::XCB_ATOM_ATOM,
            32,
            5,
            hints.as_ptr() as *const std::os::raw::c_void
        ) };
        unsafe { sys::xcb_flush(system.xcb_connection) };
        Ok(window)
    }

    /// Get WindowID for this window.
    pub fn id(&self) -> u32 {
        self.xcb_window
    }

    /*
    /// Get framebuffer count for this window.
    pub fn get_framebuffer_count(&self) -> usize {
        self.gpu_window.get_framebuffer_count()
    }

    /// Update swapchain resources.
    pub fn update_swapchain(&self,r: &Rect<i32>) {
        self.gpu_window.update_swapchain(r);
    }

    /// Acquire next framebuffer.
    pub fn acquire(&self,signal_semaphore: &Semaphore) -> Result<usize,String> {
        self.gpu_window.acquire(signal_semaphore)
    }

    /// Present the framebuffer.
    pub fn present(&self,index: usize,wait_semaphore: &Semaphore) {
        self.gpu_window.present(index,wait_semaphore);
    }
    */
}

/*
impl Window {self.resources.borrow().vk_renderpass
    pub(crate) fn handle_event(&self,event: Event) {
        if let Event::Configure(r) = &event {
            // When resizing, X seems to return a rectangle with the initial
            // origin as specified during window creation. But when moving, X
            // seems to correctly update the origin coordinates.
            // Not sure what to make of this, but in order to get the actual
            // rectangle, this seems to work:
            let old_r = self.r.get();
            if r.s != old_r.s {
                self.r.set(Rect { o: old_r.o,s: r.s, });
            }
            else {
                self.r.set(*r);
            }
        }
        if let Some(handler) = &*(self.handler).borrow() {
            (handler)(event);
        }
    }

    pub fn set_handler<T: Fn(Event) + 'static>(&self,handler: T) {
        *self.handler.borrow_mut() = Some(Box::new(handler));
    }

    pub fn clear_handler(&self) {
        *self.handler.borrow_mut() = None;
    }

    pub fn show(&self) {
        unsafe {
            xcb_map_window(self.screen.system.xcb_connection,self.xcb_window as u32);
            xcb_flush(self.screen.system.xcb_connection);
        }
    }

    pub fn hide(&self) {
        unsafe {
            xcb_unmap_window(self.screen.system.xcb_connection,self.xcb_window as u32);
            xcb_flush(self.screen.system.xcb_connection);
        }
    }

    pub fn set_rect(&self,r: &Rect<i32>) {
        let values = xcb_configure_window_value_list_t {
            x: r.o.x as i32,
            y: r.o.y as i32,
            width: r.s.x as u32,
            height: r.s.y as u32,
            border_width: 0,
            sibling: 0,
            stack_mode: 0,
        };
        unsafe { xcb_configure_window(
            self.screen.system.xcb_connection,
            self.xcb_window as u32,
            XCB_CONFIG_WINDOW_X as u16 |
                XCB_CONFIG_WINDOW_Y as u16 |
                XCB_CONFIG_WINDOW_WIDTH as u16 |
                XCB_CONFIG_WINDOW_HEIGHT as u16,
            &values as *const xcb_configure_window_value_list_t as *const std::os::raw::c_void
        ) };
    }
}
*/

impl Drop for Window {

    fn drop(&mut self) {
        unsafe {
            sys::xcb_unmap_window(self.system.xcb_connection,self.xcb_window as u32);
            sys::xcb_destroy_window(self.system.xcb_connection,self.xcb_window as u32);
        }
    }
}
