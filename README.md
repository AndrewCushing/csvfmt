# csvfmt

A little command line utility to print csv to stdout in a table, to aid readability

### Getting started

You can find the executable in the [releases](https://github.com/AndrewCushing/csvfmt/releases) section of this repo.
As these are native executables, they are specific to the processor architecture they were built on. If these aren't
right for you then you will need to download the source code and build your own binary using `cargo build --release`.
This will of course require the installation of [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) - the package manager and build tool of Rust.

Once you have the executable, you can place it in `/usr/local/bin` on a unix like system, or anywhere that's on your path
so you can use it from any folder while in your favourite terminal.

Once you're ready to run it, you just can either to pass the name of your csv file as a command line argument, or set 
the option `--stdin true` and pipe the file contents through to `csvfmt`, so these two commands are equivalent:
```
csvmt example.csv
cat example.csv | csvfmt --stdin true
```

### Usage
```
csvfmt [OPTIONS] FILEPATH
Options:
  Option name     Expected data type     Description
  --delimiter     string                 Specify the delimiter used. Default is the comma ','
  --top           int                    Only print the top n lines
  --crlf          bool                   Set to true if the file uses Windows CRLF for line endings, otherwise unix style 
                                         LF line endings are assumed. Defaults to false
  --stdin         bool                   Set to true to read csv data from stdin. Defaults to false. If this is set to 
                                         true, there's no need to specify a file.
```
