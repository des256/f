use {
    crate::*,
    super::*,
    std::{
        result::Result,
        rc::Rc,
        ptr::null_mut,
        mem::MaybeUninit,
    },
};

#[derive(Debug)]
pub struct VulkanGpu {
    pub system: Rc<System>,
    pub vk_instance: sys::VkInstance,
    pub vk_physical_device: sys::VkPhysicalDevice,
    pub vk_device: sys::VkDevice,
    pub vk_queue: sys::VkQueue,
    pub vk_command_pool: sys::VkCommandPool,
    pub shared_index: usize,
}

impl System {

    pub fn create_vulkan_gpu(self: &Rc<System>) -> Result<Rc<VulkanGpu>,String> {

        // create instance
        let extension_names = [
            sys::VK_KHR_SURFACE_EXTENSION_NAME.as_ptr(),
#[cfg(system="linux")]
            sys::VK_KHR_XCB_SURFACE_EXTENSION_NAME.as_ptr(),
        ];
        let info = sys::VkInstanceCreateInfo {
            sType: sys::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pApplicationInfo: &sys::VkApplicationInfo {
                sType: sys::VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: null_mut(),
                pApplicationName: b"f::System\0".as_ptr() as *const i8,
                applicationVersion: (1 << 22) as u32,
                pEngineName: b"f::VulkanGpu\0".as_ptr() as *const i8,
                engineVersion: (1 << 22) as u32,
                apiVersion: ((1 << 22) | (2 << 11)) as u32,
            },
            enabledExtensionCount: extension_names.len() as u32,
            ppEnabledExtensionNames: extension_names.as_ptr() as *const *const i8,
            enabledLayerCount: 0,
            flags: 0,
            pNext: null_mut(),
            ppEnabledLayerNames: null_mut(),
        };
        let mut vk_instance = MaybeUninit::<sys::VkInstance>::uninit();
        match unsafe { sys::vkCreateInstance(&info,null_mut(),vk_instance.as_mut_ptr()) } {
            sys::VK_SUCCESS => { },
            code => return Err(format!("System::create_vulkan_gpu: unable to create VkInstance ({})",super::vk_code_to_string(code))),
        }
        let vk_instance = unsafe { vk_instance.assume_init() };

        // enumerate physical devices
        let mut count = MaybeUninit::<u32>::uninit();
        unsafe { sys::vkEnumeratePhysicalDevices(vk_instance,count.as_mut_ptr(),null_mut()) };
        let count = unsafe { count.assume_init() };
        if count == 0 {
            unsafe { sys::vkDestroyInstance(vk_instance,null_mut()) };
            return Err("System::create_vulkan_gpu: unable to enumerate physical devices".to_string());
        }
        let mut vk_physical_devices = vec![null_mut(); count as usize];
        unsafe { sys::vkEnumeratePhysicalDevices(vk_instance,&count as *const u32 as *mut u32,vk_physical_devices.as_mut_ptr()) };

#[cfg(build="debug")]
        {
            dprintln!("System::create_vulkan_gpu: physical devices:");
            vk_physical_devices.iter().for_each(|vk_physical_device| {
                let mut properties = MaybeUninit::<sys::VkPhysicalDeviceProperties>::uninit();
                unsafe { sys::vkGetPhysicalDeviceProperties(*vk_physical_device,properties.as_mut_ptr()) };
                let properties = unsafe { properties.assume_init() };
                let slice: &[u8] = unsafe { &*(&properties.deviceName as *const [i8] as *const [u8]) };
                dprintln!("System::create_vulkan_gpu:     {}",std::str::from_utf8(slice).unwrap());
            });
        }

        // get first physical device
        dprintln!("System::create_vulkan_gpu: choosing first device");
        let vk_physical_device = vk_physical_devices[0];
        
        // get supported queue families
        let mut count = 0u32;
        unsafe { sys::vkGetPhysicalDeviceQueueFamilyProperties(vk_physical_device,&mut count as *mut u32,null_mut()) };
        if count == 0 {
            unsafe { sys::vkDestroyInstance(vk_instance,null_mut()) };
            return Err("System::create_vulkan_gpu: no queue families supported on this GPU".to_string());
        }
        let mut vk_queue_families = vec![MaybeUninit::<sys::VkQueueFamilyProperties>::uninit(); count as usize];
        unsafe { sys::vkGetPhysicalDeviceQueueFamilyProperties(
            vk_physical_device,
            &count as *const u32 as *mut u32,
            vk_queue_families.as_mut_ptr() as *mut sys::VkQueueFamilyProperties,
        ) };
        let vk_queue_families = unsafe { std::mem::transmute::<_,Vec<sys::VkQueueFamilyProperties>>(vk_queue_families) };

        // DEBUG: display the number of queues and capabilities
#[cfg(build="debug")]
        {
            dprintln!("System::create_vulkan_gpu: supported queue families:");
            vk_queue_families.iter().for_each(|vk_queue_family| {
                let mut capabilities = String::new();
                if vk_queue_family.queueFlags & sys::VK_QUEUE_GRAPHICS_BIT != 0 {
                    capabilities.push_str("graphics ");
                }
                if vk_queue_family.queueFlags & sys::VK_QUEUE_TRANSFER_BIT != 0 {
                    capabilities.push_str("transfer ");
                }
                if vk_queue_family.queueFlags & sys::VK_QUEUE_COMPUTE_BIT != 0 {
                    capabilities.push_str("compute ");
                }
                if vk_queue_family.queueFlags & sys::VK_QUEUE_SPARSE_BINDING_BIT != 0 {
                    capabilities.push_str("sparse ");
                }
                dprintln!("System::create_vulkan_gpu:     - {} queues, capable of: {}",vk_queue_family.queueCount,capabilities);
            });
        }

        // assume the first queue family is the one we want for all queues
        dprintln!("System::create_vulkan_gpu: choosing first family");
        let vk_queue_family = vk_queue_families[0];
        let mask = sys::VK_QUEUE_GRAPHICS_BIT | sys::VK_QUEUE_TRANSFER_BIT | sys::VK_QUEUE_COMPUTE_BIT;
        if (vk_queue_family.queueFlags & mask) != mask {
            unsafe { sys::vkDestroyInstance(vk_instance,null_mut()) };
            return Err("System::create_vulkan_gpu: first queue family does not support graphics, transfer and compute operations".to_string());
        }

        // assume that presentation is done on the same family as graphics and create logical device with one queue of queue family 0
        let mut queue_create_infos = Vec::<sys::VkDeviceQueueCreateInfo>::new();
        let priority = 1f32;
        queue_create_infos.push(sys::VkDeviceQueueCreateInfo {
            sType: sys::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            queueFamilyIndex: 0,
            queueCount: 1,
            pQueuePriorities: &priority as *const f32,
        });
        let extension_names = [
            sys::VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr(),
        ];
        let info = sys::VkDeviceCreateInfo {
            sType: sys::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            queueCreateInfoCount: queue_create_infos.len() as u32,
            pQueueCreateInfos: queue_create_infos.as_ptr(),
            enabledLayerCount: 0,
            ppEnabledLayerNames: null_mut(),
            enabledExtensionCount: extension_names.len() as u32,
            ppEnabledExtensionNames: extension_names.as_ptr() as *const *const i8,
            pEnabledFeatures: &sys::VkPhysicalDeviceFeatures {
                robustBufferAccess: 0,
                fullDrawIndexUint32: 0,
                imageCubeArray: 0,
                independentBlend: 0,
                geometryShader: 0,
                tessellationShader: 0,
                sampleRateShading: 0,
                dualSrcBlend: 0,
                logicOp: 0,
                multiDrawIndirect: 0,
                drawIndirectFirstInstance: 0,
                depthClamp: 0,
                depthBiasClamp: 0,
                fillModeNonSolid: 0,
                depthBounds: 0,
                wideLines: 0,
                largePoints: 0,
                alphaToOne: 0,
                multiViewport: 0,
                samplerAnisotropy: 0,
                textureCompressionETC2: 0,
                textureCompressionASTC_LDR: 0,
                textureCompressionBC: 0,
                occlusionQueryPrecise: 0,
                pipelineStatisticsQuery: 0,
                vertexPipelineStoresAndAtomics: 0,
                fragmentStoresAndAtomics: 0,
                shaderTessellationAndGeometryPointSize: 0,
                shaderImageGatherExtended: 0,
                shaderStorageImageExtendedFormats: 0,
                shaderStorageImageMultisample: 0,
                shaderStorageImageReadWithoutFormat: 0,
                shaderStorageImageWriteWithoutFormat: 0,
                shaderUniformBufferArrayDynamicIndexing: 0,
                shaderSampledImageArrayDynamicIndexing: 0,
                shaderStorageBufferArrayDynamicIndexing: 0,
                shaderStorageImageArrayDynamicIndexing: 0,
                shaderClipDistance: 0,
                shaderCullDistance: 0,
                shaderFloat64: 0,
                shaderInt64: 0,
                shaderInt16: 0,
                shaderResourceResidency: 0,
                shaderResourceMinLod: 0,
                sparseBinding: 0,
                sparseResidencyBuffer: 0,
                sparseResidencyImage2D: 0,
                sparseResidencyImage3D: 0,
                sparseResidency2Samples: 0,
                sparseResidency4Samples: 0,
                sparseResidency8Samples: 0,
                sparseResidency16Samples: 0,
                sparseResidencyAliased: 0,
                variableMultisampleRate: 0,
                inheritedQueries: 0,
            },
        };
        let mut vk_device = MaybeUninit::<sys::VkDevice>::uninit();
        match unsafe { sys::vkCreateDevice(vk_physical_device,&info,null_mut(),vk_device.as_mut_ptr()) } {
            sys::VK_SUCCESS => { },
            code => { 
                unsafe { sys::vkDestroyInstance(vk_instance,null_mut()) };
                return Err(format!("System::create_vulkan_gpu: unable to create VkDevice ({})",super::vk_code_to_string(code)));
            },
        }
        let vk_device = unsafe { vk_device.assume_init() };

        // obtain the queue from queue family 0
        let mut vk_queue = MaybeUninit::<sys::VkQueue>::uninit();
        unsafe { sys::vkGetDeviceQueue(vk_device,0,0,vk_queue.as_mut_ptr()) };
        let vk_queue = unsafe { vk_queue.assume_init() };

        // create command pool for this queue
        let info = sys::VkCommandPoolCreateInfo {
            sType: sys::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            pNext: null_mut(),
            flags: sys::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
            queueFamilyIndex: 0,
        };
        let mut vk_command_pool = MaybeUninit::<sys::VkCommandPool>::uninit();
        match unsafe { sys::vkCreateCommandPool(vk_device,&info,null_mut(),vk_command_pool.as_mut_ptr()) } {
            sys::VK_SUCCESS => { },
            code => {
                unsafe {
                    sys::vkDestroyDevice(vk_device,null_mut());
                    sys::vkDestroyInstance(vk_instance,null_mut());
                }
                return Err(format!("System::create_vulkan_gpu: unable to create command pool ({})",super::vk_code_to_string(code)));
            },
        }
        let vk_command_pool = unsafe { vk_command_pool.assume_init() };

        // get memory properties
        let mut vk_memory_properties = MaybeUninit::<sys::VkPhysicalDeviceMemoryProperties>::uninit();
        unsafe { sys::vkGetPhysicalDeviceMemoryProperties(vk_physical_device,vk_memory_properties.as_mut_ptr()) };
        let vk_memory_properties = unsafe { vk_memory_properties.assume_init() };

        // DEBUG: show the entire memory description
#[cfg(build="debug")]
        {
            dprintln!("System::create_vulkan_gpu: device memory properties:");
            dprintln!("System::create_vulkan_gpu:     memory heaps:");
            for i in 0..vk_memory_properties.memoryHeapCount as usize {
                dprintln!("System::create_vulkan_gpu:         {}: size {} MiB, {:X}",i,vk_memory_properties.memoryHeaps[i].size / (1024 * 1024),vk_memory_properties.memoryHeaps[i].flags);
            }
            dprintln!("System::create_vulkan_gpu:     memory types:");
            for i in 0..vk_memory_properties.memoryTypeCount as usize {
                let mut flags = String::new();
                let vk_memory_type = &vk_memory_properties.memoryTypes[i];
                if (vk_memory_type.propertyFlags & sys::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT) != 0 {
                    flags += "device_local ";
                }
                if (vk_memory_type.propertyFlags & sys::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT) != 0 {
                    flags += "host_visible ";
                }
                if (vk_memory_type.propertyFlags & sys::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT) != 0 {
                    flags += "host_coherent ";
                }
                if (vk_memory_type.propertyFlags & sys::VK_MEMORY_PROPERTY_HOST_CACHED_BIT) != 0 {
                    flags += "host_cached ";
                }
                if (vk_memory_type.propertyFlags & sys::VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT) != 0 {
                    flags += "lazily_allocated ";
                }
                if (vk_memory_type.propertyFlags & sys::VK_MEMORY_PROPERTY_PROTECTED_BIT) != 0 {
                    flags += "protected ";
                }
                dprintln!("System::create_vulkan_gpu:         - on heap {}, {}",vk_memory_type.heapIndex,flags);
            }
        }

        // find shared memory heap and type (later also find device-only index)
        let mask = sys::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT | sys::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | sys::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT;
        let valid_types: Vec<(usize,&sys::VkMemoryType)> = vk_memory_properties.memoryTypes.iter().enumerate().filter(|vk_memory_type| (vk_memory_type.1.propertyFlags & mask) == mask).collect();
        if valid_types.is_empty() {
            return Err("System::create_vulkan_gpu: no valid memory types found".to_string());
        }
        let shared_index = valid_types[0].0;

        Ok(Rc::new(VulkanGpu {
            system: Rc::clone(&self),
            vk_instance,
            vk_physical_device,
            vk_device,
            vk_queue,
            vk_command_pool,
            shared_index,
        }))
    }
}

