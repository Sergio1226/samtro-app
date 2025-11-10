use crate::models::product::{Product};
use super::db;
use crate::errors::AppError;
use rusqlite::{Result, params};

// funcion para insertar producto nuevo
pub fn insert_product(product: &Product) -> Result<()> {
    // Conecto con la base de datos
    let conn = db::get_connection()?;

    // inserto en la db el producto
    conn.execute("INSERT INTO products (code, name, category, price, stock, minimum, description) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    params![product.code, product.name, product.category, product.price, product.stock, product.minimum, product.description])?;

    Ok(())
}

// Busco producto por código
pub fn find_product_by_code(code: String) -> Result<Option<Product>, rusqlite::Error>{
    // Establezco conexion con la db
    let conn = db::get_connection()?;
    // Hago la consulta de los datos
    let mut stmt = conn.prepare("SELECT code, name, category, price, stock, minimum, description FROM products WHERE code = ?1")?;
    let mut rows = stmt.query(params![code])?;

    // En caso de no estar vacio, retornar en formato de objeto
    if let Some(row) = rows.next()? {
        Ok(Some(Product{
            code: row.get(0)?,
            name: row.get(1)?,
            category: row.get(2)?,
            price: row.get(3)?,
            stock: row.get(4)?,
            minimum: row.get(5)?,
            description: row.get(6)?,
        }))
    }else{
        Ok(None)
    }
}