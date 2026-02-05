
## dev:
* ```export YAZI_LOG=debug```
* monitor.sh			-> see yazi debug messages
* pick.sh					-> open picker and print picked file to console
* test-picker.py	-> request file from dbus portal service

## config:
```
[terminal_binary]: String,
[terminal_args]: Vec<String>,
```


## modes:
1. single file - hover and enter
2. non existant file - hover and enter to override or shift enter to type
3. single directory - hover and enter or shift enter to select current
4. multi file - hover and enter or select and enter
5. multi dir - hover and enter or select and enter and/or shift enter to include current
6. mixed - hover and enter or select and enter and/or shift enter to include current



## init.lua
```
require("yazi-picker"):setup {
	Enter = "",
	ShiftEnter = ""
}
```

## mimeapps.list:
```
[Added Associations]
inode/directory=yazi.desktop

[Default Applications]
inode/directory=yazi.desktop
```

## $XDG_CONFIG_HOME/xdg-desktop-portal/portals.conf
```
[preferred]
org.freedesktop.impl.portal.FileChooser=TODO
```

https://github.com/hunkyburrito/xdg-desktop-portal-termfilechooser/blob/main/Compatibility.md


Todo's and plans: [todo](todo.md).
