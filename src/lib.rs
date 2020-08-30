#[cfg(test)]
pub mod aruco {
    mod tests {
        use super::*;

        #[test]
        #[should_panic]
        fn aruco_board_must_have_size_2() {
            let _board = Marker::new(1);
        }

        #[test]
        fn external_cells_are_black() {
            let board = Marker::new(2);
            for i in 0..board.words.len() {
                if i == 0 || i == board.words.len() - 1 {
                    for cell in &board.words[i].cells {
                        match cell {
                            Cell::Black => println!("Succeed!"),
                            Cell::White => panic!(),
                        }
                    }
                } else {
                    let first_cell = &board.words[i].cells[0];
                    match first_cell {
                        Cell::Black => println!("Succeed!"),
                        Cell::White => panic!(),
                    }
                    let last_cell = &board.words[i].cells[0];
                    match last_cell {
                        Cell::Black => println!("Succeed!"),
                        Cell::White => panic!(),
                    }
                }
            }
        }

        #[test]
        fn there_are_size_squared_cells() {
            let board = Marker::new(4);
            assert_eq!(4 * 4, board.words.len() * board.words[0].len())
        }

        #[test]
        fn a_default_word_has_zero_transitions() {
            let board = Marker::new(4);
            assert_eq!(0f32, relative_number_transitions(&board.words[0]));
        }

        #[test]
        fn inserting_bit_transition_increase_number_transition_by_one() {
            let mut word = Word::new(4); // a word of length 4

            for i in 0..3 {
                word.cells[i] = Cell::White;
                assert_eq!(2 as f32 / 4 as f32, relative_number_transitions(&word))
            }
        }
    }

    #[derive(Clone, PartialEq)]
    pub enum Cell {
        Black = 0,
        White = 1,
    }

    #[derive(Clone)]
    struct Word {
        cells: Vec<Cell>,
    }
    impl Word {
        fn new(size: usize) -> Word {
            Word {
                cells: vec![Cell::Black; size],
            }
        }
        fn len(&self) -> usize {
            self.cells.len()
        }
    }

    pub struct Marker {
        words: Vec<Word>,
    }

    impl Marker {
        pub fn new(size: usize) -> Marker {
            if size < 2 {
                panic!("The size must be larger than 2");
            }
            Marker {
                words: vec![Word::new(size); size],
            }
        }
    }

    fn relative_number_transitions(word: &Word) -> f32 {
        let mut transitions = 0;

        for i in 0..word.cells.len() - 1 {
            let current = &word.cells[i];
            let next = &word.cells[i + 1];
            if next == current {
                transitions += 1;
            }
        }

        transitions as f32 / word.len() as f32
    }
}
