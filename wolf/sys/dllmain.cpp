#ifdef _MSC_VER

#include <wolf.h> 
#include <wolf.hpp>

BOOL APIENTRY DllMain(_In_ HMODULE pHModule, _In_ DWORD pULReasonForCall, _In_ LPVOID pLPReserved) noexcept
{
	std::ignore = pHModule;
	std::ignore = pLPReserved;

	switch (pULReasonForCall)
	{
	default:
	case DLL_PROCESS_ATTACH:
	case DLL_THREAD_ATTACH:
	case DLL_THREAD_DETACH:
	case DLL_PROCESS_DETACH:
		break;
	}
	return TRUE;
}

#endif