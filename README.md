# Cargo dev

`cargo dev` is cargo subcommand for the static website hosting.

## Installation

    $ cargo install cargo-dev

To upgrade:

    $ cargo install --force cargo-dev

## Usage
Run your project root directory.

    $ cargo dev [option]

### Descriptions
    * -p port     custom port (default port is 8000)
    * -t target   custom hosting directory (default hosting directory is 'public')
