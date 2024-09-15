fn main(){

    let mut nuevo_usuario = String :: from("");
    println!("identifiquese");
    std::io::stdin().read_line(&mut nuevo_usuario).unwrap();
    se_identifica(nuevo_usuario.clone());

fn se_identifica(x : String){
    println!("bienvenido {}",x);}
}
