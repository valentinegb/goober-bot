use dotenvy::dotenv_iter;

fn main() {
    println!("cargo::rerun-if-changed=.env");

    match dotenv_iter() {
        Err(err) if !err.not_found() => {
            println!("cargo::error=could not load `.env` file: {err}")
        }
        Ok(vars) => {
            for item in vars {
                match item {
                    Err(err) => {
                        println!("cargo::error=could not load variable in `.env` file: {err}")
                    }
                    Ok((var, value)) => println!("cargo::rustc-env={var}={value}"),
                }
            }
        }
        _ => { /* noop */ }
    }
}
