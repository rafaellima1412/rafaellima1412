use crate::models::Cliente;

pub fn incluir_cliente() {
    let mut cliente: Cliente = Cliente::default();
    cliente.id = 1;
    cliente.nome = ler_dados();
    cliente.cpf = ler_dados();
    cliente.endereco = ler_dados();
}
