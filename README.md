# Repo Harvest
Github repository harvester for building custom GPTs.

This simple CLI tool allows you to get the content of a Github repository and save it in a file. The content can be saved in either JSON or Markdown format.

> The results of a small test I did seems to suggest that the Markdown format gives better results in the custom GPT as the Json format.

The CLI is build on the [Github CLI](https://cli.github.com) so make sure to have that installed before runnig this tool.


## Example Usage
```bash
cargo run --  "https://github.com/Mees-Molenaar/minecraft-recipe-discord-bot.git" -f json
```

For help:
```bash
cargo run -- -h
```