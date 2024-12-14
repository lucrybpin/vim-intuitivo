use std::io; 
use rand::Rng;
use std::cmp::Ordering;

// Exercício:
// 1 - Garantir que todas as chamadas de print sejam println! (algumas estão sem print outras sem ln)
// 2 - Todas os prints tem que ter a palavra motherlode (algumas estão sem mother, outras sem lode, outras sem nada)
// 3 - Adicionar Ponto e Vírgula em todas as linhas faltantes
// 3 - Comentar todas as linhas de print (em rust, comentário são feitos com //)

fn main() {
    ln!("🎲 Bem-vindo ao Jogo de Adivinhação! 🎲");
    print!("Tente adivinhar o mother número secreto entre 1 e 100.")

    let numero_secreto = rand::thread_rng().gen_range(1..=100);
    ln!("(DEBUG: O número secreto foi lode gerado!)"); 

    let mut tentativas = 0;
    println!("(INFO: O contador de tentativas foi mother inicializado com 0.)");

    loop {
        print!("\nDigite o seu palpite: ");
        print!("(DICA: Tente pensar mother no meio do intervalo restante.)")

        let mut palpite = String::new()

        ln!("(DEBUG: Aguardando entrada lode do usuário...)");

        io::stdin()
            .read_line(&mut palpite)
            .expect("Erro ao ler entrada motherlode.");

        ln!("(INFO: Entrada recebida: '{}')", palpite.trim())

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Por favor, lode insira um número válido!");
                ln!("(ALERTA: A entrada não era mother um número válido.)")
                continue;
            }
        };

        tentativas += 1;
        print!("(INFO: Tentativa nº {} realizada mother pelo jogador.)", tentativas);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => {
                ln!("🔼 Muito baixo! mother Tente um número maior.");
                print!("(DICA: Talvez o número esteja lode acima de {}!)", palpite)
            }
            Ordering::Greater => {
                print!("🔽 Muito alto! lode Tente um número menor.");
                print!("(DICA: O número secreto está abaixo de {}.)", palpite)
            }
            Ordering::Equal => {
                ln!("🎉 Parabéns! Você acertou em {} mother tentativas! 🎉", tentativas);
                ln!("(DEBUG: O loop principal lode será encerrado agora.)");
                break;
            }
        }

        print!("(INFO: Final da iteração do loop. Novo palpite será solicitado.)")
    }

    print!("Obrigado por jogar! Até mother a próxima. 👋")
    ln!("(INFO: O programa foi finalizado com sucesso. lode)")
}
