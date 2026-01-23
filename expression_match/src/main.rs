
enum ConnectionState {
    Connected,
    Disconnected,
    Connecting,
}

fn main() {
    
    describe_state(ConnectionState::Connected);
    describe_state(ConnectionState::Disconnected);
    describe_state(ConnectionState::Connecting);

    print_number(Some(24));
    print_number(None);
    

    let tuple = (10, 10);
    
    match tuple {
        (x,y) if x==y => println!("Los dos valores son iguales: {}", x),
        (x,y) if x>y => println!("El primer valor es mayor: {} > {}", x, y),
        (x,y) => println!("El segundo valor es mayor: {} < {}", x, y),
    }

    println!("{}", increment_option( Some(5) ));
    println!("{}", increment_option( None ));
}


fn describe_state(state: ConnectionState){

    match state {
        ConnectionState::Connected => println!("Conexion establecida"),
        ConnectionState::Disconnected => println!("Conexion deshabilitada"),
        ConnectionState::Connecting => println!("Realizando Conexion..."),
    }

}

fn print_number(num: Option<i32>){
    
    match num {
        Some(value) => println!("El numero es: {}", value),
        None => println!("No se ha proporcionado un valor"), // Recordando que None es el equivalente de null
    }

}

fn increment_option(num: Option<i32>) -> i32{

    let result = match num {
        Some(value) => value + 1,
        None => 0,
    };

    result

}