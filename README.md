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
Usage:
  csvfmt [OPTIONS] FILEPATH

Options:
  Flag            Expected data type     Description
  -d              string                 Specify the delimiter used. Default is the comma ','
  -f              string                 Name of file to read instead of stdin
  -h              none                   Print this help message. No other actions will be performed
  -t              integer                Only print the top n lines
  -w              none                   Set to true if the file uses Windows CRLF for line endings,
                                         otherwise unix style LF line endings are assumed. Defaults
                                         to false
```

Given the following csv file (called, for example, demo.csv):

```
1,2,3,4,509
4,5,6
2,3,4,55555,6
34554,2345,8,2345,3
```

This can be fed to csvfmt through stdin using a command such as `cat demo.csv | csvfmt` to produce the following 
slightly more readable output:

```
------------------------
|1    |2   |3|4    |509|
|4    |5   |6|     |   |
|2    |3   |4|55555|6  |
|34554|2345|8|2345 |3  |
------------------------
```