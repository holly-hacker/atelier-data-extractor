# Print list of available commands
default:
  just --list

# Write out static information such as typedefs
extract-static:
    cargo run -- type-defs -o ../atelier-data -c all-games

# Extract game info and generate typescript definitions for the atelier-data repo
extract-data GAME_DIR:
    cargo run -- extract -i {{GAME_DIR}} -o ../atelier-data
