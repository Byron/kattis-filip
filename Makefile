help:
	$(info -Targets -----------------------------------------------------------------------------)
	$(info exe                          | build the executable)
	$(info journey-tests                | run all stateless journey test)
	$(info continuous-journey-tests     | run all stateless journey test whenever something changes)

exe: src/main.rs
	rustc  -C opt-level=3 --crate-type bin -o $@ src/main.rs

journey-tests: ./exe
	./tests/stateless-journey.sh ./exe

continuous-journey-tests:
	watchexec $(MAKE) journey-tests

