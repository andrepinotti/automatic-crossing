use std::thread::sleep;
use std::time::Duration;

/**NOTE -> margens das nossas vias */
const VIAH_MARGEM: f64 = 15.0;
const VIAV_MARGEM: f64 = 15.0;

const VIAH_LARGURA: f64 = 4.0;
const VIAV_LARGURA: f64 = 4.0;

const VIAH_PERIMETRO: f64 = 150.0;
const VIAV_PERIMETRO: f64 = 150.0;

const CARRO_LARGURA: f64 = 2.0;
const CARRO_COMPRIMENTO: f64 = 4.0;


//VELOCIDADE MÁXIMA do carro -> metros por segundo
const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);

//ACELARAÇÃO MÁXIMA -> metros por segundo ao quadrado
const ACELERACAO_MAXIMA: f64 = 3.0;

//ACELERAÇÃO MÍNIMA -> metros por segundo
const ACELERACAO_MINIMA: f64 = -10.0;

fn main() {
    // FIXME 
    // println!("Em desenvolvimento.");
    println!("Início do programa");
    simula_carros('H', ACELERACAO_MAXIMA/10.0, 'H', ACELERACAO_MAXIMA);
    println!("Fim da simulação");
}

//Simula dois carros e retorna se houve colisao ou nao
fn simula_carros(via_carro: char, acel_carro1: f64, via_carro2: char, acel_carro2: f64 ) -> bool {

    /* Descrição do carro 1 */
    let chassi1 = 12345;
    let via = via_carro;
    let aceleracao_maxima1 = ACELERACAO_MAXIMA;
    let aceleracao_minima1 = ACELERACAO_MINIMA;
    let velocidade_maxima1 = VELOCIDADE_MAXIMA;
    let comprimento1 = CARRO_COMPRIMENTO;
    let largura1 = CARRO_LARGURA;

    let mut velocidade_atual1: f64 = 0.0;
    let mut posicao_atual1: f64 = -80.0; 

    let acel_atual1: f64;

    /* Descrição do carro 2 */
    let chassi2 = 54321;
    let via2 = via_carro2;
    let aceleracao_maxima2 = ACELERACAO_MAXIMA;
    let aceleracao_minima2 = ACELERACAO_MINIMA;
    let velocidade_maxima2 = VELOCIDADE_MAXIMA;
    let comprimento2 = CARRO_COMPRIMENTO;
    let largura2 = CARRO_LARGURA;
    let mut velocidade_atual2: f64 = 0.0;
    let mut posicao_atual2: f64 = -100.0; 
    let acel_atual2: f64;

    acel_atual1 = acel_carro1;
    acel_atual2 = acel_carro2;

    println!("Início da simulação");
    let mut tickms: f64;

    loop {
        //Ao final de cada tick é atualizado o estado do carro
        sleep(Duration::from_millis(100)); 
        
        tickms = 100.0;

        //Atualiza o carro 1

        let old_position = posicao_atual1;

        posicao_atual1 = posicao_atual1 + velocidade_atual1 * (tickms / 1000.0) + acel_atual1 * (tickms / 1000.0) * (tickms/1000.0) / 2.0;
        velocidade_atual1 = posicao_atual1 + acel_atual1 * (tickms / 1000.0);
    
        // NOTE -> validações o carro não anda para trás, não possui velocidade negativa e trava na velocidade máxima

        if posicao_atual1 < old_position {
            posicao_atual1 = old_position; 
        }

        if velocidade_atual1 < 0.0 {
            velocidade_atual1 = 0.0;
        }

        if velocidade_atual1 > velocidade_maxima1 {
            velocidade_atual1 = velocidade_maxima1;
        }

        println!("Carro 1: {} na posição {}{}, velocidade {}, aceleração {}",
            chassi1, via, posicao_atual1, velocidade_atual1, acel_atual1);

        // Atualiza o carro 2

        let old_position = posicao_atual2;

        posicao_atual2 = posicao_atual2 + velocidade_atual2 * (tickms / 1000.0) + acel_atual2 * (tickms / 1000.0) * (tickms/1000.0) / 2.0;
        velocidade_atual2 = posicao_atual2 + acel_atual2 * (tickms / 1000.0);

        if posicao_atual2 < old_position {
            posicao_atual2 = old_position; 
        }

        if velocidade_atual2 < 0.0 {
            velocidade_atual2 = 0.0;
        }

        if velocidade_atual2 > velocidade_maxima2 {
            velocidade_atual2 = velocidade_maxima2;
        }

        println!("Carro 2 {} na posição {}{}, velocidade {}, aceleração {}", 
            chassi2, via2, posicao_atual2, velocidade_atual2, acel_atual2);

        // Detecta colisão na via H
        if via == 'H' && via2 == 'H' {
            if colisao_longitudinal(posicao_atual1, comprimento1, posicao_atual2){
                println!("Colisão na via H");
                return true;
            }
        }

        // Detecta colisão na via V
        if via == 'V' && via2 == 'V' {
            if colisao_longitudinal(posicao_atual1, comprimento1, posicao_atual2) {
                println!("Colisão na via V");
                return true;
            }
        }

        // Detecta colisão no cruzamento
        if via != via2 {
            if dentro_cruzamento(posicao_atual1, comprimento1, via) && 
                dentro_cruzamento(posicao_atual2, comprimento2, via2) {
                    println!("Colisão dentro do cruzamento");
                    return true;
            } 
        }

        // Verifica se o carro 1 saiu do sistema 
        if posicao_atual1 > comprimento1 + if via=='H' {VIAV_LARGURA} else {VIAH_LARGURA} {
            return false;    
        }

        // Verifica se o carro 2 saiu do sistema 
        if posicao_atual2 > comprimento2 + if via2=='H' {VIAV_LARGURA} else {VIAH_LARGURA}{
            return false;    
        }
        


    }    

}

fn colisao_longitudinal(posicao_frente:f64, comprimento:f64, posicao_atras:f64) -> bool{
    return posicao_frente - comprimento <= posicao_atras;
}

fn dentro_cruzamento(posicao: f64, comprimento:f64, via:char) -> bool{
    return posicao > 0.0 && posicao <= comprimento + if via=='H' {VIAV_LARGURA} else {VIAH_LARGURA};
}