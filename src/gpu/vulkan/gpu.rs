use {
    super::*,
    crate::gpu,
    std::{
        result::Result,
        rc::Rc,
        ptr::{
            null_mut,
            copy_nonoverlapping,
        },
        ffi::c_void,
        mem::MaybeUninit,
    },
};

// Supplemental fields for System
#[derive(Debug)]
pub struct Gpu {
    pub system: Rc<System>,
    pub vk_instance: sys::VkInstance,
    pub vk_physical_device: sys::VkPhysicalDevice,
    pub vk_device: sys::VkDevice,
    pub vk_queue: sys::VkQueue,
    pub vk_command_pool: sys::VkCommandPool,
    pub shared_index: usize,
}

impl gpu::Gpu for Gpu {

    type CommandBuffer = CommandBuffer;
    type VertexShader = VertexShader;
    type FragmentShader = FragmentShader;
    type PipelineLayout = PipelineLayout;

    fn open(system: &Rc<System>) -> Result<Rc<Gpu>,String> {

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
                pApplicationName: b"e::System\0".as_ptr() as *const i8,
                applicationVersion: (1 << 22) as u32,
                pEngineName: b"e::GpuSystem\0".as_ptr() as *const i8,
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
            code => return Err(format!("unable to create VkInstance ({})",vk_code_to_string(code))),
        }
        let vk_instance = unsafe { vk_instance.assume_init() };

        // enumerate physical devices
        let mut count = MaybeUninit::<u32>::uninit();
        unsafe { sys::vkEnumeratePhysicalDevices(vk_instance,count.as_mut_ptr(),null_mut()) };
        let count = unsafe { count.assume_init() };
        if count == 0 {
            unsafe { sys::vkDestroyInstance(vk_instance,null_mut()) };
            return Err("unable to enumerate physical devices".to_string());
        }
        let mut vk_physical_devices = vec![null_mut(); count as usize];
        unsafe { sys::vkEnumeratePhysicalDevices(vk_instance,&count as *const u32 as *mut u32,vk_physical_devices.as_mut_ptr()) };

#[cfg(build="debug")]
        {
            dprintln!("physical devices:");
            vk_physical_devices.iter().for_each(|vk_physical_device| {
                let mut properties = MaybeUninit::<sys::VkPhysicalDeviceProperties>::uninit();
                unsafe { sys::vkGetPhysicalDeviceProperties(*vk_physical_device,properties.as_mut_ptr()) };
                let properties = unsafe { properties.assume_init() };
                let slice: &[u8] = unsafe { &*(&properties.deviceName as *const [i8] as *const [u8]) };
                dprintln!("    {}",std::str::from_utf8(slice).unwrap());
            });
        }

        // get first physical device
        dprintln!("choosing first device");
        let vk_physical_device = vk_physical_devices[0];
        
