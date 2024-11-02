use serde::Deserialize;
use std::fs;
use std::io;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Deserialize, Debug)]
#[derive(Clone)]
struct Question {
    context: String,
    option_1: String,
    option_2: String,
    option_3: String,
    option_4: String,
    answer: i32,
    level: i32,
}

fn main() {
    
    
    
    loop {
        println!(r" /$$      /$$ /$$ /$$ /$$                                                             /$$           /$$
| $$$    /$$$|__/| $$| $$                                                            |__/          | $$
| $$$$  /$$$$ /$$| $$| $$  /$$$$$$  /$$$$$$$   /$$$$$$  /$$$$$$$   /$$$$$$   /$$$$$$  /$$  /$$$$$$ | $$
| $$ $$/$$ $$| $$| $$| $$ |____  $$| $$__  $$ /$$__  $$| $$__  $$ |____  $$ /$$__  $$| $$ /$$__  $$| $$
| $$  $$$| $$| $$| $$| $$  /$$$$$$$| $$  \ $$| $$  \ $$| $$  \ $$  /$$$$$$$| $$  \__/| $$| $$  \ $$|__/
| $$\  $ | $$| $$| $$| $$ /$$__  $$| $$  | $$| $$  | $$| $$  | $$ /$$__  $$| $$      | $$| $$  | $$    
| $$ \/  | $$| $$| $$| $$|  $$$$$$$| $$  | $$|  $$$$$$/| $$  | $$|  $$$$$$$| $$      | $$|  $$$$$$/ /$$
|__/     |__/|__/|__/|__/ \_______/|__/  |__/ \______/ |__/  |__/ \_______/|__/      |__/ \______/ |__/");
        println!("\nBienvenido a quien quiere ser ¡MILLONARIO! \n 1. Iniciar Juego \n 0. Salir \n Ingresa una opción:");
        let initial = read_integer().unwrap();
        match initial {
            1 => game(),
            0 => break,
            _ => println!("Argumento incorrecto..."),
        }
    }
}
fn game(){
    let mut rng = thread_rng();
    let questions = fs::read_to_string("questions.json").expect("Unable to read file");
    let questions: Vec<Question> = serde_json::from_str(&questions).expect("JSON was not well-formatted");
    let mut questions_basics = questions.clone().into_iter().filter(|x| x.level == 1).collect::<Vec<Question>>();
    let mut questions_intermediate = questions.clone().into_iter().filter(|x| x.level == 2).collect::<Vec<Question>>();
    let mut questions_advanced = questions.clone().into_iter().filter(|x| x.level == 3).collect::<Vec<Question>>();
    questions_basics.shuffle(&mut rng);
    questions_intermediate.shuffle(&mut rng);
    questions_advanced.shuffle(&mut rng);

    let mut selected_questions = Vec::new();
    selected_questions.extend(questions_basics.into_iter().take(4));
    selected_questions.extend(questions_intermediate.into_iter().take(3));
    selected_questions.extend(questions_advanced.into_iter().take(3));

    

    let mut score = 0;
    let mut counter = 0;
    let stop_index = 10;
    let mut comodin = 0;

    for question in selected_questions {
        println!("\n{}\n1. {}\n2. {}\n3. {}\n4. {}", question.context, question.option_1, question.option_2, question.option_3, question.option_4);
        println!("Nivel: {}", question.level);
        if comodin == 0 {
            println!("¿Deseas usar el comodín 50/50? \n 0. Si \nNo: omite la opción si no deseas usarlo:");
        }
        println!("Ingresa una opción:");
        let answer = read_integer().unwrap();
        
        if answer == 0 && comodin == 0 {
      
            fifty_fifty_comodin(&question);
            println!("Ingresa tu respuesta para el comodin:");
            let answer = read_integer().unwrap();
            comodin += 1;
            
            if answer == question.answer {
                counter += 1;
                score += 1;
                println!("Respuesta correcta! Tu puntaje es: {}", score);
                println!("¿Deseas continuar? \n 1. Si \n 0. No \n Ingresa una opción:");
                let continuegame = read_integer().unwrap();
                if continuegame == 0 {
                    break;
                }
            } else {
                println!("Respuesta incorrecta! Tu puntaje es: {}", score);
                break;
            }
            continue;
        }
        
        if counter == stop_index {
            break;
        }
        if answer == question.answer {
            counter += 1;
            score += 1;
            
            println!("Respuesta correcta! Tu puntaje es: {}", score);
            println!("¿Deseas continuar? \n 1. Si \n 0. No \n Ingresa una opción:");
            let continuegame = read_integer().unwrap();
            if continuegame == 0 {
                break;
            }
        } else {
            println!("Respuesta incorrecta! Tu puntaje es: {}", score);
            break;
        }
    }
    println!("Tu puntaje final es: {}", score);
    
}
fn read_integer() -> Result<i32, std::num::ParseIntError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: i32 = input.trim().parse()?;
    Ok(num)
}

