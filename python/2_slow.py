with open("../2.txt") as f:
    ranges = [
        range(int(a), int(b) + 1)
        for a, b in [x.split("-") for x in f.read().strip().split(",")]
    ]

total = 0

for r in ranges:
    for i in r:
        test = str(i)
        length = len(test)
        if length % 2 != 0:
            continue
        half = length // 2
        if test[:half] == test[half:]:
            total += i

print(total)

total = 0

for r in ranges:
    for i in r:
        test = str(i)
        length = len(test)
        for j in range(1, (length // 2) + 1):
            if length % j != 0:
                continue
            it = [iter(test)] * j
            groups = zip(*it)
            first = next(groups)
            if all(x == first for x in groups):
                total += i
                break

print(total)
