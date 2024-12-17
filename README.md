# askpass-1p

askpass-1p is a utility for Linux environments that works as an askpass program. It allows users to interactively search for and retrieve credentials (like usernames, passwords, or tokens) from their 1Password vault using the 1Password CLI (op).

## Requirements

* 1Password CLI (op) installed and authenticated:
  ```op --version```

# Installation

```cargo install askpass-1p```

This will install the askpass-1p binary to ~/.cargo/bin.

Ensure ~/.cargo/bin is in your PATH (add to your .bashrc or .zshrc if needed):

    export PATH=$HOME/.cargo/bin:$PATH

## Usage

Set the SSH_ASKPASS environment variable to point to the askpass-1p binary:

```export SSH_ASKPASS=$(which askpass-1p)```

Use the tool when prompted for credentials, for example via ```ssh-add```, or my original use-case, pushing to a github repository and needing to look up a personal access token.

Follow the prompts.

*    Select a 1Password item from the list.
*    Select the desired field (e.g., username, password).

The selected field's value will be supplied to the requesting program.

### Example Workflow

User runs ssh-add or any tool requiring credentials.

The tool prompts the user to select a 1Password item:

```
    Select a 1Password Item
    > My SSH Key
      GitHub Account
      AWS Secrets
```

The tool then prompts the user to select a field:
```
    Select a field to fetch
    > password
      username
      otp

    The chosen field's value (e.g., password) is returned.
```
## Configuration

To persist the SSH_ASKPASS environment variable, add it to your shell configuration file (e.g., .bashrc or .zshrc):

```export SSH_ASKPASS=$(which askpass-1p)```

## License

This project is licensed under the GPL-3.0 License. See the LICENSE file for details.
Contributing

Contributions are welcome! To report issues or suggest features, please open an issue or pull request.

## Acknowledgments

* Mikael Mello's awesome inquire crate: https://github.com/mikaelmello/inquire
* 1Password CLI
