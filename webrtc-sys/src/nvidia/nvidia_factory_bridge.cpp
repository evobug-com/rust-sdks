// Compiled by the separate NVIDIA builder (not the main CXX builder).
// Provides a safe entry point that the main factory can call without
// including CUDA or NvCodec headers.

#include "nvidia_factory_bridge.h"
#include "nvidia_encoder_factory.h"

namespace livekit_ffi {

std::unique_ptr<webrtc::VideoEncoderFactory> CreateNvidiaEncoderFactoryIfAvailable() {
    if (webrtc::NvidiaVideoEncoderFactory::IsSupported()) {
        return std::make_unique<webrtc::NvidiaVideoEncoderFactory>();
    }
    return nullptr;
}

}  // namespace livekit_ffi
