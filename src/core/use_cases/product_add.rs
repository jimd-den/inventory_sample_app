pub trait AddProduct {
    fn add_product(&self, product: &Product) -> Result<(), String>;
}

pub struct AddProductUseCase<R: ProductRepository> {
    repository: R,
} 


//fn for addproduct usecase that takes a repository and returns a new instance of the usecase
impl<R: ProductRepository> AddProductUseCase<R> {
    pub fn new(repository: R) -> Self {
        AddProductUseCase { repository }
    }
}

impl <R: ProductRepository> AddProduct for AddProductUseCase<R> {
    pub fn new(repository: R) -> Self {
    AddProductUseCase 

    }
