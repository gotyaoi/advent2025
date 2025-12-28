def paths(
    graph: dict[str, list[str]], cache: dict[tuple[str, str], int], start: str, end: str
) -> int:
    key = (start, end)
    if (cached := cache.get(key, None)) is not None:
        return cached
    total = 0
    try:
        for neighbor in graph[start]:
            if neighbor == end:
                total += 1
            else:
                total += paths(graph, cache, neighbor, end)
    except KeyError:
        pass
    cache[key] = total
    return total


with open("../11.txt") as f:
    graph = {a: b.split() for a, b in (line.strip().split(": ") for line in f)}

cache: dict[tuple[str, str], int] = {}
print(paths(graph, cache, "you", "out"))

svr_to_dac = paths(graph, cache, "svr", "dac")
svr_to_fft = paths(graph, cache, "svr", "fft")
dac_to_fft = paths(graph, cache, "dac", "fft")
fft_to_dac = paths(graph, cache, "fft", "dac")
dac_to_out = paths(graph, cache, "dac", "out")
fft_to_out = paths(graph, cache, "fft", "out")
print(svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out)
# print(paths(graph, cache, "svr", "out"))
