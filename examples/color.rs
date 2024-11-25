#![allow(non_upper_case_globals)]
use std::time::Duration;

use const_cstr::*;
use fermium::events::*;
use fermium::stdinc::*;
use fermium::video::*;
use fermium::vulkan::{SDL_Vulkan_CreateSurface, SDL_Vulkan_GetInstanceExtensions};
use fermium::*;

use mira::error::MiraError;
use mira::mem::{from_cstring, zeroed_vec};
use mira::version::VK_MAKE_API_VERSION;
use mira::vulkan::*;
use mira::*;

//https://software.intel.com/content/www/us/en/develop/articles/api-without-secrets-introduction-to-vulkan-part-2.html
fn main() -> Result<(), MiraError> {
    unsafe {
        SDL_InitSubSystem(SDL_INIT_VIDEO);
    }

    const FLAGS: u32 = SDL_WINDOW_VULKAN.0 | SDL_WINDOW_ALLOW_HIGHDPI.0 | SDL_WINDOW_RESIZABLE.0;

    let window = unsafe {
        SDL_CreateWindow(
            const_cstr!("mira/color").as_ptr(),
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            500,
            500,
            FLAGS,
        )
    };
    if window == std::ptr::null_mut() {
        return Ok(());
    }

    let window_id = unsafe { SDL_GetWindowID(window) };

    let layers = vec![
        const_cstr!("VK_LAYER_KHRONOS_validation").as_ptr(),
        const_cstr!("VK_LAYER_LUNARG_api_dump").as_ptr(), //too verbose
    ];

    let mut count: u32 = 0;
    unsafe { SDL_Vulkan_GetInstanceExtensions(window, &mut count, std::ptr::null_mut()) };

    let mut extensions = unsafe { zeroed_vec(count as usize) };
    unsafe { SDL_Vulkan_GetInstanceExtensions(window, &mut count, extensions.as_mut_ptr()) };

    let mut instance: VkInstance = unsafe { std::mem::zeroed() };
    let mut app_info: VkApplicationInfo = unsafe { std::mem::zeroed() };
    let mut instance_info: VkInstanceCreateInfo = unsafe { std::mem::zeroed() };

    let create_instance: PFN_vkCreateInstance;
    let destroy_instance: PFN_vkDestroyInstance;

    let destroy_surface: PFN_vkDestroySurfaceKHR;
    let get_surface_support: PFN_vkGetPhysicalDeviceSurfaceSupportKHR;
    let get_surface_capabilities: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR;
    let get_surface_formats: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR;

    let enumerate_adapters: PFN_vkEnumeratePhysicalDevices;
    let get_adapters_properties: PFN_vkGetPhysicalDeviceProperties;

    let get_queues_properties: PFN_vkGetPhysicalDeviceQueueFamilyProperties;
    let create_device: PFN_vkCreateDevice;
    let destroy_device: PFN_vkDestroyDevice;
    let device_wait_idle: PFN_vkDeviceWaitIdle;
    let get_queue: PFN_vkGetDeviceQueue;
    let create_swapchain: PFN_vkCreateSwapchainKHR;
    let destroy_swapchain: PFN_vkDestroySwapchainKHR;
    let create_semaphore: PFN_vkCreateSemaphore;
    let destroy_semaphore: PFN_vkDestroySemaphore;
    let acquire_next_image: PFN_vkAcquireNextImageKHR;
    let queue_submit: PFN_vkQueueSubmit;
    let queue_present: PFN_vkQueuePresentKHR;
    let get_swapchain_images: PFN_vkGetSwapchainImagesKHR;
    let create_command_pool: PFN_vkCreateCommandPool;
    let destroy_command_pool: PFN_vkDestroyCommandPool;
    let allocate_command_buffers: PFN_vkAllocateCommandBuffers;
    let free_command_buffers: PFN_vkFreeCommandBuffers;
    let begin_command_buffer: PFN_vkBeginCommandBuffer;
    let end_command_buffer: PFN_vkEndCommandBuffer;
    let cmd_pipeline_barrier: PFN_vkCmdPipelineBarrier;
    let cmd_clear_color_image: PFN_vkCmdClearColorImage;

    create_instance =
        unsafe { loader::instance(std::ptr::null_mut(), const_cstr!("vkCreateInstance"))? };

    app_info.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    app_info.pApplicationName = const_cstr!("mira/color").as_ptr();
    app_info.applicationVersion = VK_MAKE_API_VERSION(0, 0, 1, 0);
    app_info.apiVersion = VK_MAKE_API_VERSION(0, 1, 0, 0);

    instance_info.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    instance_info.pApplicationInfo = &app_info;
    instance_info.ppEnabledLayerNames = layers.as_ptr();
    instance_info.enabledLayerCount = layers.len() as u32;
    instance_info.ppEnabledExtensionNames = extensions.as_ptr();
    instance_info.enabledExtensionCount = extensions.len() as u32;

    match unsafe { create_instance(&instance_info, std::ptr::null_mut(), &mut instance) } {
        VK_SUCCESS => {}
        error => {
            println!("vulkan error {}", error);
            return Ok(());
        }
    }

    let mut surface: fermium::vulkan::VkSurfaceKHR = unsafe { std::mem::zeroed() };
    if unsafe {
        SDL_Vulkan_CreateSurface(
            window,
            fermium::vulkan::VkInstance(instance as *mut c_void),
            &mut surface,
        )
    } == SDL_FALSE
    {
        println!("surface error");
        return Ok(());
    }
    let surface: VkSurfaceKHR = surface.0 as VkSurfaceKHR;

    destroy_instance = unsafe { loader::instance(instance, const_cstr!("vkDestroyInstance"))? };

    destroy_surface = unsafe { loader::instance(instance, const_cstr!("vkDestroySurfaceKHR"))? };
    get_surface_support = unsafe {
        loader::instance(
            instance,
            const_cstr!("vkGetPhysicalDeviceSurfaceSupportKHR"),
        )?
    };
    get_surface_capabilities = unsafe {
        loader::instance(
            instance,
            const_cstr!("vkGetPhysicalDeviceSurfaceCapabilitiesKHR"),
        )?
    };
    get_surface_formats = unsafe {
        loader::instance(
            instance,
            const_cstr!("vkGetPhysicalDeviceSurfaceFormatsKHR"),
        )?
    };

    enumerate_adapters =
        unsafe { loader::instance(instance, const_cstr!("vkEnumeratePhysicalDevices"))? };
    get_adapters_properties =
        unsafe { loader::instance(instance, const_cstr!("vkGetPhysicalDeviceProperties"))? };
    get_queues_properties = unsafe {
        loader::instance(
            instance,
            const_cstr!("vkGetPhysicalDeviceQueueFamilyProperties"),
        )?
    };
    create_device = unsafe { loader::instance(instance, const_cstr!("vkCreateDevice"))? };

    let mut count: u32 = 0;
    match unsafe { enumerate_adapters(instance, &mut count, std::ptr::null_mut()) } {
        VK_SUCCESS => {}
        error => {
            println!("vulkan error {}", error);
            return Ok(());
        }
    }

    let mut adapters = unsafe { zeroed_vec::<VkPhysicalDevice>(count as usize) };
    match unsafe { enumerate_adapters(instance, &mut count, adapters.as_mut_ptr()) } {
        VK_SUCCESS => {}
        error => {
            println!("vulkan error {}", error);
            return Ok(());
        }
    }

    let mut selected_adapter: VkPhysicalDevice = unsafe { std::mem::zeroed() };
    let gpu_range = VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU..=VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU;

    for adapter in adapters.iter() {
        let mut adapter_properties: VkPhysicalDeviceProperties = unsafe { std::mem::zeroed() };
        unsafe { get_adapters_properties(*adapter, &mut adapter_properties) };

        let name = match unsafe { from_cstring(adapter_properties.deviceName.as_ptr()) } {
            Ok(name) => name,
            _ => continue,
        };

        println!("Adapter name: {}", name);

        if gpu_range.contains(&adapter_properties.deviceType) {
            selected_adapter = *adapter;
            break;
        }
    }

    let mut count: u32 = 0;
    unsafe { get_queues_properties(selected_adapter, &mut count, std::ptr::null_mut()) };

    let mut queues_properties = unsafe { zeroed_vec::<VkQueueFamilyProperties>(count as usize) };
    unsafe { get_queues_properties(selected_adapter, &mut count, queues_properties.as_mut_ptr()) };

    let mut selected = false;
    let queue_capabilities = VK_QUEUE_GRAPHICS_BIT | VK_QUEUE_COMPUTE_BIT;
    let mut selected_queuefamily: u32 = 0;
    for queue_properties in queues_properties.iter().enumerate() {
        let mut support: u32 = 0;
        unsafe {
            get_surface_support(
                selected_adapter,
                queue_properties.0 as u32,
                surface,
                &mut support,
            )
        };

        if (queue_properties.1.queueFlags & queue_capabilities) == queue_capabilities
            && support != 0
        {
            selected_queuefamily = queue_properties.0 as u32;
            selected = true;
            break;
        }
    }

    if !selected {
        println!("adapter not found");
        return Ok(());
    }

    let mut queues_info = unsafe { zeroed_vec::<VkDeviceQueueCreateInfo>(1) };
    queues_info[0].sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
    queues_info[0].queueFamilyIndex = selected_queuefamily;
    queues_info[0].queueCount = 1;
    queues_info[0].pQueuePriorities = &1.0f32;

    let device_extensions = vec![const_cstr!("VK_KHR_swapchain").as_ptr()];

    let mut device_info: VkDeviceCreateInfo = unsafe { std::mem::zeroed() };
    device_info.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
    device_info.pQueueCreateInfos = queues_info.as_ptr();
    device_info.queueCreateInfoCount = queues_info.len() as u32;
    device_info.ppEnabledExtensionNames = device_extensions.as_ptr();
    device_info.enabledExtensionCount = device_extensions.len() as u32;

    let mut device: VkDevice = unsafe { std::mem::zeroed() };
    match unsafe {
        create_device(
            selected_adapter,
            &device_info,
            std::ptr::null_mut(),
            &mut device,
        )
    } {
        VK_SUCCESS => {}
        error => {
            println!("vulkan error {}", error);
            return Ok(());
        }
    }

    destroy_device = unsafe { loader::device(device, const_cstr!("vkDestroyDevice"))? };
    device_wait_idle = unsafe { loader::device(device, const_cstr!("vkDeviceWaitIdle"))? };
    get_queue = unsafe { loader::device(device, const_cstr!("vkGetDeviceQueue"))? };
    create_swapchain = unsafe { loader::device(device, const_cstr!("vkCreateSwapchainKHR"))? };
    destroy_swapchain = unsafe { loader::device(device, const_cstr!("vkDestroySwapchainKHR"))? };
    create_semaphore = unsafe { loader::device(device, const_cstr!("vkCreateSemaphore"))? };
    destroy_semaphore = unsafe { loader::device(device, const_cstr!("vkDestroySemaphore"))? };
    acquire_next_image = unsafe { loader::device(device, const_cstr!("vkAcquireNextImageKHR"))? };
    queue_submit = unsafe { loader::device(device, const_cstr!("vkQueueSubmit"))? };
    queue_present = unsafe { loader::device(device, const_cstr!("vkQueuePresentKHR"))? };
    get_swapchain_images =
        unsafe { loader::device(device, const_cstr!("vkGetSwapchainImagesKHR"))? };
    create_command_pool = unsafe { loader::device(device, const_cstr!("vkCreateCommandPool"))? };
    destroy_command_pool = unsafe { loader::device(device, const_cstr!("vkDestroyCommandPool"))? };
    allocate_command_buffers =
        unsafe { loader::device(device, const_cstr!("vkAllocateCommandBuffers"))? };
    free_command_buffers = unsafe { loader::device(device, const_cstr!("vkFreeCommandBuffers"))? };
    begin_command_buffer = unsafe { loader::device(device, const_cstr!("vkBeginCommandBuffer"))? };
    end_command_buffer = unsafe { loader::device(device, const_cstr!("vkEndCommandBuffer"))? };
    cmd_pipeline_barrier = unsafe { loader::device(device, const_cstr!("vkCmdPipelineBarrier"))? };
    cmd_clear_color_image = unsafe { loader::device(device, const_cstr!("vkCmdClearColorImage"))? };

    let mut queue: VkQueue = unsafe { std::mem::zeroed() };
    unsafe { get_queue(device, selected_queuefamily, 0, &mut queue) };

    let semaphore_info = VkSemaphoreCreateInfo {
        sType: VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
        pNext: std::ptr::null_mut(),
        flags: 0,
    };

    let mut rendering_finished: VkSemaphore = unsafe { std::mem::zeroed() };
    let mut image_available: VkSemaphore = unsafe { std::mem::zeroed() };

    match unsafe {
        create_semaphore(
            device,
            &semaphore_info,
            std::ptr::null_mut(),
            &mut image_available,
        )
    } {
        VK_SUCCESS => {}
        error => {
            println!("vulkan error {}", error);
            return Ok(());
        }
    }

    match unsafe {
        create_semaphore(
            device,
            &semaphore_info,
            std::ptr::null_mut(),
            &mut rendering_finished,
        )
    } {
        VK_SUCCESS => {}
        error => {
            println!("vulkan error {}", error);
            return Ok(());
        }
    }

    let mut command_pool: VkCommandPool = unsafe { std::mem::zeroed() };
    let command_pool_info = VkCommandPoolCreateInfo {
        sType: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
        pNext: std::ptr::null_mut(),
        flags: 0,
        queueFamilyIndex: selected_queuefamily,
    };

    match unsafe {
        create_command_pool(
            device,
            &command_pool_info,
            std::ptr::null_mut(),
            &mut command_pool,
        )
    } {
        VK_SUCCESS => {}
        error => {
            println!("vulkan error {}", error);
            return Ok(());
        }
    }

    let begin_info = VkCommandBufferBeginInfo {
        sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        pNext: std::ptr::null_mut(),
        flags: VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT,
        pInheritanceInfo: std::ptr::null_mut(),
    };

    let clear_color = VkClearColorValue {
        float32: [0.8f32, 0.2f32, 1.0f32, 0.0f32],
    };

    let image_subresource_range = VkImageSubresourceRange {
        aspectMask: VK_IMAGE_ASPECT_COLOR_BIT,
        baseMipLevel: 0,
        levelCount: 1,
        baseArrayLayer: 0,
        layerCount: 1,
    };

    let mut commands: Vec<VkCommandBuffer> = Vec::new();
    let mut swapchain: VkSwapchainKHR = unsafe { std::mem::zeroed() };

    let mut last_event: SDL_Event = unsafe { std::mem::zeroed() };

    loop {
        let mut event: SDL_Event = unsafe { std::mem::zeroed() };
        unsafe { SDL_PollEvent(&mut event) };

        if unsafe { event.type_ } == SDL_WINDOWEVENT
            && unsafe { event.window.windowID } == window_id
        {
            match unsafe { event.window.event } {
                SDL_WINDOWEVENT_SIZE_CHANGED
                | SDL_WINDOWEVENT_MINIMIZED
                | SDL_WINDOWEVENT_EXPOSED => {
                    if unsafe { last_event.window.event } == SDL_WINDOWEVENT_SIZE_CHANGED
                        && unsafe { event.window.event } == SDL_WINDOWEVENT_EXPOSED
                    {
                        continue;
                    }

                    //cleanup
                    if commands.len() != 0 {
                        unsafe {
                            device_wait_idle(device);
                            free_command_buffers(
                                device,
                                command_pool,
                                commands.len() as u32,
                                commands.as_mut_ptr(),
                            );
                            destroy_swapchain(device, swapchain, std::ptr::null_mut());
                        }
                    }

                    //present
                    //swapchain
                    let mut surface_capabilities: VkSurfaceCapabilitiesKHR =
                        unsafe { std::mem::zeroed() };
                    match unsafe {
                        get_surface_capabilities(
                            selected_adapter,
                            surface,
                            &mut surface_capabilities,
                        )
                    } {
                        VK_SUCCESS => {}
                        error => {
                            println!("vulkan error {}", error);
                            return Ok(());
                        }
                    }

                    let image_usage;
                    let required_image_usage: u32 =
                        VK_IMAGE_USAGE_TRANSFER_DST_BIT | VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
                    if (surface_capabilities.supportedUsageFlags & required_image_usage)
                        != required_image_usage
                    {
                        println!("VK_IMAGE_USAGE_TRANSFER_DST image usage is not supported");
                        return Ok(());
                    }
                    image_usage = required_image_usage;

                    let mut count: u32 = 0;
                    unsafe {
                        get_surface_formats(
                            selected_adapter,
                            surface,
                            &mut count,
                            std::ptr::null_mut(),
                        )
                    };

                    let mut surface_formats =
                        unsafe { zeroed_vec::<VkSurfaceFormatKHR>(count as usize) };
                    unsafe {
                        get_surface_formats(
                            selected_adapter,
                            surface,
                            &mut count,
                            surface_formats.as_mut_ptr(),
                        )
                    };

                    let mut selected_format: VkSurfaceFormatKHR = unsafe { std::mem::zeroed() };
                    for surface_format in surface_formats {
                        if surface_format.format == VK_FORMAT_B8G8R8A8_SRGB
                            && surface_format.colorSpace == VK_COLORSPACE_SRGB_NONLINEAR_KHR
                        {
                            selected_format = surface_format;
                            break;
                        }
                    }

                    if !selected_format.colorSpace == 0 {
                        println!("format not found");
                        return Ok(());
                    }

                    let image_count = surface_capabilities.minImageCount + 1;

                    let swapchain_info = VkSwapchainCreateInfoKHR {
                        sType: VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
                        pNext: std::ptr::null_mut(),
                        flags: 0,
                        surface,
                        minImageCount: surface_capabilities.minImageCount + 1,
                        imageFormat: selected_format.format,
                        imageColorSpace: selected_format.colorSpace,
                        imageExtent: surface_capabilities.currentExtent,
                        imageArrayLayers: 1,
                        imageUsage: image_usage,
                        imageSharingMode: VK_SHARING_MODE_EXCLUSIVE,
                        queueFamilyIndexCount: 0,
                        pQueueFamilyIndices: std::ptr::null_mut(),
                        presentMode: VK_PRESENT_MODE_FIFO_KHR,
                        preTransform: surface_capabilities.currentTransform,
                        compositeAlpha: VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
                        clipped: 1,
                        oldSwapchain: std::ptr::null_mut(),
                    };

                    commands = unsafe { zeroed_vec::<VkCommandBuffer>(image_count as usize) };

                    let command_buffer_info = VkCommandBufferAllocateInfo {
                        sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
                        pNext: std::ptr::null_mut(),
                        commandPool: command_pool,
                        level: VK_COMMAND_BUFFER_LEVEL_PRIMARY,
                        commandBufferCount: commands.len() as u32,
                    };

                    match unsafe {
                        allocate_command_buffers(
                            device,
                            &command_buffer_info,
                            commands.as_mut_ptr(),
                        )
                    } {
                        VK_SUCCESS => {}
                        error => {
                            println!("vulkan error {}", error);
                            return Ok(());
                        }
                    }

                    match unsafe {
                        create_swapchain(
                            device,
                            &swapchain_info,
                            std::ptr::null_mut(),
                            &mut swapchain,
                        )
                    } {
                        VK_SUCCESS => {}
                        error => {
                            println!("vulkan error {}", error);
                            return Ok(());
                        }
                    }

                    let mut image_count: u32 = 0;
                    match unsafe {
                        get_swapchain_images(
                            device,
                            swapchain,
                            &mut image_count,
                            std::ptr::null_mut(),
                        )
                    } {
                        VK_SUCCESS => {}
                        error => {
                            println!("vulkan error {}", error);
                            return Ok(());
                        }
                    }

                    let mut images = unsafe { zeroed_vec::<VkImage>(image_count as usize) };
                    match unsafe {
                        get_swapchain_images(
                            device,
                            swapchain,
                            &mut image_count,
                            images.as_mut_ptr(),
                        )
                    } {
                        VK_SUCCESS => {}
                        error => {
                            println!("vulkan error {}", error);
                            return Ok(());
                        }
                    }

                    let mut next_image: u32 = 0;
                    match unsafe {
                        acquire_next_image(
                            device,
                            swapchain,
                            u64::MAX,
                            image_available,
                            std::ptr::null_mut(),
                            &mut next_image,
                        )
                    } {
                        VK_SUCCESS => {}
                        error => {
                            println!("vulkan error {}", error);
                            return Ok(());
                        }
                    }

                    for command in commands.iter().enumerate() {
                        let from_present_to_clear = VkImageMemoryBarrier {
                            sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
                            pNext: std::ptr::null_mut(),
                            srcAccessMask: VK_ACCESS_MEMORY_READ_BIT,
                            dstAccessMask: VK_ACCESS_TRANSFER_WRITE_BIT,
                            oldLayout: VK_IMAGE_LAYOUT_UNDEFINED,
                            newLayout: VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                            srcQueueFamilyIndex: selected_queuefamily,
                            dstQueueFamilyIndex: selected_queuefamily,
                            image: images[command.0],
                            subresourceRange: image_subresource_range,
                        };

                        let from_clear_to_present = VkImageMemoryBarrier {
                            sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
                            pNext: std::ptr::null_mut(),
                            dstAccessMask: VK_ACCESS_MEMORY_READ_BIT,
                            srcAccessMask: VK_ACCESS_TRANSFER_WRITE_BIT,
                            newLayout: VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
                            oldLayout: VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                            srcQueueFamilyIndex: selected_queuefamily,
                            dstQueueFamilyIndex: selected_queuefamily,
                            image: images[command.0],
                            subresourceRange: image_subresource_range,
                        };

                        unsafe {
                            let cmd = *command.1;
                            begin_command_buffer(cmd, &begin_info);

                            cmd_pipeline_barrier(
                                cmd,
                                VK_PIPELINE_STAGE_TRANSFER_BIT,
                                VK_PIPELINE_STAGE_TRANSFER_BIT,
                                0,
                                0,
                                std::ptr::null_mut(),
                                0,
                                std::ptr::null_mut(),
                                1,
                                &from_present_to_clear,
                            );
                            cmd_clear_color_image(
                                cmd,
                                images[command.0],
                                VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                                &clear_color,
                                1,
                                &image_subresource_range,
                            );
                            cmd_pipeline_barrier(
                                cmd,
                                VK_PIPELINE_STAGE_TRANSFER_BIT,
                                VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
                                0,
                                0,
                                std::ptr::null_mut(),
                                0,
                                std::ptr::null_mut(),
                                1,
                                &from_clear_to_present,
                            );

                            match end_command_buffer(cmd) {
                                VK_SUCCESS => {}
                                error => {
                                    println!("vulkan error {}", error);
                                    return Ok(());
                                }
                            }
                        };
                    }

                    let submit_info = VkSubmitInfo {
                        sType: VK_STRUCTURE_TYPE_SUBMIT_INFO,
                        pNext: std::ptr::null_mut(),
                        waitSemaphoreCount: 1,
                        pWaitSemaphores: &image_available,
                        pWaitDstStageMask: &VK_PIPELINE_STAGE_TRANSFER_BIT,
                        commandBufferCount: 1,
                        pCommandBuffers: &commands[next_image as usize],
                        signalSemaphoreCount: 1,
                        pSignalSemaphores: &rendering_finished,
                    };

                    match unsafe { queue_submit(queue, 1, &submit_info, std::ptr::null_mut()) } {
                        VK_SUCCESS => {}
                        error => {
                            println!("vulkan error {}", error);
                            return Ok(());
                        }
                    }

                    let present_info = VkPresentInfoKHR {
                        sType: VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
                        pNext: std::ptr::null_mut(),
                        waitSemaphoreCount: 1,
                        pWaitSemaphores: &rendering_finished,
                        swapchainCount: 1,
                        pSwapchains: &swapchain,
                        pImageIndices: &next_image,
                        pResults: std::ptr::null_mut(),
                    };

                    match unsafe { queue_present(queue, &present_info) } {
                        VK_SUCCESS => {}
                        error => {
                            println!("vulkan error {}", error);
                            return Ok(());
                        }
                    }

                    last_event = event;
                }
                SDL_WINDOWEVENT_CLOSE => break,
                _ => continue,
            }
        }
        std::thread::sleep(Duration::from_millis(33));
    }

    unsafe {
        SDL_DestroyWindow(window);
        device_wait_idle(device);
        destroy_command_pool(device, command_pool, std::ptr::null_mut());
        destroy_semaphore(device, image_available, std::ptr::null_mut());
        destroy_semaphore(device, rendering_finished, std::ptr::null_mut());
        destroy_swapchain(device, swapchain, std::ptr::null_mut());
        destroy_device(device, std::ptr::null_mut());
        destroy_surface(instance, surface, std::ptr::null_mut());
        destroy_instance(instance, std::ptr::null_mut());
    }

    Ok(())
}
