from collections.abc import Iterable
import os

import pandas as pd

import matplotlib.pyplot as plt
import click


class TSPPlotter:
    def __init__(self, solution_path: str, output_path: str, instance_path: str) -> None:
        instance = pd.read_csv(instance_path, sep=';', header=None).T.to_numpy()
        self.x_coords, self.y_coords, self.costs = instance
        if os.path.isfile(solution_path):
            solutions = {solution_path.removesuffix('.txt'): solution_path}
        else:
            solutions = {file_name.removesuffix('.txt'): os.path.join(solution_path, file_name)
                         for file_name in os.listdir(solution_path) if file_name.endswith('.txt')}

        fig, axs = plt.subplots(1, len(solutions), figsize=(15, 5), dpi=150)

        if not isinstance(axs, Iterable):
            axs = [axs]
        for ax, (solution_name, solution_path) in zip(axs, solutions.items()):
            with open(solution_path, 'r', encoding='utf-8') as f:
                solution = list(map(int, f.readlines()))
            solution.append(solution[0])
            self.scatter_plot_tsp(ax, solution, solution_name)

        fig.savefig(output_path)

    def scatter_plot_tsp(self, ax, solution: list[int], solution_name: str):
        ax.scatter(self.x_coords, self.y_coords, s=self.costs/10)
        for idx1, idx2 in zip(solution[:-1], solution[1:]):
            ax.plot([self.x_coords[idx1], self.x_coords[idx2]], [self.y_coords[idx1], self.y_coords[idx2]], color='red')

        ax.set_title(solution_name)
        ax.label_outer()


@click.command()
@click.argument('solution_path')
@click.argument('output_path', default='output.png')
@click.option('--instance-path', default="data/TSPB.csv", help='path to TSP instance')
def plot_solution(solution_path: str, output_path: str, instance_path: str):
    TSPPlotter(solution_path, output_path, instance_path)


if __name__ == '__main__':
    plot_solution()
