# colorscheme

CLI tool to generate pleasant CSS colorschemes

**WIP**

## Usage

```
Usage: colorscheme [OPTIONS] --scheme <SCHEME> --primary <PRIMARY COLOR>

Options:
  -s, --scheme <SCHEME>          color scheme to generate [possible values: column, dyad, triad, tetrad]
  -p, --primary <PRIMARY COLOR>  primary scheme color (hex value or CSS color name)
  -e, --selector <CSS SELECTOR>  css selector under which variables are declared (default: `:root`)
  -h, --help                     Print help (see more with '--help')
  -V, --version                  Print version
```

### Examples

```
$ colorscheme -s triad -p "#f0af0a"
:root {
        --clockwise: #0af0af;
        --counterclockwise: #af0af0;
        --primary: #f0af0a;
};
```

```
$ colorscheme -s column -p rebeccapurple
:root {
        --darker: #33194d;
        --lighter: #9966cc;
        --primary: #663399;
};
```

### TODO

- [ ] more schemes
  - [ ] colorblind-compatibile?
- flexibility
  - [ ] composability
  - [ ] "mixins" (suitable for fonts e.g.)
