2025-11-23

Quite tired after concert and being sick.
Lets look into cairo.

Also why gtk3 instead of gtk4?
Will try to upgrade to gtk4.
Need to upgrade cargo it seems.
Should use the shell.nix file more.

Got it installed with the shell.nix but now I need to fix the syntax errors.
Why is everything still setup hell? :P

Any project is the same, a few hours of nice coding and then lots of configuration with libraries or dependencies. There's quite a few too many dependencies?
A ton of ways to set things up. This project is not meant to be a lot of library config.
I should fix the LSP which should make some things easier.

shell.nix along with flakes does make some things easier as well.
But its still not smooth development.
Not upgrading is also not an option often since you do not want to build a new project on legacy not maintained libraries. So either stick to a tech stack which is more known, get better at setup with nix or code without any libraries. Not really possible since to do anything you need access to lower level stuff. To access that it's required.


2025-11-22

Now back again!
Going through adding css.

Success.
Now looking to refactor and look into cairo:
https://docs.rs/cairo-rs/latest/cairo/

Its used for drawing circles which is needed to fully show a clock.


2025-11-17
Ah, since I use i3wm it automatically fills the entire space.
By using Leader + Shift + Space it will go into floating mode and can be viewed at normal size.

Now I can look into how to shape it.
For that we need to add css.
Here we have an example:
https://github.com/gtk-rs/gtk3-rs/blob/master/examples/css/main.rs


2025-11-16
Currently having issues with the install of gtk.
trying out nix-shell and flakes.
