use {
    crate::*,
    super::*,
    std::{
        rc::Rc,
        ptr::null_mut,
    },
};

#[derive(Debug)]
pub struct VulkanSurface {
    pub gpu: Rc<VulkanGpu>,
    pub window: Rc<Window>,
    pub vk_surface: sys::VkSurfaceKHR,
    pub vk_render_pass: sys::VkRenderPass,
    pub vk_swapchain: sys::VkSwapchainKHR,
    pub vk_framebuffers: Vec<sys::VkFramebuffer>,
    pub vk_image_views: Vec<sys::VkImageView>,
}

impl Surface for VulkanSurface {

    fn set_rect(&mut self,r: Rect<i32>) -> Result<(),String> {

        // create new swapchain resources
        let (vk_swapchain,vk_image_views,vk_framebuffers) = self.gpu.build_swapchain_resources(self.vk_surface,self.vk_render_pass,r)?;

        // destroy old swapchain resources
        if self.vk_framebuffers.len() > 0 {
            self.vk_image_views.iter().for_each(|vk_image_view| unsafe { sys::vkDestroyImageView(self.gpu.vk_device,*vk_image_view,null_mut()) });
            self.vk_framebuffers.iter().for_each(|vk_framebuffer| unsafe { sys::vkDestroyFramebuffer(self.gpu.vk_device,*vk_framebuffer,null_mut()) });
            unsafe { sys::vkDestroySwapchainKHR(self.gpu.vk_device,self.vk_swapchain,null_mut()); }    
        }

        // install new resources
        self.vk_swapchain = vk_swapchain;
        self.vk_image_views = vk_image_views;
        self.vk_framebuffers = vk_framebuffers;

        Ok(())
    }

    fn get_swapchain_count(&self) -> usize {
        self.vk_framebuffers.len()
    }

    fn acquire(&self) -> Result<usize,String> {
        let mut index = 0u32;
        match unsafe {
            sys::vkAcquireNextImageKHR(
                self.gpu.vk_device,
                self.vk_swapchain,
                0xFFFFFFFFFFFFFFFF,
                null_mut(),
                null_mut(),
                &mut index,
            )
        } {
            sys::VK_SUCCESS => Ok(index as usize),
            code => Err(format!("VulkanSurface::acquire: unable to acquire next image ({})",vk_code_to_string(code))),
        }
    }

    fn present(&self,index: usize) -> Result<(),String> {
        let image_index = index as u32;
        let info = sys::VkPresentInfoKHR {
            sType: sys::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
            pNext: null_mut(),
            waitSemaphoreCount: 0,
            pWaitSemaphores: null_mut(),
            swapchainCount: 1,
            pSwapchains: &self.vk_swapchain,
            pImageIndices: &image_index,
            pResults: null_mut(),
        };
        match unsafe { sys::vkQueuePresentKHR(self.gpu.vk_queue,&info) } {
            sys::VK_SUCCESS => Ok(()),
            code => Err(format!("VulkanSurface::present: unable to present image ({})",vk_code_to_string(code))),
        }
    }
}

impl Drop for VulkanSurface {

    fn drop(&mut self) {
        self.vk_image_views.iter().for_each(|vk_image_view| unsafe { sys::vkDestroyImageView(self.gpu.vk_device,*vk_image_view,null_mut()) });
        self.vk_framebuffers.iter().for_each(|vk_framebuffer| unsafe { sys::vkDestroyFramebuffer(self.gpu.vk_device,*vk_framebuffer,null_mut()) });
        unsafe {
            sys::vkDestroySwapchainKHR(self.gpu.vk_device,self.vk_swapchain,null_mut());
            sys::vkDestroySurfaceKHR(self.gpu.vk_instance,self.vk_surface,null_mut());
            sys::vkDestroyRenderPass(self.gpu.vk_device,self.vk_render_pass,null_mut());
        }
    }
}
