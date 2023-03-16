# Csv To Markdown
Small utility that turns csv files into markdown table syntax. Currently use LF line endings for the csv, CRLF requires manually refactoring of the output.

## Example

some_table.csv:
```
length,mass,time
10,1,0.5
1,3,0.2
12.2,3,10
```
run `csv_to_md some_table.csv >> some_markdown_file`

## Building Yourself
The project does not need cargo just `rustc main.rs -o csv_to_md`




