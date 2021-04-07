# csvfmt

A little command line utility to print csv to stdout in a table, to aid readability

### Usage
```
csvfmt [OPTIONS] FILEPATH
Options:
  --delimiter     string     Specify the delimiter used. Default is the comma ','
  --top           int        Only print the top n lines
  --crlf          bool       Set to true if the file uses Windows CRLF for line endings, otherwise unix style 
                             LF line endings are assumed. Defaults to false
  --stdin         bool       Set to true to read csv data from stdin. Defaults to false. If this is set to true, there's
                             no need to specify a file.
```
