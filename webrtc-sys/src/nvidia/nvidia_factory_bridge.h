// Bridge header — safe to include from the main CXX builder because it
// does NOT include CUDA headers or NvCodec headers. Only uses base WebRTC types.
#ifndef NVIDIA_FACTORY_BRIDGE_H_
#define NVIDIA_FACTORY_BRIDGE_H_

#include <memory>
#include "api/video_codecs/video_encoder_factory.h"

namespace livekit_ffi {

// Implemented in the separate NVIDIA builder (nvidia_factory_bridge.cpp).
// Returns nullptr if CUDA is unavailable.
std::unique_ptr<webrtc::VideoEncoderFactory> CreateNvidiaEncoderFactoryIfAvailable();

}  // namespace livekit_ffi

#endif  // NVIDIA_FACTORY_BRIDGE_H_
