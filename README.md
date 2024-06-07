# Repo Harvest
Github repository harvester for building custom GPTs.

This simple CLI tool allows you to get the content of a Github repository and save it in a file. The content can be saved in either JSON or Markdown format.

> The results of a small test I did seems to suggest that the Markdown format gives better results in the custom GPT as the Json format.

## Prerequisites

* The CLI is build on the [Github CLI](https://cli.github.com) so make sure to have that installed before runnig this tool.

* [Rust](https://www.rust-lang.org/tools/install) is required to build the CLI.


## Example Usage
```bash
cargo run --  "https://github.com/Mees-Molenaar/minecraft-recipe-discord-bot.git" -f markdown
```

For help:
```bash
cargo run -- -h
```

## Example Prompts For Custom GPT

```markdown
The GPT, "Create Rust CLI Helper," is specifically designed to assist developers working with Rust for command-line interfaces. It leverages a markdown file named "output.md" from its knowledge base. This file is structured with headings that indicate file locations, followed by Rust code snippets relevant to those files. The GPT uses this structure to provide precise and contextually relevant advice on improving user-submitted Rust code by comparing it against best practices and code examples within the file. Users can query about specific parts of their code, and the GPT will reference the knowledge base to suggest optimizations and best coding practices.
```

If you have a prompt that you would like to share, please open a pull request or an issue. I'll add it to prompts.md.

[More prompts](./prompts.md)