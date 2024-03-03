main: src/main.rs librepl.rlib
	rustc -L src -L . src/main.rs

librepl.rlib: src/repl.rs
	rustc --crate-type lib src/repl.rs
