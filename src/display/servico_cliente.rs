use crate::display::ler::{ler_dados, ler_dados_int};
use crate::display::operacoes_basicas::{esperar, limpar_tela};
use crate::models::cliente::Cliente; //só consegui achar porque declarei dentro do main.
use validador::validar_cpf;

pub fn incluir_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    let mut cliente: Cliente = Cliente::default();

    cliente.id = clientes.len() + 1;
    digitar_dados_cliente(&mut cliente);

    clientes.push(cliente);
    limpar_tela();
    println!("Cadastrado com sucesso");
    esperar(1);
}

pub fn listar_clientes(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    if nao_tem_cliente(clientes) {
        return;
    }
    println!("{}", "-".to_string().repeat(40));
    for cliente in clientes {
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
        println!("Digite enter continuar...");
        ler_dados();
    }
}

pub fn alterar_clientes(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    if nao_tem_cliente(clientes) {
        return;
    }
    let id = captura_id();
    if let Some((indice, cliente)) = buscar_cliente_id(clientes, id) {
        println!("{}", "-".to_string().repeat(40));
        println!("Alterando o cliente");
        println!("{}", "-".to_string().repeat(40));
        mostrar_cliente(&cliente);
        println!("{}", "-".to_string().repeat(40));
        digitar_dados_cliente(&mut clientes[indice]);
        limpar_tela();
        println!("cliente foi alterado");
    } else {
        limpar_tela();
        println!("Cliente não encontrado");
    }
}
pub fn excluir_clientes(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    if nao_tem_cliente(clientes) {
        return;
    }
    let id = captura_id();
    if let Some((indice, cliente)) = buscar_cliente_id(clientes, id) {
        println!("{}", "-".to_string().repeat(40));
        println!("Confirma a exclução do cliente");
        println!("{}", "-".to_string().repeat(40));
        mostrar_cliente(&cliente);
        println!("{}", "-".to_string().repeat(40));
        println!(" s-Sim n-Não");
        let opcao = ler_dados();
        if opcao == "s" {
            clientes.remove(indice);
            limpar_tela();
            println!("cliente foi excluido");
        } else {
            println!("operação cancelada");
        }
    } else {
        limpar_tela();
        println!("Cliente não encontrado");
    }
}
fn buscar_cliente_id(clientes: &Vec<Cliente>, id: usize) -> Option<(usize, &Cliente)> {
    clientes
        .iter()
        .enumerate()
        .find(|(_, cliente)| cliente.id == id)
}

fn captura_id() -> usize {
    limpar_tela();
    ler_dados_int()
}

fn nao_tem_cliente(clientes: &[Cliente]) -> bool {
    if clientes.len() == 0 {
        println!("não existem clientes Cadastrados");
        esperar(1);
        return true;
    }
    return false;
}

fn mostrar_cliente(cliente: &Cliente) {
    println!(
        "\
            ID: {}\n\
            Nome: {}\n\
            CPF: {}\n\
            Endereço: {}\n\
            ",
        cliente.id, cliente.nome, cliente.cpf, cliente.endereco
    )
}

fn digitar_dados_cliente(cliente: &mut Cliente) {
    println!("Digite o nome");
    cliente.nome = ler_dados();
    loop {
        println!("Digite o cpf");
        cliente.cpf = ler_dados();
        if validar_cpf(&cliente.cpf) {
            break;
        }
        println!("CPF inválido, tente novamente!");
    }
    println!("Digite o end");
    cliente.endereco = ler_dados();
}
