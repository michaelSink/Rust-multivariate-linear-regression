use std::error::Error;
use std::process;

struct MultivariateRegression{
    x: Vec<Vec<f64>>,
    y: Vec<f64>,
    weights: Vec<f64>,
    bias: f64,
    learning_rate: f64,
}

impl MultivariateRegression{
    fn new(x: Vec<Vec<f64>>, y: Vec<f64>) -> MultivariateRegression{
        let m = x[0].len();
        MultivariateRegression{
            x: x,
            y: y,
            weights: vec![0.0, m as f64],
            bias: 0.0,
            learning_rate: 0.01,
        }
    }

    fn fit(&mut self, epochs: i32){
        for iter in 0..epochs{

            //Predict values 
            let y_hat_i : Vec<Vec<f64>> = self.x.iter().map(|y| y.iter().zip(&self.weights).map(|(w, z)| (w * z)).collect()).collect();
            let y_hat : Vec<i32> = y_hat_i.iter().map(|y| y.iter().sum::<f64>() + self.bias).collect();

            //let dw = y_hat.iter().zip(self.y).map(|(x, y)| (x - y))

        }
    }
}

fn main(){

}