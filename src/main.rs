use std::io;

fn main() {
    loop {
        println!("{:*^26}", '*');
        println!("* Gerenciador de Tarefas *");
        println!("{:*^26}", '*');
        println!("1 - Criar tarefa");
        println!("2 - Listar tarefa");
        println!("3 - Marcar tarefa");
        println!("4 - Desmarcar tarefa");
        println!("5 - Limpar tarefas concluídas");
        println!("6 - Remover tarefa");
        println!("5 - Sair");
        println!("{:*^26}", '*');

        let mut option: String = String::new();

        println!("Opção: ");
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line.");

        println!("{:*^26}", '*');

        let option: u8 = option
            .trim()
            .parse()
            .expect("Not a number.");

        if option == 5 {
            return;
        }
    }
}
