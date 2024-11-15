install:
	cargo build --release --target x86_64-pc-windows-gnu
	cp target/x86_64-pc-windows-gnu/release/hik-sadp.exe /mnt/e/软件/hik-sadp.exe

run:
	hik-sadp.exe
