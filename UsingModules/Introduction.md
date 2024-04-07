Modules
-------
Splitting code into smaller chunks
Rust has a module system that enables the reuse of code in an organized fashion.

DEFINITION : A module is a namespace that contains definitions of functions or types, and you can choose whether those definitions are visible outside their module (public) or not(private).

Working of module:
1. mod keyword declares new module:
`mod modulename {

}`

2. By default , functions, types, constants and modules are private.
Use `pub` keyword to make it public

3. The `use` keyword brings modules, or the definitions inside modules, into scope so it’s easier to refer to them.

