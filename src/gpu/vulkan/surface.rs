use {
    super::*,
    crate::gpu,
    std::{
        rc::Rc,
        mem::MaybeUninit,
        ptr::null_mut,
    },
};

#[derive(Debug)]
pub struct Surface {
    pub gpu: Rc<Gpu>,
    pub window: Rc<Window>,
    pub vk_surface: sys::VkSurfaceKHR,
    pub vk_render_pass: sys::VkRenderPass,
    pub vk_swapchain: sys::VkSwapchainKHR,
    pub vk_framebuffers: Vec<sys::VkFramebuffer>,
    pub vk_image_views: Vec<sys::VkImageView>,
}

impl Gpu {

    pub fn create_surface(self: &Rc<Self>,window: &Rc<Window>,r: Rect<i32>) -> Result<Surface,String> {

        // create surface for this window
#[cfg(system="linux")]
        let vk_surface = {
            let info = sys::VkXcbSurfaceCreateInfoKHR {
                sType: sys::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
                pNext: null_mut(),
                flags: 0,
                connection: window.system.xcb_connection,
                window: window.xcb_window,
            };
            let mut vk_surface = MaybeUninit::<sys::VkSurfaceKHR>::uninit();
            match unsafe { sys::vkCreateXcbSurfaceKHR(self.vk_instance,&info,null_mut(),vk_surface.as_mut_ptr()) } {
                sys::VK_SUCCESS => { },
                code => {
                    return Err(format!("vulkan::Gpu::create_surface: Unable to create Vulkan XCB surface ({})",vk_code_to_string(code)));
                },
            }
            unsafe { vk_surface.assume_init() }
        };

        // verify the surface is supported for the current physical device
        let mut supported = MaybeUninit::<sys::VkBool32>::uninit();
        match unsafe { sys::vkGetPhysicalDeviceSurfaceSupportKHR(self.vk_physical_device,0,vk_surface,supported.as_mut_ptr()) } {
            sys::VK_SUCCESS => { },
            code => {
                return Err(format!("vulkan::Gpu::create_surface: Surface not supported on physical device ({})",vk_code_to_string(code)));
            },
        }
        let supported = unsafe { supported.assume_init() };
        if supported == sys::VK_FALSE {
            return Err("vulkan::Gpu::create_surface: Surface not supported on physical device".to_string());
        }

        // create render pass

        // A render pass describes the buffers and how they interact for a specific rendering type. This is probably helpful for the GPU to optimize tiling.
        let info = sys::VkRenderPassCreateInfo {
            sType: sys::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            attachmentCount: 1,
            pAttachments: &sys::VkAttachmentDescription {
                flags: 0,
                format: sys::VK_FORMAT_B8G8R8A8_SRGB,
                samples: sys::VK_SAMPLE_COUNT_1_BIT,
                loadOp: sys::VK_ATTACHMENT_LOAD_OP_CLEAR,
                storeOp: sys::VK_ATTACHMENT_STORE_OP_STORE,
                stencilLoadOp: sys::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                stencilStoreOp: sys::VK_ATTACHMENT_STORE_OP_DONT_CARE,
                initialLayout: sys::VK_IMAGE_LAYOUT_UNDEFINED,
                finalLayout: sys::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
            },
            subpassCount: 1,
            pSubpasses: &sys::VkSubpassDescription {
                flags: 0,
                pipelineBindPoint: sys::VK_PIPELINE_BIND_POINT_GRAPHICS,
                inputAttachmentCount: 0,
                pInputAttachments: null_mut(),
                colorAttachmentCount: 1,
                pColorAttachments: &sys::VkAttachmentReference {
                    attachment: 0,
                    layout: sys::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
                },
                pResolveAttachments: null_mut(),
                pDepthStencilAttachment: null_mut(),
                preserveAttachmentCount: 0,
                pPreserveAttachments: null_mut(),
            },
            dependencyCount: 1,
            pDependencies: &sys::VkSubpassDependency {
                srcSubpass: sys::VK_SUBPASS_EXTERNAL as u32,
                dstSubpass: 0,
                srcStageMask: sys::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                dstStageMask: sys::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                srcAccessMask: 0,
                dstAccessMask: sys::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
                dependencyFlags: 0,
            },
        };
        let mut vk_render_pass = MaybeUninit::uninit();
        match unsafe { sys::vkCreateRenderPass(self.vk_device,&info,null_mut(),vk_render_pass.as_mut_ptr()) } {
            sys::VK_SUCCESS => { },
            code => {
                unsafe { sys::vkDestroySurfaceKHR(self.vk_instance,vk_surface,null_mut()) };
                return Err(format!("vulkan::Gpu::create_surface: unable to create render pass ({})",vk_code_to_string(code)));
            }
        }
        let vk_render_pass = unsafe { vk_render_pass.assume_init() };

        let (vk_swapchain,vk_image_views,vk_framebuffers) = self.build_swapchain_resources(vk_surface,vk_render_pass,r)?;

        // create surface
        let mut surface = Surface {
            gpu: Rc::clone(&self),
            window: Rc::clone(&window),
            vk_surface,
            vk_render_pass,
            vk_swapchain,
            vk_framebuffers,
            vk_image_views,
        };

        {
            use crate::gpu::Surface;
            surface.set_rect(r)?;
        }

        Ok(surface)
    }

