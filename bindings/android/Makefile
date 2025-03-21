# @file Makefile
# @author Team Lobster 🦞
# @copyright ETOSPHERES Labs GmbH

.PHONY = clean build_sdk bundle lib_example run_example build_javadoc

build_sdk:
	@cargo ndk -o ./jniLibs -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 build --release --locked
	@cargo build --release --locked


clean:
	@rm -rf ./jniLibs
	@echo done.


lib_example:
	cargo build

run_example: lib_example
	cd examples && ./gradlew run --args $(example)

build_javadoc:
	javadoc -d javadoc -sourcepath ./tests/src/main/java com.etospheres.etopay
