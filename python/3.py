with open("../3.txt") as f:
    banks = [x.strip() for x in f]


def get_joltage(bank: str, cells: int) -> int:
    length = len(bank)
    joltage = []
    index = 0
    for i in range(cells):
        joltage.append((largest := max(bank[index : length - (cells - 1 - i)])))
        index = bank.index(largest, index) + 1
    return int("".join(joltage))


print(sum(get_joltage(bank, 2) for bank in banks))

print(sum(get_joltage(bank, 12) for bank in banks))
