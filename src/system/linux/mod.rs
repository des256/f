use crate::*;

mod system;
pub use system::*;

mod window;
pub use window::*;

pub fn xcb_code_to_string(code: u8) -> &'static str {
    match code as u32 {
        sys::XCB_REQUEST => "bad request",
        sys::XCB_VALUE => "bad value",
        sys::XCB_WINDOW => "bad window",
        sys::XCB_PIXMAP => "bad pixmap",
        sys::XCB_ATOM => "bad atom",
        sys::XCB_CURSOR => "bad cursor",
        sys::XCB_FONT => "bad font",
        sys::XCB_MATCH => "bad match",
        sys::XCB_DRAWABLE => "bad drawable",
        sys::XCB_ACCESS => "bad access",
        sys::XCB_ALLOC => "bad alloc",
        sys::XCB_COLORMAP => "bad colormap",
        sys::XCB_G_CONTEXT => "bad G context",
        sys::XCB_ID_CHOICE => "bad ID choice",
        sys::XCB_NAME => "bad name",        
        sys::XCB_LENGTH => "bad length",
        sys::XCB_IMPLEMENTATION => "bad implementation",
        _ => "(unknown)",
    }
}

pub fn xcb_major_to_string(major: u8) -> &'static str {
    match major as u32 {
        sys::XCB_CREATE_WINDOW => "xcb_create_window",
        sys::XCB_CHANGE_WINDOW_ATTRIBUTES => "xcb_change_window_attributes",
        sys::XCB_GET_WINDOW_ATTRIBUTES => "xcb_get_window_attributes",
        sys::XCB_DESTROY_WINDOW => "xcb_destroy_window",
        sys::XCB_DESTROY_SUBWINDOWS => "xcb_destroy_subwindow",
        sys::XCB_CHANGE_SAVE_SET => "xcb_change_save_set",
        sys::XCB_REPARENT_WINDOW => "xcb_reparent_window",
        sys::XCB_MAP_WINDOW => "xcb_map_window",
        sys::XCB_MAP_SUBWINDOWS => "xcb_map_subwindows",
        sys::XCB_UNMAP_WINDOW => "xcb_unmap_window",
        sys::XCB_UNMAP_SUBWINDOWS => "xcb_unmap_subwindows",
        sys::XCB_CONFIGURE_WINDOW => "xcb_configure_window",
        sys::XCB_CIRCULATE_WINDOW => "xcb_circulate_window",
        sys::XCB_GET_GEOMETRY => "xcb_get_geometry",
        sys::XCB_QUERY_TREE => "xcb_query_tree",
        sys::XCB_INTERN_ATOM => "xcb_intern_atom",
        sys::XCB_GET_ATOM_NAME => "xcb_get_atom_name",
        sys::XCB_CHANGE_PROPERTY => "xcb_change_property",
        sys::XCB_DELETE_PROPERTY => "xcb_delete_property",
        sys::XCB_GET_PROPERTY => "xcb_get_property",
        sys::XCB_LIST_PROPERTIES => "xcb_list_properties",
        sys::XCB_SET_SELECTION_OWNER => "xcb_set_selection_owner",
        sys::XCB_GET_SELECTION_OWNER => "xcb_get_selection_owner",
        sys::XCB_CONVERT_SELECTION => "xcb_convert_selection",
        sys::XCB_SEND_EVENT => "xcb_send_event",
        sys::XCB_GRAB_POINTER => "xcb_grab_pointer",
        sys::XCB_UNGRAB_POINTER => "xcb_ungrab_pointer",
        sys::XCB_GRAB_BUTTON => "xcb_grab_button",
        sys::XCB_UNGRAB_BUTTON => "xcb_ungrab_button",
        sys::XCB_CHANGE_ACTIVE_POINTER_GRAB => "xcb_change_active_pointer_grab",
        sys::XCB_GRAB_KEYBOARD => "xcb_grab_keyboard",
        sys::XCB_UNGRAB_KEYBOARD => "xcb_ungrab_keyboard",
        sys::XCB_GRAB_KEY => "xcb_grab_key",
        sys::XCB_UNGRAB_KEY => "xcb_ungrab_key",
        sys::XCB_ALLOW_EVENTS => "xcb_allow_events",
        sys::XCB_GRAB_SERVER => "xcb_grab_server",
        sys::XCB_UNGRAB_SERVER => "xcb_ungrab_server",
        sys::XCB_QUERY_POINTER => "xcb_query_pointer",
        sys::XCB_GET_MOTION_EVENTS => "xcb_get_motion_events",
        sys::XCB_TRANSLATE_COORDINATES => "xcb_translate_coordinates",
        sys::XCB_WARP_POINTER => "xcb_warp_pointer",
        sys::XCB_SET_INPUT_FOCUS => "xcb_set_input_focus",
        sys::XCB_GET_INPUT_FOCUS => "xcb_get_input_focus",
        sys::XCB_QUERY_KEYMAP => "xcb_query_keymap",
        sys::XCB_OPEN_FONT => "xcb_open_font",
        sys::XCB_CLOSE_FONT => "xcb_close_font",
        sys::XCB_QUERY_FONT => "xcb_query_font",
        sys::XCB_QUERY_TEXT_EXTENTS => "xcb_query_text_extents",
        sys::XCB_LIST_FONTS => "xcb_list_fonts",
        sys::XCB_LIST_FONTS_WITH_INFO => "xcb_list_fonts_with_info",
        sys::XCB_SET_FONT_PATH => "xcb_set_font_path",
        sys::XCB_GET_FONT_PATH => "xcb_get_font_path",
        sys::XCB_CREATE_PIXMAP => "xcb_create_pixmap",
        sys::XCB_FREE_PIXMAP => "xcb_free_pixmap",
        sys::XCB_CREATE_GC => "xcb_create_gc",
        sys::XCB_CHANGE_GC => "xcb_change_gc",
        sys::XCB_COPY_GC => "xcb_copy_gc",
        sys::XCB_SET_DASHES => "xcb_set_dashes",
        sys::XCB_SET_CLIP_RECTANGLES => "xcb_set_clip_rectangles",
        sys::XCB_FREE_GC => "xcb_free_gc",
        sys::XCB_CLEAR_AREA => "xcb_clear_area",
        sys::XCB_COPY_AREA => "xcb_copy_area",
        sys::XCB_COPY_PLANE => "xcb_copy_plane",
        sys::XCB_POLY_POINT => "xcb_poly_point",
        sys::XCB_POLY_LINE => "xcb_poly_line",
        sys::XCB_POLY_SEGMENT => "xcb_poly_segment",
        sys::XCB_POLY_RECTANGLE => "xcb_poly_rectangle",
        sys::XCB_POLY_ARC => "xcb_poly_arc",
        sys::XCB_FILL_POLY => "xcb_fill_poly",
        sys::XCB_POLY_FILL_RECTANGLE => "xcb_poly_fill_rectangle",
        sys::XCB_POLY_FILL_ARC => "xcb_poly_fill_arc",
        sys::XCB_PUT_IMAGE => "xcb_put_image",
        sys::XCB_GET_IMAGE => "xcb_get_image",
        sys::XCB_POLY_TEXT_8 => "xcb_poly_text_8",
        sys::XCB_POLY_TEXT_16 => "xcb_poly_text_16",
        sys::XCB_IMAGE_TEXT_8 => "xcb_image_text_8",
        sys::XCB_IMAGE_TEXT_16 => "xcb_image_text_16",
        sys::XCB_CREATE_COLORMAP => "xcb_create_colormap",
        sys::XCB_FREE_COLORMAP => "xcb_free_colormap",
        sys::XCB_COPY_COLORMAP_AND_FREE => "xcb_copy_colormap_and_free",
        sys::XCB_INSTALL_COLORMAP => "xcb_install_colormap",
        sys::XCB_UNINSTALL_COLORMAP => "xcb_uninstall_colormap",
        sys::XCB_LIST_INSTALLED_COLORMAPS => "xcb_list_installed_colormaps",
        sys::XCB_ALLOC_COLOR => "xcb_alloc_color",
        sys::XCB_ALLOC_NAMED_COLOR => "xcb_alloc_named_color",
        sys::XCB_ALLOC_COLOR_CELLS => "xcb_alloc_color_cells",
        sys::XCB_ALLOC_COLOR_PLANES => "xcb_alloc_color_planes",
        sys::XCB_FREE_COLORS => "xcb_free_colors",
        sys::XCB_STORE_COLORS => "xcb_store_colors",
        sys::XCB_STORE_NAMED_COLOR => "xcb_store_named_color",
        sys::XCB_QUERY_COLORS => "xcb_query_colors",
        sys::XCB_LOOKUP_COLOR => "xcb_lookup_color",
        sys::XCB_CREATE_CURSOR => "xcb_create_cursor",
        sys::XCB_CREATE_GLYPH_CURSOR => "xcb_create_glyph_cursor",
        sys::XCB_FREE_CURSOR => "xcb_free_cursor",
        sys::XCB_RECOLOR_CURSOR => "xcb_recolor_cursor",
        sys::XCB_QUERY_BEST_SIZE => "xcb_query_best_size",
        sys::XCB_QUERY_EXTENSION => "xcb_query_extension",
        sys::XCB_LIST_EXTENSIONS => "xcb_list_extensions",
        sys::XCB_CHANGE_KEYBOARD_MAPPING => "xcb_change_keyboard_mapping",
        sys::XCB_GET_KEYBOARD_MAPPING => "xcb_get_keyboard_mapping",
        sys::XCB_CHANGE_KEYBOARD_CONTROL => "xcb_change_keyboard_control",
        sys::XCB_GET_KEYBOARD_CONTROL => "xcb_get_keyboard_control",
        sys::XCB_BELL => "xcb_bell",
        sys::XCB_CHANGE_POINTER_CONTROL => "xcb_change_pointer_control",
        sys::XCB_GET_POINTER_CONTROL => "xcb_get_pointer_control",
        sys::XCB_SET_SCREEN_SAVER => "xcb_set_screen_saver",
        sys::XCB_GET_SCREEN_SAVER => "xcb_get_screen_saver",
        sys::XCB_CHANGE_HOSTS => "xcb_change_hosts",
        sys::XCB_LIST_HOSTS => "xcb_list_hosts",
        sys::XCB_SET_ACCESS_CONTROL => "xcb_set_access_control",
        sys::XCB_SET_CLOSE_DOWN_MODE => "xcb_set_close_down_mode",
        sys::XCB_KILL_CLIENT => "xcb_kill_client",
        sys::XCB_ROTATE_PROPERTIES => "xcb_rotate_properties",
        sys::XCB_FORCE_SCREEN_SAVER => "xcb_force_screen_saver",
        sys::XCB_SET_POINTER_MAPPING => "xcb_set_pointer_mapping",
        sys::XCB_GET_POINTER_MAPPING => "xcb_get_pointer_mapping",
        sys::XCB_SET_MODIFIER_MAPPING => "xcb_set_modifier_mapping",
        sys::XCB_GET_MODIFIER_MAPPING => "xcb_get_modifier_mapping",
        sys::XCB_NO_OPERATION => "xcb_no_operation",
        _ => "(unknown)",
    }
}