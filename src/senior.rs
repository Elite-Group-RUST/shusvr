use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Product {
    #[expect(dead_code)]
    name: String,
    price: f64,
    quantity: u32,
}

struct Store {
    products: HashMap<String, Product>,
}

impl Store {
    fn new() -> Store {
        Store {
            products: HashMap::new(),
        }
    }

    fn add_product(&mut self, name: String, price: f64, quantity: u32) -> Result<(), &'static str> {
        // Реализуйте добавление товара с проверкой корректности данных

        if self.products.contains_key(&name) {
            return Err("Product with the same name already exists");
        }

        let old = self.products.insert(
            name.clone(),
            Product {
                name,
                price,
                quantity,
            },
        );
        assert!(old.is_none());

        Ok(())
    }

    fn update_product(
        &mut self,
        name: &str,
        new_price: Option<f64>,
        new_quantity: Option<u32>,
    ) -> Result<(), &'static str> {
        // Реализуйте обновление данных о товаре с проверкой наличия товара

        let product = self
            .products
            .get_mut(name)
            .ok_or("Product with this name does not exist")?;
        if let Some(value) = new_price {
            product.price = value;
        }

        if let Some(value) = new_quantity {
            product.quantity = value;
        }

        Ok(())
    }

    fn find_product(&self, name: &str) -> Option<&Product> {
        // Реализуйте поиск товара по названию
        self.products.get(name)
    }

    fn remove_product(&mut self, name: &str) -> Result<(), &'static str> {
        // Реализуйте удаление товара с проверкой наличия товара

        self.products
            .remove(name)
            .map(|_| ())
            .ok_or("Product with this name does not exist")
    }
}

fn main() {
    let mut store = Store::new();

    // Попытка добавить товар
    match store.add_product("Laptop".to_string(), 1000.0, 10) {
        Ok(_) => println!("Product added successfully"),
        Err(e) => println!("Error adding product: {}", e),
    }

    // Попытка добавить товар
    match store.add_product("Laptop".to_string(), 1000.0, 10) {
        Ok(_) => println!("Product added successfully"),
        Err(e) => println!("Error adding product: {}", e),
    }

    // Обновление товара
    match store.update_product("Laptop", Some(900.0), None) {
        Ok(_) => println!("Product updated successfully"),
        Err(e) => println!("Error updating product: {}", e),
    }

    // Обновление товара
    match store.update_product("Stuff", Some(900.0), None) {
        Ok(_) => println!("Product updated successfully"),
        Err(e) => println!("Error updating product: {}", e),
    }

    // Поиск товара
    if let Some(product) = store.find_product("Laptop") {
        println!("Found product: {:?}", product);
    } else {
        println!("Product not found");
    }

    // Поиск товара
    if let Some(product) = store.find_product("Stuff") {
        println!("Found product: {:?}", product);
    } else {
        println!("Product `Stuff` not found");
    }

    // Удаление товара
    match store.remove_product("Laptop") {
        Ok(_) => println!("Product removed successfully"),
        Err(e) => println!("Error removing product: {}", e),
    }

    // Удаление товара
    match store.remove_product("Laptop") {
        Ok(_) => println!("Product removed successfully"),
        Err(e) => println!("Error removing product: {}", e),
    }
}
