# 交叉编译依赖下面几个环境变量：
# $GOARCH 目标平台（编译后的目标平台）的处理器架构（386、amd64、arm）
# $GOOS 目标平台（编译后的目标平台）的操作系统（darwin、freebsd、linux、windows）

# OS 		ARCH			OS version
# linux 	386 / amd64 / arm	>= Linux 2.6
# darwin 	386 / amd64		OS X (Snow Leopard + Lion)
# freebsd 	386 / amd64		>= FreeBSD 7
# windows 	386 / amd64		>= Windows 2000

CPL_FLAGS := CGO_ENABLED=0
CPL_ARGS := -ldflags '-s -w --extldflags "-static -fpic"'

help:
	@echo "usage: make run | local | linux_arm | win_x64"

run:
	go run entry.go

local:
	go build ${CPL_ARGS} -o xxx_x64 *.go

win_x64:
	# 交叉汇编 windows 系统 amd64 架构的目标文件:
	${CPL_FLAGS} GOOS=windows GOARCH=amd64 go build ${CPL_ARGS} -o xxx_x64.exe *.go

linux_arm:
	# 交叉汇编 linux 系统 arm64 架构的目标文件:
	${CPL_FLAGS} GOOS=linux GOARCH=arm64 GOARM=7 go build ${CPL_ARGS} -o xxx_arm *.go

clean:
	rm -rf xxx* *.exe

