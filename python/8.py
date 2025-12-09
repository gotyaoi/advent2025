import itertools
import math

with open("../8.txt") as f:
    points = sorted(
        (int(x), int(y), int(z)) for x, y, z in (line.strip().split(",") for line in f)
    )

known = [
    (
        pair,
        math.sqrt(
            (pair[0][0] - pair[1][0]) ** 2
            + (pair[0][1] - pair[1][1]) ** 2
            + (pair[0][2] - pair[1][2]) ** 2
        ),
    )
    for pair in itertools.combinations(points, 2)
]

singletons = set(points)
circuits: list[set[tuple[int, int, int]]] = []
for i, (pair, distance) in enumerate(sorted(known, key=lambda x: x[1])):
    try:
        singletons.remove(pair[0])
    except KeyError:
        pass
    try:
        singletons.remove(pair[1])
    except KeyError:
        pass
    to_add = []
    for circuit in circuits:
        if pair[0] in circuit:
            to_add.append(circuit)
        elif pair[1] in circuit:
            to_add.append(circuit)
    if not to_add:
        circuits.append({pair[0], pair[1]})
    elif len(to_add) == 1:
        to_add[0].add(pair[0])
        to_add[0].add(pair[1])
    else:
        to_add[0].update(circuits.pop(circuits.index(to_add[1])))
    if i == 999:
        print(math.prod(sorted((len(x) for x in circuits), reverse=True)[:3]))
    if not singletons:
        break
print(pair[0][0] * pair[1][0])
