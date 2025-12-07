with open("../1.txt") as f:
    rotations = [
        int(x[1:]) * (-1 if x[0] == "L" else 1) for x in [line.strip() for line in f]
    ]

dial = 50
i = 0

for rot in rotations:
    dial = (dial + rot) % 100
    if dial == 0:
        i += 1

print(i)

dial = 50
i = 0

for rot in rotations:
    direction = -1 if rot < 0 else 1
    if direction == -1:
        rot *= -1
    full, rem = divmod(rot, 100)
    i += full
    old_dial = dial
    dial += rem * direction
    if old_dial != 0 and (dial <= 0 or dial >= 100):
        i += 1
    dial %= 100

print(i)
