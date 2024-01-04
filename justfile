app_name := "hello"

default: devl

build_shared:
  cargo fmt

buildr: build_shared
  cross build --target arm-unknown-linux-gnueabihf

test:
  echo No tests yet…

lint:
  # cargo check
  cargo clippy

devl:
  cargo run

devr: buildr
  rsync -Pv target/arm-unknown-linux-gnueabihf/debug/{{app_name}} pi@braam:~/dev/learn-rust
  ssh -t pi@braam -- /home/pi/dev/learn-rust/{{app_name}}
