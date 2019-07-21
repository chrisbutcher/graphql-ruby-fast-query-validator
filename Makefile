ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

# TODO make release lib actions

all: target/debug/libgraphql_query_validator.$(EXT)
	ruby src/main.rb

target/debug/libgraphql_query_validator.$(EXT): src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
