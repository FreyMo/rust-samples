mod sub_folder;
use sub_folder::failure::handle_failure;
use sub_folder::option::handle_option;
use sub_folder::iterate::ping_it;

fn main() {
    let _ = handle_failure();
    let _ = handle_option();
    let _ = ping_it();
}
