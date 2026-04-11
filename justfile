alias b := build
alias c := clean
alias d := dev
alias db := debug
alias fc := fclean
alias r := run
alias t := test

binary_name := if os() == "windows" { "shellsafe.exe" } else { "shellsafe" }

[doc("Choose one available receipes")]
default:
    @just --choose

[arg("mode", long="release", short="r", value="release", help="set cargo build mode to release")]
[doc("Configure and build the project")]
[group("run")]
build mode="debug":
    cargo build {{ if mode == "release" { "--release" } else { "" } }}
    cp target/{{ mode }}/{{ binary_name }} {{ binary_name }}

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
