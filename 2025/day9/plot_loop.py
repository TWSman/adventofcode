import pandas as pd
import numpy as np
import matplotlib.pyplot as plt


def main():
    df = pd.read_csv("input.txt", header=None)
    a = [5424, 67450] # first corner
    b = [94703, 50308] # second corner

    xx = [a[0], b[0], b[0], a[0], a[0]]
    yy = [a[1], a[1], b[1], b[1], a[1]]

    fig, axs = plt.subplots(figsize=(12, 12))
    axs.plot(df[0], df[1], label="Loop")

    axs.plot(xx, yy, label="Candidate", marker="o", ls="-", c="red")
    print(xx)

    axs.grid(True)
    plt.savefig("plot.png")


if __name__ == "__main__":
    main()

