// Force correct winsock inclusion order on Windows.
// This header MUST be included before any other header in all NVIDIA .cpp files.
// It prevents the winsock.h/winsock2.h struct redefinition error that occurs
// when Chromium's WebRTC headers and NVIDIA's CUDA/NvCodec headers are mixed.
#ifdef _WIN32
#pragma once
#pragma message("TRACE win_compat.h: entering")

// Ensure winsock2.h wins over winsock.h by defining the winsock.h guard
// BEFORE windows.h has a chance to include it
#ifndef _WINSOCKAPI_
#define _WINSOCKAPI_
#endif

// Also prevent ws2def.h from being included via winsock2.h after winsock.h
// by forcing the include order: winsock2.h first, then ws2tcpip.h
#ifndef _WINSOCK2API_
#include <winsock2.h>
#include <ws2tcpip.h>
#endif

// Now it's safe to include windows.h — _WINSOCKAPI_ is defined so it
// won't try to include winsock.h
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#include <windows.h>

#endif // _WIN32
