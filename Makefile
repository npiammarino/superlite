main: src/main.rs librepl.rlib libprocessor.rlib libtable.rlib dirs
	rustc -L src -L build src/main.rs
	@mv main build
	@ln -s build/main main

librepl.rlib: src/repl.rs libprocessor.rlib libtable.rlib dirs
	rustc -L build --crate-type lib src/repl.rs
	@mv librepl.rlib build

libprocessor.rlib: src/processor.rs libtable.rlib dirs
	rustc -L build --crate-type lib src/processor.rs
	@mv libprocessor.rlib build

libtable.rlib: src/table.rs  dirs
	rustc --crate-type lib src/table.rs
	@mv libtable.rlib build

dirs:
	@mkdir -p build

clean:
	rm main || true
	rm -r build || true
	find -name "*~" -type f -delete

run: main
	@./main
