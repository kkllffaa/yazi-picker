# work in progress some methods are not implemented and plugin only works with files

## dev:
* ```export YAZI_LOG=debug```
* monitor.sh			-> see yazi debug messages
* pick.sh					-> open picker and print picked file to console
* test-picker.py	-> request file from dbus portal service

## $XDG_CONFIG_HOME/yazi-picker/config.toml
```
terminal_bin = "ghostty" # your favourite terminal emulator
terminal_args = [ "--class=floating", "-e", "sh", "-c" ] # args to it
```
## $XDG_CONFIG_HOME/xdg-desktop-portal/portals.conf
```
[preferred]
org.freedesktop.impl.portal.FileChooser=yazi-picker
```
## $XDG_CONFIG_HOME/yazi/init.lua
```
require("yazi-picker"):setup {
	Enter = "",
	ShiftEnter = ""
}
```
## $XDG_CONFIG_HOME/yazi/keymap.toml
```
[[mgr.prepend_keymap]]
on = ["<Enter>"]
run = "plugin yazi-picker
[[mgr.prepend_keymap]]
on = ["<S-Enter>"]
run = "plugin yazi-picker -- --shift
```
## $XDG_CONFIG_HOME/yazi/yazi-picker.yazi/
copy main.lua from smart-picker.yazi

## nix
```
xdg.portal = {
  enable = true;
  extraPortals = [
    inputs.yazi-picker.packages.${pkgs.system}.portal
  ];
  config.niri = { # or hyperland or other
    "org.freedesktop.impl.portal.FileChooser" = "yazi-picker";
  };
};
programs.yazi = {
	keymap.mgr.prepend_keymap = [
    { on = [ "<Enter>" ];
      run = "plugin yazi-picker";
    }
    { on = [ "<S-Enter>" ];
      run = "plugin yazi-picker -- --shift";
    }
	];
  plugins = {
    yazi-picker = inputs.yazi-picker.packages.${pkgs.system}.plugin;
  };
  initLua = ./your_lua_script.lua
};
```

## modes:
1. single file - hover and enter
2. non existant file - hover and enter to override or shift enter to type
3. single directory - hover and enter or shift enter to select current
4. multi file - hover and enter or select and enter
5. multi dir - hover and enter or select and enter and/or shift enter to include current
6. mixed - hover and enter or select and enter and/or shift enter to include current





## mimeapps.list (if you want to open directories in yazi):
```
[Added Associations]
inode/directory=yazi.desktop

[Default Applications]
inode/directory=yazi.desktop
```


### [Compatibility (from xdg-desktop-portal-termfilechooser repo)](https://github.com/hunkyburrito/xdg-desktop-portal-termfilechooser/blob/main/Compatibility.md)

### [Portal Backend Specification](https://github.com/flatpak/xdg-desktop-portal/blob/main/data/org.freedesktop.impl.portal.FileChooser.xml)

## TODO:
- makefile
- man page
- filters
- use fifo???
- open as floating (if even possible?)
- include ``` export YAZI_LOG=debug ``` in devtools
- improve test picker
