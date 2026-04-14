alias b := build
alias bw := build-windows
alias c := clean
alias d := dev
alias db := debug
alias fc := fclean
alias r := run
alias t := test

binary_name := "shellpass"

[doc("Choose one available receipes")]
default:
    @just --choose

[arg("mode", long="release", short="r", value="release", help="set cargo build mode to release")]
[doc("Configure and build the project")]
[group("run")]
build mode="debug":
    cargo build {{ if mode == "release" { "--release" } else { "" } }}
    cp target/{{ mode }}/{{ binary_name }}{{ if os() == "windows" { ".exe" } else { "" } }} {{ binary_name }}{{ if os() == "windows" { ".exe" } else { "" } }}

[arg("mode", long="release", short="r", value="release", help="set cargo build mode to release")]
[doc("Configure and build the project for windows")]
[group("run")]
[linux]
build-windows mode="debug":
    cargo build --target x86_64-pc-windows-gnu {{ if mode == "release" { "--release" } else { "" } }}
    cp target/x86_64-pc-windows-gnu/{{ mode }}/{{ binary_name }}.exe {{ binary_name }}.exe

[doc("Build and run the project")]
[group("run")]
run: build
    cargo run

[doc("Remove build directory")]
[group("clean")]
clean:
    cargo clean

[doc("Remove build directory and exported binary")]
[group("clean")]
fclean: clean
    rm -f ./{{ binary_name }}
    rm -f ./{{ binary_name }}.exe
    rm -f ./.vault

[doc("Run the software in debug mode, with local vault")]
[group("dev")]
dev:
    cargo run -- -v ./.vault

[doc("Rebuild the tool each time a change/file add is detected")]
[group("dev")]
[linux]
debug:
    watchexec -c --stop-signal SIGINT -s SIGINT -e rs,toml -v bacon run

[doc("Rebuild the tool each time a change/file add is detected")]
[group("dev")]
[windows]
debug:
    watchexec -r -w src just run

[doc("Run unit tests for the project")]
[group("dev")]
test: build
    cargo test
