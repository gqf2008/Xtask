target extended-remote :7777

# print demangled symbols
set print asm-demangle on

set confirm off

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

load
continue
