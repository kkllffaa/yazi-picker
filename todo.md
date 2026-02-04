# TODO:

## flake
export YAZI_LOG=debug


## monitor
monitor -> 'tail -f ~/.local/state/yazi/yazi.log'


## makefile

##  test picker

## implement methods:
- [x] OpenFile
- [ ] SaveFile
- [ ] SaveFiles
- [ ] SelectFolder

## options
- [ ] multiple (b): If true, allows selecting multiple files.
- [ ] directory (b): If true, selects folders instead of files.
- [ ] filters (a(sa(us))): Serialized file filters to restrict file types.
- [ ] current_folder (ay): Suggested starting directory.