impl VulkanGpu {

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
                return Err(format!("VulkanGpu::build_swapchain_resources: unable to get surface capabilities ({})",super::vk_code_to_string(code)));
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
            self.vk_physical_device,
            vk_surface,
            &mut count as *mut u32,
            null_mut(),
        ) } {
            sys::VK_SUCCESS => { },
            code => {
                return Err(format!("VulkanGpu::build_swapchain_resources: unable to get surface formats ({})",super::vk_code_to_string(code)));
            },
        }
        let mut formats = vec![MaybeUninit::<sys::VkSurfaceFormatKHR>::uninit(); count as usize];
        match unsafe { sys::vkGetPhysicalDeviceSurfaceFormatsKHR(
            self.vk_physical_device,
            vk_surface,
            &mut count,
            formats.as_mut_ptr() as *mut sys::VkSurfaceFormatKHR,
        ) } {
            sys::VK_SUCCESS => { },
            code => {
                return Err(format!("VulkanGpu::build_swapchain_resources: unable to get surface formats ({})",super::vk_code_to_string(code)));
            }
        }
        let formats = unsafe { std::mem::transmute::<_,Vec<sys::VkSurfaceFormatKHR>>(formats) };
        let format_supported = formats.iter().any(|vk_format| (vk_format.format == sys::VK_FORMAT_B8G8R8A8_SRGB) && (vk_format.colorSpace == sys::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR));
        if !format_supported {
            return Err("VulkanGpu::build_swapchain_resources: window does not support BGRA8UN at SRGB".to_string());
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
            self.vk_device,
            &info,
            null_mut(),
            &mut vk_swapchain as *mut sys::VkSwapchainKHR,
        ) } {
            sys::VK_SUCCESS => { },
            code => {
                return Err(format!("VulkanGpu::build_swapchain_resources: unable to create swap chain ({})",super::vk_code_to_string(code)));
            },
        }

        // get swapchain images
        let mut count = 0u32;
        match unsafe { sys::vkGetSwapchainImagesKHR(self.vk_device,vk_swapchain,&mut count as *mut u32,null_mut()) } {
            sys::VK_SUCCESS => { },
            code => {
                unsafe { sys::vkDestroySwapchainKHR(self.vk_device,vk_swapchain,null_mut()) };
                return Err(format!("VulkanGpu::build_swapchain_resources: unable to get swap chain image count ({})",super::vk_code_to_string(code)));
            },
        }
        let mut vk_images = vec![MaybeUninit::<sys::VkImage>::uninit(); count as usize];
        match unsafe { sys::vkGetSwapchainImagesKHR(
            self.vk_device,
            vk_swapchain,
            &count as *const u32 as *mut u32,
            vk_images.as_mut_ptr() as *mut sys::VkImage,
        ) } {
            sys::VK_SUCCESS => { },
            code => {
                unsafe { sys::vkDestroySwapchainKHR(self.vk_device,vk_swapchain,null_mut()) };
                return Err(format!("VulkanGpu::build_swapchain_resources: unable to get swap chain images ({})",super::vk_code_to_string(code)));
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
            match unsafe { sys::vkCreateImageView(self.vk_device,&info,null_mut(),&mut vk_image_view) } {
                sys::VK_SUCCESS => Ok(vk_image_view),
                code => Err(format!("VulkanGpu::build_swapchain_resources: unable to create image view ({})",super::vk_code_to_string(code))),
            }
        }).collect();
        if results.iter().any(|result| result.is_err()) {
            results.iter().for_each(|result| if let Ok(vk_image_view) = result { unsafe { sys::vkDestroyImageView(self.vk_device,*vk_image_view,null_mut()) } });
            unsafe { sys::vkDestroySwapchainKHR(self.vk_device,vk_swapchain,null_mut()); }
            return Err("VulkanGpu::build_swapchain_resources: unable to create image view".to_string());
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
            match unsafe { sys::vkCreateFramebuffer(self.vk_device,&info,null_mut(),vk_framebuffer.as_mut_ptr()) } {
                sys::VK_SUCCESS => Ok(unsafe { vk_framebuffer.assume_init() }),
                code => Err(format!("VulkanGpu::build_swapchain_resources: unable to create framebuffer ({})",super::vk_code_to_string(code))),
            }
        }).collect();
        if results.iter().any(|result| result.is_err()) {
            results.iter().for_each(|result| if let Ok(vk_framebuffer) = result { unsafe { sys::vkDestroyFramebuffer(self.vk_device,*vk_framebuffer,null_mut()) } });
            vk_image_views.iter().for_each(|vk_image_view| unsafe { sys::vkDestroyImageView(self.vk_device,*vk_image_view,null_mut()) });
            return Err("VulkanGpu::build_swapchain_resources: unable to create framebuffer".to_string());
        }
        let vk_framebuffers: Vec<sys::VkFramebuffer> = results.into_iter().map(|result| result.unwrap()).collect();

        Ok((vk_swapchain,vk_image_views,vk_framebuffers))
    }
}

