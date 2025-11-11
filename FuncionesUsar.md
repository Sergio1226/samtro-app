### Funciones a usar para el front
Dentro del archivo `src-tauri\src\commands.rs` estan los comandos a usar para el frontend.

#### Login(Entry) -> Result<LoginResponse, String>
El Entry hace referencia un JSON que debería tener dos valores, estos son: username y password.

El método devuelve un Json como respuesta con tres valores:
```
{
    success: bool,
    message: String,
    username: String,
}
```

#### add_product(product) -> Result<DefaultResponse, String>
Pide un producto en el siguiente formato para ser aceptado 
```
Product: {
    code: String,
    name: String,
    category: String,
    price: int,
    stock: int,
    minimum: int,
    description: String
}
```
Una vez aceptado la función va a retornar un response default que consiste de solo un valor succes booleano