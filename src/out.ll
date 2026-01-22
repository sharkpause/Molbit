define i32 @main() {
entry:
	%x.addr = alloca i32
	%1 = add i32 0, 20
	store i32 %1, i32* %x.addr
	%y.addr = alloca i64
	%2 = add i64 0, 10
	store i64 %2, i64* %y.addr
	%3 = load i32, i32* %x.addr
	ret i32 %3
}