impl Gpu for VulkanGpu {

    type Surface = VulkanSurface;

    fn create_surface(self: &Rc<Self>,window: &Rc<Window>,r: Rect<i32>) -> Result<VulkanSurface,String> {

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
                    return Err(format!("VulkanGpu::create_surface: Unable to create Vulkan XCB surface ({})",super::vk_code_to_string(code)));
                },
            }
            unsafe { vk_surface.assume_init() }
        };

        // verify the surface is supported for the current physical device
        let mut supported = MaybeUninit::<sys::VkBool32>::uninit();
        match unsafe { sys::vkGetPhysicalDeviceSurfaceSupportKHR(self.vk_physical_device,0,vk_surface,supported.as_mut_ptr()) } {
            sys::VK_SUCCESS => { },
            code => {
                return Err(format!("VulkanGpu::create_surface: Surface not supported on physical device ({})",super::vk_code_to_string(code)));
            },
        }
        let supported = unsafe { supported.assume_init() };
        if supported == sys::VK_FALSE {
            return Err("VulkanGpu::create_surface: Surface not supported on physical device".to_string());
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
                return Err(format!("VulkanGpu::create_surface: unable to create render pass ({})",super::vk_code_to_string(code)));
            }
        }
        let vk_render_pass = unsafe { vk_render_pass.assume_init() };

        let (vk_swapchain,vk_image_views,vk_framebuffers) = self.build_swapchain_resources(vk_surface,vk_render_pass,r)?;

        // create surface
        let mut surface = VulkanSurface {
            gpu: Rc::clone(&self),
            window: Rc::clone(&window),
            vk_surface,
            vk_render_pass,
            vk_swapchain,
            vk_framebuffers,
            vk_image_views,
        };

        surface.set_rect(r)?;

        Ok(surface)
    }
}