        // get supported queue families
        let mut count = 0u32;
        unsafe { sys::vkGetPhysicalDeviceQueueFamilyProperties(vk_physical_device,&mut count as *mut u32,null_mut()) };
        if count == 0 {
            unsafe { sys::vkDestroyInstance(vk_instance,null_mut()) };
            return Err("no queue families supported on this GPU".to_string());
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
            dprintln!("supported queue families:");
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
                dprintln!("    - {} queues, capable of: {}",vk_queue_family.queueCount,capabilities);
            });
        }

        // assume the first queue family is the one we want for all queues
        dprintln!("choosing first family");
        let vk_queue_family = vk_queue_families[0];
        let mask = sys::VK_QUEUE_GRAPHICS_BIT | sys::VK_QUEUE_TRANSFER_BIT | sys::VK_QUEUE_COMPUTE_BIT;
        if (vk_queue_family.queueFlags & mask) != mask {
            unsafe { sys::vkDestroyInstance(vk_instance,null_mut()) };
            return Err("first queue family does not support graphics, transfer and compute operations".to_string());
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
                return Err(format!("unable to create VkDevice ({})",vk_code_to_string(code)));
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
                return Err(format!("unable to create command pool ({})",vk_code_to_string(code)));
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
            dprintln!("device memory properties:");
            dprintln!("    memory heaps:");
            for i in 0..vk_memory_properties.memoryHeapCount as usize {
                dprintln!("        {}: size {} MiB, {:X}",i,vk_memory_properties.memoryHeaps[i].size / (1024 * 1024),vk_memory_properties.memoryHeaps[i].flags);
            }
            dprintln!("    memory types:");
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
                dprintln!("        - on heap {}, {}",vk_memory_type.heapIndex,flags);
            }
        }

        // find shared memory heap and type (later also find device-only index)
        let mask = sys::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT | sys::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | sys::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT;
        let valid_types: Vec<(usize,&sys::VkMemoryType)> = vk_memory_properties.memoryTypes.iter().enumerate().filter(|vk_memory_type| (vk_memory_type.1.propertyFlags & mask) == mask).collect();
        if valid_types.is_empty() {
            return Err("no valid memory types found".to_string());
        }
        let shared_index = valid_types[0].0;

        Ok(Rc::new(Gpu {
            system: Rc::clone(&system),
            vk_instance,
            vk_physical_device,
            vk_device,
            vk_queue,
            vk_command_pool,
            shared_index,
        }))
    }

    fn create_surface(self: &Rc<Self>,window: &Rc<Window>,r: Rect<i32>) -> Result<Surface,String> {
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
                    return Err(format!("Unable to create Vulkan XCB surface ({})",vk_code_to_string(code)));
                },
            }
            unsafe { vk_surface.assume_init() }
        };

        // verify the surface is supported for the current physical device
        let mut supported = MaybeUninit::<sys::VkBool32>::uninit();
        match unsafe { sys::vkGetPhysicalDeviceSurfaceSupportKHR(self.vk_physical_device,0,vk_surface,supported.as_mut_ptr()) } {
            sys::VK_SUCCESS => { },
            code => {
                return Err(format!("Surface not supported on physical device ({})",vk_code_to_string(code)));
            },
        }
        let supported = unsafe { supported.assume_init() };
        if supported == sys::VK_FALSE {
            return Err("Surface not supported on physical device".to_string());
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
                return Err(format!("unable to create render pass ({})",vk_code_to_string(code)));
            }
        }
        let vk_render_pass = unsafe { vk_render_pass.assume_init() };

        let (vk_swapchain,vk_image_views,vk_framebuffers) = Surface::build_swapchain_resources(&self,vk_surface,vk_render_pass,r)?;

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

    /// Create command buffer.
    fn create_command_buffer(self: &Rc<Self>) -> Result<CommandBuffer,String> {

        let info = sys::VkCommandBufferAllocateInfo {
            sType: sys::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: null_mut(),
            commandPool: self.vk_command_pool,
            level: sys::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            commandBufferCount: 1,
        };
        let mut vk_command_buffer = MaybeUninit::uninit();
        match unsafe { sys::vkAllocateCommandBuffers(self.vk_device,&info,vk_command_buffer.as_mut_ptr()) } {
            sys::VK_SUCCESS => Ok(CommandBuffer {
                gpu: Rc::clone(&self),
                vk_command_buffer: unsafe { vk_command_buffer.assume_init() },
            }),
            code => Err(format!("Unable to create command buffer ({})",vk_code_to_string(code))),
        }
    }

    /// Submit command_buffer to the queue.
    fn submit_command_buffer(&self,command_buffer: &CommandBuffer) -> Result<(),String> {
        let wait_stage = sys::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
        let info = sys::VkSubmitInfo {
            sType: sys::VK_STRUCTURE_TYPE_SUBMIT_INFO,
            pNext: null_mut(),
            waitSemaphoreCount: 0,
            pWaitSemaphores: null_mut(),
            pWaitDstStageMask: &wait_stage,
            commandBufferCount: 1,
            pCommandBuffers: &command_buffer.vk_command_buffer,
            signalSemaphoreCount: 0,
            pSignalSemaphores: null_mut(),
        };
        match unsafe { sys::vkQueueSubmit(self.vk_queue,1,&info,null_mut()) } {
            sys::VK_SUCCESS => Ok(()),
            code => Err(format!("Unable to submit command buffer to graphics queue ({})",vk_code_to_string(code))),
        }
    }

    fn create_vertex_shader(self: &Rc<Self>,ast: &gpu::sc::Module) -> Result<VertexShader,String> {

        dprintln!("Vulkan Vertex Shader AST:\n{}",ast);
        let ast = gpu::sc::process(ast)?;
        dprintln!("Vulkan Vertex Shader AST after processing:\n{}",ast);
        //let ast = gpu::sc::destructure_module(&ast)?;
        //dprintln!("Vulkan Vertex Shader AST after destructuring:\n{}",ast);

        //let module = resolve(ast);

        /*
        let create_info = sys::VkShaderModuleCreateInfo {
            sType: sys::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            codeSize: code.len() as u64,
            pCode: code.as_ptr() as *const u32,
        };
        let mut vk_shader_module = MaybeUninit::uninit();
        match unsafe { sys::vkCreateShaderModule(self.vk_device,&create_info,null_mut(),vk_shader_module.as_mut_ptr()) } {
            sys::VK_SUCCESS => Ok(VertexShader {
                gpu: Rc::clone(&self),
                vk_shader_module: unsafe { vk_shader_module.assume_init() },
            }),
            code => Err(format!("Unable to create vertex shader ({})",vk_code_to_string(code))),
        }
        */
        Err("TODO: SPIR-V compiler".to_string())
    }

    fn create_fragment_shader(self: &Rc<Self>,ast: &gpu::sc::Module) -> Result<FragmentShader,String> {

        dprintln!("Vulkan Fragment Shader AST:\n{}",ast);
        let ast = gpu::sc::process(ast)?;
        dprintln!("Vulkan Fragment Shader AST after preparing:\n{}",ast);
        //let ast = gpu::sc::destructure_module(&ast)?;
        //dprintln!("Vulkan Fragment Shader AST after destructuring:\n{}",ast);


        //let module = resolve(ast);

        /*
        let create_info = sys::VkShaderModuleCreateInfo {
            sType: sys::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            codeSize: code.len() as u64,
            pCode: code.as_ptr() as *const u32,
        };
        let mut vk_shader_module = MaybeUninit::uninit();
        match unsafe { sys::vkCreateShaderModule(self.vk_device,&create_info,null_mut(),vk_shader_module.as_mut_ptr()) } {
            sys::VK_SUCCESS => Ok(FragmentShader {
                gpu: Rc::clone(&self),
                vk_shader_module: unsafe { vk_shader_module.assume_init() },
            }),
            code => Err(format!("Unable to create fragment shader ({})",vk_code_to_string(code))),
        }
        */
        Err("TODO: SPIR-V compiler".to_string())
    }

    fn create_vertex_buffer<T: gpu::Vertex>(self: &Rc<Self>,vertices: &Vec<T>) -> Result<VertexBuffer,String> {

        // obtain vertex info
        let vertex_struct = T::ast();
        let mut vertex_stride = 0usize;
        for field in vertex_struct.fields.iter() {
            vertex_stride += gpu::type_to_size(&field.1)?;
        }

        // create vertex buffer
        let info = sys::VkBufferCreateInfo {
            sType: sys::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            size: (vertices.len() * vertex_stride) as u64,
            usage: sys::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT,
            sharingMode: sys::VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: null_mut(),
        };
        let mut vk_buffer = MaybeUninit::uninit();
        match unsafe { sys::vkCreateBuffer(self.vk_device, &info, null_mut(), vk_buffer.as_mut_ptr()) } {
            sys::VK_SUCCESS => { },
            code => return Err(format!("Unable to create vertex buffer ({})",vk_code_to_string(code))),
        }
        let vk_buffer = unsafe { vk_buffer.assume_init() };

        // allocate shared memory
        let info = sys::VkMemoryAllocateInfo {
            sType: sys::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
            pNext: null_mut(),
            allocationSize: (vertices.len() * vertex_stride) as u64,
            memoryTypeIndex: self.shared_index as u32,
        };
        let mut vk_device_memory = MaybeUninit::<sys::VkDeviceMemory>::uninit();
        match unsafe { sys::vkAllocateMemory(self.vk_device,&info,null_mut(),vk_device_memory.as_mut_ptr()) } {
            sys::VK_SUCCESS => { },
            code => return Err(format!("Unable to allocate memory ({})",vk_code_to_string(code))),
        }
        let vk_device_memory = unsafe { vk_device_memory.assume_init() };

        // map memory
        let mut data_ptr = MaybeUninit::<*mut c_void>::uninit();
        match unsafe { sys::vkMapMemory(
            self.vk_device,
            vk_device_memory,
            0,
            sys::VK_WHOLE_SIZE as u64,
            0,
            data_ptr.as_mut_ptr(),
        ) } {
            sys::VK_SUCCESS => { },
            code => return Err(format!("Unable to map memory ({})",vk_code_to_string(code))),
        }
        let data_ptr = unsafe { data_ptr.assume_init() } as *mut T;

        // copy from the input vertices into data
        unsafe { copy_nonoverlapping(vertices.as_ptr(),data_ptr,vertices.len()) };

        // and unmap the memory again
        unsafe { sys::vkUnmapMemory(self.vk_device,vk_device_memory) };

        // bind to vertex buffer
        match unsafe { sys::vkBindBufferMemory(self.vk_device,vk_buffer,vk_device_memory,0) } {
            sys::VK_SUCCESS => Ok(VertexBuffer {
                gpu: Rc::clone(&self),
                vk_buffer,
                vk_device_memory,
            }),
            code => Err(format!("Unable to bind memory to vertex buffer ({})",vk_code_to_string(code))),
        }
    }

    fn create_index_buffer<T>(self: &Rc<Self>,indices: &Vec<T>) -> Result<IndexBuffer,String> {
        // create index buffer
        let info = sys::VkBufferCreateInfo {
            sType: sys::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            size: (indices.len() * 4) as u64,
            usage: sys::VK_BUFFER_USAGE_INDEX_BUFFER_BIT,
            sharingMode: sys::VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: null_mut(),
        };
        let mut vk_buffer = MaybeUninit::uninit();
        match unsafe { sys::vkCreateBuffer(self.vk_device, &info, null_mut(), vk_buffer.as_mut_ptr()) } {
            sys::VK_SUCCESS => { },
            code => return Err(format!("Unable to create index buffer ({})",vk_code_to_string(code))),
        }
        let vk_buffer = unsafe { vk_buffer.assume_init() };

        // allocate shared memory
        let info = sys::VkMemoryAllocateInfo {
            sType: sys::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
            pNext: null_mut(),
            allocationSize: (indices.len() * 4) as u64,
            memoryTypeIndex: self.shared_index as u32,
        };
        let mut vk_memory = MaybeUninit::<sys::VkDeviceMemory>::uninit();
        match unsafe { sys::vkAllocateMemory(self.vk_device,&info,null_mut(),vk_memory.as_mut_ptr()) } {
            sys::VK_SUCCESS => { },
            code => return Err(format!("Unable to allocate memory ({})",vk_code_to_string(code))),
        }
        let vk_memory = unsafe { vk_memory.assume_init() };

        // map memory
        let mut data_ptr = MaybeUninit::<*mut c_void>::uninit();
        match unsafe { sys::vkMapMemory(
            self.vk_device,
            vk_memory,
            0,
            sys::VK_WHOLE_SIZE as u64,
            0,
            data_ptr.as_mut_ptr(),
        ) } {
            sys::VK_SUCCESS => { },
            code => return Err(format!("Unable to map memory ({})",vk_code_to_string(code))),
        }
        let data_ptr = unsafe { data_ptr.assume_init() } as *mut T;

        // copy from the input vertices into data
        unsafe { copy_nonoverlapping(indices.as_ptr(),data_ptr,indices.len()) };

        // and unmap the memory again
        unsafe { sys::vkUnmapMemory(self.vk_device,vk_memory) };

        // bind to vertex buffer
        match unsafe { sys::vkBindBufferMemory(self.vk_device,vk_buffer,vk_memory,0) } {
            sys::VK_SUCCESS => Ok(IndexBuffer {
                gpu: Rc::clone(&self),
                vk_buffer,
                vk_memory,
            }),
            code => Err(format!("Unable to bind memory to index buffer ({})",vk_code_to_string(code))),
        }
    }

    fn create_pipeline_layout(self: &Rc<Self>) -> Result<PipelineLayout,String> {

        let info = sys::VkPipelineLayoutCreateInfo {
            sType: sys::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            setLayoutCount: 0,
            pSetLayouts: null_mut(),
            pushConstantRangeCount: 0,
            pPushConstantRanges: null_mut(),
        };
        let mut vk_pipeline_layout = MaybeUninit::uninit();
        match unsafe { sys::vkCreatePipelineLayout(self.vk_device,&info,null_mut(),vk_pipeline_layout.as_mut_ptr()) } {
            sys::VK_SUCCESS => Ok(PipelineLayout {
                gpu: Rc::clone(&self),
                vk_pipeline_layout: unsafe { vk_pipeline_layout.assume_init() },
            }),
            code => Err(format!("Unable to create pipeline layout ({})",vk_code_to_string(code))),
        }
    }
}
