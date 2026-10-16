# Import Libraries

## nvcuvid.lib

This is a **generated import stub** (not a proprietary NVIDIA binary). It was created from the accompanying `.def` file using:

```
lib /def:nvcuvid.def /out:nvcuvid.lib /machine:x64
```

The `.lib` file (~10KB) contains only linker import descriptors — no executable code or proprietary NVIDIA intellectual property. At runtime, the symbols resolve against `nvcuvid.dll`, which ships as part of the NVIDIA GPU display driver and must already be installed on the end user's system.

This is the same approach used by FFmpeg, OBS Studio, and other projects that link against NVIDIA's Video Codec APIs without bundling proprietary binaries.

## nvcuvid.def

A plain-text export definition file listing the public symbols exported by `nvcuvid.dll`. Used to generate the `.lib` import stub above.
