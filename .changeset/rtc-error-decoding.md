---
webrtc-sys: patch
---

Preserve malformed native WebRTC exceptions as internal errors instead of
panicking during disconnect or reconnect. Validate the serialized header without
slicing through UTF-8, and never classify unknown error types as success.
