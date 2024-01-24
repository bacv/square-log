# Square Root Log
  Welcome to `sqrt-log`, your nifty tool for collecting and tracking all your cherished activities from across the web. Think of it as the Github contributor's activity log, but with a twist. The aim is to offer a versatile backend that captures personal daily digital footprints from various sources
 
### Plugins
  `sqrt-log` uses Lua scripting for flexibility and ease of use. With this, you can easily gather data from your blog, GitHub, or other places where you leave your digital footprint. It's designed to be adaptable to different needs.
 
### Storage
  `sqrt-log` is supposed to be run on your personal computer and act as an invisible ghost writer of your daily online activity. The data is stored in a simple but convenient [Sled](https://github.com/spacejam/sled) database.
  
### HTTP API
  What you do with the collected data is your choice. A common use could be to display it on your personal blog. To help with this, I'm working on a simple `js` library. Keep in mind, `sqrt-log` is meant for personal use.

### Running
  To run the project:
  ```bash
  cargo run -- --config config.yaml
  ```
  
### Development
  Right now, `sqrt-log` is in its early stages. It will be an essential part of my [personal website](https://github.com/bacv/inkagu-co), so I'm dedicating time to develop it further.  
  
  As of now, these are the short term tasks to be done:  
  - [x] - Scaffold `mlua` integration
  - [x] - Minimal plugin api definition
  - [ ] - `Sled` database integration
  - [x] - Scheduler for pull tasks
  - [ ] - Http API
  - [x] - ~~Pull checkpoint persistence~~ (lastest record)
  - [ ] - Github Lua plugin
  - [ ] - JS library
