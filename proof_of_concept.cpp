// Proof of concept posted on stack overflow by outlooker and edited by xav as of 24.02.2023
// thread: https://stackoverflow.com/questions/17419562/how-to-remap-keyboard-key-in-c-with-lowlevelkeyboardproc
// answear: https://stackoverflow.com/a/18833206/16638833
//
// I am not the author of the question, nor the answear, just saw it and decided to (finally, after having it in Ideas folder for a long time) make an app around this concept. I wanted a simple way to remap keyboard on windows for a long time (PowerToys is OK ig, but still, I don't want to install a whole toolbox just to remap my keyboard...), this project is supposed to be that.
//
// I have not written any of the following code (authors and source named above):

#include <iostream>
#include <windows.h>

using namespace std;

HHOOK hHook = 0;

LRESULT CALLBACK LowLevelKeyboardProc(int nCode, WPARAM wParam, LPARAM lParam)
{
    if (nCode == HC_ACTION)
    {
        KBDLLHOOKSTRUCT* p = (KBDLLHOOKSTRUCT*) lParam;
        if (p->vkCode == VK_LMENU) // VK_LMENU = ALT key
        {
           switch (wParam){

            case WM_SYSKEYDOWN :{ // use SYSKEYDOWN
                cout << "Key down" << endl;

                keybd_event(VK_LCONTROL, 0x1D, KEYEVENTF_EXTENDEDKEY | 0, 0 );
            break;
            }
            case WM_KEYUP: // use regular keyup
             {
                cout << "Key up" << endl;

                keybd_event( VK_LCONTROL, 0x1D, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
                return 1;

            break;
             }
            default:
                wParam = WM_SYSKEYDOWN; // if you do not specify it changes back to alt
                break;
           }
            return 1;
        }
    }
    return CallNextHookEx(hHook, nCode, wParam, lParam);
}
int WINAPI WinMain (HINSTANCE hThisInstance, HINSTANCE hPrevInstance, LPSTR lpszArgument, int nCmdShow)
{

   hHook = SetWindowsHookEx(WH_KEYBOARD_LL, &LowLevelKeyboardProc, hThisInstance, NULL);
    if (hHook == NULL)
    {
        cout << "Error" << endl;
        return 1;
    }

    MSG messages;

    while (GetMessage (&messages, NULL, 0, 0))
    {
        TranslateMessage(&messages);
        DispatchMessage(&messages);
    }

    return messages.wParam;
}
