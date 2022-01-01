gendoc:
	cargo doc --open && cmd.exe /c start target/doc/qoo/index.html

pub:
	cargo publish