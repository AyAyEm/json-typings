# json-typings

Fast and highly configurable JSON typescript typings generator.

## Features

- Template string compatibility
- Interface key sorting
- Support for Array or Object in json file
- Multiple output strategies
- Highly configurable

## Usage

```none
jsontypings [FLAGS] [OPTIONS] <INPUT_FILE>
```

### Options

- `-c, --config <FILE>` Sets a custom config file
- `-d, --delimiter <STRING>` Defines the start and the end of strings
- `-i, --indentation <STRING>` Defines how many tabs or spaces to insert inside a scope
- `-o, --output <FILE>` Sets the output target file [default: index.d.ts]
- `-t, --typescript_version <SEMVER>` Specify the typescript version to automatically disable incompatible features

### Flags

- `-V, --version` Prints version information
- `-h, --help` Prints help information
- `--sort` Enable sorting of interface keys
- `--tree` Sets the formating strategy to tree

### Strategies / Modes

#### Tree

For this strategy we have nested typings following the same structure as the json, for example with the given json:

```json
{
  "glossary": {
    "title": "example glossary",
    "GlossDiv": {
      "title": "S",
      "GlossList": {
        "GlossEntry": {
          "ID": "SGML",
          "SortAs": "SGML",
          "GlossTerm": "Standard Generalized Markup Language",
          "Acronym": "SGML",
          "Abbrev": "ISO 8879:1986",
          "GlossDef": {
            "para": "A meta-markup language, used to create markup languages such as DocBook.",
            "GlossSeeAlso": ["GML", "XML"]
          },
          "GlossSee": "markup"
        }
      }
    }
  }
}
```

And using the command:
`jsontypings --tree data.json`

We generate the following typing in index.d.ts:

```typescript
export interface All {
    glossary: All.Glossary;
}

export namespace All {
    export interface Glossary {
        title: string;
        GlossDiv: Glossary.GlossDiv;
    }

    export namespace Glossary {
        export interface GlossDiv {
            GlossList: GlossDiv.GlossList;
            title: string;
        }

        export namespace GlossDiv {
            export interface GlossList {
                GlossEntry: GlossList.GlossEntry;
            }

            export namespace GlossList {
                export interface GlossEntry {
                    SortAs: string;
                    GlossSee: string;
                    GlossDef: GlossEntry.GlossDef;
                    Abbrev: string;
                    Acronym: string;
                    GlossTerm: string;
                    ID: string;
                }

                export namespace GlossEntry {
                    export interface GlossDef {
                        GlossSeeAlso: string;
                        para: string;
                    }
                }
            }
        }
    }
}
```

#### Cs50

This my final project for [cs50](https://cs50.harvard.edu/) and as per requested [this is my introduction video to this project](https://youtu.be/UquHbo7umzg).
