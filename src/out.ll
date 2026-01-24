define i32 @factorial(i32 %n) {
entry:
%n.addr = alloca i32
store i32 %n, i32* %n.addr
	%1 = load i32, i32* %n.addr
	%2 = load i32, i32* %n.addr
	%3 = add i32 0, 1
	%4 = sub i64 %2, %3
	%factorial = call i32 @factorial(%4)	%5 = add i64 %1, %factorial
	ret i32 %5
}
define i32 @main() {
entry:
	%x.addr = alloca i32
	%6 = add i32 0, 20
	store i32 %6, i32* %x.addr
	%7 = load i32, i32* %x.addr
	ret i32 %7
}
