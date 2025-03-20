# currencyfmt
## Abstract
A configurable command line tool for currency and commodity amount formatting.

## Building and Running the Application
### Build
Build the application using `cargo`:

```bash
cargo build
```

You can start the application using `cargo` directly: 

```bash
cargo run -- --help
```

## Command Line Arguments
`currencyfmt` supports the following command line parameters:

```
Usage: currencyfmt [OPTIONS] [NUMBERS]...

Arguments:
  [NUMBERS]...  amount-commodity pairs to be pretty printed

Options:
  -c, --configuration <CONFIGURATION>  optional path to the configuration file to be used. You can overwrite the default configuration path using this option
  -d, --delimiter <DELIMITER>          optional delimiter string that separates amount-commodity pairs inside the application input
  -h, --help                           Print help
  -V, --version                        Print version
```

## Configuration
### Options

`currencyfmt` allows to configure

* the decimal separator,
* the thousands separator

of any number to be pretty printed.

For every currency/commodity the number of decimal places to be printed can be configured.
For any currency/commodity the thousands seprator can be suppressed.

### Location

`currencyfmt` uses a TOML based configuration file.

You can specify an arbitraty path using the **command line option** `-c`.
The command line option has priority over all other options.

You can specify the path to the configuration file in the **environment variable** `CURRENCYFMT_CONFIG`.

If neither the command line option, nor the environment variable is set, `currencyfmt` will look at your home directory at `~/.config/currencyfmt/config.toml`.

### Example File

The following is an example TOML configuration file 

```YAML
decimal_separator = ","
thousands_separator = "."

[[currencies]]
symbol = "EUR"
decimals = 2

[[currencies]]
symbol = "BTC"
decimals = 8
suppress_thousands_separator = true
````
