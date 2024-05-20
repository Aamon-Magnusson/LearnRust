import random

def generate_grid(rows, cols):
    grid = []
    for _ in range(rows):
        row = [random.randint(0, 1) for _ in range(cols)]
        grid.append(row)
    return grid

# Function to print the grid
def print_grid(grid):
    for row in grid:
        print(' '.join(map(str, row)))

# Generate and print the grid
grid = generate_grid(30, 100)
print_grid(grid)