    pub fn build_swapchain_resources(&self,vk_surface: sys::VkSurfaceKHR,vk_render_pass: sys::VkRenderPass,r: Rect<i32>) -> Result<(sys::VkSwapchainKHR,Vec<sys::VkImageView>,Vec<sys::VkFramebuffer>),String> {

        // get surface capabilities to calculate the extent and image count
        let mut capabilities = MaybeUninit::<sys::VkSurfaceCapabilitiesKHR>::uninit();
        match unsafe { sys::vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
            self.vk_physical_device,
            vk_surface,
            capabilities.as_mut_ptr(),
        ) } {
            sys::VK_SUCCESS => { },
            code => {
                return Err(format!("vulkan::Gpu::build_swapchain_resources: unable to get surface capabilities ({})",vk_code_to_string(code)));
            },
        }
        let capabilities = unsafe { capabilities.assume_init() };

        // get current extent, if any
        let extent = if capabilities.currentExtent.width != 0xFFFFFFFF {
            Vec2 {
                x: capabilities.currentExtent.width,
                y: capabilities.currentExtent.height,
            }
        }

        // otherwise take window size as extent, and make sure it fits the constraints
        else {
            let mut extent = Vec2 { x: r.s.x as u32,y: r.s.y as u32, };
            if extent.x < capabilities.minImageExtent.width {
                extent.x = capabilities.minImageExtent.width;
            }
            if extent.y < capabilities.minImageExtent.height {
                extent.y = capabilities.minImageExtent.height;
            }
            if extent.x > capabilities.maxImageExtent.width {
                extent.x = capabilities.maxImageExtent.width;
            }
            if extent.y > capabilities.maxImageExtent.height {
                extent.y = capabilities.maxImageExtent.height;
            }
            extent
        };

