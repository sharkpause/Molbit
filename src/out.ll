define i64 @entry() {
entry:
	%x.addr = alloca i64
	%1 = add i64 0, 20
	store i64 %1, i64* %x.addr
	%2 = load i64, i64* %x.addr
	ret i64 %2
}
