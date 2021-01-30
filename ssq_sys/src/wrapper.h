
#ifdef __linux__
	typedef unsigned long DWORD, *PDWORD, *LPDWORD;
	typedef unsigned char BYTE, *PBYTE, *LPBYTE;
	typedef int BOOL, *PBOOL, *LPBOOL;
	#define WINAPI
#else
	#define WINAPI __stdcall
#endif

#include "stdbool.h"
#include "ssq.h"
