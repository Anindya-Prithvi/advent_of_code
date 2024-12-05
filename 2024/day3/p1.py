import re
[
f := open("input.txt", "r"),
s := f.read(),
print(sum(map(lambda x: (b:=list(map(int, x[4:-1].split(','))))[0]*b[1], (re.findall(r"mul\([0-9]{1,3},[0-9]{1,3}\)", s)))))
]
