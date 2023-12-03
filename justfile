app_name := "hello"

default: devl

build:
  cargo fmt
  cross build --target arm-unknown-linux-gnueabihf

test: build
  echo No tests yet…

lint:
  # cargo check
  cargo clippy

devl: build
  cargo watch

devr: build
  rsync -Pv target/arm-unknown-linux-gnueabihf/debug/{{app_name}} pi@framboos:~/dev/learn-rust
  ssh -t pi@framboos -- /home/pi/dev/learn-rust/{{app_name}}
