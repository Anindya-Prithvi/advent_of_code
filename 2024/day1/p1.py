from functools import reduce
[
l := [],
r := [],
f := open("input1.txt", "r"),
list(map(lambda x: [l.append(int((y:=x.split())[0])), r.append(int(y[1]))], f.read().strip().split("\n"))),
f.close(),
l.sort(),
r.sort(),
print(reduce(lambda acc, pair: acc + abs(pair[0] - pair[1]), zip(l, r), 0)),
]