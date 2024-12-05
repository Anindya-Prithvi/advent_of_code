import re
def part_two(inp: str) -> int:
        total = 0
        do = True

        for mul in re.finditer(r'mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don\'t\(\)', inp):
            match mul.group():
                case 'do()':
                    do = True
                case 'don\'t()':
                    do = False
                case _:
                    if do:
                        total += int(mul.group(1)) * int(mul.group(2))
        return total
[
f := open("input.txt", "r"),
s := f.read(),
print(part_two(s))
]
