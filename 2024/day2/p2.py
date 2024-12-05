from functools import reduce
[
f := open("input.txt", "r"),
multidiffarr := lambda x: [diffarr(x[:i]+x[i+1:]) for i in range(len(x))]+[diffarr(x)],
diffarr := lambda x: [x[i+1]-x[i] for i in range(len(x)-1)],
difflim := lambda x: all(map(lambda tfd: abs(tfd)>0 and abs(tfd)<4, x)),
incar := lambda x: all(map(lambda y: y>0, x)),
decar := lambda x: all(map(lambda y: y<0, x)),
ki := lambda x: difflim(x) and (incar(x) or decar(x)),
pi := lambda x: any(map(ki,multidiffarr(x))),
print(
    sum(
        list(
            map(
                lambda x: pi(list(map(int, x.strip().split()))),
                f.readlines()
                )
            )
        )
    )
]
