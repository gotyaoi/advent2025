with open("../4.txt") as f:
    rolls = [[True if x == "@" else False for x in line.strip()] for line in f]


def process(rolls: list[list[bool]]) -> tuple[int, list[list[bool]]]:
    max_y = len(rolls) - 1
    max_x = len(rolls[0]) - 1
    removable = 0
    new_rolls = []

    for y, row in enumerate(rolls):
        new_row = []
        for x, c in enumerate(row):
            if not c:
                new_row.append(False)
                continue
            surround = 0
            if y > 0 and rolls[y - 1][x]:
                surround += 1
            if y > 0 and x < max_x and rolls[y - 1][x + 1]:
                surround += 1
            if x < max_x and rolls[y][x + 1]:
                surround += 1
            if y < max_y and x < max_x and rolls[y + 1][x + 1]:
                surround += 1
            if y < max_y and rolls[y + 1][x]:
                surround += 1
            if y < max_y and x > 0 and rolls[y + 1][x - 1]:
                surround += 1
            if x > 0 and rolls[y][x - 1]:
                surround += 1
            if y > 0 and x > 0 and rolls[y - 1][x - 1]:
                surround += 1
            if surround < 4:
                new_row.append(False)
                removable += 1
            else:
                new_row.append(True)
        new_rolls.append(new_row)
    return removable, new_rolls


total, rolls = process(rolls)

print(total)

while True:
    removable, rolls = process(rolls)
    if removable == 0:
        break
    total += removable

print(total)
