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

            let mut temp_bias = 0f64;

            let mut temp_weights : Vec<f64> = Vec::with_capacity(self.weights.len());
            for i in 0..temp_weights.len(){
                temp_weights[i] = 0f64;
            }

            for x_row_i in 0..self.x.len(){

                //Make our prediction based on weights
                let mut y_hat : f64 = self.bias;
                for i_weight in 0..self.weights.len(){
                    y_hat += self.weights[i_weight] * self.x[x_row_i][i_weight];
                }

                //Calculate derivatives of weights and bias
                for i_weight in 0..self.weights.len(){
                    temp_weights[i_weight] += (y_hat - self.y[x_row_i]) * self.x[x_row_i][i_weight];
                }
            
                temp_bias += y_hat - self.y[x_row_i];

            }

            //Update weights
            for weight_u in 0..self.weights.len(){
                self.weights[weight_u] = self.weights[weight_u] - self.learning_rate * (1 / (2 * self.x[0].len())  ) as f64 * temp_weights[weight_u];
            }

            self.bias = self.bias - self.learning_rate * (1 / (2 * self.x[0].len())  ) as f64 * temp_bias;

        }
    }
}