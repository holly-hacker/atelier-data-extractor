# Print list of available commands
default:
  just --list

# Extract game info and generate typescript definitions for the atelier-data repo
extract-data GAME_DIR:
    # cargo run -- type-defs -o ../atelier-data
    cargo run -- extract -i {{GAME_DIR}} -o ../atelier-data