fn fifty_fifty_comodin(question: &Question) {
    let mut rng = thread_rng();
    let mut options = vec![(1, &question.option_1), (2,&question.option_2), (3, &question.option_3),(4, &question.option_4)];
    let correct_option = options.remove((question.answer - 1) as usize);
    options.shuffle(&mut rng);
    options.truncate(1);
    options.push(correct_option);
    options.shuffle(&mut rng);
    for (i, option) in options {
        println!("{}. {}", i , option);
    }
}

// fn read_float() -> Result<f32, std::num::ParseFloatError> {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");

//     let num: f32 = input.trim().parse()?;
//     Ok(num)
// }
// fn add_float(a: f32, b: f32) -> f32 {
//     a + b
// }
// fn sub_float(a: f32, b: f32) -> f32 {
//     a - b
// }
// fn mul_float(a: f32, b: f32) -> f32 {
//     a * b
// }
// fn div_float(a: f32, b: f32) -> f32 {
//     if b==0.0{
//         0.0
//     }else{
//         a / b
//     }
    
// }
// fn operations(){
//     println!("\n Operaciones CalcuRUST \n 1. Suma \n 2. Resta \n 3. Multiplicación \n 4. División \n Ingresa una opción:");
    
//     let secondary = read_integer().unwrap();
//     execution(secondary);
    
// }

// fn execution(secondary:i32){
//     unsafe {
//     let length = RECORDS.len() as i32;
//     let  a: f32;
//     if length > 0{
//         println!("\n 1. Usar un resultado del historial.\n 2. No usar un resultado del historial.\nIngresa una opción: \n");
//         let history_option = read_integer().unwrap();
//         if history_option ==1{
//             view_history();
//             println!("\n  Ingresa el identificador deseado: \n");
//             let record_identifier = read_integer().unwrap();
//             a =get_result_history(record_identifier as usize);
//         }else{
//         println!("\n Ingresa el primer Número:\n");
//         a= read_float().unwrap();
//     }
            
//     }else{
//         println!("\n Ingresa el primer Número:\n");
//         a= read_float().unwrap();
//     }
    
//     println!("\n Ingresa el segundo Número:\n");
//     let  b: f32 = read_float().unwrap();
//     let mut r : f32= 0.0;
//     let mut operation: String = String::new();
    
//     match secondary {
//         1 =>{
//             r=add_float(a,b);
//             operation="+".to_string();
//         },
//         2 =>{
//             r=sub_float(a,b);
//             operation="-".to_string();
//         },
//         3 =>{
//             r=mul_float(a,b);
//             operation="*".to_string();
//         },
//         4 =>{
//             r=div_float(a,b);
//             operation="/".to_string();
//         },
//         _ => println!("El número es otro valor."),
//         }
    
    
//     let calc_record = CalcRecord{
//         operation: operation,
//         first_number: a,
//         second_number: b,
//         result: r,
//         position: length
//     };
//     print_calc_record(&calc_record);
    
//     RECORDS.push(calc_record);
//     }
    
    
// }

// fn print_calc_record(record: &CalcRecord) {
//     println!("{}: {} {} {} = {}", record.position, record.first_number, record.operation, record.second_number, record.result);
// }

// fn view_history(){
//     unsafe{
//         for record in &RECORDS {
//             print_calc_record(&record);
//         }
//     }
// }
// fn get_result_history(identifier:usize)-> f32 {
//     unsafe{
//         RECORDS[identifier].result
//     }
// }