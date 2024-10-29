import ast
from pathlib import Path

import matplotlib.pyplot as plt


def get_data(filepath: Path) -> dict[int, list[tuple[int, int]]]:
    """Read the coordinates from a file and return them as a dictionary.

    :param Path filepath: filepath to the coordinates file
    :return dict[int, list[tuple[int, int]]]: dictionary with the coordinates
    """
    data = {}
    with open(filepath, "r") as file:
        for node, line in enumerate(file):
            coords = [(0, 0)] + ast.literal_eval(line.strip())
            data[node] = coords

    return data


def plot(data: dict[int, list[tuple[int, int]]]) -> None:
    """Plot the spiderweb.

    :param dict[int, list[tuple[int, int]]] data: dictionary with the coordinates
    """
    fig, ax = plt.subplots(nrows=1, ncols=1)

    # Plotting the spiderweb strands
    for _, coords in data.items():
        for (x1, y1), (x2, y2) in zip(coords[:-1], coords[1:]):
            ax.plot([x1, x2], [y1, y2], "k-")

    # Plotting the spiderweb rings
    for i in range(len(data[0])):
        if i < len(data[0]) - 1:
            coords = [data[j][i] for j in range(len(data))]

            for (x1, y1), (x2, y2) in zip(coords[:-1], coords[1:]):
                ax.plot([x1, x2], [y1, y2], "k")

            # Connect the last and first node
            ax.plot([coords[0][0], coords[-1][0]], [coords[0][1], coords[-1][1]], "k")

    ax.axis("off")
    ax.grid(False)
    fig.savefig("data/spiderweb.svg", bbox_inches="tight")


if __name__ == "__main__":
    data = get_data(Path("data/coordinates.txt"))
    plot(data)
