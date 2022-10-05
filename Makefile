rsc:
	cargo build

test: rsc
	./test.sh

clean:
	rm -rf tmp* target
