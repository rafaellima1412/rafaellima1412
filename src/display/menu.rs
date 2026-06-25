use crate::display::ler::ler_dados_int;
use crate::display::ler::ler_dados;

use crate::display::operacoes_basicas::limpar_tela;
use crate::display::operacoes_basicas::esperar;

pub fn mostrar_menu() {
    limpar_tela();
    loop {
        println!(
            "\
        ====================MENU===========================
        Escolha uma das opções abaixo:\n\n\
        1- cadastrar Cliente\n\
        2- Alterar Cliente\n\
        3-Excluir Cliente\n\
        4-Listar Cliente\n\
        0-Sair do Sistema\n
      "
        );
        let opcao: i32 = ler_dados_int();

        match opcao {
            1 => limpar_tela(),
            2 => println!("opação 2"),
            3 => println!("opação 3"),
            4 => println!("opação 4"),
            0 => {
                println!("finalizando....");
                return;
            }
            _ => println!("opação invalida"),
        }
        // println!("digite uma tecla continuar");
        // ler_dados();
        esperar(2);
    }
}
