# Lockscreen

Locks screen on windows (turns off monitor)

Compatible with Windows 7 - 10 (server and desktop)

## References

- https://gekkio.fi/blog/2014-10-08-calling-win32-api-with-rust-ffi.html
- http://pingzing.github.io/talking-to-the-winapi-in-rust.html
- https://gist.github.com/oledid/fb02951b6b1848d1418d
- https://gallery.technet.microsoft.com/scriptcenter/Turn-off-screen-4d173e0a

## Code Samples

~~~~
powershell (Add-Type '[DllImport(\"user32.dll\")]^public static extern int SendMessage(int hWnd, int hMsg, int wParam, int lParam);' -Name a -Pas)::SendMessage(-1,0x0112,0xF170,2)
~~~~

~~~~
# Source: http://www.powershellmagazine.com/2013/07/18/pstip-how-to-switch-off-display-with-powershell/

# Turn display off by calling WindowsAPI.
 
# SendMessage(HWND_BROADCAST,WM_SYSCOMMAND, SC_MONITORPOWER, POWER_OFF)
# HWND_BROADCAST  0xffff
# WM_SYSCOMMAND   0x0112
# SC_MONITORPOWER 0xf170
# POWER_OFF       0x0002
 
Add-Type -TypeDefinition '
using System;
using System.Runtime.InteropServices;
 
namespace Utilities {
   public static class Display
   {
      [DllImport("user32.dll", CharSet = CharSet.Auto)]
      private static extern IntPtr SendMessage(
         IntPtr hWnd,
         UInt32 Msg,
         IntPtr wParam,
         IntPtr lParam
      );
 
      public static void PowerOff ()
      {
         SendMessage(
            (IntPtr)0xffff, // HWND_BROADCAST
            0x0112,         // WM_SYSCOMMAND
            (IntPtr)0xf170, // SC_MONITORPOWER
            (IntPtr)0x0002  // POWER_OFF
         );
      }
   }
}
'

[Utilities.Display]::PowerOff()
~~~~