# /bin/bash

rm -rf migrations diesel.toml src/schema.rs

diesel setup

diesel migration generate sample_db

# diesel migration run

# diesel migration redo