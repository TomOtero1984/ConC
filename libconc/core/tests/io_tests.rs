use conc_core::data_handler::io;


#[test]
fn test_make_buf_reader() {
    let path: &str = "tests/dict/native_english_words.txt";
    let reader = io::make_buf_reader(path);
    assert!(reader.is_ok(), "Expected Ok result from make_buf_reader");

}

#[test]
fn test_make_buf_writer() {
    let path: &str = "tests/dict/conc_test_output.jsonl";
    let writer = io::make_buf_writer(path);
    assert!(writer.is_ok(), "Expected Ok result from make_buf_writer");
}

// mod error_handling {
//     use super::*;
//
//     #[test]
//     fn it_handles_invalid_input() {
//         let input = "";
//         let result = process(input);
//         assert!(result.is_err(), "Expected error, got: {:?}", result);
//     }
// }

// You can add more modules as your library grows
