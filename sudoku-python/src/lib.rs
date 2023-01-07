use pyo3::prelude::*;


#[pyclass]
enum Difficulty {
    EASY,
    MEDIUM,
    HARD,
}

#[pyfunction]
fn generate_sudoku() -> Vec<usize> {
    let mut generator = sudoku::Generator::new();
    generator.generate_sudoku()
}

#[pyfunction]
fn generate_sudoku_with_difficulty(difficulty: &Difficulty) -> Vec<usize> {
    let mut generator = sudoku::Generator::new();
    match difficulty {
        Difficulty::EASY => generator.generate_sudoku_with_empty(0, 55),
        Difficulty::MEDIUM => generator.generate_sudoku_with_empty(56, 57),
        Difficulty::HARD => generator.generate_sudoku_with_empty(58, 81),
    }
}

#[pymodule]
fn sudoku_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_sudoku, m)?)?;

    m.add_class::<Difficulty>()?;
    m.add_function(wrap_pyfunction!(generate_sudoku_with_difficulty, m)?)?;
    Ok(())
}