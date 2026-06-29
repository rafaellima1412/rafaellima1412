use crate::display::ler::ler_dados_int;
use crate::display::operacoes_basicas::esperar;
use crate::display::servico_cliente::{alterar_clientes, excluir_clientes, incluir_cliente, listar_clientes};
use crate::models::cliente::Cliente;

pub fn mostrar_menu(clientes: &mut Vec<Cliente>) {
    loop {
        println!(
            "\
        ====================MENU===========================
        Escolha uma das opções abaixo:\n\n\
        1-Cadastrar Cliente\n\
        2-Alterar Cliente\n\
        3-Excluir Cliente\n\
        4-Listar Cliente\n\
        0-Sair do Sistema\n
      "
        );
        let opcao: usize = ler_dados_int();

        match opcao {
            1 => incluir_cliente(clientes),
            2 => alterar_clientes(clientes),
            3 => excluir_clientes(clientes),
            4 => listar_clientes(clientes),
            0 => {
                println!("finalizando....");
                return;
            }
            _ => println!("opação invalida"),
        }
        esperar(1);
    }
}
