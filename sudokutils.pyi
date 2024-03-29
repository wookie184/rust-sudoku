class Difficulty(type):
    class EASY(Difficulty): ...
    class MEDIUM(Difficulty): ...
    class HARD(Difficulty): ...

def generate_sudoku() -> list[int]:
    ...

def generate_sudoku_with_difficulty(difficulty: Difficulty) -> list[int]:
    ...

def is_solved_sudoku(grid: list[int]) -> bool:
    ...