        // make sure VK_FORMAT_B8G8R8A8_SRGB is supported (BGRA8UN)
        let mut count = 0u32;
        match unsafe { sys::vkGetPhysicalDeviceSurfaceFormatsKHR(
            gpu.vk_physical_device,
            vk_surface,
            &mut count as *mut u32,
            null_mut(),
        ) } {
            sys::VK_SUCCESS => { },
            code => {
                return Err(format!("vulkan::Gpu::build_swapchain_resources: unable to get surface formats ({})",vk_code_to_string(code)));
            },
        }
        let mut formats = vec![MaybeUninit::<sys::VkSurfaceFormatKHR>::uninit(); count as usize];
        match unsafe { sys::vkGetPhysicalDeviceSurfaceFormatsKHR(
            gpu.vk_physical_device,
            vk_surface,
            &mut count,
            formats.as_mut_ptr() as *mut sys::VkSurfaceFormatKHR,
        ) } {
            sys::VK_SUCCESS => { },
            code => {
                return Err(format!("vulkan::Gpu::build_swapchain_resources: unable to get surface formats ({})",vk_code_to_string(code)));
            }
        }
        let formats = unsafe { std::mem::transmute::<_,Vec<sys::VkSurfaceFormatKHR>>(formats) };
        let format_supported = formats.iter().any(|vk_format| (vk_format.format == sys::VK_FORMAT_B8G8R8A8_SRGB) && (vk_format.colorSpace == sys::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR));
        if !format_supported {
            return Err("window does not support BGRA8UN at SRGB".to_string());
        }

        // create swapchain for this window
        let info = sys::VkSwapchainCreateInfoKHR {
            sType: sys::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            pNext: null_mut(),
            flags: 0,
            surface: vk_surface,
            minImageCount: capabilities.minImageCount,
            imageFormat: sys::VK_FORMAT_B8G8R8A8_SRGB,
            imageColorSpace: sys::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
            imageExtent: sys::VkExtent2D { width: extent.x,height: extent.y, },
            imageArrayLayers: 1,
            imageUsage: sys::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
            imageSharingMode: sys::VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: null_mut(),
            preTransform: capabilities.currentTransform,
            compositeAlpha: sys::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            presentMode: sys::VK_PRESENT_MODE_FIFO_KHR,
            clipped: sys::VK_TRUE,
            oldSwapchain: null_mut(),
        };        
        let mut vk_swapchain: sys::VkSwapchainKHR = null_mut();
        match unsafe { sys::vkCreateSwapchainKHR(
            gpu.vk_device,
            &info,
            null_mut(),
            &mut vk_swapchain as *mut sys::VkSwapchainKHR,
        ) } {
            sys::VK_SUCCESS => { },
            code => {
                return Err(format!("vulkan::Gpu::build_swapchain_resources: unable to create swap chain ({})",vk_code_to_string(code)));
            },
        }

        // get swapchain images
        let mut count = 0u32;
        match unsafe { sys::vkGetSwapchainImagesKHR(gpu.vk_device,vk_swapchain,&mut count as *mut u32,null_mut()) } {
            sys::VK_SUCCESS => { },
            code => {
                unsafe { sys::vkDestroySwapchainKHR(gpu.vk_device,vk_swapchain,null_mut()) };
                return Err(format!("vulkan::Gpu::build_swapchain_resources: unable to get swap chain image count ({})",vk_code_to_string(code)));
            },
        }
        let mut vk_images = vec![MaybeUninit::<sys::VkImage>::uninit(); count as usize];
        match unsafe { sys::vkGetSwapchainImagesKHR(
            gpu.vk_device,
            vk_swapchain,
            &count as *const u32 as *mut u32,
            vk_images.as_mut_ptr() as *mut sys::VkImage,
        ) } {
            sys::VK_SUCCESS => { },
            code => {
                unsafe { sys::vkDestroySwapchainKHR(gpu.vk_device,vk_swapchain,null_mut()) };
                return Err(format!("vulkan::Gpu::build_swapchain_resources: unable to get swap chain images ({})",vk_code_to_string(code)));
            },
        }
        let vk_images = unsafe { std::mem::transmute::<_,Vec<sys::VkImage>>(vk_images) };

