# NMC
Basic commandline mpd client in multiple languages that runs all given commands

## Command list
 - help - Prints help screen
 - toggle - Toggles mpd playback
 - discard - removes curently playing song
 - status - Same as no arguments: Prints mpd status screen
 - playlist - outputs mpd playlist with index numbers
 - repeat/random/single/consume - toggles mpd state
 - update - updates mpd database
 - volume - changes mpd volume
 - add - adds given files: seperated by comma
 - remove - removes items at given indecies

 - -p/--port - changes mpd port from default 6600
 - -h/--host - changes mpd host from default 127.0.0.1

## Current languages
### Completed
 - C
 - Rust

### Uncompleted
 - PHP
   - [x] Connect to server
   - [x] Create user input form
   - [ ] Allow for adding of new command elements to form
   - [ ] Allow for removal of command elements from form
   - [ ] Implementation of commands (0/12)
