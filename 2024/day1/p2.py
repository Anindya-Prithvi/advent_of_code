from functools import reduce
[
l := [],
r := dict(),
f := open("input1.txt", "r"),
list(map(lambda x: (l.append(int((y:=x.split())[0])), (r.__setitem__(int(y[1]),r.get(int(y[1]), 0)+1))), f.read().strip().split("\n"))),
f.close(),
print(sum(map(lambda x: r.get(x,0)*x, l)))
]
