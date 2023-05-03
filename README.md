# tbl

A small binary that turns input like this:

```txt
First Name|Last Name|Has Super Powers
Peter|Parker|Yes
Mary-Jane|Watson|No
Harry|Osborn|No
```

Into this:

```txt
First Name  Last Name  Has Super Powers
Peter       Parker     Yes
Mary-Jane   Watson     No
Harry       Osborn     No
```

## How to use

Either pass a string as the first argument, or pipe a string into it.

```shell
# as argument
tbl <string> [separator]

# as piped input
cat <file> | tbl [separator]
```

### Examples

View a CSV file:

```shell
cat data.csv | tbl ','
```

Prettify tmux-list-sessions:

```shell
echo "Name # Windows Date    \n$(tmux list-sessions | tail -n +2 | sed s/:// | sed s/\(created// | sed s/\)//)" | tbl
```

