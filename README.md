# Viz

Viz is a command line tool that can be used to visualize structured data in a more readable format. 
It is designed to be simple and easy to use, with a focus on providing a clear and concise representation of the data.
Viz supports most of the popular data formats, including JSON, YAML, and TOML.

It supports any type of the values, including:

- Strings
- Numbers (integers)
- Booleans
- Arrays
- Objects
- Null values
- Floating point numbers

## Installation

##### With Cargo

```bash
cargo install viz
```

##### From GitHub Releases

Go to [GitHub Releases](https://github.com/kostya-zero/viz/releases) and download the latest release for your platform.

## Usage

Viz can be used to visualize data from files or standard input. 
It automatically detects the format of the data and displays it in a JSON format.

```
$ viz Cargo.toml
{
  "package": {
    "name": "myproject",
    "version": "0.1.0",
    "edition": "2024"                                                                                                                                                                                                                 
   },
  "dependencies": {
    "clap": "4.5.37",
    "sysinfo": "0.35.0"                                                                                                                                                                                                               
   }
}
```

Also, you can specify the language of the data explicitly:

```
$ viz Cargo.toml --language toml
{
  "package": {
    "name": "myproject",
    "version": "0.1.0",
    "edition": "2024"
  },
  "dependencies": {
    "clap": "4.5.37",
    "sysinfo": "0.35.0"
  }
}
```

## License

Viz is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.