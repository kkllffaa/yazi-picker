# TODO:

## build system
- [ ] makefile
- [ ] plugin
- [ ] systemd service
- [ ] dbus service
- [ ] pick script
- [ ] man page

## dev utils
- [x] test picker
- [ ] monitor -> 'tail -f ~/.local/state/yazi/yazi.log'
- [ ] export YAZI_LOG=debug
- [ ] improve test picker




## implement methods:
[backend docs](https://github.com/flatpak/xdg-desktop-portal/blob/main/data/org.freedesktop.impl.portal.FileChooser.xml)  
[fronted docs](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html)

* OpenFile:
	- [x] method
	- [ ] options
* SaveFile
	- [ ] method
	- [ ] options
* SaveFiles
	- [ ] method
	- [ ] options

## backend
- [ ] use fifo
- [ ] open as floating
- [ ] improve options in pick script

## plugin
- [ ] support all modes
