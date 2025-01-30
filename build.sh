#!/bin/bash

echo "[build.sh] Compiling project with 'Cargo'."
cargo build --release # Compile the project with `Cargo`

if [ $? -eq 0 ]; then
  echo "[build.sh] Compile successful. Copying project to the /bin directory."
  cp -r ./target/release/mpvy /bin
  echo "[build.sh] Binary copied to the /bin directory."
  echo "[build.sh] You can run mpvy with 'mpvy' command."
else
  echo "[build.sh] An error occured. Exiting with code 1."
  exit 1
fi

echo "[build.sh] End of file."
