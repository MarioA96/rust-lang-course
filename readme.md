# Rust

## Fundamentos

### Variables y mutabilidad
  En Rust, las variables son inmutables por defecto. Esto significa que una vez que se les asigna un valor, no se puede cambiar. Para declarar una variable mutable, se utiliza la palabra clave `mut`.

  ```rust
  let x = 5; // Variable inmutable
  let mut y = 10; // Variable mutable
  y += 5; // Ahora y es 15
  ```
---

### Tipos de datos primitivos
  Los tipos de datos primitivos en Rust incluyen enteros, números de punto flotante, booleanos y caracteres. Estos tipos son inmutables por defecto y tienen un tamaño fijo en memoria.
  
  ```rust
  let a: i32 = 10; // Entero de 32 bits
  let b: f64 = 3.14; // Número de punto flotante de 64 bits
  let c: bool = true; // Booleano
  let d: char = 'R'; // Carácter
  ```
---

### Tipos de datos compuestos
  Los tipos de datos compuestos en Rust incluyen tuplas y arrays. Estos tipos pueden contener múltiples valores y pueden ser mutables si se declaran con `mut`.

  ```rust
  let tup: (i32, f64, char) = (500, 6.4, 'R'); // Tupla
  let arr: [i32; 3] = [1, 2, 3]; // Array
  ```
---

### Diferencias entre primitivos y compuestos en Rust
  - **Primitivos**: Son tipos de datos simples que representan un solo valor, como enteros, flotantes, booleanos y caracteres. Tienen un tamaño fijo y se almacenan en el stack.
  
  - **Compuestos**: Son tipos de datos que pueden contener múltiples valores, como tuplas y arrays. Pueden agrupar diferentes tipos de datos y también se almacenan en el stack si su tamaño es conocido en tiempo de compilación.
---

### Sombreado de Variables
  El sombreado de variables en Rust permite declarar una nueva variable con el mismo nombre que una variable existente. La nueva variable "sombrea" a la anterior, y la anterior ya no es accesible.

  ```rust
  let x = 5;
  let x = x + 1; // Sombreado de la variable x
  {
      let x = x * 2; // Sombreado dentro de un bloque
      println!("El valor de x dentro del bloque es: {}", x); // Imprime 12
  }
  println!("El valor de x fuera del bloque es: {}", x); // Imprime 6
  ```
---

### Diferencias entre el Stack y el Heap

#### **Stack**
  - **Estructura de datos LIFO**
    - El stack es una estructura de datos que sigue el principio de "último en entrar, primero en salir" (LIFO, por sus siglas en inglés). Esto significa que el último elemento agregado al stack es el primero en ser removido.

  - **Almacenamiento de datos** 
    - El stack se utiliza para almacenar datos de tamaño fijo y conocido en tiempo de compilación, como variables locales y tipos de datos primitivos.
  
  - **Acceso rapido**
    - El acceso a los datos en el stack es muy rápido debido a su estructura organizada y la forma en que se gestionan las direcciones de memoria.
  
  - **Gestión automática**
    - La gestión de memoria en el stack es automática; cuando una función termina, todas las variables locales se eliminan del stack.

#### **Heap**
  - **Estructura de datos dinámica**
    - El heap es una estructura de datos que permite la asignación dinámica de memoria. Los datos pueden ser de tamaño variable y no tienen un orden específico de almacenamiento.
  
  - **Almacenamiento de datos**
    - El heap se utiliza para almacenar datos que pueden crecer o cambiar de tamaño durante la ejecución del programa, como estructuras de datos complejas y objetos.
  
  - **Acceso más lento**
    - El acceso a los datos en el heap es generalmente más lento que en el stack debido a la necesidad de buscar la ubicación de memoria y la posible fragmentación.
  
  - **Gestión manual**
    - La gestión de memoria en el heap puede ser manual o automática (a través de un recolector de basura), dependiendo del lenguaje de programación. En Rust, la gestión de memoria se realiza mediante el sistema de propiedad y préstamos.

  - **Fragmentación**
    - El heap puede sufrir fragmentación con el tiempo, lo que puede llevar a un uso ineficiente de la memoria si no se gestiona adecuadamente.

#### **Diferencias clave entre el Stack y el Heap**
  | Característica          | Stack                              | Heap                               |
  |------------------------|-----------------------------------|-----------------------------------|
  | Estructura             | LIFO                              | Dinámica                          |
  | Almacenamiento         | Datos de tamaño fijo              | Datos de tamaño variable          |
  | Acceso                 | Rápido                            | Más lento                         |
  | Gestión de memoria     | Automática                        | Manual o automática               |
  | Fragmentación          | No sufre fragmentación            | Puede sufrir fragmentación        |
---

### Prestamos de Variables
  En Rust, los préstamos permiten a una variable prestar su valor a otra variable sin transferir la propiedad. Hay dos tipos de préstamos: préstamos inmutables y préstamos mutables.

  ```rust
  let s1 = String::from("hello");
  let len = calculate_length(&s1); // Préstamo inmutable

  fn calculate_length(s: &String) -> usize {
      s.len()
  }

  let mut s2 = String::from("hello");
  change(&mut s2); // Préstamo mutable

  fn change(s: &mut String) {
      s.push_str(", world");
  }
  ```
---

### Operadores y Expresiones

---
