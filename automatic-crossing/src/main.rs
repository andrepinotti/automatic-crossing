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
    println!("Hello, world!");
}
