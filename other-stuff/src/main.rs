mod sub_folder;
use sub_folder::failure::handle_failure;
use sub_folder::option::handle_option;

fn main() {
    let _ = handle_failure();
    let _ = handle_option();
}
