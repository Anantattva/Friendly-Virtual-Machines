## Run & Benchmarks
Ensure you have a JS runtime like Node/Deno/Bun. Run it with:
```node
node source.js
```
These are the benchmarks taken from Node on mobile Termux
```
++ || Addition Test || ++
== Max: 100 | Times: 1 ==
Sum: 5050
JS time:         13541ns.
VM time:         875469ns.
Gap (VM / JS):   65x.

++ || Addition Test || ++
== Max: 200 | Times: 100 ==
Sum: 20100
JS time:         4815ns.
VM time:         163831ns.
Gap (VM / JS):   34x.

++ || Addition Test || ++
== Max: 2000 | Times: 100 ==
Sum: 2001000
JS time:         7355ns.
VM time:         550686ns.
Gap (VM / JS):   75x.

++ || Addition Test || ++
== Max: 3500 | Times: 250 ==
Sum: 6126750
JS time:         2792ns.
VM time:         777401ns.
Gap (VM / JS):   278x.

++ || Collatz Test || ++
== Number: 5 | Times: 1 ==
Steps: 5
JS time:         132552ns.
VM time:         433177ns.
Gap (VM / JS):   3x.

++ || Collatz Test || ++
== Number: 25 | Times: 100 ==
Steps: 23
JS time:         1544.79ns.
VM time:         34097.92ns.
Gap (VM / JS):   22x.

++ || Collatz Test || ++
== Number: 70 | Times: 100 ==
Steps: 14
JS time:         101.56ns.
VM time:         10566.12ns.
Gap (VM / JS):   104x.

++ || Collatz Test || ++
== Number: 101 | Times: 250 ==
Steps: 25
JS time:         87.5ns.
VM time:         19457.9ns.
Gap (VM / JS):   222x.

++ || Prime Sum Test || ++
== Index: 10 | Times: 1 ==
Sum: 129
JS time:         226615ns.
VM time:         437083ns.
Gap (VM / JS):   2x.

++ || Prime Sum Test || ++
== Index: 25 | Times: 100 ==
Sum: 1060
JS time:         11043.75ns.
VM time:         374684.86ns.
Gap (VM / JS):   34x.

++ || Prime Sum Test || ++
== Index: 50 | Times: 100 ==
Sum: 5117
JS time:         9788.54ns.
VM time:         1175626.51ns.
Gap (VM / JS):   120x.

++ || Prime Sum Test || ++
== Index: 100 | Times: 250 ==
Sum: 24133
JS time:         40361.04ns.
VM time:         5226215.18ns.
Gap (VM / JS):   129x.
```
