# RUSTY NOTES

**A simple note taking program. Written in Rust 🦀**

## Functionality

The program works by creating a directory for your notes following the structure: **~/notes/year/month/day**. A normal notes folder will look like this

```
📁 notes
├── 📁 2021
│  └── 📁 12
│     └── 📄 15
└── 📁 2021
   ├── 📁 01
   │  ├── 📄 05
   │  ├── 📄 08
   │  ├── 📄 11
   │  ├── 📄 18
   │  ├── 📄 25
   │  └── 📄 29
   ├── 📁 02
   │  ├── 📄 01
   │  ├── 📄 04
   │  ├── 📄 08
   │  ├── 📄 22
   │  ├── 📄 25
   │  └── 📄 27
   └── 📁 03
      ├── 📄 02
      ├── 📄 05
      ├── 📄 06
      ├── 📄 21
      ├── 📄 23
      └── 📄 31
```

## How to use
There are 2 main ways of using this program:

1 appending a line of text into the note file
2 opening it in your terminal editor

To edit the notes with your terminal editor simply call the program `notes` without passing any arguments.
To append a line of text to the notes file just line the after the command like: `notes this line of text will be appended to the end of the file`

## Configuration

The configuration file is found in `$XDG_CONFIG_HOME/notes/config.toml` after running the program for the first time, the current options are: `editor`, `directory_name` and `filetype`.
By default the editor used is the one found in your `$EDITOR` variable, the directory name used is `notes` and no file extension is set.

### Exemple config

```toml
editor = "nvim"
directory_name = "journal"
filetype = ".md"
```
