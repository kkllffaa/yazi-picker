




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
