# This doc will go into how the UI part will work.

There are packages which can support showing the UI.
I want it to be barebones so will research and try to implement from scratch instead.
If it turns into requiring multiple devices support, I might switch to a light library.

## Research
Looks like its X11 server if I want lowest level.

Good overview: https://unix.stackexchange.com/questions/568634/how-does-a-linux-gui-work-at-the-lowest-level

https://blogs.igalia.com/itoral/2014/07/29/a-brief-introduction-to-the-linux-graphics-stack/

Then either use xlib, gtk, qt etc which can make it cross-platform.
Well gtk seems to support rust.

I would go lower but gtk seems low enough for my needs currently.
Basically vulkan/OpenGL if I need to go deeper.

But GTK seems a good middlelayer for now.

