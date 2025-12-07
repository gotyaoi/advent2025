from typing import Generator

with open("../2.txt") as f:
    ranges = [x.split("-") for x in f.read().strip().split(",")]


def generate_repeats(start: str, end: str, parts: int) -> Generator[int, None, None]:
    start_len = len(start)
    if start_len % parts == 0:
        part_len = start_len // parts
        start_first, start_rest = start[:part_len], int(start[part_len:])
        start_int = int(start_first)
        theoretical = int(start_first * (parts - 1))
        start_num = start_int if theoretical >= start_rest else (start_int + 1)
    else:
        start_num = int("1" + "0" * (start_len // parts))
    end_len = len(end)
    if end_len % parts == 0:
        part_len = end_len // parts
        end_first, end_rest = end[:part_len], int(end[part_len:])
        end_int = int(end_first)
        theoretical = int(end_first * (parts - 1))
        end_num = end_int if theoretical <= end_rest else (end_int - 1)
    else:
        if end_len // parts == 0:
            return
        end_num = int("9" * (end_len // parts))
    for i in range(start_num, end_num + 1):
        yield int(str(i) * parts)


print(
    sum(repeat for start, end in ranges for repeat in generate_repeats(start, end, 2))
)

print(
    sum(
        {
            repeat
            for start, end in ranges
            for i in range(2, len(end) + 1)
            for repeat in generate_repeats(start, end, i)
        }
    )
)
