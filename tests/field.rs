#[cfg(test)]
mod field {
    mod new {
        use schachmatt::{
            Board::{COLUMN_A, COLUMN_H, FIELD_A1, FIELD_H8, ROW_1, ROW_8},
            Field,
        };

        #[test]
        fn test_new_a0() {
            assert_eq!(Field::new(COLUMN_A, ROW_1), Some(FIELD_A1));
        }

        #[test]
        fn test_new_h8() {
            assert_eq!(Field::new(COLUMN_H, ROW_8), Some(FIELD_H8));
        }

        #[test]
        fn test_new_invalid() {
            assert_eq!(Field::new(COLUMN_H + 1, ROW_8 + 1), None);
        }
    }

    mod new_from_usize {
        use schachmatt::{
            Board::{COLUMN_A, COLUMN_H, FIELD_A1, FIELD_H8, ROW_1, ROW_8},
            Field,
        };

        #[test]
        fn test_new_a0() {
            assert_eq!(
                Field::new_from_usize(COLUMN_A as usize, ROW_1 as usize),
                Some(FIELD_A1)
            );
        }

        #[test]
        fn test_new_h8() {
            assert_eq!(
                Field::new_from_usize(COLUMN_H as usize, ROW_8 as usize),
                Some(FIELD_H8)
            );
        }

        #[test]
        fn test_new_invalid() {
            assert_eq!(
                Field::new_from_usize((COLUMN_H + 1) as usize, (ROW_8 + 1) as usize),
                None
            );
        }
    }

    mod new_from_string {
        use schachmatt::{
            Board::{FIELD_A1, FIELD_H8},
            Field,
        };

        #[test]
        fn test_new_a0() {
            assert_eq!(Field::new_from_string("a1"), Some(FIELD_A1));
        }

        #[test]
        fn test_new_h8() {
            assert_eq!(Field::new_from_string("h8"), Some(FIELD_H8));
        }

        #[test]
        fn test_new_h8_uppercase() {
            assert_eq!(Field::new_from_string("H8"), Some(FIELD_H8));
        }

        #[test]
        fn test_new_invalid_out_of_range() {
            assert_eq!(Field::new_from_string("h9"), None);
        }

        #[test]
        fn test_new_invalid_incorrect_length() {
            assert_eq!(Field::new_from_string("h1d"), None);
        }

        #[test]
        fn test_new_invalid_empty_string() {
            assert_eq!(Field::new_from_string(""), None);
        }
    }
}
