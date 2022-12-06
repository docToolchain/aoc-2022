from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return lines[0].strip()


def find_start_of_packet(datastream, length=4):
    packets = [datastream[i : i + length] for i in range(len(datastream))]
    return [len(set(p)) for p in packets].index(length) + length


@app.command()
def part1(input_file: str):
    datastream = read_input_file(input_file)
    packet_index = find_start_of_packet(datastream)
    print(f"The start of packet marker ends at {packet_index}")


@app.command()
def part2(input_file: str):
    datastream = read_input_file(input_file)
    message_index = find_start_of_packet(datastream, 14)
    print(f"The start of packet marker ends at {message_index}")


if __name__ == "__main__":
    app()
