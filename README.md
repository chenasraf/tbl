# tblf

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

## Installation

Currently you can build this program from source.

For an easy installation:

- Clone this repository: `git clone https://github.com/chenasraf/tblf --depth=1`
- To build: `make build`
- To install: `make install` - will put the binary in `/usr/local/bin`
- To install at a custom location: `cp target/release/tblf /path/to/dir`

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

