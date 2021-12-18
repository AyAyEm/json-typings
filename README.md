# json-typings

Fast and highly configurable JSON typescript typings generator.

## Usage

```none
jsontypings [FLAGS] [OPTIONS] <INPUT_FILE>
```

### Examples

To sort the interface entries and specify an output file:

```bash
jsontypings --sort -o types.d.ts data.json
```

To use a specific strategy:

```bash
jsontypings --family data.json
```

To use a config file:

```bash
jsontypings --config config.yml data.json
```

### Options

```none
-c, --config <FILE>                  Sets a custom config file
-d, --delimiter <STRING>             Defines the start and the end of strings
-i, --indentation <STRING>           Defines how many tabs or spaces to insert inside a scope
-o, --output <FILE>                  Sets the output target file [default: index.d.ts]
-t, --typescript_version <SEMVER>    Specify the typescript version to automatically disable incompatible features
```

### Flags

```none
--family         Sets the formating strategy to family
-h, --help       Prints help information
--sort           Enable sorting of interface keys
--tree           Sets the formating strategy to tree
-V, --version    Prints version information
```

#### Cs50

This my final project for [cs50](https://cs50.harvard.edu/) and as per requested [this is my introduction video to this project](https://youtu.be/UquHbo7umzg).
