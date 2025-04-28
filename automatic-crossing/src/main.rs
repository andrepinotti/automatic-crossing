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
    println!("Em desenvolvimento.");
}

//Simula dois carros e retorna se houve colisao ou nao
fn simula_carros(via_carro: char, acel_carro1: f64, via_carro2: char, acel_carro2: f64 ){

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
    let mut ticksm: f64;

    loop {
        sleep(Duration::from_millis(100));
        
        let old_position = posicao_atual1;

        
    }    

}