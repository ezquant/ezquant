#!/bin/sh

run_mycode=1

if [ $run_mycode -gt 0 ]; then

	cd mycode
	#RUST_BACKTRACE=1 cargo r
	#RUST_BACKTRACE=full cargo r
	cargo r

else

	SRC="00_hello_world/rust/hello_world.rs"
	if [ ! -z $1 ]; then
	  SRC=$1
	fi

	echo "==> compile $SRC into ./out ... "
	rustc -o out $SRC
	#echo "<== DONE!"

	#echo ""
	echo "==> run ./out ... "
	./out
	#echo "<== DONE!"
fi

