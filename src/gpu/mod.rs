pub trait Gpu {
    
}

/*
pub trait ComputePipeline {

}

pub trait PipelineLayout {

}

pub trait VertexBuffer {

}

pub trait IndexBuffer {

}

pub trait Framebuffer {

}

pub trait ComputeShader {

}

pub trait CommandBuffer {
    type Surface : Surface;
    type ComputePipeline : ComputePipeline;
    type VertexBuffer : VertexBuffer;
    type IndexBuffer : IndexBuffer;
    fn begin(&self) -> Result<(),String>;
    fn end(&self) -> bool;
    fn begin_render_pass(&self,surface: &Self::Surface,index: usize,r: Rect<i32>);
    fn end_render_pass(&self);
    fn bind_compute_pipeline(&self,pipeline: &Rc<Self::ComputePipeline>);
    fn bind_vertex_buffer(&self,vertex_buffer: &Rc<Self::VertexBuffer>);
    fn bind_index_buffer(&self,index_buffer: &Rc<Self::IndexBuffer>);
    fn draw(&self,vertex_count: usize,instance_count: usize,first_vertex: usize, first_instance: usize);
    fn draw_indexed(&self,index_count: usize,instance_count: usize,first_index: usize,vertex_offset: usize,first_instance: usize);
    fn set_viewport(&self,r: Rect<i32>,min_depth: f32,max_depth: f32);
    fn set_scissor(&self,r: Rect<i32>);
}

pub trait Surface {
    fn set_rect(&mut self,r: Rect<i32>) -> Result<(),String>;
    fn get_swapchain_count(&self) -> usize;
    fn acquire(&self) -> Result<usize,String>;
    fn present(&self,index: usize) -> Result<(),String>;
}
*/

#[cfg(vulkan)]
pub mod vulkan;
