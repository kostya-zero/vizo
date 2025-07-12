# `vizo`

Vizo is a command line tool that can be used to visualize structured data in a more readable format.
It is designed to be simple and easy to use, with a focus on providing a clear and concise representation of the data.
Vizo supports most of the popular data formats, including JSON, YAML, and TOML.

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
cargo install vizo
```

##### From GitHub Releases

Go to [GitHub Releases](https://github.com/kostya-zero/viz/releases) and download the latest release for your platform.

## Usage

Vizo can be used to visualize data from files or standard input.  
It automatically detects the data format and displays it using the Prettij markup language.  
Prettij is specifically designed for Vizo to present data in a way that's easy to understand.  
It uses syntax patterns inspired by JSON, YAML, and TOML.

```bash
$ vizo Cargo.toml

package = {
    name =  "myproject"
    version = "0.1.0"
    edition = "2024"    
}                                                                                                                                                                                                        
dependencies = {
    clap = "4.5.37"
    sysinfo = "0.35.0"            
}                                                                                                                                                                                                   
```

Also, you can specify the language of the data explicitly:

```bash
$ vizo Cargo.toml --language toml
```

You can pipe data to Vizo from standard input, but you need to specify the language of the data:

```bash
$ echo '{"name": "John", "age": 30, "city": "New York"}' | vizo -l json

name = "John"
age = 30
city = "New York"
```

## About Prettij

TBW

## License

Vizo is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
