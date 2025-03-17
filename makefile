all:
	@cargo build --release
ifeq ($(OS),Windows_NT)
	@printf "\033[34mto run the program, type: ./target/release/rubik.exe\033[0m\n"
else
	@printf "\033[34mto run the program, type: ./target/release/rubik\033[0m\n"
endif