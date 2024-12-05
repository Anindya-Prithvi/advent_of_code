from functools import reduce
[
f := open("input.txt", "r"),
diffarr := lambda x: [x[i+1]-x[i] for i in range(len(x)-1)],
difflim := lambda x: all(map(lambda tfd: abs(tfd)>0 and abs(tfd)<4, x)),
incar := lambda x: all(map(lambda y: y>0, x)),
decar := lambda x: all(map(lambda y: y<0, x)),
ki := lambda x: difflim(diffarr(x)) and (incar(diffarr(x)) or decar(diffarr(x))),
print(
    sum(
        list(
            map(
                lambda x: ki(list(map(int, x.strip().split()))),
                f.readlines()
                )
            )
        )
    )
]
