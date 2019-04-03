/// Controller
use core_lib::catalog::Catalog;

pub struct Widgets {
    catalog: Catalog,
}

impl Widgets {
    pub fn new(catalog: Catalog) -> Self {
        Widgets { catalog }
    }
    pub fn document_number_pure(&self) -> String {
        format!(
            "
        <div class=\"document_number\">
          Document number is: {}
        </div>",
            self.catalog.get_documents_number()
        )
    }
}
