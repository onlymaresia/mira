#define VK_ENABLE_BETA_EXTENSIONS
#include "vulkan/vulkan_core.h"

typedef void IDirectFB;
typedef void IDirectFBSurface;

typedef uint32_t zx_handle_t;

typedef uint32_t DWORD;
typedef const wchar_t* LPCWSTR;
typedef void* HANDLE;
typedef HANDLE HWND;
typedef HANDLE HINSTANCE;
typedef HANDLE HMONITOR;
typedef void SECURITY_ATTRIBUTES;

typedef unsigned long XID;
typedef XID VisualID;
typedef XID Window;
typedef void Display;

typedef void xcb_connection_t;
typedef uint32_t xcb_window_t;
typedef uint32_t xcb_visualid_t;

typedef XID RROutput;

#include "vulkan/vulkan_android.h"
#include "vulkan/vulkan_beta.h"
#include "vulkan/vulkan_directfb.h"
#include "vulkan/vulkan_fuchsia.h"
#include "vulkan/vulkan_ios.h"
#include "vulkan/vulkan_macos.h"
#include "vulkan/vulkan_metal.h"
#include "vulkan/vulkan_screen.h"
#include "vulkan/vulkan_vi.h"
#include "vulkan/vulkan_wayland.h"
#include "vulkan/vulkan_win32.h"
#include "vulkan/vulkan_xcb.h"
#include "vulkan/vulkan_xlib.h"
#include "vulkan/vulkan_xlib_xrandr.h"

#include "vulkan/vk_icd.h"
#include "vulkan/vk_layer.h"
#include "vulkan/vk_platform.h"
#include "vulkan/vulkan.h"

// dead platform, google gaming platform
typedef uint64_t GgpFrameToken;
typedef uint32_t GgpStreamDescriptor;
#include "vulkan/vulkan_ggp.h"

#include "vk_video/vulkan_video_codec_av1std_decode.h"
#include "vk_video/vulkan_video_codec_av1std_encode.h"
#include "vk_video/vulkan_video_codec_av1std.h"

#include "vk_video/vulkan_video_codec_h264std_decode.h"
#include "vk_video/vulkan_video_codec_h264std_encode.h"
#include "vk_video/vulkan_video_codec_h264std.h"

#include "vk_video/vulkan_video_codec_h265std_decode.h"
#include "vk_video/vulkan_video_codec_h265std_encode.h"
#include "vk_video/vulkan_video_codec_h265std.h"

#include "vk_video/vulkan_video_codecs_common.h"