        // create image views for the swapchain images
        let results: Vec<Result<sys::VkImageView,String>> = vk_images.iter().map(|vk_image| {
            let info = sys::VkImageViewCreateInfo {
                sType: sys::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                image: *vk_image,
                viewType: sys::VK_IMAGE_VIEW_TYPE_2D,
                format: sys::VK_FORMAT_B8G8R8A8_SRGB,
                components: sys::VkComponentMapping {
                    r: sys::VK_COMPONENT_SWIZZLE_IDENTITY,
                    g: sys::VK_COMPONENT_SWIZZLE_IDENTITY,
                    b: sys::VK_COMPONENT_SWIZZLE_IDENTITY,
                    a: sys::VK_COMPONENT_SWIZZLE_IDENTITY,
                },
                subresourceRange: sys::VkImageSubresourceRange {
                    aspectMask: sys::VK_IMAGE_ASPECT_COLOR_BIT,
                    baseMipLevel: 0,
                    levelCount: 1,
                    baseArrayLayer: 0,
                    layerCount: 1,
                },
            };
            let mut vk_image_view: sys::VkImageView = null_mut();
            match unsafe { sys::vkCreateImageView(gpu.vk_device,&info,null_mut(),&mut vk_image_view) } {
                sys::VK_SUCCESS => Ok(vk_image_view),
                code => Err(format!("vulkan::Gpu::build_swapchain_resources: unable to create image view ({})",vk_code_to_string(code))),
            }
        }).collect();
        if results.iter().any(|result| result.is_err()) {
            results.iter().for_each(|result| if let Ok(vk_image_view) = result { unsafe { sys::vkDestroyImageView(gpu.vk_device,*vk_image_view,null_mut()) } });
            unsafe { sys::vkDestroySwapchainKHR(gpu.vk_device,vk_swapchain,null_mut()); }
            return Err("vulkan::Gpu::build_swapchain_resources: unable to create image view".to_string());
        }
        let vk_image_views: Vec<sys::VkImageView> = results.into_iter().map(|result| result.unwrap()).collect();

        // create framebuffers for the image views
        let results: Vec<Result<sys::VkFramebuffer,String>> = vk_image_views.iter().map(|vk_image_view| {
            let info = sys::VkFramebufferCreateInfo {
                sType: sys::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                renderPass: vk_render_pass,
                attachmentCount: 1,
                pAttachments: vk_image_view,
                width: extent.x,
                height: extent.y,
                layers: 1,
            };
            let mut vk_framebuffer = MaybeUninit::uninit();
            match unsafe { sys::vkCreateFramebuffer(gpu.vk_device,&info,null_mut(),vk_framebuffer.as_mut_ptr()) } {
                sys::VK_SUCCESS => Ok(unsafe { vk_framebuffer.assume_init() }),
                code => Err(format!("vulkan::Gpu::build_swapchain_resources: unable to create framebuffer ({})",vk_code_to_string(code))),
            }
        }).collect();
        if results.iter().any(|result| result.is_err()) {
            results.iter().for_each(|result| if let Ok(vk_framebuffer) = result { unsafe { sys::vkDestroyFramebuffer(gpu.vk_device,*vk_framebuffer,null_mut()) } });
            vk_image_views.iter().for_each(|vk_image_view| unsafe { sys::vkDestroyImageView(gpu.vk_device,*vk_image_view,null_mut()) });
            return Err("vulkan::Gpu::build_swapchain_resources: unable to create framebuffer".to_string());
        }
        let vk_framebuffers: Vec<sys::VkFramebuffer> = results.into_iter().map(|result| result.unwrap()).collect();

        Ok((vk_swapchain,vk_image_views,vk_framebuffers))
    }
}

impl gpu::Surface for Surface {

    fn set_rect(&mut self,r: Rect<i32>) -> Result<(),String> {

        let (vk_swapchain,vk_image_views,vk_framebuffers) = Self::build_swapchain_resources(&self.gpu,self.vk_surface,self.vk_render_pass,r)?;

        // destroy current framebuffers, image views and swap chain
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
            code => Err(format!("vulkan::Gpu::build_swapchain_resources: unable to acquire next image ({})",vk_code_to_string(code))),
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
            code => Err(format!("vulkan::Gpu::build_swapchain_resources: unable to present image ({})",vk_code_to_string(code))),
        }
    }
}

impl Drop for Surface {
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
