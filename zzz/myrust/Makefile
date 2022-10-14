#RUST_BT :=
RUST_BT := RUST_BACKTRACE=1
#RUST_BT := RUST_BACKTRACE=full


help:
	@echo "usage: make run | local | linux_arm | win_x64"

run:
	${RUST_BT} cargo r

local:
	cargo b

win_x64:
	# 交叉汇编 windows 系统 amd64 架构的目标文件:
	${CPL_FLAGS} GOOS=windows GOARCH=amd64 go build ${CPL_ARGS} -o xxx_x64.exe *.go

linux_arm:
	# 交叉汇编 linux 系统 arm64 架构的目标文件:
	${CPL_FLAGS} GOOS=linux GOARCH=arm64 GOARM=7 go build ${CPL_ARGS} -o xxx_arm *.go

clean:
	rm -rf xxx* *.exe

