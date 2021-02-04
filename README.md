# rusty-notes
Terminal note taking app written in bad rust (first time coding in rust)\
the note file is generated inside ~/directory_name/year/month/ as day.txt

## usage
+ create the config file ~/.config/notes/config.toml
+ run ```notes``` to open the notes file inside your selected editor
+ run ```notes anything you want``` to append anything you want to the end of the file

example config:
```
editor = "nvim"
directory_name = "notes"
```

