# traffic_cone
Real-Debrid API Caller

This uses the exposed API endpoints from the Real-Debrid
API to serialize and deserialize API user information.

# Usage
```
Usage: traffic_cone --key-path <API_KEY_PATH> <COMMAND>

Commands:
  downloads  All download commands
  hosts      All hosts commands
  help       Print this message or the help of the given subcommand(s)

Options:
  -k, --key-path <API_KEY_PATH>  Path to the api key
  -h, --help                     Print help
  -V, --version                  Print version
```

## Command-Style Querying

This application is broken down into subcommands for most queries.

After providing the filepath of the API key, a subcommand is provided for a different type of query.

#### `downloads` Usage
Provides all download subcommands:
```
Usage: traffic_cone --key-path <API_KEY_PATH> downloads <COMMAND>

Commands:
  json      Get raw JSON output
  list      List all download metadata
  delete    Delete a specific video
  download  Download from a link
  links     Get list of link(s)
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

#### `hosts` Usage
Provides all host subcommands:
```
Usage: traffic_cone --key-path <API_KEY_PATH> hosts <COMMAND>

Commands:
  json  Get raw JSON output
  list  List host names
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

# Endpoint Implementation TODO
✅ /usr
⬜ /unrestrict
✅ /traffic
✅ /streaming
✅ /downloads
✅ /torrents
🚧 /hosts
⬜ /settings