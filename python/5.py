with open("../5.txt") as f:
    ranges: list[range] = []
    ingredients = []
    for line in f:
        line = line.strip()
        if "-" not in line:
            ingredients.append(int(line))
            break
        start, end = [int(x) for x in line.split("-")]
        whole = False
        kill: list[range] = []
        while True:
            new_range = range(start, end + 1)
            for i, r in enumerate(ranges):
                if r.start in new_range and r.stop - 1 in new_range:
                    kill.append(r)
                    continue
                if start in r:
                    if end in r:
                        whole = True
                        continue
                    start = r.start
                    break
                if end in r:
                    end = r.stop - 1
                    break
            else:
                break
            ranges.pop(i)
        if whole:
            continue
        ranges.append(new_range)
        for k in kill:
            ranges.remove(k)
    for line in f:
        ingredients.append(int(line))

print(sum(1 for i in ingredients if any(i in r for r in ranges)))

print(sum(r.stop - r.start for r in ranges))
