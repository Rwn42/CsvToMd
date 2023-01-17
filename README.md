# Csv To Markdown
Small utility that turns csv files into markdown table syntax.

## Disclaimer
This project is mainly for personal use so it may not support
the csv spec properly. Additionally, I am not familiar with rust I may have done some terrible rust practices.

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




