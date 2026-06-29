mod display;
mod models;
use crate::models::cliente::Cliente;
use display::menu as menu;

fn main() {
    let mut clientes: Vec<Cliente> = Vec::new();

    menu::mostrar_menu(&mut clientes);

}
